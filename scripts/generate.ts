import { program } from "commander";
import { SpawnOptions, spawn } from "node:child_process";
import fs from "node:fs/promises";
import path from "node:path";
import { finished } from "node:stream/promises";
import { rimraf } from "rimraf";

import { generateRosMsg, generateRosMsgDefinition } from "../typescript/schemas/src/internal";
import { exportTypeScriptSchemas } from "../typescript/schemas/src/internal/exportTypeScriptSchemas";
import {
  BYTE_VECTOR_FB,
  DURATION_FB,
  TIME_FB,
  generateFlatbuffers,
} from "../typescript/schemas/src/internal/generateFlatbufferSchema";
import { generateJsonSchema } from "../typescript/schemas/src/internal/generateJsonSchema";
import { generateMarkdown } from "../typescript/schemas/src/internal/generateMarkdown";
import {
  DURATION_IDL,
  TIME_IDL,
  generateOmgIdl,
} from "../typescript/schemas/src/internal/generateOmgIdl";
import { generateProto } from "../typescript/schemas/src/internal/generateProto";
import {
  generateSchemaModuleRegistration,
  generateSchemaPrelude,
  generatePyclass,
  generatePySchemaStub,
  generateTimeTypes,
  generateChannelClasses,
  generatePyChannelStub,
  generatePySchemaModule,
  generatePyChannelModule,
} from "../typescript/schemas/src/internal/generatePyclass";
import {
  foxgloveEnumSchemas,
  foxgloveMessageSchemas,
} from "../typescript/schemas/src/internal/schemas";

async function logProgress(message: string, body: () => Promise<void>) {
  process.stderr.write(`${message}... `);
  await body();
  process.stderr.write("done\n");
}

async function logProgressLn(message: string, body: () => Promise<void>) {
  process.stderr.write(`${message}...\n`);
  await body();
  process.stderr.write("done\n");
}

async function exec(command: string, args: string[], { cwd }: Pick<SpawnOptions, "cwd">) {
  process.stderr.write(`  ==> ${command} ${args.join(" ")}\n`);

  await new Promise<void>((resolve, reject) => {
    const child = spawn(command, args, {
      stdio: "inherit",
      cwd,
    });

    child.on("close", (code: number) => {
      if (code !== 0) {
        const fullCommand = `${command} ${args.join(" ")}`;
        console.error(`Command failed: \`${fullCommand}\``);
        reject(new Error(`${command} failed with exit code ${code}`));
      } else {
        resolve();
      }
    });
  });
}

