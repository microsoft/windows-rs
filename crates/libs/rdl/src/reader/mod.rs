mod attribute;
mod attribute_ref;
mod callback;
mod class;
mod r#const;
mod delegate;
mod r#enum;
mod field;
mod file;
mod r#fn;
pub(crate) mod guid;
mod index;
mod interface;
mod item;
mod method;
mod module;
mod param;
mod resolved;
mod resolver;
mod r#struct;
mod typedef;
mod union;
mod validate;

use super::*;
use attribute::*;
use callback::*;
use class::*;
use r#const::*;
use delegate::*;
use r#enum::*;
use field::*;
use file::*;
use r#fn::*;
use index::*;
use interface::*;
use item::*;
use method::*;
use module::*;
use resolved::*;
use resolver::*;
use r#struct::*;
use typedef::*;
use union::*;
use validate::*;
use windows_metadata as metadata;

fn fixed_signed_value(value: i64) -> metadata::Value {
    i32::try_from(value)
        .map(metadata::Value::I32)
        .unwrap_or(metadata::Value::I64(value))
}

fn fixed_unsigned_value(value: u64) -> metadata::Value {
    u32::try_from(value)
        .map(metadata::Value::U32)
        .unwrap_or(metadata::Value::U64(value))
}

#[derive(Default)]
/// Builder that compiles RDL files into `.winmd` metadata.
pub struct Reader {
    input: Vec<PathBuf>,
    input_text: Vec<InputText>,
    reference: Vec<PathBuf>,
    reference_default: bool,
    reference_bytes: Vec<Vec<u8>>,
    output: PathBuf,
}

struct InputText {
    name: String,
    text: String,
}

impl Reader {
    /// Creates a new builder with default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an input `.rdl` file or directory.
    pub fn input(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.input.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds inline RDL source text to compile instead of a file on disk.
    pub fn input_text(&mut self, input: &str) -> &mut Self {
        self.input_text_named(".rdl", input)
    }

    /// Adds named inline RDL source text to compile instead of a file on disk.
    pub fn input_text_named(&mut self, name: impl AsRef<str>, input: &str) -> &mut Self {
        self.input_text.push(InputText {
            name: name.as_ref().to_string(),
            text: input.to_string(),
        });
        self
    }

    /// Adds inline RDL source texts to compile instead of files on disk.
    pub fn input_texts<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input_text(input.as_ref());
        }
        self
    }

    /// Adds named inline RDL source texts to compile instead of files on disk.
    pub fn input_texts_named<I, N, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = (N, S)>,
        N: AsRef<str>,
        S: AsRef<str>,
    {
        for (name, input) in inputs {
            self.input_text_named(name, input.as_ref());
        }
        self
    }

    /// Adds a `.winmd` reference file or directory.
    pub fn reference(&mut self, input: impl AsRef<Path>) -> &mut Self {
        self.reference.push(input.as_ref().to_path_buf());
        self
    }

    /// Adds multiple `.winmd` reference files or directories.
    pub fn references<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.reference(input);
        }
        self
    }

    /// Adds a `.winmd` reference from memory.
    pub fn reference_bytes(&mut self, input: &[u8]) -> &mut Self {
        self.reference_bytes.push(input.to_vec());
        self
    }

    /// Adds `.winmd` references from memory.
    pub fn reference_byte_sets<I, B>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = B>,
        B: AsRef<[u8]>,
    {
        for input in inputs {
            self.reference_bytes(input.as_ref());
        }
        self
    }

    /// Adds the default Windows metadata references.
    pub fn reference_default(&mut self) -> &mut Self {
        self.reference_default = true;
        self
    }

    /// Adds multiple input `.rdl` files or directories.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<Path>,
    {
        for input in inputs {
            self.input(input);
        }
        self
    }

    /// Sets the output `.winmd` file path.
    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    /// Compiles the inputs and writes the `.winmd` to the configured output.
    pub fn write(&self) -> Result<(), Error> {
        if self.output.as_os_str().is_empty() {
            return Err(Error::new("output is required", "", 0, 0));
        }

        let assembly_name = self
            .output
            .file_stem()
            .and_then(|file_name| file_name.to_str())
            .ok_or_else(|| Error::new("invalid output", &self.output.to_string_lossy(), 0, 0))?;

        let output = self.compile(assembly_name)?;
        std::fs::write(&self.output, output)
            .map_err(|error| Error::new(&error.to_string(), &self.output.to_string_lossy(), 0, 0))
    }

    /// Parses, validates, resolves, and encodes the inputs without writing a `.winmd`.
    pub fn check(&self) -> Result<(), Error> {
        self.compile("check").map(|_| ())
    }

    /// Checks all inputs and returns every independent diagnostic found before encoding.
    pub fn check_all(&self) -> DiagnosticReport {
        self.compile_report("check").1
    }

    fn compile(&self, assembly_name: &str) -> Result<Vec<u8>, Error> {
        let (output, report) = self.compile_report(assembly_name);
        if let Some(error) = report.into_error() {
            Err(error)
        } else {
            Ok(output.unwrap())
        }
    }

    fn compile_report(&self, assembly_name: &str) -> (Option<Vec<u8>>, DiagnosticReport) {
        let mut report = DiagnosticReport::default();
        let rdl_paths = match expand_input_files(&self.input, "rdl") {
            Ok(paths) => Some(paths),
            Err(error) => {
                report.push(error);
                None
            }
        };
        let reference_paths = match expand_input_files(&self.reference, "winmd") {
            Ok(paths) => Some(paths),
            Err(error) => {
                report.push(error);
                None
            }
        };
        let Some(rdl_paths) = rdl_paths else {
            return (None, report);
        };
        let Some(reference_paths) = reference_paths else {
            return (None, report);
        };

        let input = expand_rdl_files(&rdl_paths, &self.input_text, &mut report);

        let mut index = Index::new();

        for file in &input {
            for item in &file.items {
                index.insert(file, "", item);
            }
        }

        report.extend(validate_symbols(&index));

        let mut reference = vec![];
        let mut invalid_reference = false;

        for file_name in &reference_paths {
            let source = file_name.to_string_lossy();
            if let Some(file) = metadata::reader::File::read(file_name) {
                reference.push(file);
            } else {
                invalid_reference = true;
                report.push(Error::new("invalid reference", &source, 0, 0));
            }
        }

        if self.reference_default {
            reference.extend(
                [windows_default::WINRT, windows_default::WIN32]
                    .into_iter()
                    .map(|bytes| metadata::reader::File::new(bytes.to_vec()).unwrap()),
            );
        }

        for bytes in &self.reference_bytes {
            if let Some(file) = metadata::reader::File::new(bytes.clone()) {
                reference.push(file);
            } else {
                invalid_reference = true;
                report.push(Error::new("invalid reference", "<memory>", 0, 0));
            }
        }

        let reference = metadata::reader::Index::new(reference);
        if !invalid_reference {
            report.extend(validate_use_declarations(&input, &index, &reference));
            let (model, diagnostics) = resolve_model(&index, &reference);
            report.extend(diagnostics);
            report.extend(validate_resolved_symbols(&model));
        }
        if !report.is_success() {
            return (None, report);
        }

        match encode(assembly_name, &index, reference) {
            Ok(output) => (Some(output), report),
            Err(error) => {
                report.push(error);
                (None, report)
            }
        }
    }
}

