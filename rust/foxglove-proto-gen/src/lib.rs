use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::{self, rename, File},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use prost::Message;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use tempfile::NamedTempFile;
use walkdir::WalkDir;

/// Recursively builds a file descriptor set for a file descriptor and its dependencies.
fn build_fds(
    fd: &FileDescriptorProto,
    fd_map: &HashMap<String, &FileDescriptorProto>,
) -> FileDescriptorSet {
    let mut fds = FileDescriptorSet::default();
    let mut seen: HashSet<String> = HashSet::new();
    build_fds_inner(fd, fd_map, &mut fds, &mut seen);
    fds
}

/// Recursive step for `build_fds`.
fn build_fds_inner(
    fd: &FileDescriptorProto,
    fd_map: &HashMap<String, &FileDescriptorProto>,
    fds: &mut FileDescriptorSet,
    seen: &mut HashSet<String>,
) {
    let mut dependencies = fd.dependency.iter().map(|d| d.as_str()).collect::<Vec<_>>();
    dependencies.sort_unstable();
    for name in dependencies {
        if seen.insert(name.to_string()) {
            if let Some(dep_fd) = fd_map.get(name) {
                build_fds_inner(dep_fd, fd_map, fds, seen);
            }
        }
    }
    fds.file.push(FileDescriptorProto {
        source_code_info: None,
        ..fd.clone()
    });
}

/// Helper function to convert `CamelCase` to `CONSTANT_CASE`.
fn camel_case_to_constant_case(camel: &str) -> String {
    let mut output = String::new();
    let mut first = true;
    let mut upper_count = 0;
    for c in camel.chars() {
        if c.is_ascii_uppercase() {
            if !first && upper_count == 0 {
                output.push('_');
            }
            output.push(c);
            upper_count += 1;
        } else {
            if upper_count > 1 {
                output.insert(output.len() - 1, '_');
            }
            output.extend(c.to_uppercase());
            upper_count = 0;
        }
        first = false;
    }
    output
}

/// Generates base64-encoded file descriptor sets for each foxglove message.
fn generate_descriptors(out_dir: &Path, fds: &FileDescriptorSet) -> anyhow::Result<()> {
    let fd_map: HashMap<_, _> = fds
        .file
        .iter()
        .filter_map(|f| f.name.as_ref().map(|n| (n.clone(), f)))
        .collect();

    let descr_dir = out_dir.join("data");
    if descr_dir.exists() {
        fs::remove_dir_all(&descr_dir).context("Failed to remove descriptor directory")?;
    }
    fs::create_dir_all(&descr_dir).context("Failed to create descriptor directory")?;

    let mut descr_map = BTreeMap::new();
    for fd in &fds.file {
        if let Some(name) = fd
            .name
            .as_ref()
            .and_then(|n| n.strip_prefix("foxglove/"))
            .and_then(|n| n.strip_suffix(".proto"))
            .filter(|n| !n.ends_with("Primitive"))
        {
            let file_name = format!("{name}.bin");
            let var_name = camel_case_to_constant_case(name);
            let path = descr_dir.join(&file_name);
            let mut descr_file = File::create(&path).context("Failed to create descriptor file")?;
            let bin = build_fds(fd, &fd_map).encode_to_vec();
            descr_file
                .write_all(&bin)
                .context("Failed to write descriptor")?;
            descr_map.insert(var_name, file_name);
        }
    }

    let mut module =
        File::create(out_dir.join("descriptors.rs")).context("Failed to create descriptors.rs")?;

    writeln!(module, "// This file is @generated by foxglove-proto-gen")
        .context("Failed to write descriptors.rs")?;

    for (var_name, file_name) in descr_map {
        writeln!(
            module,
            "pub const {var_name}: &[u8] = include_bytes!(\"data/{file_name}\");"
        )
        .context("Failed to write descirptors.rs")?;
    }

    Ok(())
}

