mod attribute;
mod class;
mod r#const;
mod delegate;
mod r#enum;
mod r#fn;
mod index;
mod interface;
mod param;
mod r#struct;
mod union;

use super::*;
use attribute::*;
use class::*;
use delegate::*;
use index::*;
use interface::*;
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

        let mut input = expand_input(&self.input, &self.input_str)?;

        let mut index = Index::new();

        for file in input.iter_mut() {
            while let Some(item) = file.items.pop() {
                index.insert(&file.source, "", item);
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

        let output = encode(index, &reference)?;

        std::fs::write(&self.output, output)
            .map_err(|error| Error::new(&error.to_string(), &self.output, 0, 0))
    }
}

fn expand_input(input: &[String], input_str: &[String]) -> Result<Vec<syntax::File>, Error> {
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

        let mut file = syn::parse_str::<syntax::File>(&contents).map_err(|error| {
            let start = error.span().start();
            Error::new(&error.to_string(), path, start.line, start.column)
        })?;

        file.source = path.to_string();
        input.push(file);
    }

    for contents in input_str {
        let mut file = syn::parse_str::<syntax::File>(contents).map_err(|error| {
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

fn resolve_winrt(
    item: &mut syntax::Item,
    source_file: &str,
    parent: Option<bool>,
) -> Result<(), Error> {
    match item {
        syntax::Item::Enum(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        syntax::Item::Interface(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        syntax::Item::Struct(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        syntax::Item::Delegate(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        syntax::Item::Attribute(item) => {
            item.winrt = read_winrt_expected(source_file, &item.token, &item.attrs, parent)?;
        }
        syntax::Item::Module(item) => {
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

fn encode(index: Index, reference: &metadata::reader::TypeIndex) -> Result<Vec<u8>, Error> {
    let mut output = metadata::writer::File::new("");

    for (namespace, members) in &index.namespaces {
        for (name, (source, item)) in &members.types {
            encode_item(
                &mut output,
                &index,
                reference,
                source,
                namespace,
                name,
                item,
            )?;
        }

        if !members.functions.is_empty() || !members.constants.is_empty() {
            let class = metadata::writer::TypeDefOrRef::TypeRef(output.TypeRef("System", "Object"));

            output.TypeDef(
                namespace,
                "Apis",
                class,
                metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed,
            );

            for (name, (source, item)) in &members.functions {
                encode_item(
                    &mut output,
                    &index,
                    reference,
                    source,
                    namespace,
                    name,
                    item,
                )?;
            }

            for (name, (source, item)) in &members.constants {
                encode_item(
                    &mut output,
                    &index,
                    reference,
                    source,
                    namespace,
                    name,
                    item,
                )?;
            }
        }
    }

    Ok(output.into_stream())
}

struct Encoder<'a> {
    output: &'a mut metadata::writer::File,
    index: &'a Index<'a>,
    reference: &'a metadata::reader::TypeIndex,
    source_file: &'a str,
    namespace: &'a str,
    name: &'a str,
    generics: Vec<String>,
}

impl Encoder<'_> {
    fn error<S: syn::spanned::Spanned>(&self, spanned: S, message: &str) -> Error {
        let start = spanned.span().start();

        Error::new(message, self.source_file, start.line, start.column)
    }

    fn err<T, S: syn::spanned::Spanned>(&self, spanned: S, message: &str) -> Result<T, Error> {
        Err(self.error(spanned, message))
    }
}

fn encode_item(
    output: &mut metadata::writer::File,
    index: &Index,
    reference: &metadata::reader::TypeIndex,
    source_file: &str,
    namespace: &str,
    name: &str,
    item: &syntax::Item,
) -> Result<(), Error> {
    let encoder = &mut Encoder {
        output,
        index,
        reference,
        source_file,
        namespace,
        name,
        generics: vec![],
    };

    match item {
        syntax::Item::Struct(ty) => encode_struct(encoder, ty),
        syntax::Item::Enum(ty) => encode_enum(encoder, ty),
        syntax::Item::Interface(ty) => encode_interface(encoder, ty),
        syntax::Item::Union(ty) => encode_union(encoder, ty),
        syntax::Item::Fn(ty) => encode_fn(encoder, ty),
        syntax::Item::Const(ty) => encode_const(encoder, ty),
        syntax::Item::Class(ty) => encode_class(encoder, ty),
        syntax::Item::Delegate(ty) => encode_delegate(encoder, ty),
        syntax::Item::Attribute(ty) => encode_attribute(encoder, ty),
        rest => todo!("{rest:?}"),
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
            "HSTRING" => return Ok(metadata::Type::String),
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

    contains(&namespace).ok_or_else(|| encoder.error(ty, "type not found"))
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