fn encode(
    assembly_name: &str,
    index: &Index,
    reference: metadata::reader::Index,
) -> Result<Vec<u8>, Error> {
    let mut output = metadata::writer::File::new(assembly_name);
    output.set_reference(reference);

    for (namespace, members) in &index.namespaces {
        for variants in members.types.values() {
            for (file, item) in variants {
                let name = item.to_string();
                let encoder = &mut Encoder {
                    output: &mut output,
                    index,
                    file,
                    namespace,
                    name: &name,
                    generics: vec![],
                };
                match item {
                    Item::Attribute(ty) => encoder.encode_attribute(ty),
                    Item::Callback(ty) => encoder.encode_callback(ty),
                    Item::Class(ty) => encoder.encode_class(ty),
                    Item::Const(ty) => encoder.encode_const(ty),
                    Item::Delegate(ty) => encoder.encode_delegate(ty),
                    Item::Enum(ty) => encoder.encode_enum(ty),
                    Item::Fn(ty) => encoder.encode_fn(ty),
                    Item::Interface(ty) => encoder.encode_interface(ty),
                    Item::Struct(ty) => encoder.encode_struct(ty),
                    Item::Typedef(ty) => encoder.encode_typedef(ty),
                    Item::Union(ty) => encoder.encode_union(ty),
                    Item::Module(_) => unreachable!(
                        "Module items are expanded during indexing and never encoded directly"
                    ),
                }?;
            }
        }

        if !members.functions.is_empty() || !members.constants.is_empty() {
            let class = metadata::writer::TypeDefOrRef::TypeRef(output.TypeRef("System", "Object"));

            output.TypeDef(
                namespace,
                "Apis",
                class,
                metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed,
            );

            for (name, variants) in &members.functions {
                for (file, item) in variants {
                    let Item::Fn(ty) = item else {
                        unreachable!("functions index only contains Item::Fn")
                    };
                    Encoder {
                        output: &mut output,
                        index,
                        file,
                        namespace,
                        name,
                        generics: vec![],
                    }
                    .encode_fn(ty)?;
                }
            }

            for (name, variants) in &members.constants {
                for (file, item) in variants {
                    let Item::Const(ty) = item else {
                        unreachable!("constants index only contains Item::Const")
                    };
                    Encoder {
                        output: &mut output,
                        index,
                        file,
                        namespace,
                        name,
                        generics: vec![],
                    }
                    .encode_const(ty)?;
                }
            }
        }
    }

    Ok(output.into_stream())
}

