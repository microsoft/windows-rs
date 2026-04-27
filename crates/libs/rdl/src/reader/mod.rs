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
mod r#struct;
mod typedef;
mod union;

use super::*;
use attribute::*;
use callback::*;
use class::*;
use delegate::*;
use field::*;
use file::*;
use index::*;
use interface::*;
use item::*;
use method::*;
use module::*;
use r#const::*;
use r#enum::*;
use r#fn::*;
use r#struct::*;
use typedef::*;
use union::*;
use windows_metadata as metadata;

#[derive(Default)]
pub struct Reader {
    input: Vec<String>,
    input_str: Vec<String>,
    output: String,
}

impl Reader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    pub fn input_str(&mut self, input: &str) -> &mut Self {
        self.input_str.push(input.to_string());
        self
    }

    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.input.push(input.as_ref().to_string());
        }

        self
    }

    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    pub fn write(&self) -> Result<(), Error> {
        if self.output.is_empty() {
            return Err(Error::new("output is required", "", 0, 0));
        }

        let (rdl_paths, reference_paths) = expand_input_paths(&self.input, "rdl", "winmd")?;

        let input = expand_rdl_files(&rdl_paths, &self.input_str)?;

        let mut index = Index::new();

        for file in &input {
            for item in &file.items {
                index.insert(file, "", item);
            }
        }

        let mut reference = vec![];

        for file_name in &reference_paths {
            reference.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid reference", file_name, 0, 0))?,
            );
        }

        let reference = metadata::reader::TypeIndex::new(reference);
        validate_use_declarations(&input, &index, &reference)?;

        let assembly_name = std::path::Path::new(&self.output)
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .ok_or_else(|| Error::new("invalid output", &self.output, 0, 0))?;

        let mut output = metadata::writer::File::new(assembly_name);
        output.set_reference(reference);

        for (namespace, members) in &index.namespaces {
            for variants in members.types.values() {
                for (file, item) in variants {
                    let name = item.to_string();
                    let encoder = &mut Encoder {
                        output: &mut output,
                        index: &index,
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
                let class =
                    metadata::writer::TypeDefOrRef::TypeRef(output.TypeRef("System", "Object"));

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
                            index: &index,
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
                            index: &index,
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

        std::fs::write(&self.output, output.into_stream())
            .map_err(|error| Error::new(&error.to_string(), &self.output, 0, 0))
    }
}

/// Replace `#[in]` with `#[r#in]` so that syn can parse it.
///
/// `in` is a Rust keyword and cannot appear as a bare identifier in attribute
/// position. The RDL format uses `#[in]` as a parameter attribute, so we
/// normalise it to the raw-identifier form before handing the source to syn.
fn preprocess_rdl(contents: &str) -> std::borrow::Cow<'_, str> {
    if contents.contains("#[in]") {
        std::borrow::Cow::Owned(contents.replace("#[in]", "#[r#in]"))
    } else {
        std::borrow::Cow::Borrowed(contents)
    }
}

fn expand_rdl_files(paths: &[String], input_str: &[String]) -> Result<Vec<File>, Error> {
    let mut input = vec![];

    for path in paths {
        let Ok(contents) = std::fs::read_to_string(path) else {
            return Err(Error::new("failed to read binary file", path, 0, 0));
        };

        let contents = preprocess_rdl(&contents);
        let mut file = syn::parse_str::<File>(&contents).map_err(|error| {
            let start = error.span().start();
            Error::new(&error.to_string(), path, start.line, start.column)
        })?;

        file.source = path.to_string();
        input.push(file);
    }

    for contents in input_str {
        let contents = preprocess_rdl(contents);
        let mut file = syn::parse_str::<File>(&contents).map_err(|error| {
            let start = error.span().start();
            Error::new(&error.to_string(), ".rdl", start.line, start.column)
        })?;

        file.source = ".rdl".to_string();
        input.push(file);
    }

    for file in &mut input {
        for item in &mut file.items {
            resolve_winrt(item, &file.source, None)?;
        }
    }

    Ok(input)
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
        // Remaining types are not ambiguous.
        _ => {}
    }

    Ok(())
}

