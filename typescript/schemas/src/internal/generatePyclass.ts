import {
  FoxgloveEnumSchema,
  FoxgloveMessageField,
  FoxgloveMessageSchema,
  FoxglovePrimitive,
  FoxgloveSchema,
} from "./types";

/**
 * Generate the module header for schema pyclass definitions.
 */
export function generateSchemaPrelude(): string {
  const docs = [
    `//! Definitions for well-known Foxglove schemas`,
    `//! Generated by https://github.com/foxglove/foxglove-sdk`,
    // Accept any number of arguments for Python constructors
    `#![allow(clippy::too_many_arguments)]`,
    // Match Foxglove enum conventions elsewhere (shared prefix)
    `#![allow(clippy::enum_variant_names)]`,
    // Allow capitalized single-letter enum variants
    `#![allow(non_snake_case)]`,
  ];

  const imports = ["use pyo3::{prelude::*, types::PyBytes};"];

  const outputSections = [docs.join("\n"), imports.join("\n")];

  return outputSections.join("\n") + "\n\n";
}

/**
 * Generate a `pyclass`-annotated struct or enum definition for the given schema.
 */
export function generatePyclass(schema: FoxgloveSchema): string {
  return isMessageSchema(schema) ? generateMessageClass(schema) : generateEnumClass(schema);
}

/**
 * Generate a .pyi stub for the given schemas.
 */
export function generatePySchemaStub(schemas: FoxgloveSchema[]): string {
  const header = [
    "# Generated by https://github.com/foxglove/foxglove-sdk",
    "from enum import Enum",
    "from typing import List, Optional",
  ].join("\n") + "\n";

  const timeTypes = generateTimeTypeStubs();

  const enums = schemas
    .filter((schema) => schema.type === "enum")
    .map((schema) => {
      const name = enumName(schema);
      const doc = ['    """', `    ${schema.description}`, '    """'];
      const values = schema.values.map((value) => {
        return `    ${constantToTitleCase(value.name)} = ${value.value}`;
      });
      return {
        name,
        source: [`class ${name}(Enum):`, ...doc, ...values].join("\n") + "\n\n",
      };
    });

  const classes = schemas.filter(isMessageSchema).map((schema) => {
    const name = structName(schema.name);
    const doc = ['    """', `    ${schema.description}`, '    """'];
    const params = schema.fields
      .map((field) => {
        return `        ${field.name}: "${pythonCtorType(field)}" = ${pythonDefaultValue(field)}`;
      })
      .join(",\n");

    return {
      name,
      source: [
        `class ${name}:`,
        ...doc,
        `    def __new__(`,
        "        cls,",
        "        *,",
        params,
        `    ) -> "${name}": ...`,
      ].join("\n") + "\n\n",
    };
  });

  // Enums come first to provide default values for constructor parameters. Otherwise, sort by name.
  const enumSources = enums.sort((a, b) => a.name.localeCompare(b.name)).map(({ source }) => source);
  const classSources = [...classes, ...timeTypes]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(({ source }) => source);

  const enumMarker = '#\n# Enums\n#\n';
  const classMarker = '#\n# Classes\n#\n';

  return [header, enumMarker, ...enumSources, classMarker, ...classSources].join("\n");
}

/**
 * Generate a module __init__ file re-exporting all schemas to the public API.
 */
export function generatePySchemaModule(schemas: FoxgloveSchema[]): string {
  const classNames = schemas.map((schema) => pyClassName(schema)).concat(["Timestamp", "Duration"]).sort();
  const headers = [
    `"""`,
    `This module contains the definitions of the well-known Foxglove schemas for logging.`,
    ``,
    "Log messages to a corresponding channel type from :py:mod:`foxglove.channels`.",
    `"""`,
    "# Generated by https://github.com/foxglove/foxglove-sdk"
  ];
  const imports = classNames.map((name) =>  `from foxglove._foxglove_py.schemas import ${name}`);
  const exports = [
    "__all__ = [",
    ...classNames.map((name) => `    "${name}",`),
    "]"
  ];

  return [...headers, ...imports, ...exports].join("\n") + "\n";
}

function rustDoc(str: string, opts: { indent?: number } = {}): string {
  const ws = " ".repeat(opts.indent ?? 0);
  return str
    .split("\n")
    .map((line) => `${ws}/// ${line}`)
    .join("\n");
}

function firstLine(str: string): string {
  return str.split("\n")[0]?.trim() ?? str;
}

