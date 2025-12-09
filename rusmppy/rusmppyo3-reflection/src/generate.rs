// Copyright (c) Facebook, Inc. and its affiliates
// SPDX-License-Identifier: MIT OR Apache-2.0

// Modified version of https://github.com/zefchain/serde-reflection/blob/311c5de12b7f88630a8fa10ab2acc10905364903/serde-generate/src/rust.rs
// This file was modified to do the following
// - Generate rust structs and enums annotated with `#[pyo3::pyclass]` and `#[pyo3_stub_gen_derive]`
// - Generate enums empty variants like `Nothing()` instead of `Nothing`, since `#[pyo3::pyclass]` does not support empty variants.
// - Generate `impl From` for Rusmpp types for all generated types.

use serde_generate::{
    CodeGeneratorConfig, analyzer,
    indent::{IndentConfig, IndentedWriter},
};
use serde_reflection::{ContainerFormat, Format, Named, Registry, VariantFormat};
use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap, HashSet},
    io::{Result, Write},
    sync::OnceLock,
};

const NON_EXHAUSTIVE_RUSMPP_ENUMS: &[&str] = &[
    "TlvTag",
    "TlvValue",
    "BroadcastRequestTlvValue",
    "BroadcastResponseTlvValue",
    "CancelBroadcastTlvValue",
    "MessageDeliveryRequestTlvValue",
    "MessageDeliveryResponseTlvValue",
    "MessageSubmissionRequestTlvValue",
    "MessageSubmissionResponseTlvValue",
    "QueryBroadcastResponseTlvValue",
];
const NON_DEFAULT_RUSMPP_TYPES: &[&str] = &[
    "CommandId",
    "DistributionListName",
    "SmeAddress",
    "DestAddress",
    "Pdu",
    "Tlv",
    "TlvTag",
    "TlvValue",
    "BroadcastRequestTlvValue",
    "BroadcastResponseTlvValue",
    "CancelBroadcastTlvValue",
    "MessageDeliveryRequestTlvValue",
    "MessageDeliveryResponseTlvValue",
    "MessageSubmissionRequestTlvValue",
    "MessageSubmissionResponseTlvValue",
    "QueryBroadcastResponseTlvValue",
];

fn new_py_signature(name: &str) -> &'static str {
    static CUSTOM_NEW_PY_SIGNATURE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

    CUSTOM_NEW_PY_SIGNATURE
        .get_or_init(|| {
            let mut m = HashMap::new();
            m.insert(
                "EsmClass",
                "#[pyo3(signature=(
        messaging_mode=crate::generated::MessagingMode::default_(),
        message_type=crate::generated::MessageType::default_(),
        ansi41_specific=crate::generated::Ansi41Specific::default_(),
        gsm_features=crate::generated::GsmFeatures::default_()
    ))]",
            );

            m.insert(
                "RegisteredDelivery",
                "#[pyo3(signature=(
        mc_delivery_receipt=crate::generated::MCDeliveryReceipt::default_(),
        sme_originated_acknowledgement=crate::generated::SmeOriginatedAcknowledgement::default_(),
        intermediate_notification=crate::generated::IntermediateNotification::default_(),
        other=0
    ))]",
            );

            m
        })
        .get(name)
        .copied()
        .unwrap_or("")
}

fn py_additional_methods(name: &str) -> &'static str {
    static METHODS: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

    METHODS
        .get_or_init(|| {
            let mut m = HashMap::new();

            m.insert(
                "RegisteredDelivery",
                "#[classmethod]
                #[pyo3(signature=())]
                pub fn request_all<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {
                    Self::from(rusmpp_types::RegisteredDelivery::request_all())
                }",
            );

            m
        })
        .get(name)
        .copied()
        .unwrap_or("")
}

