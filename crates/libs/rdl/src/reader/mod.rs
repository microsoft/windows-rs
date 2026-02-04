mod r#enum;
mod index;
mod interface;
mod r#struct;
mod union;
mod r#fn;

use r#fn::*;
use super::*;
use index::*;
use interface::*;
use r#enum::*;
use r#struct::*;
use union::*;
use windows_metadata as metadata;

#[derive(Default)]
pub struct Reader {
    input: Vec<String>,
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

        let mut input = vec![];

        for source_file in &self.input {
            let contents = std::fs::read_to_string(source_file)
                .map_err(|error| Error::new(&error.to_string(), source_file, 0, 0))?;

            let mut file = syn::parse_str::<syntax::File>(&contents).map_err(|error| {
                let start = error.span().start();
                Error::new(&error.to_string(), source_file, start.line, start.column)
            })?;

            for item in &mut file.items {
                resolve_winrt(item, source_file, None)?;
            }

            input.push(file);
        }

        let mut index = Index::new();

        // TODO: should also be able to read nested source files via the `mod Nested;` syntax?
        for (source_file, file) in self.input.iter().zip(input.iter_mut()) {
            while let Some(item) = file.items.pop() {
                index.insert(source_file, "", item);
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

    for (namespace, name, source_file, item) in index.items() {
            encode_item(
                &mut output,
                &index,
                reference,
                source_file,
                namespace,
                name,
                item,
            )?;
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
    };

    match item {
        syntax::Item::Struct(ty) => encode_struct(encoder, ty),
        syntax::Item::Enum(ty) => encode_enum(encoder, ty),
        syntax::Item::Interface(ty) => encode_interface(encoder, ty),
        syntax::Item::Union(ty) => encode_union(encoder, ty),
        syntax::Item::Fn(ty) => encode_fn(encoder, ty),
        rest => todo!("{rest:?}"),
    }
}

fn encode_type(encoder: &Encoder, ty: &syn::Type) -> Result<metadata::Type, Error> {
    match ty {
        syn::Type::Path(ty) => encode_type_path(encoder, ty),
        syn::Type::Ptr(ty) => encode_type_ptr(encoder, ty),
        syn::Type::Reference(ty) => encode_type_reference(encoder, ty),
        rest => todo!("{rest:?}"),
    }
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
    let mut name = String::new();

    for segment in &ty.segments {
        if !name.is_empty() {
            name.push('.');
        }

        name.push_str(&segment.ident.to_string());
    }

    match name.as_str() {
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
        "String" => return Ok(metadata::Type::String),
        _ => {}
    }

    let (type_namespace, type_name) = name
        .rsplit_once('.')
        .unwrap_or_else(|| (encoder.namespace, &name));

    // TODO: resolve any "super::" path segments...

    if encoder.index.contains(type_namespace, type_name) {
        return Ok(metadata::Type::named(type_namespace, type_name));
    }

    let nested_namespace = format!("{}.{}", encoder.namespace, type_namespace);

    if encoder.index.contains(&nested_namespace, type_name) {
        return Ok(metadata::Type::named(&nested_namespace, type_name));
    }

    if encoder.reference.contains(type_namespace, type_name) {
        return Ok(metadata::Type::named(type_namespace, type_name));
    }

    encoder.err(ty, "type not found")
}

fn encode_return_type(encoder: &Encoder, ty: &syn::ReturnType) -> Result<metadata::Type, Error> {
    match ty {
        syn::ReturnType::Type(_, ty) => encode_type(encoder, ty),
        _ => Ok(metadata::Type::Void),
    }
}