function generateMessageClass(schema: FoxgloveMessageSchema): string {
  const className = structName(schema.name);
  const schemaFields = Array.from(schema.fields).map((field) => ({
    fieldName: safeName(field.name),
    argName: safeName(field.name),
    // description: rustDoc(field.description, { indent: 4 }),
    field,
  }));
  const struct = [
    rustDoc(schema.description),
    `///`,
    ...schemaFields.map(({ fieldName, field }) =>
      rustDoc(`:param ${fieldName}: ${firstLine(field.description)}`),
    ),
    `///`,
    `/// See https://docs.foxglove.dev/docs/visualization/message-schemas/${constantToKebabCase(className)}`,
    `#[pyclass(module = "foxglove.schemas")]`,
    `#[derive(Clone)]`,
    `pub(crate) struct ${className}(pub(crate) foxglove::schemas::${className});`,
  ];

  function fieldValue(field: FoxgloveMessageField): string {
    if (field.type.type === "primitive" && field.type.name === "bytes") {
      // Special case — this is an `Option<Bound<'_, PyBytes>>`; see `rustOutputType`
      return `data.map(|x| x.as_bytes().to_vec()).unwrap_or_default()`;
    }
    switch (field.type.type) {
      case "primitive":
        if (field.type.name === "time" || field.type.name === "duration") {
          return `${safeName(field.name)}.map(Into::into)`;
        }
        return safeName(field.name);
      case "nested":
        if (field.array != undefined) {
          return `${safeName(field.name)}.into_iter().map(|x| x.into()).collect()`;
        }
        return `${safeName(field.name)}.map(Into::into)`;
      case "enum":
        return `${safeName(field.name)} as i32`;
    }
  }

  function fieldAssignment(field: FoxgloveMessageField): string {
    const name = protoName(field.name);
    const value = fieldValue(field);
    if (name === value) {
      return name;
    }
    return `${name}: ${value}`;
  }

  const signature = schemaFields.map(({ argName, field }) => `${argName}=${rustDefaultValue(field)}`).join(", ");

  const impl = [
    `#[pymethods]`,
    `impl ${className} {`,
    `    #[new]`,
    `    #[pyo3(signature = (*, ${signature}) )]`,
    `    fn new(`,
    ...schemaFields.map(({ argName, field }) => `        ${argName}: ${rustOutputType(field)},`),
    `    ) -> Self {`,
    `        Self(foxglove::schemas::${className} {`,
    schemaFields.map(({ field }) => `            ${fieldAssignment(field)},`).join("\n"),
    `        })`,
    `    }`,
    `    fn __repr__(&self) -> String {`,
    `        format!(`,
    `            "${className}(${schemaFields.map(({ argName }) => `${argName}={:?}`).join(", ")})",`,
    schemaFields.map(({ fieldName }) => `            self.0.${protoName(fieldName)},`).join("\n"),
    `        )`,
    `    }`,
    `}\n\n`,
  ];

  const fromTrait = [
    `impl From<${structName(schema.name)}> for foxglove::schemas::${structName(schema.name)} {`,
    `    fn from(value: ${structName(schema.name)}) -> Self {`,
    `        value.0`,
    `    }`,
    `}\n\n`,
  ];

  return [...struct, ...impl, ...fromTrait].join("\n");
}

function generateEnumClass(schema: FoxgloveEnumSchema): string {
  const enumLines = [
    rustDoc(schema.description),
    `#[pyclass(eq, eq_int, module = "foxglove.schemas")]`,
    `#[derive(PartialEq, Clone)]`,
    `pub(crate) enum ${enumName(schema)} {`,
    ...schema.values.map((value) => `    ${constantToTitleCase(value.name)} = ${value.value},`),
    "}\n\n",
  ];

  return enumLines.join("\n");
}

/**
 * For enums with parent schemas, prepend the parent schema name to the enum name,
 * removing duplicated prefixes.
 */
function enumName(schema: FoxgloveEnumSchema): string {
  const name = schema.name.replace(new RegExp("^" + schema.parentSchemaName), "");
  return `${schema.parentSchemaName}${name}`;
}

/**
 * Deal with reserved keywords in identifiers
 */
function safeName(name: string): string {
  if (name === "type") {
    return "r#type";
  }
  return name;
}

/**
 * A schema is either a message schema or an enum.
 */
function isMessageSchema(schema: FoxgloveSchema): schema is FoxgloveMessageSchema {
  return schema.type === "message";
}

/**
 * Get the rust type for a field.
 * Types are assumed to be owned, and wrapped in a `Vec` if the field is an array.
 * Nested types are optional, unless the field is an array.
 */
