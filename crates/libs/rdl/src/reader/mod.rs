mod attribute;
mod attribute_ref;
mod class;
mod r#const;
mod delegate;
mod r#enum;
mod file;
mod r#fn;
pub(super) mod guid;
mod index;
mod interface;
mod item;
mod method;
mod module;
mod param;
mod r#struct;
mod union;

use super::*;
use attribute::*;
use attribute_ref::*;
use class::*;
use delegate::*;
use file::*;
use index::*;
use interface::*;
use item::*;
use method::*;
use module::*;
use param::*;
use r#const::*;
use r#enum::*;
use r#fn::*;
use r#struct::*;
use union::*;
use windows_metadata as metadata;

#[derive(Default)]
pub struct Reader {
    input: Vec<String>,
    input_str: Vec<String>,
    reference: Vec<String>,
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

    // TODO: is this really necessary or can we assume that missing types will be resolved "later"?
    pub fn reference(&mut self, reference: &str) -> &mut Self {
        self.reference.push(reference.to_string());
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

        let input = expand_input(&self.input, &self.input_str)?;

        let mut index = Index::new();

        for file in &input {
            for item in &file.items {
                index.insert(file, "", item);
            }
        }

        let mut reference = vec![];

        for file_name in &self.reference {
            reference.push(
                metadata::reader::File::read(file_name)
                    .ok_or_else(|| Error::new("invalid reference", file_name, 0, 0))?,
            );
        }

        let reference = metadata::reader::TypeIndex::new(reference);
        validate_use_declarations(&input, &index, &reference)?;
        let output = encode(index, &reference)?;

        std::fs::write(&self.output, output)
            .map_err(|error| Error::new(&error.to_string(), &self.output, 0, 0))
    }
}