async function main({ clean }: { clean: boolean }) {
  const repoRoot = path.resolve(__dirname, "..");
  const outDir = path.join(repoRoot, "schemas");
  const rosOutDir = path.join(repoRoot, "ros_foxglove_msgs");
  const typescriptTypesDir = path.join(repoRoot, "typescript/schemas/src/types");

  const pythonSdkRoot = path.resolve(repoRoot, "python", "foxglove-sdk");
  const pythonSdkGeneratedRoot = path.join(pythonSdkRoot, "src", "generated");
  const pythonSdkPyRoot = path.join(pythonSdkRoot, "python/foxglove");

  await logProgress("Removing existing output directories", async () => {
    await rimraf(outDir);
    await rimraf(path.join(rosOutDir, "ros1"));
    await rimraf(path.join(rosOutDir, "ros2"));
    await rimraf(typescriptTypesDir);
    await rimraf(path.join(repoRoot, "rust/foxglove/src/schemas"));
    await rimraf(pythonSdkGeneratedRoot);
    await rimraf(path.join(pythonSdkPyRoot, "_foxglove_py/schemas.pyi"));
    await rimraf(path.join(pythonSdkPyRoot, "schemas/__init__.py"));
    await rimraf(path.join(pythonSdkPyRoot, "_foxglove_py/channels.pyi"));
    await rimraf(path.join(pythonSdkPyRoot, "channels/__init__.py"));
  });

  if (clean) {
    // we're all done here
    return;
  }

  await logProgress("Generating JSONSchema definitions", async () => {
    await fs.mkdir(path.join(outDir, "jsonschema"), { recursive: true });
    let indexTS = "// Generated by https://github.com/foxglove/foxglove-sdk\n\n";
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      const json = JSON.stringify(generateJsonSchema(schema), undefined, 2);
      await fs.writeFile(path.join(outDir, "jsonschema", `${schema.name}.json`), json + "\n");
      indexTS += `export const ${schema.name} = ${json};\n\n`;
    }
    await fs.writeFile(path.join(outDir, "jsonschema", `index.ts`), indexTS);
  });

  await logProgress("Generating ROS 1 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros1"), { recursive: true });
    await fs.mkdir(path.join(rosOutDir, "ros1"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      const msg = generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 1 }), {
        rosVersion: 1,
      });
      await fs.writeFile(path.join(outDir, "ros1", `${schema.name}.msg`), msg);
      await fs.writeFile(path.join(rosOutDir, "ros1", `${schema.name}.msg`), msg);
    }
  });

  await logProgress("Generating ROS 2 msg files", async () => {
    await fs.mkdir(path.join(outDir, "ros2"), { recursive: true });
    await fs.mkdir(path.join(rosOutDir, "ros2"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      if (schema.rosEquivalent != undefined) {
        continue;
      }
      const msg = generateRosMsg(generateRosMsgDefinition(schema, { rosVersion: 2 }), {
        rosVersion: 2,
      });
      await fs.writeFile(path.join(outDir, "ros2", `${schema.name}.msg`), msg);
      await fs.writeFile(path.join(rosOutDir, "ros2", `${schema.name}.msg`), msg);
    }
  });

  await logProgress("Generating Protobuf definitions", async () => {
    await fs.mkdir(path.join(outDir, "proto", "foxglove"), { recursive: true });
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      const enums = Object.values(foxgloveEnumSchemas).filter(
        (enumSchema) => enumSchema.parentSchemaName === schema.name,
      );
      await fs.writeFile(
        path.join(outDir, "proto", "foxglove", `${schema.name}.proto`),
        generateProto(schema, enums),
      );
    }
  });

  await logProgress("Generating FlatBuffer definitions", async () => {
    await fs.mkdir(path.join(outDir, "flatbuffer"), { recursive: true });
    await fs.writeFile(path.join(outDir, "flatbuffer", "ByteVector.fbs"), BYTE_VECTOR_FB);
    await fs.writeFile(path.join(outDir, "flatbuffer", "Time.fbs"), TIME_FB);
    await fs.writeFile(path.join(outDir, "flatbuffer", "Duration.fbs"), DURATION_FB);

    for (const schema of Object.values(foxgloveMessageSchemas)) {
      // want enums with their corresponding parent tables for usage
      const enums = Object.values(foxgloveEnumSchemas).filter(
        (enumSchema) => enumSchema.parentSchemaName === schema.name,
      );
      await fs.writeFile(
        path.join(outDir, "flatbuffer", `${schema.name}.fbs`),
        generateFlatbuffers(schema, enums),
      );
    }
  });

  await logProgress("Generating TypeScript definitions", async () => {
    await fs.mkdir(typescriptTypesDir, { recursive: true });

    const schemas = exportTypeScriptSchemas();
    for (const [name, source] of schemas.entries()) {
      await fs.writeFile(path.join(typescriptTypesDir, `${name}.ts`), source);
    }
  });

  await logProgress("Generating OMG IDL definitions", async () => {
    await fs.mkdir(path.join(outDir, "omgidl", "foxglove"), { recursive: true });
    await fs.writeFile(path.join(outDir, "omgidl", "foxglove", "Time.idl"), TIME_IDL);
    await fs.writeFile(path.join(outDir, "omgidl", "foxglove", "Duration.idl"), DURATION_IDL);
    for (const schema of Object.values(foxgloveMessageSchemas)) {
      await fs.writeFile(
        path.join(outDir, "omgidl", "foxglove", `${schema.name}.idl`),
        generateOmgIdl(schema),
      );
    }
    for (const schema of Object.values(foxgloveEnumSchemas)) {
      await fs.writeFile(
        path.join(outDir, "omgidl", "foxglove", `${schema.name}.idl`),
        generateOmgIdl(schema),
      );
    }
  });

  await logProgress("Generating README.md", async () => {
    await fs.writeFile(
      path.join(outDir, "README.md"),
      generateMarkdown(Object.values(foxgloveMessageSchemas), Object.values(foxgloveEnumSchemas)),
    );
  });

  // This must run before generating the Pyclass definitions
  await logProgressLn("Generating Rust code", async () => {
    await exec("cargo", ["run", "--bin", "foxglove-proto-gen"], {
      cwd: path.join(repoRoot, "rust"),
    });
  });

  // Generate schemas and supporting source for the Foxglove SDK
  // These are exported to the SDK directory, and not stored with general-purpose schemas.
  // Requires rust and python dependencies to be installed.
  await logProgressLn("Generating Pyclass definitions", async () => {
    // Source files (.rs) are re-generated.
    // Stub file is placed into the existing hierarchy.
    const schemasFile = path.join(pythonSdkGeneratedRoot, "schemas.rs");
    await fs.mkdir(pythonSdkGeneratedRoot, { recursive: true });
    await fs.mkdir(path.join(pythonSdkPyRoot, "schemas"), { recursive: true });
    await fs.mkdir(path.join(pythonSdkPyRoot, "channels"), { recursive: true });

    // Schemas file
    const writer = (await fs.open(schemasFile, "wx")).createWriteStream();
    writer.write(generateSchemaPrelude());

    const enumSchemas = Object.values(foxgloveEnumSchemas);
    for (const enumSchema of enumSchemas) {
      writer.write(generatePyclass(enumSchema));
    }

    writer.write(generateTimeTypes());

    const messageSchemas = Object.values(foxgloveMessageSchemas);
    for (const schema of messageSchemas) {
      writer.write(generatePyclass(schema));
    }

    const allSchemas = [...enumSchemas, ...messageSchemas];

    writer.write(generateSchemaModuleRegistration(allSchemas));
    writer.end();
    await finished(writer);

    const channelClassesFile = path.join(pythonSdkGeneratedRoot, "channels.rs");
    await fs.writeFile(channelClassesFile, generateChannelClasses(messageSchemas));

    // Stubs are written to the location of the pyo3-generated module
    // Python module indexes are added for the public API.
    const schemasStubFile = path.join(pythonSdkPyRoot, "_foxglove_py/schemas.pyi");
    const schemasStubModule = path.join(pythonSdkPyRoot, "schemas/__init__.py");
    const channelStubFile = path.join(pythonSdkPyRoot, "_foxglove_py/channels.pyi");
    const channelStubModule = path.join(pythonSdkPyRoot, "channels/__init__.py");

    await fs.writeFile(schemasStubFile, generatePySchemaStub(allSchemas));
    await fs.writeFile(schemasStubModule, generatePySchemaModule(allSchemas));
    await fs.writeFile(channelStubFile, generatePyChannelStub(messageSchemas));
    await fs.writeFile(channelStubModule, generatePyChannelModule(messageSchemas));

    await exec("cargo", ["fmt", "--", path.resolve(channelClassesFile, schemasFile)], {
      cwd: repoRoot,
    });

    const pythonFiles = [
      path.resolve(schemasStubFile),
      path.resolve(channelStubFile),
      path.resolve(schemasStubModule),
      path.resolve(channelStubModule),
    ];
    await exec("poetry", ["run", "black", ...pythonFiles], { cwd: repoRoot });
    await exec("poetry", ["run", "isort", ...pythonFiles], { cwd: repoRoot });
  });

  await logProgressLn("Updating Jest snapshots", async () => {
    await exec("yarn", ["test", "--updateSnapshot"], {
      cwd: repoRoot,
    });
  });
}

program.option("--clean", "remove all generated files");
program.action(main);
program.parseAsync().catch(console.error);