fn read_winrt_expected<S: syn::spanned::Spanned>(
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

fn read_winrt<S: syn::spanned::Spanned>(
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
    } else if !winrt && !win32 {
        if let Some(parent) = parent {
            if parent {
                winrt = true;
            } else {
                win32 = true;
            }
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
    reference: &metadata::reader::TypeIndex,
) -> Result<(), Error> {
    for file in input {
        for use_item in &file.uses {
            if let Some(ns) = glob_use_namespace(use_item) {
                if !index.namespaces.contains_key(&ns) && !reference.contains_namespace(&ns) {
                    let start = use_item.span().start();
                    return Err(Error::new(
                        "use namespace not found",
                        &file.source,
                        start.line,
                        start.column,
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Parses a `syn::LitInt` as a `u128`, supporting both `0x…` hex and decimal literals.
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
    fn error<S: syn::spanned::Spanned>(&self, spanned: S, message: &str) -> Error {
        let start = spanned.span().start();

        Error::new(message, &self.file.source, start.line, start.column)
    }

    fn err<T, S: syn::spanned::Spanned>(&self, spanned: S, message: &str) -> Result<T, Error> {
        Err(self.error(spanned, message))
    }

    /// Parse an optional `#[packed(N)]` attribute from `attrs`.  Returns `Some(N)` if
    /// the attribute is present and well-formed, `None` if absent, or an error if the
    /// attribute is malformed.
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

    /// Parse an optional `#[arch(...)]` attribute from `attrs`.  Returns `Some(bits)` where
    /// `bits` is the architecture bitmask (1=X86, 2=X64, 4=Arm64, combinable with `|`) if the
    /// attribute is present, `None` if absent, or an error if the attribute is malformed or uses
    /// an unknown architecture name.
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
        match ty {
            syn::Type::Path(ty) => self.encode_type_path(ty),
            syn::Type::Ptr(ty) => self.encode_type_ptr(ty),
            syn::Type::Reference(ty) => self.encode_type_reference(ty),
            syn::Type::Slice(ty) => self.encode_type_slice(ty),
            syn::Type::Array(ty) => self.encode_type_array(ty),
            rest => self.err(rest, "type not supported"),
        }
    }

    /// Like [`Self::encode_type`] but tries `attr_ns` as the primary base namespace for
    /// unqualified type names before falling back to `self.namespace`.
    fn encode_type_in_attr_ns(
        &self,
        attr_ns: &str,
        ty: &syn::Type,
    ) -> Result<metadata::Type, Error> {
        if attr_ns == self.namespace {
            return self.encode_type(ty);
        }

        if let syn::Type::Path(type_path) = ty {
            if type_path.qself.is_none() && type_path.path.leading_colon.is_none() {
                let segs: Vec<String> = type_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect();

                if !segs.is_empty() && !segs.iter().any(|s| s == "super") {
                    let name = segs.last().unwrap();
                    let candidate_ns = if segs.len() == 1 {
                        attr_ns.to_string()
                    } else {
                        format!("{}.{}", attr_ns, segs[..segs.len() - 1].join("."))
                    };

                    if self.index.contains(&candidate_ns, name)
                        || self
                            .output
                            .reference()
                            .is_some_and(|r| r.contains(&candidate_ns, name))
                    {
                        let tn = metadata::TypeName {
                            namespace: candidate_ns.clone(),
                            name: name.to_string(),
                            generics: vec![],
                        };
                        return Ok(if self.type_is_value(&candidate_ns, name) {
                            metadata::Type::ValueName(tn)
                        } else {
                            metadata::Type::ClassName(tn)
                        });
                    }
                }
            }
        }

        self.encode_type(ty)
    }

    /// Returns `true` if the named type is a value type (struct or enum), by checking
    /// the local index first and then the reference `TypeIndex`.
    fn type_is_value(&self, namespace: &str, name: &str) -> bool {
        self.index.is_value_type(namespace, name)
            || self
                .output
                .reference()
                .and_then(|r| r.get(namespace, name).next())
                .map(|def| {
                    matches!(
                        def.category(),
                        metadata::reader::TypeCategory::Struct
                            | metadata::reader::TypeCategory::Enum
                    )
                })
                .unwrap_or(false)
    }

    fn encode_type_slice(&self, ty: &syn::TypeSlice) -> Result<metadata::Type, Error> {
        Ok(metadata::Type::Array(Box::new(self.encode_type(&ty.elem)?)))
    }

    fn encode_type_array(&self, ty: &syn::TypeArray) -> Result<metadata::Type, Error> {
        Ok(metadata::Type::ArrayFixed(
            Box::new(self.encode_type(&ty.elem)?),
            self.encode_lit_int::<usize>(&ty.len)?,
        ))
    }

    fn encode_value(
        &self,
        ty: &metadata::Type,
        value: &syn::Expr,
    ) -> Result<metadata::Value, Error> {
        let value = match ty {
            metadata::Type::I8 => metadata::Value::I8(self.encode_neg_lit_int::<i8>(value)?),
            metadata::Type::U8 => metadata::Value::U8(self.encode_lit_int::<u8>(value)?),
            metadata::Type::I16 => metadata::Value::I16(self.encode_neg_lit_int::<i16>(value)?),
            metadata::Type::U16 => metadata::Value::U16(self.encode_lit_int::<u16>(value)?),
            metadata::Type::I32 => metadata::Value::I32(self.encode_neg_lit_int::<i32>(value)?),
            metadata::Type::U32 => metadata::Value::U32(self.encode_lit_int::<u32>(value)?),
            metadata::Type::I64 => metadata::Value::I64(self.encode_neg_lit_int::<i64>(value)?),
            metadata::Type::U64 => metadata::Value::U64(self.encode_lit_int::<u64>(value)?),
            metadata::Type::F32 => metadata::Value::F32(self.encode_neg_lit_float::<f32>(value)?),
            metadata::Type::F64 => metadata::Value::F64(self.encode_neg_lit_float::<f64>(value)?),
            metadata::Type::String => metadata::Value::Utf16(self.encode_lit_string(value)?),
            metadata::Type::ISize => metadata::Value::I64(self.encode_neg_lit_int::<i64>(value)?),
            metadata::Type::USize => metadata::Value::I64(self.encode_neg_lit_int::<i64>(value)?),
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
                        return self.err(value, &format!("constant type not supported: {ty:?}"))
                    }
                }
            }
            rest => return self.err(value, &format!("constant type not supported: {rest:?}")),
        };

        Ok(value)
    }

    fn rdl_underlying_type(&self, namespace: &str, name: &str) -> Option<metadata::Type> {
        let item = self.index.get(namespace, name).next()?;

        match item {
            Item::Typedef(t) => self.encode_type(&t.ty).ok(),
            Item::Struct(s) => {
                let mut fields = s.fields.iter();

                if let Some(field) = fields.next() {
                    if fields.next().is_none() {
                        return self.encode_type(&field.ty).ok();
                    }
                }

                None
            }
            _ => None,
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

    fn encode_type_reference(&self, ty: &syn::TypeReference) -> Result<metadata::Type, Error> {
        let is_mut = ty.mutability.is_some();
        let ty = self.encode_type(&ty.elem)?;

        let ty = if is_mut {
            metadata::Type::RefMut(Box::new(ty))
        } else {
            metadata::Type::RefConst(Box::new(ty))
        };

        Ok(ty)
    }

    fn encode_type_ptr(&self, ty: &syn::TypePtr) -> Result<metadata::Type, Error> {
        let is_mut = ty.mutability.is_some();
        let ty = self.encode_type(&ty.elem)?;

        let ty = match ty {
            metadata::Type::PtrMut(ty, pointers) => metadata::Type::PtrMut(ty, pointers + 1),
            metadata::Type::PtrConst(ty, pointers) => metadata::Type::PtrConst(ty, pointers + 1),
            _ => {
                if is_mut {
                    metadata::Type::PtrMut(Box::new(ty), 1)
                } else {
                    metadata::Type::PtrConst(Box::new(ty), 1)
                }
            }
        };

        Ok(ty)
    }

    fn encode_type_path(&self, ty: &syn::TypePath) -> Result<metadata::Type, Error> {
        self.encode_path(&ty.path)
    }

    fn encode_path(&self, ty: &syn::Path) -> Result<metadata::Type, Error> {
        let mut path = vec![];

        for segment in &ty.segments {
            if segment.ident == "super" {
                if path.is_empty() {
                    for part in self.namespace.split('.') {
                        path.push(part.to_string());
                    }
                }

                if path.pop().is_none() {
                    return self.err(ty, "too many leading `super` keywords");
                }
            } else {
                path.push(segment.ident.to_string());
            }
        }

        let mut generics = vec![];

        if let Some(last) = ty.segments.last() {
            if let syn::PathArguments::AngleBracketed(arguments) = &last.arguments {
                for argument in &arguments.args {
                    if let syn::GenericArgument::Type(ty) = argument {
                        generics.push(self.encode_type(ty)?);
                    }
                }
            }
        }

        if path.len() == 1 {
            if let Some(number) = self.generics.iter().position(|generic| *generic == path[0]) {
                return Ok(metadata::Type::Generic(
                    path[0].clone(),
                    number.try_into().unwrap(),
                ));
            }

            match path[0].as_str() {
                "bool" => return Ok(metadata::Type::Bool),
                "i8" => return Ok(metadata::Type::I8),
                "u8" => return Ok(metadata::Type::U8),
                "i16" => return Ok(metadata::Type::I16),
                "u16" => return Ok(metadata::Type::U16),
                "i32" => return Ok(metadata::Type::I32),
                "u32" => return Ok(metadata::Type::U32),
                "i64" => return Ok(metadata::Type::I64),
                "u64" => return Ok(metadata::Type::U64),
                "f32" => return Ok(metadata::Type::F32),
                "f64" => return Ok(metadata::Type::F64),
                "isize" => return Ok(metadata::Type::ISize),
                "usize" => return Ok(metadata::Type::USize),

                "void" => return Ok(metadata::Type::Void),
                "String" => return Ok(metadata::Type::String),
                "Object" => return Ok(metadata::Type::Object),
                "Type" => return Ok(metadata::Type::class_named("System", "Type")),
                "GUID" => return Ok(metadata::Type::value_named("System", "Guid")),
                "HRESULT" => {
                    return Ok(metadata::Type::value_named("Windows.Foundation", "HResult"))
                }

                _ => {}
            }
        }

        let (name, namespace) = path.split_last().unwrap();

        let namespace = if namespace.is_empty() {
            self.namespace.to_string()
        } else {
            namespace.join(".")
        };

        let make_type = |namespace: &str| -> Option<metadata::Type> {
            if self.index.contains(namespace, name)
                || self
                    .output
                    .reference()
                    .is_some_and(|r| r.contains(namespace, name))
            {
                let tn = metadata::TypeName {
                    namespace: namespace.to_string(),
                    name: name.to_string(),
                    generics: generics.clone(),
                };
                if self.type_is_value(namespace, name) {
                    Some(metadata::Type::ValueName(tn))
                } else {
                    Some(metadata::Type::ClassName(tn))
                }
            } else {
                None
            }
        };

        if let Some(ty) = make_type(&namespace) {
            return Ok(ty);
        }

        let namespace = format!("{}.{}", self.namespace, namespace);

        if let Some(ty) = make_type(&namespace) {
            return Ok(ty);
        }

        // Last resort: try glob use declarations
        for use_item in &self.file.uses {
            if let Some(ns) = glob_use_namespace(use_item) {
                if let Some(ty) = make_type(&ns) {
                    return Ok(ty);
                }
            }
        }

        Err(self.error(ty, "type not found"))
    }

    fn encode_return_type(&self, ty: &syn::ReturnType) -> Result<metadata::Type, Error> {
        match ty {
            syn::ReturnType::Type(_, ty) => self.encode_type(ty),
            _ => Ok(metadata::Type::Void),
        }
    }

    /// Validates that a resolved `metadata::Type` is a WinRT-compatible type.
    ///
    /// WinRT types may not refer to non-WinRT types in fields, parameters, return types,
    /// or anywhere else.  Primitive types and generic type parameters are always valid.
    /// Named types (structs, enums, interfaces, …) are checked against the local index
    /// first, then against any loaded reference metadata.
    fn validate_type_is_winrt<S: syn::spanned::Spanned + quote::ToTokens>(
        &self,
        span: &S,
        ty: &metadata::Type,
    ) -> Result<(), Error> {
        match ty {
            metadata::Type::ValueName(tn) | metadata::Type::ClassName(tn) => {
                // Recursively validate generic type arguments.
                for generic_ty in &tn.generics {
                    self.validate_type_is_winrt(span, generic_ty)?;
                }

                // Check the local RDL index first.
                if let Some(is_winrt) = self.index.is_winrt(&tn.namespace, &tn.name) {
                    if !is_winrt {
                        return self.err(span, "WinRT types cannot refer to non-WinRT types");
                    }
                } else if let Some(reference) = self.output.reference() {
                    // Fall back to the external reference metadata.
                    if let Some(def) = reference.get(&tn.namespace, &tn.name).next() {
                        if !def
                            .flags()
                            .contains(metadata::TypeAttributes::WindowsRuntime)
                        {
                            return self.err(span, "WinRT types cannot refer to non-WinRT types");
                        }
                    }
                    // If the type isn't found in the reference either it is a hard-coded
                    // system alias (e.g. System.Guid) and is considered WinRT-compatible.
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
            // Primitives (Bool, I8, U8, …), String, Object, Void, Generic, … are always OK.
            _ => {}
        }

        Ok(())
    }
}

/// Parses a `#[arch(...)]` expression into an architecture bitmask.
///
/// Accepts:
/// - A single identifier: `X86` → 1, `X64` → 2, `Arm64` → 4.
/// - A bitwise-OR combination: `X86 | X64` → 3, `X64 | Arm64` → 6, etc.
///
/// Returns `None` when the expression contains an unknown architecture name or an
/// unsupported expression form.
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

/// Maps a well-known architecture name to its bitmask bit.
///   `X86`   → 1
///   `X64`   → 2
///   `Arm64` → 4
fn arch_name_to_bits(name: &str) -> Option<i32> {
    match name {
        "X86" => Some(1),
        "X64" => Some(2),
        "Arm64" => Some(4),
        _ => None,
    }
}

/// Builds a `syn::Signature` with all the optional fields (`constness`, `asyncness`,
/// `unsafety`, `abi`) set to `None`.  Shared by `Callback`, `Fn`, `Delegate`, and `Method`.
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

/// Parse the function parameter list including variadic handling.
/// Returns the inputs and an optional variadic marker.
pub(crate) fn parse_fn_inputs(
    content: &syn::parse::ParseBuffer,
) -> syn::Result<(
    syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    Option<syn::Variadic>,
)> {
    let mut args = syn::punctuated::Punctuated::new();
    let mut variadic = None;

    while !content.is_empty() {
        // Use a fork to peek past any outer attrs to see if we have `...` (variadic).
        let fork = content.fork();
        let _ = fork.call(syn::Attribute::parse_outer);
        if fork.peek(syn::Token![...]) {
            // It's variadic — consume the attrs and dots from the real stream.
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

        // Regular fn arg — syn handles inner attrs itself.
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

/// Parse an optional `-> #[attr]* Type` return type, extracting any outer attributes
/// that appear between `->` and the type.  Returns `(return_type, return_attrs)`.
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

fn glob_use_namespace(use_item: &syn::ItemUse) -> Option<String> {
    fn extract(tree: &syn::UseTree, parts: &mut Vec<String>) -> bool {
        match tree {
            syn::UseTree::Path(p) => {
                parts.push(p.ident.to_string());
                extract(&p.tree, parts)
            }
            syn::UseTree::Glob(_) => true,
            _ => false,
        }
    }
    let mut parts = vec![];
    if extract(&use_item.tree, &mut parts) && !parts.is_empty() {
        Some(parts.join("."))
    } else {
        None
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
        .input_str(
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
        .output(&output.to_string_lossy())
        .write()
        .unwrap();
}
