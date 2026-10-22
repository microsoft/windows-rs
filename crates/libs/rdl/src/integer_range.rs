use super::*;
use metadata::{BufferPhase, IntegerBound, IntegerRange};

pub(crate) fn parse_integer_range(
    attr: &syn::Attribute,
    names: &[&str],
    field: bool,
) -> syn::Result<IntegerRange> {
    attr.parse_args_with(|input: syn::parse::ParseStream| {
        let phase = if field {
            BufferPhase::Value
        } else {
            let phase: syn::Ident = input.parse()?;
            let phase = match phase.to_string().as_str() {
                "entry" => BufferPhase::Entry,
                "success" => BufferPhase::Success,
                _ => return Err(input.error("expected entry or success")),
            };
            input.parse::<syn::Token![,]>()?;
            phase
        };
        let mut result = IntegerRange {
            phase,
            indirect: false,
            lower: None,
            upper: None,
        };
        if input.peek(syn::Token![>]) || input.peek(syn::Token![<]) {
            let lower = input.peek(syn::Token![>]);
            if lower {
                input.parse::<syn::Token![>]>()?;
            } else {
                input.parse::<syn::Token![<]>()?;
            }
            let exclusive = !input.peek(syn::Token![=]);
            if !exclusive {
                input.parse::<syn::Token![=]>()?;
            }
            let expression = integer_expression::parse_integer_expression(
                &input.parse::<syn::Expr>()?,
                names,
                field,
            )?;
            let bound = Some(IntegerBound {
                expression,
                exclusive,
            });
            if lower {
                result.lower = bound;
            } else {
                result.upper = bound;
            }
        } else {
            let range: syn::ExprRange = input.parse()?;
            let parse = |expression: &syn::Expr| {
                integer_expression::parse_integer_expression(expression, names, field)
            };
            if let Some(lower) = range.start {
                result.lower = Some(IntegerBound {
                    expression: parse(&lower)?,
                    exclusive: false,
                });
            }
            if let Some(upper) = range.end {
                result.upper = Some(IntegerBound {
                    expression: parse(&upper)?,
                    exclusive: matches!(range.limits, syn::RangeLimits::HalfOpen(_)),
                });
            } else if matches!(range.limits, syn::RangeLimits::Closed(_)) {
                return Err(input.error("inclusive range requires an upper bound"));
            }
        }
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            let flag: syn::Ident = input.parse()?;
            if flag != "indirect" {
                return Err(input.error("expected indirect"));
            }
            result.indirect = true;
        }
        if !input.is_empty() || !result.is_valid() {
            return Err(input.error("invalid integer range"));
        }
        Ok(result)
    })
}

pub fn write_integer_range(range: &IntegerRange, names: &[&str]) -> Option<TokenStream> {
    if !range.is_valid() {
        return None;
    }
    let write = |bound: &IntegerBound| {
        integer_expression::write_integer_expression(&bound.expression, names)
    };
    let bounds = match (&range.lower, &range.upper) {
        (Some(lower), Some(upper)) if !lower.exclusive => {
            let (left, right) = (write(lower)?, write(upper)?);
            if upper.exclusive {
                quote! { #left..#right }
            } else {
                quote! { #left..=#right }
            }
        }
        (Some(lower), None) => {
            let value = write(lower)?;
            if lower.exclusive {
                quote! { >#value }
            } else {
                quote! { >=#value }
            }
        }
        (None, Some(upper)) => {
            let value = write(upper)?;
            if upper.exclusive {
                quote! { <#value }
            } else {
                quote! { <=#value }
            }
        }
        _ => return None,
    };
    let phase = match range.phase {
        BufferPhase::Entry => quote! { entry, },
        BufferPhase::Success => quote! { success, },
        BufferPhase::Value => TokenStream::new(),
    };
    let indirect = range.indirect.then(|| quote! { , indirect });
    Some(quote! { #[range(#phase #bounds #indirect)] })
}
