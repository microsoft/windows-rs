use super::*;
use metadata::{BufferInfo, BufferSize};
use syn::ext::IdentExt;

pub(crate) fn is_buffer_count<I: IntoIterator<Item = metadata::Type>>(
    count: &BufferSize,
    types: &[metadata::Type],
    underlying: impl Fn(metadata::Type) -> I,
) -> bool {
    let index = match *count {
        BufferSize::Constant(_) => return true,
        BufferSize::Parameter(index) | BufferSize::Dereference(index) => index,
    };
    types.get(index as usize).is_some_and(|ty| {
        matches_underlying(
            ty.clone(),
            matches!(count, BufferSize::Dereference(_)),
            underlying,
            |ty| {
                matches!(
                    ty,
                    metadata::Type::I8
                        | metadata::Type::U8
                        | metadata::Type::I16
                        | metadata::Type::U16
                        | metadata::Type::I32
                        | metadata::Type::U32
                        | metadata::Type::I64
                        | metadata::Type::U64
                        | metadata::Type::ISize
                        | metadata::Type::USize
                )
            },
        )
    })
}

pub(crate) fn pointee(ty: metadata::Type) -> Option<metadata::Type> {
    match ty {
        metadata::Type::PtrMut(inner, depth) if depth > 1 => {
            Some(metadata::Type::PtrMut(inner, depth - 1))
        }
        metadata::Type::PtrConst(inner, depth) if depth > 1 => {
            Some(metadata::Type::PtrConst(inner, depth - 1))
        }
        metadata::Type::PtrMut(inner, _)
        | metadata::Type::PtrConst(inner, _)
        | metadata::Type::RefMut(inner)
        | metadata::Type::RefConst(inner) => Some(*inner),
        _ => None,
    }
}

pub(crate) fn is_buffer_storage<I: IntoIterator<Item = metadata::Type>>(
    ty: metadata::Type,
    indirect: bool,
    underlying: impl Fn(metadata::Type) -> I,
) -> bool {
    matches_underlying(ty, indirect, underlying, |ty| {
        matches!(
            ty,
            metadata::Type::PtrMut(..)
                | metadata::Type::PtrConst(..)
                | metadata::Type::RefMut(_)
                | metadata::Type::RefConst(_)
                | metadata::Type::Array(_)
                | metadata::Type::ArrayFixed(..)
        )
    })
}

fn matches_underlying<I: IntoIterator<Item = metadata::Type>>(
    ty: metadata::Type,
    indirect: bool,
    underlying: impl Fn(metadata::Type) -> I,
    predicate: impl Fn(metadata::Type) -> bool,
) -> bool {
    let mut types = underlying(ty).into_iter().peekable();
    types.peek().is_some()
        && types.all(|ty| {
            if indirect {
                let Some(ty) = pointee(ty) else {
                    return false;
                };
                let mut types = underlying(ty).into_iter().peekable();
                types.peek().is_some() && types.all(&predicate)
            } else {
                predicate(ty)
            }
        })
}

fn parameter(input: syn::parse::ParseStream, names: &[&str]) -> syn::Result<u16> {
    let name = input.call(syn::Ident::parse_any)?.unraw().to_string();
    let mut matches = names
        .iter()
        .enumerate()
        .filter(|(_, candidate)| **candidate == name);
    let (index, _) = matches
        .next()
        .ok_or_else(|| input.error(format!("unknown buffer count `{name}`")))?;
    if matches.next().is_some() {
        return Err(input.error(format!("ambiguous buffer count `{name}`")));
    }
    u16::try_from(index).map_err(|_| input.error("buffer count index exceeds metadata range"))
}

fn parameter_ident(index: u16, names: &[&str]) -> Option<TokenStream> {
    use syn::parse::Parser;
    let name = names.get(index as usize)?;
    if names.iter().filter(|candidate| *candidate == name).count() != 1 {
        return None;
    }
    syn::Ident::parse_any.parse_str(name).ok()?;
    Some(write_ident(name))
}

pub(crate) fn parse_buffer_info(attr: &syn::Attribute, names: &[&str]) -> syn::Result<BufferInfo> {
    fn count(input: syn::parse::ParseStream, names: &[&str]) -> syn::Result<BufferSize> {
        if input.peek(syn::LitInt) {
            let value: syn::LitInt = input.parse()?;
            if !value.suffix().is_empty() {
                return Err(input.error("buffer count must be an unsuffixed integer"));
            }
            Ok(BufferSize::Constant(value.base10_parse()?))
        } else if input.peek(syn::Token![return]) {
            Err(input.error("buffer count requires a constant, parameter or *parameter"))
        } else if input.peek(syn::Token![*]) {
            input.parse::<syn::Token![*]>()?;
            Ok(BufferSize::Dereference(parameter(input, names)?))
        } else {
            Ok(BufferSize::Parameter(parameter(input, names)?))
        }
    }
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let fork = input.fork();
        let bytes = fork.parse::<syn::Ident>().is_ok_and(|name| name == "bytes")
            && fork.peek(syn::token::Paren);
        let count = if bytes {
            input.parse::<syn::Ident>()?;
            let inner;
            syn::parenthesized!(inner in input);
            let count = count(&inner, names)?;
            if !inner.is_empty() {
                return Err(
                    inner.error("buffer count requires a constant, parameter or *parameter")
                );
            }
            count
        } else {
            count(input, names)?
        };
        if !input.is_empty() {
            return Err(input.error("buffer count requires a constant, parameter or *parameter"));
        }
        Ok(BufferInfo { count, bytes })
    })
}

pub fn write_buffer_info(info: &BufferInfo, names: &[&str]) -> Option<TokenStream> {
    let count = match info.count {
        BufferSize::Constant(value) => {
            let value = Literal::u64_unsuffixed(value);
            quote! { #value }
        }
        BufferSize::Parameter(index) | BufferSize::Dereference(index) => {
            let name = parameter_ident(index, names)?;
            if matches!(info.count, BufferSize::Dereference(_)) {
                quote! { *#name }
            } else {
                name
            }
        }
    };
    let count = if info.bytes {
        quote! { bytes(#count) }
    } else {
        count
    };
    Some(quote! { #[buffer(#count)] })
}