/// Parses one `.rdl` file and returns the items it defines under `namespace`.
pub(crate) fn item_names(path: impl AsRef<Path>, namespace: &str) -> Result<Vec<String>, Error> {
    let path = path.as_ref().to_path_buf();
    let mut report = DiagnosticReport::default();
    let input = expand_rdl_files(std::slice::from_ref(&path), &[], &mut report);
    if let Some(error) = report.into_error() {
        return Err(error);
    }
    let mut index = Index::new();
    for file in &input {
        for item in &file.items {
            index.insert(file, "", item);
        }
    }
    let mut names = vec![];
    if let Some(ns) = index.namespaces.get(namespace) {
        names.extend(ns.types.keys().cloned());
        names.extend(ns.functions.keys().cloned());
        names.extend(ns.constants.keys().cloned());
    }
    Ok(names)
}

/// Rewrites RDL tokens that would otherwise confuse `syn`.
fn preprocess_rdl(contents: &str) -> std::borrow::Cow<'_, str> {
    let needs_in = contents.contains("#[in]");
    let needs_doc = contents.contains("//!");
    if !needs_in && !needs_doc {
        return std::borrow::Cow::Borrowed(contents);
    }
    let mut result = contents.to_string();
    if needs_in {
        result = result.replace("#[in]", "#[r#in]");
    }
    if needs_doc {
        result = result.replace("//!", "//");
    }
    std::borrow::Cow::Owned(result)
}

pub(crate) fn parse_source(name: &str, input: &str) -> Result<(), Error> {
    let contents = preprocess_rdl(input);
    syn::parse_str::<File>(&contents)
        .map(|_| ())
        .map_err(|error| {
            let start = error.span().start();
            Error::new(&error.to_string(), name, start.line, start.column)
        })
}

fn expand_rdl_files(
    paths: &[PathBuf],
    input_text: &[InputText],
    report: &mut DiagnosticReport,
) -> Vec<File> {
    let mut input = vec![];

    for path in paths {
        let source = path.to_string_lossy();
        let Ok(contents) = std::fs::read_to_string(path) else {
            report.push(Error::new("failed to read input file", &source, 0, 0));
            continue;
        };
        report.add_source(source.to_string(), contents.clone());

        let contents = preprocess_rdl(&contents);
        match syn::parse_str::<File>(&contents) {
            Ok(mut file) => {
                file.source = source.into_owned();
                input.push(file);
            }
            Err(error) => {
                let start = error.span().start();
                report.push(Error::new(
                    &error.to_string(),
                    &source,
                    start.line,
                    start.column,
                ));
            }
        }
    }

    for input_text in input_text {
        report.add_source(input_text.name.clone(), input_text.text.clone());
        let contents = preprocess_rdl(&input_text.text);
        match syn::parse_str::<File>(&contents) {
            Ok(mut file) => {
                file.source.clone_from(&input_text.name);
                input.push(file);
            }
            Err(error) => {
                let start = error.span().start();
                report.push(Error::new(
                    &error.to_string(),
                    &input_text.name,
                    start.line,
                    start.column,
                ));
            }
        }
    }

    for file in &mut input {
        for item in &mut file.items {
            if let Err(error) = resolve_winrt(item, &file.source, None) {
                report.push(error);
            }
        }
    }

    input
}