/// Issue: [Rusmpp #102](https://github.com/Rusmpp/Rusmpp/issues/102)
fn gen_stub_pymethods(name: &str) -> &'static str {
    static STUB_GEN: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

    STUB_GEN
        .get_or_init(|| {
            let mut m = HashMap::new();
            m.insert(
                "EsmClass",
                "#[cfg_attr(not(any(PyPy, GraalPy)), ::pyo3_stub_gen_derive::gen_stub_pymethods)]",
            );
            m.insert(
                "RegisteredDelivery",
                "#[cfg_attr(not(any(PyPy, GraalPy)), ::pyo3_stub_gen_derive::gen_stub_pymethods)]",
            );
            m
        })
        .get(name)
        .copied()
        .unwrap_or("#[::pyo3_stub_gen_derive::gen_stub_pymethods]")
}

/// Main configuration object for code-generation in Rust.
pub struct CodeGenerator<'a> {
    /// Language-independent configuration.
    config: &'a CodeGeneratorConfig,
    /// Which derive macros should be added (independently from serialization).
    derive_macros: Vec<String>,
    /// Additional block of text added before each new container definition.
    custom_derive_block: Option<String>,
    /// Whether definitions and fields should be marked as `pub`.
    track_visibility: bool,
}

/// Shared state for the code generation of a Rust source file.
struct RustEmitter<'a, T> {
    /// Writer.
    out: IndentedWriter<T>,
    /// Generator.
    generator: &'a CodeGenerator<'a>,
    /// Track which definitions have a known size. (Used to add `Box` types.)
    known_sizes: Cow<'a, HashSet<&'a str>>,
    /// Current namespace (e.g. vec!["my_package", "my_module", "MyClass"])
    current_namespace: Vec<String>,
}

impl<'a> CodeGenerator<'a> {
    /// Create a Rust code generator for the given config.
    pub fn new(config: &'a CodeGeneratorConfig) -> Self {
        Self {
            config,
            derive_macros: vec!["Clone", "Debug", "PartialEq", "PartialOrd"]
                .into_iter()
                .map(String::from)
                .collect(),
            custom_derive_block: None,
            track_visibility: true,
        }
    }

    /// Which derive macros should be added (independently from serialization).
    pub fn with_derive_macros(mut self, derive_macros: Vec<String>) -> Self {
        self.derive_macros = derive_macros;
        self
    }

    /// Additional block of text added after `derive_macros` (if any), before each new
    /// container definition.
    pub fn with_custom_derive_block(mut self, custom_derive_block: Option<String>) -> Self {
        self.custom_derive_block = custom_derive_block;
        self
    }

    /// Whether definitions and fields should be marked as `pub`.
    pub fn with_track_visibility(mut self, track_visibility: bool) -> Self {
        self.track_visibility = track_visibility;
        self
    }

    /// Write container definitions in Rust.
    pub fn output(
        &self,
        out: &mut dyn Write,
        registry: &Registry,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let external_names = self
            .config
            .external_definitions
            .values()
            .flatten()
            .cloned()
            .collect();
        let dependencies =
            analyzer::get_dependency_map_with_external_dependencies(registry, &external_names)?;
        let entries = analyzer::best_effort_topological_sort(&dependencies);

        let known_sizes = external_names
            .iter()
            .map(<String as std::ops::Deref>::deref)
            .collect::<HashSet<_>>();

        let current_namespace = self
            .config
            .module_name
            .split('.')
            .map(String::from)
            .collect();
        let mut emitter = RustEmitter {
            out: IndentedWriter::new(out, IndentConfig::Space(4)),
            generator: self,
            known_sizes: Cow::Owned(known_sizes),
            current_namespace,
        };

        emitter.output_preamble()?;
        for name in entries {
            let format = &registry[name];
            emitter.output_container(name, format)?;
            emitter.known_sizes.to_mut().insert(name);
        }

        emitter.output_add_classes(registry)?;

        Ok(())
    }

    /// For each container, generate a Rust definition.
    pub fn quote_container_definitions(
        &self,
        registry: &Registry,
    ) -> std::result::Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
        let dependencies = analyzer::get_dependency_map(registry)?;
        let entries = analyzer::best_effort_topological_sort(&dependencies);

        let mut result = BTreeMap::new();
        let mut known_sizes = HashSet::new();
        let current_namespace = self
            .config
            .module_name
            .split('.')
            .map(String::from)
            .collect::<Vec<_>>();