function rustOutputType(field: FoxgloveMessageField): string {
  const isVec = field.array != undefined;
  let type: string;
  switch (field.type.type) {
    case "primitive":
      switch (field.type.name) {
        case "string":
          type = "String";
          break;
        case "float64":
          type = "f64";
          break;
        case "uint32":
          type = "u32";
          break;
        case "boolean":
          type = "bool";
          break;
        case "bytes":
          // Special case: we don't take a Vec<u8> directly because pyo3 will iterate the vec and
          // copy each element https://github.com/PyO3/pyo3/issues/2888
          return "Option<Bound<'_, PyBytes>>";
        case "time":
          type = "Option<Timestamp>";
          break;
        case "duration":
          type = "Option<Duration>";
          break;
      }
      break;
    case "nested":
      // Don't wrap in an optional if part of a Vec
      type = isVec ? field.type.schema.name : `Option<${field.type.schema.name}>`;
      break;
    case "enum":
      type = enumName(field.type.enum);
      break;
  }

  return isVec ? `Vec<${type}>` : type;
}

/**
 * Get the Python type for a constructor parameter.
 * All types are optional.
 */
function pythonCtorType(field: FoxgloveMessageField): string {
  let type: string;
  switch (field.type.type) {
    case "primitive":
      type = pythonType(field.type.name);
      break;
    case "nested":
      type = field.type.schema.name;
      break;
    case "enum":
      type = enumName(field.type.enum);
      break;
  }
  return field.array != undefined ? `Optional[List[${type}]]` : `Optional[${type}]`;
}

/**
 * Get the Python default for a constructor parameter
 */
function pythonDefaultValue(field: FoxgloveMessageField): string {
  if (field.array != undefined) {
    return "[]";
  }
  switch (field.type.type) {
    case "primitive":
      switch (field.type.name) {
        case "string":
          return `""`;
        case "float64":
          return "0.0";
        case "uint32":
          return "0";
        case "boolean":
          return "False";
        case "bytes":
          return 'b""';
        case "time":
        case "duration":
          return "None";
      }
      // exhaustive check above
      // eslint-disable-next-line no-fallthrough
    case "nested":
      return "None";
    case "enum": {
      const value = constantToTitleCase(field.type.enum.values[0]!.name);
      return `${enumName(field.type.enum)}.${value}`;
    }
  }
}

/**
 * Get the Rust default for a field; used in pyo3 constructor signatures.
 */
function rustDefaultValue(field: FoxgloveMessageField): string {
  if (field.type.type === "primitive" && field.type.name === "bytes") {
    // Special case — this is an `Option<Bound<'_, PyBytes>>`; see `rustOutputType`
    return "None";
  }
  if (field.array != undefined) {
    return "vec![]";
  }
  switch (field.type.type) {
    case "primitive":
      switch (field.type.name) {
        case "string":
          return `"".to_string()`;
        case "float64":
          return "0.0";
        case "uint32":
          return "0";
        case "boolean":
          return "false";
        case "bytes":
          return "vec![]";
        case "time":
        case "duration":
          return "None";
      }
      // exhaustive check above
      // eslint-disable-next-line no-fallthrough
    case "nested":
      return "None";
    case "enum": {
      const value = constantToTitleCase(field.type.enum.values[0]!.name);
      return `${enumName(field.type.enum)}::${value}`;
    }
  }
}

/**
 * Map Foxglove primitive types to Python primitives.
 */
function pythonType(foxglovePrimitive: FoxglovePrimitive): string {
  switch (foxglovePrimitive) {
    case "string":
      return "str";
    case "float64":
      return "float";
    case "uint32":
      return "int";
    case "boolean":
      return "bool";
    case "bytes":
      return "bytes";
    case "time":
      return "Timestamp";
    case "duration":
      return "Duration";
  }
}

function protoName(name: string): string {
  if (/^[A-Z]$/.exec(name)) {
    // Schemas may include single-letter capitals; generated proto structs use lowercase
    return name.toLowerCase();
  }
  return safeName(name);
}