fn resolve_winrt(item: &mut Item, source_file: &str, parent: Option<bool>) -> Result<(), Error> {
    match item {
        Item::Enum(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        Item::Interface(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        Item::Struct(item) => {
            item.winrt = read_winrt_expected(source_file, &item.span, &item.attrs, parent)?;
        }
        Item::Attribute(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        Item::Module(item) => {
            let parent = read_winrt(source_file, &item.token, &item.attrs, parent)?;

            for child in &mut item.items {
                resolve_winrt(child, source_file, parent)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn read_winrt_expected<S: Spanned>(
    source_file: &str,
    span: &S,
    attrs: &[syn::Attribute],
    parent: Option<bool>,
) -> Result<bool, Error> {
    if let Some(winrt) = read_winrt(source_file, span, attrs, parent)? {
        Ok(winrt)
    } else {
        let start = span.span().start();

        Err(Error::new(
            "`winrt` or `win32` attribute required",
            source_file,
            start.line,
            start.column,
        ))
    }
}

fn read_winrt<S: Spanned>(
    source_file: &str,
    span: &S,
    attrs: &[syn::Attribute],
    parent: Option<bool>,
) -> Result<Option<bool>, Error> {
    let mut winrt = false;
    let mut win32 = false;

    for attr in attrs {
        if attr.path().is_ident("winrt") {
            winrt = true;
        } else if attr.path().is_ident("win32") {
            win32 = true;
        }
    }

    if winrt && win32 {
        let start = span.span().start();

        return Err(Error::new(
            "`winrt` and `win32` attributes are mutually exclusive",
            source_file,
            start.line,
            start.column,
        ));
    } else if !winrt
        && !win32
        && let Some(parent) = parent
    {
        if parent {
            winrt = true;
        } else {
            win32 = true;
        }
    }

    if winrt {
        Ok(Some(true))
    } else if win32 {
        Ok(Some(false))
    } else {
        Ok(None)
    }
}

fn validate_use_declarations(
    input: &[File],
    index: &Index,
    reference: &metadata::reader::Index,
) -> Vec<Error> {
    let mut diagnostics = vec![];

    for file in input {
        let mut names = HashMap::<&str, &Import>::new();
        for import in &file.imports {
            let target = import.path.join(".");
            let found = if import.glob {
                namespace_exists(index, reference, &target)
            } else {
                import_target_exists(index, reference, &import.path)
            };
            if !found {
                let start = import.span.start();
                let end = import.span.end();
                diagnostics.push(
                    Error::new(
                        &format!("import target `{target}` not found"),
                        &file.source,
                        start.line,
                        start.column,
                    )
                    .with_code("RDL0003")
                    .with_primary_label(
                        Label::primary(&file.source, start.line, start.column)
                            .with_end(end.line, end.column)
                            .with_message("unknown import target"),
                    ),
                );
            }

            let Some(local) = import.local.as_deref() else {
                continue;
            };
            if let Some(previous) = names.insert(local, import)
                && previous.path != import.path
            {
                let start = import.span.start();
                let end = import.span.end();
                let previous_start = previous.span.start();
                let previous_end = previous.span.end();
                diagnostics.push(
                    Error::new(
                        &format!("import name `{local}` is defined more than once"),
                        &file.source,
                        start.line,
                        start.column,
                    )
                    .with_code("RDL0004")
                    .with_primary_label(
                        Label::primary(&file.source, start.line, start.column)
                            .with_end(end.line, end.column)
                            .with_message("conflicting import"),
                    )
                    .with_label(
                        Label::secondary(
                            &file.source,
                            previous_start.line,
                            previous_start.column,
                            "first imported here",
                        )
                        .with_end(previous_end.line, previous_end.column),
                    )
                    .with_help("use distinct aliases for different import targets"),
                );
            }
        }
    }
    diagnostics
}

fn namespace_exists(index: &Index, reference: &metadata::reader::Index, namespace: &str) -> bool {
    index.namespaces.contains_key(namespace) || reference.contains_namespace(namespace)
}

fn import_target_exists(
    index: &Index,
    reference: &metadata::reader::Index,
    path: &[String],
) -> bool {
    let Some((name, namespace)) = path.split_last() else {
        return false;
    };
    let namespace = namespace.join(".");
    namespace_exists(index, reference, &path.join("."))
        || index.contains(&namespace, name)
        || reference.contains(&namespace, name)
        || index.contains(&namespace, &format!("{name}Attribute"))
        || reference.contains(&namespace, &format!("{name}Attribute"))
}

/// Parses a `syn::LitInt` as a `u128`, supporting both `0x...` hex and decimal literals.
/// Underscore separators (e.g. `0x005023ca_72b1_11d3_9fc4_00c04f79a0a3`) are accepted.
pub(super) fn parse_guid_u128(lit: &syn::LitInt) -> Result<u128, ()> {
    let s: String = lit
        .token()
        .to_string()
        .chars()
        .filter(|&c| c != '_')
        .collect();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u128::from_str_radix(hex, 16).map_err(|_| ())
    } else {
        s.parse::<u128>().map_err(|_| ())
    }
}

struct Encoder<'a> {
    output: &'a mut metadata::writer::File,
    index: &'a Index<'a>,
    file: &'a File,
    namespace: &'a str,
    name: &'a str,
    generics: Vec<String>,
}

impl Encoder<'_> {
    fn resolver(&self) -> Resolver<'_, '_> {
        Resolver {
            index: self.index,
            reference: self.output.reference().unwrap(),
            file: self.file,
            namespace: self.namespace,
            generics: &self.generics,
        }
    }

    fn error<S: Spanned>(&self, spanned: S, message: &str) -> Error {
        let start = spanned.span().start();

        Error::new(message, &self.file.source, start.line, start.column)
    }

    fn err<T, S: Spanned>(&self, spanned: S, message: &str) -> Result<T, Error> {
        Err(self.error(spanned, message))
    }

    fn read_packed(&self, attrs: &[syn::Attribute]) -> Result<Option<u16>, Error> {
        for attr in attrs {
            if !attr.path().is_ident("packed") {
                continue;
            }

            let Ok(size_literal) = attr.parse_args::<syn::LitInt>() else {
                return self.err(attr, "`packed` attribute requires an integer argument");
            };

            let Ok(size) = size_literal.base10_parse::<u16>() else {
                return self.err(attr, "`packed` size must be a valid u16");
            };

            return Ok(Some(size));
        }

        Ok(None)
    }

    /// Reads forced over-alignment from `#[align(N)]`.
    fn read_align(&self, attrs: &[syn::Attribute]) -> Result<Option<u16>, Error> {
        for attr in attrs {
            if !attr.path().is_ident("align") {
                continue;
            }

            let Ok(size_literal) = attr.parse_args::<syn::LitInt>() else {
                return self.err(attr, "`align` attribute requires an integer argument");
            };

            let Ok(size) = size_literal.base10_parse::<u16>() else {
                return self.err(attr, "`align` size must be a valid u16");
            };

            return Ok(Some(size));
        }

        Ok(None)
    }

    fn read_arch(&self, attrs: &[syn::Attribute]) -> Result<Option<i32>, Error> {
        for attr in attrs {
            if !attr.path().is_ident("arch") {
                continue;
            }

            let expr = attr.parse_args::<syn::Expr>().map_err(|_| {
                self.error(
                    attr,
                    "`arch` attribute requires architecture arguments (e.g. `#[arch(X86)]`)",
                )
            })?;

            let bits = parse_arch_bitmask(&expr).ok_or_else(|| {
                self.error(
                    attr,
                    "invalid `arch` value; expected `X86`, `X64`, `Arm64`, or a `|`-combination",
                )
            })?;

            return Ok(Some(bits));
        }

        Ok(None)
    }

    fn encode_type(&self, ty: &syn::Type) -> Result<metadata::Type, Error> {
        self.resolver().resolve_type(ty)
    }

    fn encode_value(
        &self,
        ty: &metadata::Type,
        value: &syn::Expr,
    ) -> Result<metadata::Value, Error> {
        if matches!(ty, metadata::Type::ISize | metadata::Type::USize)
            && let Some(value) = self.encode_fixed_width_value(value)?
        {
            return Ok(value);
        }

        let value = match ty {
            metadata::Type::I8 => metadata::Value::I8(self.encode_lit_sint(value, 8)? as i8),
            metadata::Type::U8 => metadata::Value::U8(self.encode_lit_uint(value, 8)? as u8),
            metadata::Type::I16 => metadata::Value::I16(self.encode_lit_sint(value, 16)? as i16),
            metadata::Type::U16 => metadata::Value::U16(self.encode_lit_uint(value, 16)? as u16),
            metadata::Type::I32 => metadata::Value::I32(self.encode_lit_sint(value, 32)? as i32),
            metadata::Type::U32 => metadata::Value::U32(self.encode_lit_uint(value, 32)? as u32),
            metadata::Type::I64 => metadata::Value::I64(self.encode_lit_sint(value, 64)?),
            metadata::Type::U64 => metadata::Value::U64(self.encode_lit_uint(value, 64)?),
            metadata::Type::F32 => metadata::Value::F32(self.encode_neg_lit_float::<f32>(value)?),
            metadata::Type::F64 => metadata::Value::F64(self.encode_neg_lit_float::<f64>(value)?),
            metadata::Type::String => metadata::Value::Utf16(self.encode_lit_string(value)?),
            metadata::Type::ISize => fixed_signed_value(self.encode_lit_sint(value, 64)?),
            metadata::Type::USize => fixed_unsigned_value(self.encode_lit_uint(value, 64)?),
            metadata::Type::PtrMut(_, _) | metadata::Type::PtrConst(_, _) => {
                let v = self.encode_neg_lit_int::<i64>(value)?;
                if let Ok(v) = i32::try_from(v) {
                    metadata::Value::I32(v)
                } else {
                    metadata::Value::I64(v)
                }
            }
            metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
                let underlying = self
                    .output
                    .reference()
                    .and_then(|r| r.get(&tn.namespace, &tn.name).next())
                    .and_then(|def| def.underlying_type())
                    .or_else(|| self.rdl_underlying_type(&tn.namespace, &tn.name));

                match underlying {
                    Some(underlying) => return self.encode_value(&underlying, value),
                    None => {
                        return self.err(value, &format!("constant type not supported: {ty:?}"));
                    }
                }
            }
            rest => return self.err(value, &format!("constant type not supported: {rest:?}")),
        };

        Ok(value)
    }

    fn encode_fixed_width_value(
        &self,
        value: &syn::Expr,
    ) -> Result<Option<metadata::Value>, Error> {
        let int = match value {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) => int,
            syn::Expr::Unary(syn::ExprUnary { expr, .. }) => {
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) = expr.as_ref()
                else {
                    return Ok(None);
                };
                int
            }
            _ => return Ok(None),
        };

        let value = match int.suffix() {
            "i32" => metadata::Value::I32(self.encode_lit_sint(value, 32)? as i32),
            "u32" => metadata::Value::U32(self.encode_lit_uint(value, 32)? as u32),
            "i64" => metadata::Value::I64(self.encode_lit_sint(value, 64)?),
            "u64" => metadata::Value::U64(self.encode_lit_uint(value, 64)?),
            _ => return Ok(None),
        };
        Ok(Some(value))
    }

    fn rdl_underlying_type(&self, namespace: &str, name: &str) -> Option<metadata::Type> {
        let item = self.index.get(namespace, name).next()?;

        match item {
            Item::Typedef(t) => self.encode_underlying(&t.ty, namespace),
            Item::Enum(e) => {
                // Enum-typed constants encode against the enum's `#[repr(iN)]` type.
                let repr = e.attrs.iter().find(|a| a.path().is_ident("repr"))?;
                let path = repr.parse_args::<syn::Path>().ok()?;
                self.encode_path(&path).ok()
            }
            Item::Struct(s) => {
                let mut fields = s.fields.iter();

                if let Some(field) = fields.next()
                    && fields.next().is_none()
                    && let FieldType::Type(ty) = &field.ty
                {
                    return self.encode_underlying(ty, namespace);
                }

                None
            }
            _ => None,
        }
    }

    /// Resolves a typedef's bare sibling names in the typedef namespace, not the constant namespace.
    fn encode_underlying(&self, ty: &syn::Type, namespace: &str) -> Option<metadata::Type> {
        match ty {
            // MAKEINTRESOURCE-style pointer constants only need the pointer kind.
            syn::Type::Ptr(ptr) => {
                let pointee = self
                    .encode_underlying(&ptr.elem, namespace)
                    .unwrap_or(metadata::Type::Void);
                Some(if ptr.mutability.is_some() {
                    metadata::Type::PtrMut(Box::new(pointee), 1)
                } else {
                    metadata::Type::PtrConst(Box::new(pointee), 1)
                })
            }
            syn::Type::Path(tp)
                if tp.qself.is_none()
                    && tp.path.segments.len() == 1
                    && matches!(tp.path.segments[0].arguments, syn::PathArguments::None) =>
            {
                if let Ok(resolved) = self.encode_type(ty)
                    && !matches!(
                        resolved,
                        metadata::Type::ValueName(_) | metadata::Type::ClassName(_)
                    )
                {
                    return Some(resolved);
                }
                let ident = tp.path.segments[0].ident.unraw_to_string();
                Some(metadata::Type::value_named(namespace, &ident))
            }
            _ => self.encode_type(ty).ok(),
        }
    }

    fn encode_neg_lit_int<T>(&self, expr: &syn::Expr) -> Result<T, Error>
    where
        T: std::str::FromStr + TryFrom<i128>,
        T::Err: std::fmt::Display,
    {
        let value = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) => int.base10_parse().ok(),
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => match expr.as_ref() {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int
                    .base10_parse::<u64>()
                    .ok()
                    .and_then(|v| T::try_from(-(v as i128)).ok()),
                _ => None,
            },
            _ => None,
        };

        value.ok_or_else(|| self.error(expr, "value not valid"))
    }

    fn encode_lit_int<T>(&self, expr: &syn::Expr) -> Result<T, Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let value = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) => int.base10_parse().ok(),

            _ => None,
        };

        value.ok_or_else(|| self.error(expr, "value not valid"))
    }

    /// Accepts C unsigned sentinels spelled as negated casts by masking to the target width.
    fn encode_lit_uint(&self, expr: &syn::Expr, bits: u32) -> Result<u64, Error> {
        let mask: u128 = if bits >= 128 {
            u128::MAX
        } else {
            (1u128 << bits) - 1
        };
        let value = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) => int.base10_parse::<u64>().ok(),
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => match expr.as_ref() {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int
                    .base10_parse::<u64>()
                    .ok()
                    .map(|v| ((v as i128).wrapping_neg() as u128 & mask) as u64),
                _ => None,
            },
            _ => None,
        };

        value.ok_or_else(|| self.error(expr, "value not valid"))
    }

    /// Reinterprets signed constants from their C bit pattern, including overflowing HRESULTs.
    fn encode_lit_sint(&self, expr: &syn::Expr, bits: u32) -> Result<i64, Error> {
        let raw: Option<u64> = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(int),
                ..
            }) => int.base10_parse::<u64>().ok(),
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => match expr.as_ref() {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int
                    .base10_parse::<u64>()
                    .ok()
                    .map(|v| (v as i128).wrapping_neg() as u64),
                _ => None,
            },
            _ => None,
        };

        let raw = raw.ok_or_else(|| self.error(expr, "value not valid"))?;

        if bits >= 64 {
            Ok(raw as i64)
        } else {
            let mask = (1u64 << bits) - 1;
            let masked = raw & mask;
            let sign_bit = 1u64 << (bits - 1);
            Ok(if masked & sign_bit != 0 {
                (masked | !mask) as i64
            } else {
                masked as i64
            })
        }
    }

    fn encode_neg_lit_float<T>(&self, expr: &syn::Expr) -> Result<T, Error>
    where
        T: std::str::FromStr + std::ops::Neg<Output = T>,
        T::Err: std::fmt::Display,
    {
        let value = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Float(float),
                ..
            }) => float.base10_parse().ok(),
            syn::Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Neg(_),
                expr,
                ..
            }) => match expr.as_ref() {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Float(float),
                    ..
                }) => float.base10_parse().ok().map(|value: T| -value),
                _ => None,
            },
            _ => None,
        };

        value.ok_or_else(|| self.error(expr, "value not valid"))
    }

    fn encode_lit_string(&self, expr: &syn::Expr) -> Result<String, Error> {
        let value = match expr {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(string),
                ..
            }) => Some(string.value()),
            _ => None,
        };

        value.ok_or_else(|| self.error(expr, "value not valid"))
    }

    fn encode_path(&self, ty: &syn::Path) -> Result<metadata::Type, Error> {
        self.resolver().resolve_path(ty)
    }

    fn encode_return_type(&self, ty: &syn::ReturnType) -> Result<metadata::Type, Error> {
        match ty {
            syn::ReturnType::Type(_, ty) => self.encode_type(ty),
            _ => Ok(metadata::Type::Void),
        }
    }

    /// Rejects references from WinRT types to non-WinRT named types.
    fn validate_type_is_winrt<S: Spanned + quote::ToTokens>(
        &self,
        span: &S,
        ty: &metadata::Type,
    ) -> Result<(), Error> {
        match ty {
            metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
                for generic_ty in &tn.generics {
                    self.validate_type_is_winrt(span, generic_ty)?;
                }

                if let Some(is_winrt) = self.index.is_winrt(&tn.namespace, &tn.name) {
                    if !is_winrt {
                        return self.err(span, "WinRT types cannot refer to non-WinRT types");
                    }
                } else if let Some(reference) = self.output.reference()
                    && let Some(def) = reference.get(&tn.namespace, &tn.name).next()
                    && !def
                        .flags()
                        .contains(metadata::TypeAttributes::WindowsRuntime)
                {
                    return self.err(span, "WinRT types cannot refer to non-WinRT types");
                }
            }
            metadata::Type::PtrMut(inner, _) | metadata::Type::PtrConst(inner, _) => {
                self.validate_type_is_winrt(span, inner)?;
            }
            metadata::Type::RefMut(inner) | metadata::Type::RefConst(inner) => {
                self.validate_type_is_winrt(span, inner)?;
            }
            metadata::Type::Array(inner) | metadata::Type::ArrayFixed(inner, _) => {
                self.validate_type_is_winrt(span, inner)?;
            }
            _ => {}
        }

        Ok(())
    }
}