fn generate_impls(out_dir: &Path, fds: &FileDescriptorSet) -> anyhow::Result<()> {
    let mut module = File::create(out_dir.join("impls.rs")).context("Failed to create impls.rs")?;

    let mut result = writeln!(module, "// This file is @generated by foxglove-proto-gen");
    result = result.and(writeln!(
        module,
        "use crate::schemas::{{descriptors, foxglove::*}};"
    ));
    result = result.and(writeln!(module, "use crate::{{Schema, Encode}};"));
    result = result.and(writeln!(module, "use bytes::BufMut;"));
    result.context("Failed to write impls.rs")?;

    for fd in &fds.file {
        let Some(mut name) = fd
            .name
            .as_ref()
            .and_then(|n| n.strip_prefix("foxglove/"))
            .and_then(|n| n.strip_suffix(".proto"))
            .filter(|n| !n.ends_with("Primitive"))
        else {
            continue;
        };
        // Special case for GeoJSON casing
        if name == "GeoJSON" {
            name = "GeoJson";
        }
        let descriptor_name = camel_case_to_constant_case(name);
        writeln!(
            module,
            "\nimpl Encode for {name} {{
    type Error = ::prost::EncodeError;

    fn get_schema() -> Option<Schema> {{
        Some(Schema::new(
            \"foxglove.{name}\",
            \"protobuf\",
            descriptors::{descriptor_name},
        ))
    }}

    fn get_message_encoding() -> String {{
        \"protobuf\".to_string()
    }}

    fn encode(&self, buf: &mut impl BufMut) -> Result<(), prost::EncodeError> {{
        ::prost::Message::encode(self, buf)
    }}

    fn encoded_len(&self) -> Option<usize> {{ Some(::prost::Message::encoded_len(self)) }}
}}"
        )
        .context("Failed to write trait impl in impls.rs")?;
    }

    Ok(())
}

/// Generates protobuf structs and descriptors.
pub fn generate_protos(proto_path: &Path, out_dir: &Path) -> anyhow::Result<()> {
    let proto_path = fs::canonicalize(proto_path).context("Failed to canonicalize proto path")?;

    if let Err(err) = fs::create_dir(out_dir) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            panic!("Failed to create directory: {}", err);
        }
    }

    let mut proto_files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(&proto_path) {
        let entry = entry.expect("Failed to read entry");
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().is_some_and(|ext| ext == "proto") {
            proto_files.push(entry.path().to_path_buf());
        }
    }

    let mut config = prost_build::Config::new();
    config.extern_path(".google.protobuf.Duration", "crate::schemas::Duration");
    config.extern_path(".google.protobuf.Timestamp", "crate::schemas::Timestamp");
    config.out_dir(out_dir);
    config.bytes(["."]);

    let mut fds = config
        .load_fds(&proto_files, &[proto_path])
        .context("Failed to load protos")?;
    fds.file.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    generate_descriptors(out_dir, &fds).context("Failed to generate descriptor files")?;

    generate_impls(out_dir, &fds).context("Failed to generate impls")?;

    config
        .compile_fds(fds)
        .context("Failed to compile protos")?;

    fix_generated_comments(out_dir).context("Failed to fix docstrings")?;

    Ok(())
}

/// Convert all documentation code blocks to text to avoid errors when running doc tests (hack)
fn fix_generated_comments(out_dir: &Path) -> anyhow::Result<()> {
    let schema_path = out_dir.join("foxglove.rs");

    let mut tmpfile = NamedTempFile::new_in(out_dir).context("Failed to create tempfile")?;
    let input = File::open(schema_path.clone()).context("Failed to open schema file")?;
    let input = io::BufReader::new(input).lines();
    let mut in_code_block = false;

    for line in input {
        let mut line = line.context("Failed to read line")?;
        if line.trim_start().eq("/// ```") {
            if !in_code_block {
                line = format!("{line}text");
            }
            in_code_block = !in_code_block;
        } else if in_code_block {
            // Protoc turns this:
            //
            // ```
            //     [a 0 0]
            // P = [0 b 0]
            //     [0 0 c]
            // ```
            //
            // Into this:
            //
            // ```
            //      \[a 0 0\]
            // P = \[0 b 0\]
            //      \[0 0 c\]
            // ```
            //
            // Remove the escapes, and the extra space added to lines that begin with whitespace.
            line = line.replace('\\', "");
            line = line.replace("///  ", "/// ")
        }
        writeln!(tmpfile, "{line}").context("Failed to write to output file")?;
    }

    rename(tmpfile.path(), schema_path).context("Failed to rename tempfile")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_camel_case_to_constant_case() {
        let cases = [
            ("A", "A"),
            ("a", "A"),
            ("Abc", "ABC"),
            ("abc", "ABC"),
            ("ABC", "ABC"),
            ("AbcDef", "ABC_DEF"),
            ("abcDef", "ABC_DEF"),
            ("abcdef", "ABCDEF"),
            ("AbcDEF", "ABC_DEF"),
            ("ABCDef", "ABC_DEF"),
            ("ABCDEF", "ABCDEF"),
        ];
        for (input, output) in cases {
            dbg!(input, output);
            assert_eq!(camel_case_to_constant_case(input), output);
        }
    }
}