fn expand_input(input: &[String], input_str: &[String]) -> Result<Vec<File>, Error> {
    #[track_caller]
    fn expand_input(result: &mut Vec<String>, input: &str) -> Result<(), Error> {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .map_err(|_| Error::new("failed to read directory", input, 0, 0))?
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("rdl"))
                {
                    result.push(path.to_string_lossy().replace('\\', "/"));
                }
            }

            if result.len() == prev_len {
                return Err(Error::new(
                    "failed to find .rdl files in directory",
                    input,
                    0,
                    0,
                ));
            }
        } else {
            result.push(input.to_string());
        }

        Ok(())
    }

    let mut paths = vec![];

    for input in input {
        expand_input(&mut paths, input)?;
    }

    let mut input = vec![];

    for path in &paths {
        let Ok(contents) = std::fs::read_to_string(path) else {
            return Err(Error::new("failed to read binary file", path, 0, 0));
        };

        let mut file = syn::parse_str::<File>(&contents).map_err(|error| {
            let start = error.span().start();
            Error::new(&error.to_string(), path, start.line, start.column)
        })?;

        file.source = path.to_string();
        input.push(file);
    }

    for contents in input_str {
        let mut file = syn::parse_str::<File>(contents).map_err(|error| {
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
        Item::Delegate(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
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

fn encode(index: Index, reference: &metadata::reader::TypeIndex) -> Result<Vec<u8>, Error> {
    let mut output = metadata::writer::File::new("");

    for (namespace, members) in &index.namespaces {
        for (name, (file, item)) in &members.types {
            item.encode(&mut output, &index, reference, file, namespace, name)?;
        }

        if !members.functions.is_empty() || !members.constants.is_empty() {
            let class = metadata::writer::TypeDefOrRef::TypeRef(output.TypeRef("System", "Object"));

            output.TypeDef(
                namespace,
                "Apis",
                class,
                metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed,
            );

            for (name, (file, item)) in &members.functions {
                item.encode(&mut output, &index, reference, file, namespace, name)?;
            }

            for (name, (file, item)) in &members.constants {
                item.encode(&mut output, &index, reference, file, namespace, name)?;
            }
        }
    }

    Ok(output.into_stream())
}

fn err<T, S: syn::spanned::Spanned>(
    spanned: S,
    source_file: &str,
    message: &str,
) -> Result<T, Error> {
    let start = spanned.span().start();

    Err(Error::new(message, source_file, start.line, start.column))
}

struct Encoder<'a> {
    output: &'a mut metadata::writer::File,
    index: &'a Index<'a>,
    reference: &'a metadata::reader::TypeIndex,
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
}

fn encode_type(encoder: &Encoder, ty: &syn::Type) -> Result<metadata::Type, Error> {
    match ty {
        syn::Type::Path(ty) => encode_type_path(encoder, ty),
        syn::Type::Ptr(ty) => encode_type_ptr(encoder, ty),
        syn::Type::Reference(ty) => encode_type_reference(encoder, ty),
        syn::Type::Slice(ty) => encode_type_slice(encoder, ty),
        syn::Type::Array(ty) => encode_type_array(encoder, ty),
        rest => todo!("{rest:?}"),
    }
}

/// Like [`encode_type`] but tries `attr_ns` as the primary base namespace for
/// unqualified type names before falling back to `encoder.namespace`.
///
/// This is needed when resolving constructor-parameter types for an attribute
/// that lives in a namespace other than the one currently being encoded.  For
/// example, `MarshalingBehaviorAttribute` (in `Windows.Foundation.Metadata`)
/// takes a `MarshalingType` parameter; if the *calling* item is in
/// `Windows.Something`, the plain name `MarshalingType` must still be looked
/// up in `Windows.Foundation.Metadata`, not in `Windows.Something`.
fn encode_type_in_attr_ns(
    encoder: &Encoder,
    attr_ns: &str,
    ty: &syn::Type,
) -> Result<metadata::Type, Error> {
    // Fast path: no special handling needed when the namespaces already agree.
    if attr_ns == encoder.namespace {
        return encode_type(encoder, ty);
    }

    // For a plain relative (non-`::`) path we first attempt to resolve it in
    // `attr_ns`.  If the lookup succeeds we return the fully-qualified type
    // immediately; otherwise we delegate to the regular `encode_type` so that
    // builtin aliases ("u32", "String", …) and types genuinely local to the
    // caller's namespace are still handled correctly.
    if let syn::Type::Path(type_path) = ty {
        if type_path.qself.is_none() && type_path.path.leading_colon.is_none() {
            let segs: Vec<String> = type_path
                .path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect();

            // Paths containing `super` need the regular encoder context.
            if !segs.is_empty() && !segs.iter().any(|s| s == "super") {
                let name = segs.last().unwrap();
                let candidate_ns = if segs.len() == 1 {
                    attr_ns.to_string()
                } else {
                    format!("{}.{}", attr_ns, segs[..segs.len() - 1].join("."))
                };

                if encoder.index.contains(&candidate_ns, name)
                    || encoder.reference.contains(&candidate_ns, name)
                {
                    return Ok(metadata::Type::Name(metadata::TypeName {
                        namespace: candidate_ns,
                        name: name.to_string(),
                        generics: vec![],
                    }));
                }
            }
        }
    }

    encode_type(encoder, ty)
}

fn encode_type_slice(encoder: &Encoder, ty: &syn::TypeSlice) -> Result<metadata::Type, Error> {
    Ok(metadata::Type::Array(Box::new(encode_type(
        encoder, &ty.elem,
    )?)))
}

fn encode_type_array(encoder: &Encoder, ty: &syn::TypeArray) -> Result<metadata::Type, Error> {
    Ok(metadata::Type::ArrayFixed(
        Box::new(encode_type(encoder, &ty.elem)?),
        encode_lit_int::<usize>(encoder, &ty.len)?,
    ))
}

fn encode_value(
    encoder: &Encoder,
    ty: &metadata::Type,
    value: &syn::Expr,
) -> Result<metadata::Value, Error> {
    let value = match ty {
        metadata::Type::I8 => metadata::Value::I8(encode_neg_lit_int::<i8>(encoder, value)?),
        metadata::Type::U8 => metadata::Value::U8(encode_lit_int::<u8>(encoder, value)?),
        metadata::Type::I16 => metadata::Value::I16(encode_neg_lit_int::<i16>(encoder, value)?),
        metadata::Type::U16 => metadata::Value::U16(encode_lit_int::<u16>(encoder, value)?),
        metadata::Type::I32 => metadata::Value::I32(encode_neg_lit_int::<i32>(encoder, value)?),
        metadata::Type::U32 => metadata::Value::U32(encode_lit_int::<u32>(encoder, value)?),
        metadata::Type::I64 => metadata::Value::I64(encode_neg_lit_int::<i64>(encoder, value)?),
        metadata::Type::U64 => metadata::Value::U64(encode_lit_int::<u64>(encoder, value)?),
        metadata::Type::F32 => metadata::Value::F32(encode_neg_lit_float::<f32>(encoder, value)?),
        metadata::Type::F64 => metadata::Value::F64(encode_neg_lit_float::<f64>(encoder, value)?),
        metadata::Type::String => metadata::Value::Utf16(encode_lit_string(encoder, value)?),
        rest => todo!("{rest:?}"),
    };

    Ok(value)
}

fn encode_neg_lit_int<T>(encoder: &Encoder, expr: &syn::Expr) -> Result<T, Error>
where
    T: std::str::FromStr + std::ops::Neg<Output = T>,
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
            }) => int.base10_parse().ok().map(|value: T| -value),
            _ => None,
        },
        _ => None,
    };

    value.ok_or_else(|| encoder.error(expr, "value not valid"))
}