function capitalize(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function constantToTitleCase(str: string): string {
  return str
    .split("_")
    .map((word) => word.toLowerCase())
    .map(capitalize)
    .join("");
}

function constantToKebabCase(name: string): string {
  return name
    .replace(/^[A-Z]/, (match) => match.toLowerCase())
    .replace(/([A-Z])/g, (match) => `-${match.toLowerCase()}`);
}

function structName(name: string): string {
  // Match special case handling in protobuf gen
  if (name === "GeoJSON") {
    return "GeoJson";
  }
  return name;
}

/**
 * .pyi stubs for Timestamp and Duration.
 */
function generateTimeTypeStubs(): { name: string, source: string }[] {
  const timestamp = `
class Timestamp:
    """
    A timestamp in seconds and nanoseconds
    """
    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Timestamp": ...
`;

  const duration = `
class Duration:
    """
    A duration in seconds and nanoseconds
    """
    def __new__(
        cls,
        sec: int,
        nsec: Optional[int] = None,
    ) -> "Duration": ...
`;

  return [{ name: "Timestamp", source: timestamp }, { name: "Duration", source: duration }];
}

/**
 * Defines a struct for representing Timestamp and Duration.
 *
 * This also provides a `From` implementation into prost types for proto serialization.
 */
export function generateTimeTypes(): string {
  return `
/// A timestamp in seconds and nanoseconds
///
/// :param sec: The number of seconds since a user-defined epoch.
/// :param nsec: The number of nanoseconds since the :py:attr:\`sec\` value.
#[pyclass(module = "foxglove.schemas")]
#[derive(Clone)]
pub struct Timestamp(pub(crate) foxglove::schemas::Timestamp);

#[pymethods]
impl Timestamp {
    #[new]
    #[pyo3(signature = (sec, nsec=None))]
    fn new(sec: u32, nsec: Option<u32>) -> Self {
        let nsec = nsec.unwrap_or(0);
        Self(foxglove::schemas::Timestamp{ sec, nsec })
    }

    fn __repr__(&self) -> String {
        format!("Timestamp(sec={}, nsec={})", self.0.sec, self.0.nsec).to_string()
    }
}

impl From<Timestamp> for foxglove::schemas::Timestamp {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

/// A duration, composed of seconds and nanoseconds
///
/// :param sec: The number of seconds in the duration.
/// :param nsec: The number of nanoseconds in the positive direction.
#[pyclass(module = "foxglove.schemas")]
#[derive(Clone)]
pub struct Duration(pub(crate) foxglove::schemas::Duration);

#[pymethods]
impl Duration {
    #[new]
    #[pyo3(signature = (sec, nsec=None))]
    fn new(sec: i32, nsec: Option<u32>) -> Self {
        let nsec = nsec.unwrap_or(0);
        Self(foxglove::schemas::Duration{ sec, nsec })
    }

    fn __repr__(&self) -> String {
      format!("Duration(sec={}, nsec={})", self.0.sec, self.0.nsec).to_string()
    }
}

impl From<Duration> for foxglove::schemas::Duration {
    fn from(value: Duration) -> Self {
        value.0
    }
}
`;
}

/**
 * Generate a rust function to register the schemas in a submodule.
 * https://pyo3.rs/v0.23.4/module.html
 */
export function generateSchemaModuleRegistration(schemas: FoxgloveSchema[]): string {
  return `
pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "schemas")?;

    module.add_class::<Duration>()?;
    module.add_class::<Timestamp>()?;
    ${schemas.map((schema) => `module.add_class::<${pyClassName(schema)}>()?;`).join("\n    ")}

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.schemas", &module)?;

    parent_module.add_submodule(&module)
}
`;
}

function shouldGenerateChannelClass(schema: FoxgloveMessageSchema): boolean {
  return !schema.name.endsWith("Primitive");
}

/**
 * Generate a rust function to register the channels in a submodule.
 * https://pyo3.rs/v0.23.4/module.html
 */
function generateChannelModuleRegistration(messageSchemas: FoxgloveMessageSchema[]): string {
  const schemas = messageSchemas.filter(shouldGenerateChannelClass);
  return `
pub fn register_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new(parent_module.py(), "channels")?;

    ${schemas.map((schema) => `module.add_class::<${structName(schema.name)}Channel>()?;`).join("\n    ")}

    // Define as a package
    // https://github.com/PyO3/pyo3/issues/759
    let py = parent_module.py();
    py.import("sys")?
        .getattr("modules")?
        .set_item("foxglove._foxglove_py.channels", &module)?;

    parent_module.add_submodule(&module)
}
`;
}

function pyClassName(schema: FoxgloveSchema): string {
  return isMessageSchema(schema) ? structName(schema.name) : enumName(schema);
}

/**
 * Generate a concrete `pyclass`-annotated Channel struct for each message schema, since generics
 * can't be exported directly to Python.
 */
export function generateChannelClasses(messageSchemas: FoxgloveMessageSchema[]): string {
  const schemas = messageSchemas.filter((schema) => !schema.name.endsWith("Primitive"));

  const imports = [
    `use foxglove::{TypedChannel, PartialMetadata};`,
    `use pyo3::prelude::*;`,
    `use crate::errors::PyFoxgloveError;`,
    `use super::schemas;`,
  ].join("\n");

  const channelModuleRegistration = generateChannelModuleRegistration(schemas);

  const classes = schemas.map((schema) => {
    const schemaClass = structName(schema.name);
    const channelClass = `${schemaClass}Channel`;
    return `
/// A channel for logging :py:class:\`foxglove.schemas.${schemaClass}\` messages.
#[pyclass(module = "foxglove.channels")]
struct ${channelClass}(TypedChannel<foxglove::schemas::${schemaClass}>);

#[pymethods]
impl ${channelClass} {
    /// Create a new channel.
    ///
    /// :param topic: The topic to log messages to.
    #[new]
    fn new(topic: &str) -> PyResult<Self> {
        let base = TypedChannel::new(topic).map_err(PyFoxgloveError::from)?;
        Ok(Self(base))
    }

    /// Log a :py:class:\`foxglove.schemas.${schemaClass}\` message to the channel.
    ///
    /// :param msg: The message to log.
    /// :param log_time: The log time is the time, as nanoseconds from the unix epoch, that the
    ///     message was recorded. Usually this is the time log() is called. If omitted, the
    ///     current time is used.
    /// :param publish_time: The publish_time is the time at which the message was published. e.g.
    ///     the timestamp at which the sensor reading was taken. If omitted, log time is used.
    /// :param sequence: The sequence number is unique per channel and allows for ordering of
    ///     messages as well as detecting missing messages. If omitted, a monotonically increasing
    ///     sequence number unique to the channel is used.
    #[pyo3(signature = (msg, *, log_time=None, publish_time=None, sequence=None))]
    fn log(
        &self,
        msg: &schemas::${schemaClass},
        log_time: Option<u64>,
        publish_time: Option<u64>,
        sequence: Option<u32>,
    ) {
        let metadata = PartialMetadata{ log_time, publish_time, sequence };
        self.0.log_with_meta(&msg.0, metadata);
    }

    fn __repr__(&self) -> String {
        format!("${channelClass}(topic='{}')", self.0.topic()).to_string()
    }
}
`;
  });

  return [imports, channelModuleRegistration, ...classes].join("\n\n");
}

/**
 * Generate a .pyi stub for the given schemas.
 */
export function generatePyChannelStub(messageSchemas: FoxgloveMessageSchema[]): string {
  const header = "# Generated by https://github.com/foxglove/foxglove-sdk";
  const schemas = messageSchemas.filter((schema) => !schema.name.endsWith("Primitive"));

  const imports = [
    ...schemas.map((schema) =>  `from .schemas import ${structName(schema.name)}`),
  ];

  const classes = schemas.map((schema) => {
    const schemaClass = structName(schema.name);
    const channelClass = `${schemaClass}Channel`;
    const doc = ['    """', `    A channel for logging ${schemaClass} messages`, '    """'];

    return {
      name: channelClass,
      source: [
        `class ${channelClass}:`,
        ...doc,
        `    def __new__(`,
        `        cls,`,
        `        topic: str,`,
        `    ) -> "${channelClass}": ...\n`,
        `    def log(`,
        `        self,`,
        `        message: "${schemaClass}",`,
        `        log_time: int | None = None,`,
        `        publish_time: int | None = None,`,
        `        sequence: int | None = None,`,
        `    ) -> None: ...\n`,
      ].join("\n"),
    };
  });

  const definitions = [...classes]
    .sort((a, b) => a.name.localeCompare(b.name))
    .map(({ source }) => source);

  return [header, ...imports, ...definitions].join("\n");
}

/**
 * Generate a module __init__ file re-exporting all channels to the public API.
 */
export function generatePyChannelModule(schemas: FoxgloveMessageSchema[]): string {
  const headers = [
    `"""`,
    `This defines channels to easily log messages conforming to well-known Foxglove schemas.`,
    ``,
    "See the :py:mod:`foxglove.schemas` module for available definitions.",
    `"""`,
    "# Generated by https://github.com/foxglove/foxglove-sdk"
  ];
  const classNames = schemas.filter(shouldGenerateChannelClass).map((schema) => `${structName(schema.name)}Channel`).sort();
  const imports = classNames.map((name) =>  `from foxglove._foxglove_py.channels import ${name}`);
  const exports = [
    "__all__ = [",
    ...classNames.map((name) => `    "${name}",`),
    "]"
  ];

  return [...headers, ...imports, ...exports].join("\n") + "\n";
}