/// Parses a `#[arch(...)]` expression into the X86/X64/Arm64 bitmask.
pub(crate) fn parse_arch_bitmask(expr: &syn::Expr) -> Option<i32> {
    match expr {
        syn::Expr::Path(p)
            if p.qself.is_none()
                && p.path.leading_colon.is_none()
                && p.path.segments.len() == 1 =>
        {
            arch_name_to_bits(&p.path.segments[0].ident.to_string())
        }
        syn::Expr::Binary(syn::ExprBinary {
            left,
            op: syn::BinOp::BitOr(_),
            right,
            ..
        }) => {
            let l = parse_arch_bitmask(left)?;
            let r = parse_arch_bitmask(right)?;
            Some(l | r)
        }
        _ => None,
    }
}

fn arch_name_to_bits(name: &str) -> Option<i32> {
    match name {
        "X86" => Some(1),
        "X64" => Some(2),
        "Arm64" => Some(4),
        _ => None,
    }
}

pub(crate) fn make_sig(
    fn_token: syn::Token![fn],
    ident: syn::Ident,
    generics: syn::Generics,
    paren_token: syn::token::Paren,
    inputs: syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    variadic: Option<syn::Variadic>,
    output: syn::ReturnType,
) -> syn::Signature {
    syn::Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token,
        ident,
        generics,
        paren_token,
        inputs,
        variadic,
        output,
    }
}