fn encode_lit_int<T>(encoder: &Encoder, expr: &syn::Expr) -> Result<T, Error>
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

    value.ok_or_else(|| encoder.error(expr, "value not valid"))
}

fn encode_neg_lit_float<T>(encoder: &Encoder, expr: &syn::Expr) -> Result<T, Error>
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

    value.ok_or_else(|| encoder.error(expr, "value not valid"))
}

fn encode_lit_string(encoder: &Encoder, expr: &syn::Expr) -> Result<String, Error> {
    let value = match expr {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(string),
            ..
        }) => Some(string.value()),
        _ => None,
    };

    value.ok_or_else(|| encoder.error(expr, "value not valid"))
}

fn encode_type_reference(
    encoder: &Encoder,
    ty: &syn::TypeReference,
) -> Result<metadata::Type, Error> {
    let is_mut = ty.mutability.is_some();
    let ty = encode_type(encoder, &ty.elem)?;

    let ty = if is_mut {
        metadata::Type::RefMut(Box::new(ty))
    } else {
        metadata::Type::RefConst(Box::new(ty))
    };

    Ok(ty)
}

fn encode_type_ptr(encoder: &Encoder, ty: &syn::TypePtr) -> Result<metadata::Type, Error> {
    let is_mut = ty.mutability.is_some();
    let ty = encode_type(encoder, &ty.elem)?;

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

fn encode_type_path(encoder: &Encoder, ty: &syn::TypePath) -> Result<metadata::Type, Error> {
    encode_path(encoder, &ty.path)
}

fn encode_path(encoder: &Encoder, ty: &syn::Path) -> Result<metadata::Type, Error> {
    let mut path = vec![];

    for segment in &ty.segments {
        if segment.ident == "super" {
            if path.is_empty() {
                for part in encoder.namespace.split('.') {
                    path.push(part.to_string());
                }
            }

            if path.pop().is_none() {
                return encoder.err(ty, "too many leading `super` keywords");
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
                    generics.push(encode_type(encoder, ty)?);
                }
            }
        }
    }

    if path.len() == 1 {
        if let Some(number) = encoder
            .generics
            .iter()
            .position(|generic| *generic == path[0])
        {
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
            "IInspectable" => return Ok(metadata::Type::Object),
            "Type" => return Ok(metadata::Type::named("System", "Type")),
            "GUID" => return Ok(("System", "Guid").into()),
            "HRESULT" => return Ok(("Windows.Foundation", "HResult").into()),

            _ => {}
        }
    }

    let (name, namespace) = path.split_last().unwrap();

    let namespace = if namespace.is_empty() {
        encoder.namespace.to_string()
    } else {
        namespace.join(".")
    };

    let contains = |namespace: &str| -> Option<metadata::Type> {
        if encoder.index.contains(namespace, name) || encoder.reference.contains(namespace, name) {
            Some(metadata::Type::Name(metadata::TypeName {
                namespace: namespace.to_string(),
                name: name.to_string(),
                generics: generics.clone(),
            }))
        } else {
            None
        }
    };

    if let Some(ty) = contains(&namespace) {
        return Ok(ty);
    }

    let namespace = format!("{}.{}", encoder.namespace, namespace);

    if let Some(ty) = contains(&namespace) {
        return Ok(ty);
    }

    // Last resort: try glob use declarations
    for use_item in &encoder.file.uses {
        if let Some(ns) = glob_use_namespace(use_item) {
            if let Some(ty) = contains(&ns) {
                return Ok(ty);
            }
        }
    }

    Err(encoder.error(ty, "type not found"))
}

fn encode_return_type(encoder: &Encoder, ty: &syn::ReturnType) -> Result<metadata::Type, Error> {
    match ty {
        syn::ReturnType::Type(_, ty) => encode_type(encoder, ty),
        _ => Ok(metadata::Type::Void),
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
#[should_panic(
    expected = r#"{ message: "use namespace not found", file_name: ".rdl", line: 2, column: 0 }"#
)]
fn use_glob_invalid_path() {
    Reader::new()
        .input_str(
            r#"
use NonExistent::*;

#[winrt]
mod Test {
    struct Thing {
        a: NoSuchType,
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "type not found", file_name: ".rdl", line: 7, column: 11 }"#
)]
fn use_glob_unresolved_type() {
    Reader::new()
        .input_str(
            r#"
use Other::*;

#[winrt]
mod Test {
    struct Thing {
        a: NoSuchType,
    }
}

#[winrt]
mod Other {
    struct ExistingThing {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
fn use_glob_resolves_type() {
    let output = std::env::temp_dir().join("windows_rdl_use_glob_resolves_type.winmd");

    Reader::new()
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
