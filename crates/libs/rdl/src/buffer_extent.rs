use super::*;
use metadata::{BufferAccess, BufferCount, BufferExtent, BufferPhase, BufferUnits};
use syn::ext::IdentExt;

pub(crate) fn parse_buffer_extent(
    attr: &syn::Attribute,
    names: &[&str],
    field: bool,
) -> syn::Result<BufferExtent> {
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let phase = if field {
            BufferPhase::Value
        } else {
            let phase: syn::Ident = input.parse()?;
            let phase = match phase.to_string().as_str() {
                "entry" => BufferPhase::Entry,
                "success" => BufferPhase::Success,
                _ => return Err(input.error("expected entry or success for parameters")),
            };
            input.parse::<syn::Token![,]>()?;
            phase
        };
        let mut extent = BufferExtent::new(
            if attr.path().is_ident("readable") {
                BufferAccess::Read
            } else {
                BufferAccess::Write
            },
            phase,
            BufferUnits::Elements,
            BufferCount::Constant(0),
        );
        if wrapper(input, "bytes") {
            input.parse::<syn::Ident>()?;
            let inner;
            syn::parenthesized!(inner in input);
            parse_count(&inner, names, field, &mut extent)?;
            if !inner.is_empty() {
                return Err(inner.error("unexpected byte-count expression"));
            }
            extent.units = BufferUnits::Bytes;
        } else {
            parse_count(input, names, field, &mut extent)?;
        }
        while input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            if !input.is_empty() {
                let flag: syn::Ident = input.parse()?;
                match flag.to_string().as_str() {
                    "indirect" if !extent.indirect => extent.indirect = true,
                    "nullable" if !extent.nullable => extent.nullable = true,
                    _ => return Err(input.error("unknown or repeated extent flag")),
                }
            }
        }
        if !input.is_empty() || !extent.is_valid() {
            return Err(input.error("invalid buffer extent"));
        }
        Ok(extent)
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

fn wrapper(input: syn::parse::ParseStream, name: &str) -> bool {
    let fork = input.fork();
    fork.parse::<syn::Ident>().is_ok_and(|ident| ident == name) && fork.peek(syn::token::Paren)
}

fn parse_count(
    input: syn::parse::ParseStream,
    names: &[&str],
    field: bool,
    extent: &mut BufferExtent,
) -> syn::Result<()> {
    if input.peek(syn::Ident) {
        let ident = input.fork().parse::<syn::Ident>()?;
        if wrapper(input, "on_entry") || wrapper(input, "or") || wrapper(input, "if_present") {
            input.parse::<syn::Ident>()?;
            let inner;
            syn::parenthesized!(inner in input);
            parse_count(&inner, names, field, extent)?;
            if ident == "on_entry" {
                if extent.count_on_entry {
                    return Err(inner.error("duplicate on_entry count"));
                }
                extent.count_on_entry = true;
            } else if ident == "if_present" {
                if extent.count_optional {
                    return Err(inner.error("duplicate if_present count"));
                }
                extent.count_optional = true;
            } else {
                inner.parse::<syn::Token![,]>()?;
                if extent.null_fallback.is_some() {
                    return Err(inner.error("duplicate count fallback"));
                }
                extent.null_fallback = Some(parameter(&inner, names)?);
            }
            if !inner.is_empty() {
                return Err(inner.error("unexpected count expression"));
            }
            return Ok(());
        }
    }
    extent.count = if input.peek(syn::LitInt) {
        BufferCount::Constant(input.parse::<syn::LitInt>()?.base10_parse()?)
    } else if input.peek(syn::Token![return]) {
        input.parse::<syn::Token![return]>()?;
        BufferCount::Return
    } else if input.peek(syn::Token![*]) && !field {
        input.parse::<syn::Token![*]>()?;
        BufferCount::Dereference(parameter(input, names)?)
    } else {
        let index = parameter(input, names)?;
        if field {
            BufferCount::Field(names[index as usize].into())
        } else {
            BufferCount::Parameter(index)
        }
    };
    if input.peek(syn::Token![+]) {
        input.parse::<syn::Token![+]>()?;
        extent.offset = input.parse::<syn::LitInt>()?.base10_parse()?;
        if extent.offset == 0 {
            return Err(input.error("zero count offset is redundant"));
        }
    }
    Ok(())
}

/// Renders a validated extent using names from its owning signature.
pub fn write_buffer_extent(extent: &BufferExtent, names: &[&str]) -> Option<TokenStream> {
    if !extent.is_valid() {
        return None;
    }
    let name = match extent.access {
        BufferAccess::Read => quote! { readable },
        BufferAccess::Write => quote! { writable },
    };
    let phase = match extent.phase {
        BufferPhase::Entry => quote! { entry, },
        BufferPhase::Success => quote! { success, },
        BufferPhase::Value => TokenStream::new(),
    };
    let mut count = match &extent.count {
        BufferCount::Constant(value) => {
            let value = Literal::u64_unsuffixed(*value);
            quote! { #value }
        }
        BufferCount::Parameter(index) | BufferCount::Dereference(index) => {
            let name = parameter_ident(*index, names)?;
            if matches!(extent.count, BufferCount::Dereference(_)) {
                quote! { *#name }
            } else {
                quote! { #name }
            }
        }
        BufferCount::Field(name) => extent_ident(name)?,
        BufferCount::Return => quote! { return },
    };
    if extent.offset != 0 {
        let offset = Literal::u64_unsuffixed(extent.offset);
        count = quote! { #count + #offset };
    }
    if let Some(index) = extent.null_fallback {
        let fallback = parameter_ident(index, names)?;
        count = quote! { or(#count, #fallback) };
    }
    if extent.count_optional {
        count = quote! { if_present(#count) };
    }
    if extent.count_on_entry {
        count = quote! { on_entry(#count) };
    }
    if extent.units == BufferUnits::Bytes {
        count = quote! { bytes(#count) };
    }
    let indirect = extent.indirect.then(|| quote! { , indirect });
    let nullable = extent.nullable.then(|| quote! { , nullable });
    Some(quote! { #[#name(#phase #count #indirect #nullable)] })
}

fn extent_ident(name: &str) -> Option<TokenStream> {
    use syn::parse::Parser;
    syn::Ident::parse_any.parse_str(name).ok()?;
    Some(write_ident(name))
}

fn parameter_ident(index: u16, names: &[&str]) -> Option<TokenStream> {
    let name = names.get(index as usize)?;
    if names.iter().filter(|candidate| *candidate == name).count() != 1 {
        return None;
    }
    extent_ident(name)
}