        for name in entries {
            let mut content = Vec::new();
            {
                let mut emitter = RustEmitter {
                    out: IndentedWriter::new(&mut content, IndentConfig::Space(4)),
                    generator: self,
                    known_sizes: Cow::Borrowed(&known_sizes),
                    current_namespace: current_namespace.clone(),
                };
                let format = &registry[name];
                emitter.output_container(name, format)?;
            }
            known_sizes.insert(name);
            result.insert(
                name.to_string(),
                String::from_utf8_lossy(&content).trim().to_string() + "\n",
            );
        }
        Ok(result)
    }
}

impl<'a, T> RustEmitter<'a, T>
where
    T: std::io::Write,
{
    fn output_comment(&mut self, name: &str) -> std::io::Result<()> {
        let mut path = self.current_namespace.clone();
        path.push(name.to_string());
        if let Some(doc) = self.generator.config.comments.get(&path) {
            let text = textwrap::indent(doc, "/// ").replace("\n\n", "\n///\n");
            write!(self.out, "\n{text}")?;
        }
        Ok(())
    }

    fn output_custom_code(&mut self, name: &str) -> std::io::Result<()> {
        let mut path = self.current_namespace.clone();
        path.push(name.to_string());
        if let Some(code) = self.generator.config.custom_code.get(&path) {
            write!(self.out, "\n{code}")?;
        }
        Ok(())
    }

    fn output_preamble(&mut self) -> Result<()> {
        let external_names = self
            .generator
            .config
            .external_definitions
            .values()
            .flatten()
            .cloned()
            .collect::<HashSet<_>>();

        writeln!(self.out, "#![allow(clippy::enum_variant_names)]")?;
        writeln!(self.out, "#![allow(clippy::useless_conversion)]")?;
        writeln!(self.out, "#![allow(clippy::too_many_arguments)]")?;

        writeln!(self.out)?;

        if self.generator.config.serialization {
            writeln!(self.out, "use serde::{{Serialize, Deserialize}};")?;
        }

        if self.generator.config.serialization && !external_names.contains("Bytes") {
            writeln!(self.out, "use serde_bytes::ByteBuf as Bytes;")?;
        }

        for (module, definitions) in &self.generator.config.external_definitions {
            // Skip the empty module name.
            if !module.is_empty() {
                writeln!(
                    self.out,
                    "use {}::{{{}}};",
                    module,
                    definitions.to_vec().join(", "),
                )?;
            }
        }

        writeln!(self.out)?;

        // We add a module here called rusmpp_types. inside of this module we reexport all rusmpp types.
        // When we generate the 'impl From', we use `rusmpp_types::` prefix.
        writeln!(self.out, "pub mod rusmpp_types {{")?;

        self.out.indent();

        writeln!(
            self.out,
            "pub use ::rusmpp::{{pdus::*,tlvs::*, values::*, Command, CommandId, CommandStatus, Pdu}};"
        )?;

        self.out.unindent();

        writeln!(self.out, "}}")?;

        writeln!(self.out)?;

        Ok(())
    }

    fn quote_type(format: &Format, known_sizes: Option<&HashSet<&str>>) -> String {
        use Format::*;
        match format {
            TypeName(x) => {
                if let Some(set) = known_sizes
                    && !set.contains(x.as_str())
                {
                    return format!("Box<{x}>");
                }
                x.to_string()
            }
            Unit => "()".into(),
            Bool => "bool".into(),
            I8 => "i8".into(),
            I16 => "i16".into(),
            I32 => "i32".into(),
            I64 => "i64".into(),
            I128 => "i128".into(),
            U8 => "u8".into(),
            U16 => "u16".into(),
            U32 => "u32".into(),
            U64 => "u64".into(),
            U128 => "u128".into(),
            F32 => "f32".into(),
            F64 => "f64".into(),
            Char => "char".into(),
            Str => "String".into(),
            // Use Vec<u8> instead of bytes
            Bytes => "Vec<u8>".into(),

            Option(format) => format!("Option<{}>", Self::quote_type(format, known_sizes)),
            Seq(format) => format!("Vec<{}>", Self::quote_type(format, None)),
            Map { key, value } => format!(
                "Map<{}, {}>",
                Self::quote_type(key, None),
                Self::quote_type(value, None)
            ),
            Tuple(formats) => format!("({})", Self::quote_types(formats, known_sizes)),
            TupleArray { content, size } => {
                format!("[{}; {}]", Self::quote_type(content, known_sizes), *size)
            }

            Variable(_) => panic!("unexpected value"),
        }
    }

    fn quote_types(formats: &[Format], known_sizes: Option<&HashSet<&str>>) -> String {
        formats
            .iter()
            .map(|x| Self::quote_type(x, known_sizes))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn output_fields(&mut self, base: &[&str], fields: &[Named<Format>]) -> Result<()> {
        // Do not add 'pub' within variants.
        let prefix = if base.len() <= 1 && self.generator.track_visibility {
            "pub "
        } else {
            ""
        };
        for field in fields {
            self.output_comment(&field.name)?;
            writeln!(
                self.out,
                "{}{}: {},",
                prefix,
                field.name,
                Self::quote_type(&field.value, Some(&self.known_sizes)),
            )?;
        }
        Ok(())
    }

    fn output_variant(&mut self, base: &str, name: &str, variant: &VariantFormat) -> Result<()> {
        self.output_comment(name)?;
        use VariantFormat::*;
        match variant {
            Unit => writeln!(self.out, "{name}(),"),
            NewType(format) => writeln!(
                self.out,
                "{}({}),",
                name,
                Self::quote_type(format, Some(&self.known_sizes))
            ),
            Tuple(formats) => writeln!(
                self.out,
                "{}({}),",
                name,
                Self::quote_types(formats, Some(&self.known_sizes))
            ),
            Struct(fields) => {
                writeln!(self.out, "{name} {{")?;
                self.current_namespace.push(name.to_string());
                self.out.indent();
                self.output_fields(&[base, name], fields)?;
                self.out.unindent();
                self.current_namespace.pop();
                writeln!(self.out, "}},")
            }
            Variable(_) => panic!("incorrect value"),
        }
    }

    fn output_variants(
        &mut self,
        base: &str,
        variants: &BTreeMap<u32, Named<VariantFormat>>,
    ) -> Result<()> {
        for (expected_index, (index, variant)) in variants.iter().enumerate() {
            assert_eq!(*index, expected_index as u32);
            self.output_variant(base, &variant.name, &variant.value)?;
        }
        Ok(())
    }

    fn output_container(&mut self, name: &str, format: &ContainerFormat) -> Result<()> {
        self.output_comment(name)?;
        let mut derive_macros = self.generator.derive_macros.clone();
        if self.generator.config.serialization {
            derive_macros.push("Serialize".to_string());
            derive_macros.push("Deserialize".to_string());
        }
        let mut prefix = String::new();
        if !derive_macros.is_empty() {
            prefix.push_str(&format!("#[derive({})]\n", derive_macros.join(", ")));
        }
        if let Some(text) = &self.generator.custom_derive_block {
            prefix.push_str(text);
            prefix.push('\n');
        }
        if self.generator.track_visibility {
            prefix.push_str("pub ");
        }

        use ContainerFormat::*;
        match format {
            UnitStruct => writeln!(self.out, "{prefix}struct {name};\n")?,
            NewTypeStruct(format) => writeln!(
                self.out,
                "{}struct {}({}{});\n",
                prefix,
                name,
                if self.generator.track_visibility {
                    "pub "
                } else {
                    ""
                },
                Self::quote_type(format, Some(&self.known_sizes))
            )?,
            TupleStruct(formats) => writeln!(
                self.out,
                "{}struct {}({});\n",
                prefix,
                name,
                Self::quote_types(formats, Some(&self.known_sizes))
            )?,
            Struct(fields) => {
                writeln!(self.out, "#[::pyo3_stub_gen_derive::gen_stub_pyclass]")?;
                writeln!(self.out, "{prefix}struct {name} {{")?;
                self.current_namespace.push(name.to_string());
                self.out.indent();
                self.output_fields(&[name], fields)?;
                self.out.unindent();
                self.current_namespace.pop();
                writeln!(self.out, "}}\n")?;

                // Implement `From` for Rusmpp types
                writeln!(self.out, "impl From<rusmpp_types::{name}> for {name} {{")?;
                self.out.indent();
                writeln!(self.out, "fn from(value: rusmpp_types::{name}) -> Self {{")?;
                self.out.indent();
                writeln!(self.out, "let value = value.into_parts();")?;
                writeln!(self.out, "Self {{")?;
                self.out.indent();

                for field in fields {
                    let field_name = &field.name;
                    let conversion = match &field.value {
                        Format::Option(_) => {
                            format!("value.{field_name}.map(Into::into)")
                        }
                        Format::Seq(inner_format) => {
                            // If its a sequence of bytes we can use `Into` directly.
                            if matches!(**inner_format, Format::U8) {
                                format!("value.{field_name}.into()")
                            } else {
                                format!("value.{field_name}.into_iter().map(Into::into).collect()")
                            }
                        }
                        _ => {
                            format!("value.{field_name}.into()")
                        }
                    };

                    writeln!(self.out, "{field_name}: {conversion},")?;
                }

                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}\n")?;

                // default
                self.default_impl(name)?;

                // Generate the methods
                let gen_stub_pymethods = gen_stub_pymethods(name);
                writeln!(self.out, "{gen_stub_pymethods}")?;
                writeln!(self.out, "#[::pyo3::pymethods]")?;
                writeln!(self.out, "impl {name} {{")?;
                self.out.indent();

                // __new__
                writeln!(self.out, "#[new]")?;
                let params: Vec<String> = fields
                    .iter()
                    .map(|f| {
                        format!(
                            "{}: {}",
                            f.name,
                            Self::quote_type(&f.value, Some(&self.known_sizes))
                        )
                    })
                    .collect();
                writeln!(self.out, "{}", new_py_signature(name))?;
                writeln!(self.out, "fn new({}) -> Self {{", params.join(", "))?;
                self.out.indent();
                writeln!(self.out, "Self {{")?;
                self.out.indent();
                for field in fields {
                    writeln!(self.out, "{},", field.name)?;
                }
                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}")?;

                // default
                self.py_default_impl(name)?;

                // additional methods
                let additional_methods = py_additional_methods(name);
                writeln!(self.out, "{additional_methods}")?;

                // __repr__
                writeln!(self.out, "fn __repr__(&self) -> String {{")?;
                self.out.indent();
                writeln!(self.out, "format!(\"{{self:?}}\")")?;
                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}\n")?;
            }
            Enum(variants) => {
                writeln!(
                    self.out,
                    "#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]"
                )?;
                writeln!(self.out, "{prefix}enum {name} {{")?;
                self.current_namespace.push(name.to_string());
                self.out.indent();
                self.output_variants(name, variants)?;
                self.out.unindent();
                self.current_namespace.pop();
                writeln!(self.out, "}}\n")?;

                // Implement `From` for Rusmpp types
                writeln!(self.out, "impl From<rusmpp_types::{name}> for {name} {{")?;
                self.out.indent();
                writeln!(self.out, "fn from(value: rusmpp_types::{name}) -> Self {{")?;
                self.out.indent();
                writeln!(self.out, "match value {{")?;
                self.out.indent();

                for variant in variants.values() {
                    let vname = &variant.name;
                    use VariantFormat::*;
                    match &variant.value {
                        Unit => {
                            writeln!(
                                self.out,
                                "rusmpp_types::{name}::{vname} => {name}::{vname}(),"
                            )?;
                        }
                        NewType(_) => {
                            writeln!(
                                self.out,
                                "rusmpp_types::{name}::{vname}(inner) => {name}::{vname}(inner.into()),"
                            )?;
                        }
                        Tuple(fields) => {
                            let vars: Vec<String> =
                                (0..fields.len()).map(|i| format!("f{i}")).collect();
                            writeln!(
                                self.out,
                                "rusmpp_types::{name}::{vname}({}) => {name}::{vname}({}),",
                                vars.join(", "),
                                vars.iter()
                                    .map(|v| format!("{v}.into()"))
                                    .collect::<Vec<_>>()
                                    .join(", "),
                            )?;
                        }
                        Struct(fields) => {
                            writeln!(self.out, "rusmpp_types::{name}::{vname} {{")?;
                            self.out.indent();
                            for field in fields {
                                writeln!(self.out, "{},", field.name)?;
                            }
                            self.out.unindent();
                            writeln!(self.out, "}} => {name}::{vname} {{")?;
                            self.out.indent();
                            for field in fields {
                                writeln!(self.out, "{}: {}.into(),", field.name, field.name)?;
                            }
                            self.out.unindent();
                            writeln!(self.out, "}},")?;
                        }
                        Variable(_) => panic!("unexpected Variable variant"),
                    }
                }

                // handle the _ case for NON_EXHAUSTIVE_RUSMPP_ENUMS
                if NON_EXHAUSTIVE_RUSMPP_ENUMS.contains(&name) {
                    writeln!(
                        self.out,
                        "_ => panic!(\"Unexpected variant in Rusmpp type {name}\"),"
                    )?;
                }

                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}\n")?;

                // default
                self.default_impl(name)?;

                // Generate the methods
                let gen_stub_pymethods = gen_stub_pymethods(name);
                writeln!(self.out, "{gen_stub_pymethods}")?;
                writeln!(self.out, "#[::pyo3::pymethods]")?;
                writeln!(self.out, "impl {name} {{")?;
                self.out.indent();

                // default
                self.py_default_impl(name)?;

                // additional methods
                let additional_methods = py_additional_methods(name);
                writeln!(self.out, "{additional_methods}")?;

                // __repr__
                writeln!(self.out, "fn __repr__(&self) -> String {{")?;
                self.out.indent();
                writeln!(self.out, "format!(\"{{self:?}}\")")?;
                self.out.unindent();
                writeln!(self.out, "}}")?;
                self.out.unindent();
                writeln!(self.out, "}}\n")?;
            }
        }
        self.output_custom_code(name)
    }

    fn default_impl(&mut self, name: &str) -> Result<()> {
        let _: () = if !NON_DEFAULT_RUSMPP_TYPES.contains(&name) {
            writeln!(self.out, "impl {name} {{")?;
            self.out.indent();
            writeln!(self.out, "pub fn default_() -> Self {{")?;
            self.out.indent();
            writeln!(self.out, "Self::from(rusmpp_types::{name}::default())")?;
            self.out.unindent();
            writeln!(self.out, "}}")?;
            self.out.unindent();
            writeln!(self.out, "}}\n")?;
        };

        Ok(())
    }

    fn py_default_impl(&mut self, name: &str) -> Result<()> {
        let _: () = if !NON_DEFAULT_RUSMPP_TYPES.contains(&name) {
            writeln!(self.out, "#[classmethod]")?;
            writeln!(self.out, "#[pyo3(signature=())]")?;
            writeln!(
                self.out,
                "pub fn default<'p>(_cls: &'p ::pyo3::Bound<'p, ::pyo3::types::PyType>) -> Self {{"
            )?;
            self.out.indent();
            writeln!(self.out, "Self::default_()")?;
            self.out.unindent();
            writeln!(self.out, "}}")?;
        };

        Ok(())
    }

    fn output_add_classes(&mut self, registry: &Registry) -> Result<()> {
        writeln!(
            self.out,
            "pub fn add_classes(m: &::pyo3::Bound<'_, ::pyo3::prelude::PyModule>) -> ::pyo3::PyResult<()> {{"
        )?;

        writeln!(self.out, "use ::pyo3::types::PyModuleMethods;")?;

        self.out.indent();

        for name in registry.keys() {
            writeln!(self.out, "m.add_class::<{}>()?;", name)?;
        }

        writeln!(self.out, "Ok(())")?;

        self.out.unindent();

        writeln!(self.out, "}}")?;

        Ok(())
    }
}
