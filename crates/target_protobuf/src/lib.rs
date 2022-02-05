use jtd_codegen::target::{self, inflect, metadata};
use jtd_codegen::Result;
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

lazy_static! {
    static ref KEYWORDS: BTreeSet<String> = include_str!("keywords")
        .lines()
        .map(str::to_owned)
        .collect();
    static ref TYPE_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::CombiningInflector::new(inflect::Case::pascal_case())
        ));
    static ref ENUM_MEMBER_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
    static ref FIELD_NAMING_CONVENTION: Box<dyn inflect::Inflector + Send + Sync> =
        Box::new(inflect::KeywordAvoidingInflector::new(
            KEYWORDS.clone(),
            inflect::TailInflector::new(inflect::Case::pascal_case())
        ));
}
pub struct Target {}

impl Target {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct FileState {
    last_field_number: usize,
}

impl jtd_codegen::target::Target for Target {
    type FileState = FileState;

    fn strategy(&self) -> target::Strategy {
        target::Strategy {
            file_partitioning: target::FilePartitioningStrategy::SingleFile("index.proto".into()),
            enum_member_naming: target::EnumMemberNamingStrategy::Modularized,
            optional_property_handling: target::OptionalPropertyHandlingStrategy::NativeSupport,
            booleans_are_nullable: false,
            int8s_are_nullable: false,
            uint8s_are_nullable: false,
            int16s_are_nullable: false,
            uint16s_are_nullable: false,
            int32s_are_nullable: false,
            uint32s_are_nullable: false,
            float32s_are_nullable: false,
            float64s_are_nullable: false,
            strings_are_nullable: false,
            timestamps_are_nullable: false,
            arrays_are_nullable: false,
            dicts_are_nullable: false,
            aliases_are_nullable: false,
            enums_are_nullable: false,
            structs_are_nullable: false,
            discriminators_are_nullable: false,
        }
    }

    fn name(&self, kind: target::NameableKind, parts: &[String]) -> String {
        match kind {
            target::NameableKind::Type => TYPE_NAMING_CONVENTION.inflect(parts),
            target::NameableKind::EnumMember => ENUM_MEMBER_NAMING_CONVENTION.inflect(parts),

            // Not used. TypeScript maps directly to the JSON data, so we don't
            // have the option of distinguishing the JSON name from the
            // TypeScript name
            target::NameableKind::Field => FIELD_NAMING_CONVENTION.inflect(parts),
        }
    }

    fn expr(
        &self,
        state: &mut Self::FileState,
        metadata: metadata::Metadata,
        expr: target::Expr,
    ) -> String {
        match expr {
            target::Expr::String => "string".into(),
            target::Expr::Boolean => "bool".into(),
            target::Expr::Float32 => "float".into(),
            target::Expr::Float64 => "double".into(),
            target::Expr::Uint32 => "uint32".into(),
            target::Expr::Uint16 => "uint32".into(),
            target::Expr::Uint8 => "uint32".into(),
            target::Expr::Int32 => "int32".into(),
            target::Expr::Int16 => "int32".into(),
            target::Expr::Int8 => "int32".into(),
            // Extremely conveniently: Both the typed json spec and the protobuf spec define
            // timestamps to be strings as per RFC 3339
            target::Expr::Timestamp => "string".into(),
            target::Expr::NullableOf(t) => t,
            target::Expr::ArrayOf(t) => format!("repeated {}", t),
            target::Expr::DictOf(t) => format!("map<string, {}>", t),
            _ => todo!("{:?}", expr),
        }
    }

    fn item(
        &self,
        out: &mut dyn Write,
        state: &mut Self::FileState,
        item: target::Item,
    ) -> Result<Option<String>> {
        let item = match item {
            // Protobuf does not natively support type aliases, so this is quite tricky
            target::Item::Alias {
                metadata,
                name,
                type_
            } => {
                dbg!(&name, &type_);
                if name == "Root" {
                    write!(out, "\t{}", description(&metadata, 1))?;
                    writeln!(out, "{} root = {};", type_, 1)?;
                    None
                } else {
                    unimplemented!()
                }
            },
            target::Item::Struct {
                metadata,
                name,
                has_additional: _,
                fields,
            } => {
                writeln!(out)?;
                write!(out, "\t{}", description(&metadata, 1))?;
                writeln!(out, "message {} {{", name)?;

                for (index, field) in fields.iter().enumerate() {
                    let description = description(&field.metadata, 1);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }

                    write!(out, "\t{}", &description)?;
                    // in protobuf fields are always optional
                    // @note: Maybe we can figure out a way to retain metadata about required fields for later use?
                    writeln!(out, "\t{} {} = {};", field.type_, field.json_name.clone(), index + 1)?;
                }

                writeln!(out, "\t}}")?;

                if name == "Root" {
                    state.last_field_number += 1;
                    writeln!(out, "\t{} root = {};", name, state.last_field_number)?;
                }
                
                None
            },
            target::Item::Enum {
                metadata,
                name,
                members,
            } => {
                writeln!(out)?;
                write!(out, "\t{}", description(&metadata, 1))?;
                writeln!(out, "enum {} {{", name)?;
                for (index, member) in members.iter().enumerate() {
                    let description = enum_variant_description(&metadata, 2, &member.json_value);

                    if index != 0 && !description.is_empty() {
                        writeln!(out)?;
                    }

                    write!(out, "\t{}", &description)?;
                    writeln!(out, "\t{} = {};", member.name, index)?;
                }
                writeln!(out, "\t}}")?;

                if &name == "Root" {
                    // Enums as the root item mean we want that enum to have an instance in the message
                    state.last_field_number += 1;
                    writeln!(out, "\t{} root = {};", name, state.last_field_number)?;
                }

                None
            }
            target::Item::Preamble => {
                writeln!(
                    out,
                    "// Code generated by jtd-codegen for ProtoBuffers v{}",
                    env!("CARGO_PKG_VERSION")
                )?;
                writeln!(out, r#"syntax = "proto3";"#)?;

                writeln!(out)?;
                writeln!(out, "message {} {{", "RootMessage")?;
                None
            }
            target::Item::Postamble => {
                writeln!(out, "}}")?;
                None
            }
            target::Item::Auxiliary { .. } => None,
            _ => todo!("{:?}", item),
        };

        Ok(item)
    }
}

pub fn description(metadata: &BTreeMap<String, Value>, indent: usize) -> String {
    doc(indent, jtd_codegen::target::metadata::description(metadata))
}

pub fn enum_variant_description(
    metadata: &BTreeMap<String, Value>,
    indent: usize,
    value: &str,
) -> String {
    doc(
        indent,
        jtd_codegen::target::metadata::enum_variant_description(metadata, value),
    )
}

fn doc(ident: usize, s: &str) -> String {
    let prefix = "  ".repeat(ident);
    jtd_codegen::target::fmt::comment_block(
        &format!("{}/**", prefix),
        &format!("{} * ", prefix),
        &format!("{} */", prefix),
        s,
    )
}

#[cfg(test)]
mod tests {
    mod std_tests {
        jtd_codegen_test::std_test_cases!(&crate::Target::new());
    }

    mod optional_std_tests {
        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_properties
        );

        jtd_codegen_test::strict_std_test_case!(
            &crate::Target::new(),
            empty_and_nonascii_enum_values
        );
    }
}