pub(crate) fn parse_fn_inputs(
    content: &syn::parse::ParseBuffer,
) -> syn::Result<(
    syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    Option<syn::Variadic>,
)> {
    let mut args = syn::punctuated::Punctuated::new();
    let mut variadic = None;

    while !content.is_empty() {
        let fork = content.fork();
        let _ = fork.call(syn::Attribute::parse_outer);
        if fork.peek(syn::Token![...]) {
            let attrs = content.call(syn::Attribute::parse_outer)?;
            let dots: syn::Token![...] = content.parse()?;
            variadic = Some(syn::Variadic {
                attrs,
                pat: None,
                dots,
                comma: if content.is_empty() {
                    None
                } else {
                    Some(content.parse()?)
                },
            });
            break;
        }

        let arg: syn::FnArg = content.parse()?;
        args.push_value(arg);

        if content.is_empty() {
            break;
        }

        let comma: syn::Token![,] = content.parse()?;
        args.push_punct(comma);
    }

    Ok((args, variadic))
}

/// Parses `-> #[attr]* Type`, keeping return-value attributes with the return type.
pub(crate) fn parse_return_type_with_attrs(
    input: syn::parse::ParseStream,
) -> syn::Result<(syn::ReturnType, Vec<syn::Attribute>)> {
    if input.peek(syn::Token![->]) {
        let arrow = input.parse::<syn::Token![->]>()?;
        let return_attrs = input.call(syn::Attribute::parse_outer)?;
        let ty: syn::Type = input.parse()?;
        Ok((syn::ReturnType::Type(arrow, Box::new(ty)), return_attrs))
    } else {
        Ok((syn::ReturnType::Default, vec![]))
    }
}

fn push_import_candidate<'a>(
    candidates: &mut Vec<(&'a Import, metadata::Type)>,
    import: &'a Import,
    ty: metadata::Type,
) {
    if !candidates.iter().any(|(_, existing)| existing == &ty) {
        candidates.push((import, ty));
    }
}

fn display_named_type(ty: &metadata::Type) -> String {
    let (metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn)) = ty else {
        return format!("{ty:?}");
    };
    if tn.namespace.is_empty() {
        tn.name.clone()
    } else {
        format!("{}.{}", tn.namespace, tn.name)
    }
}

trait IdentMethods {
    fn unraw_to_string(&self) -> String;
}

impl IdentMethods for syn::Ident {
    fn unraw_to_string(&self) -> String {
        use syn::ext::IdentExt;
        self.unraw().to_string()
    }
}

#[test]
fn use_glob_resolves_type() {
    let output = std::env::temp_dir().join("windows_rdl_use_glob_resolves_type.winmd");

    reader()
        .input_text(
            r#"
use Other::*;

#[winrt]
mod Test {
    struct Thing {
        a: Point,
    }
}

#[winrt]
mod Other {
    struct Point {
        x: i32,
        y: i32,
    }
}
        "#,
        )
        .output(&output)
        .write()
        .unwrap();
}
