use super::*;
use metadata::{IntegerExpression, IntegerLiteral, IntegerReference};
use syn::ext::IdentExt;

pub(crate) fn parse_integer_expression(
    expression: &syn::Expr,
    names: &[&str],
    field: bool,
) -> syn::Result<IntegerExpression> {
    fn parse(
        expression: &syn::Expr,
        names: &[&str],
        field: bool,
        depth: usize,
    ) -> syn::Result<IntegerExpression> {
        let invalid = || syn::Error::new_spanned(expression, "unsupported integer expression");
        if depth > 32 {
            return Err(invalid());
        }
        let recurse = |expression| parse(expression, names, field, depth + 1);
        Ok(match expression {
            syn::Expr::Paren(value) => recurse(&value.expr)?,
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(value),
                ..
            }) => IntegerExpression::Literal(match value.suffix() {
                "" | "i32" => IntegerLiteral::I32(value.base10_parse()?),
                "u32" => IntegerLiteral::U32(value.base10_parse()?),
                "i64" => IntegerLiteral::I64(value.base10_parse()?),
                "u64" => IntegerLiteral::U64(value.base10_parse()?),
                _ => return Err(invalid()),
            }),
            syn::Expr::Path(value) if value.qself.is_none() && value.path.get_ident().is_some() => {
                let name = value.path.get_ident().unwrap().unraw().to_string();
                let mut matches = names
                    .iter()
                    .enumerate()
                    .filter(|(_, value)| **value == name);
                let (index, _) = matches.next().ok_or_else(invalid)?;
                if matches.next().is_some() {
                    return Err(invalid());
                }
                IntegerExpression::Reference(if field {
                    IntegerReference::Field(name)
                } else {
                    IntegerReference::Parameter(index.try_into().map_err(|_| invalid())?)
                })
            }
            syn::Expr::Return(value) if value.expr.is_none() && !field => {
                IntegerExpression::Reference(IntegerReference::Return)
            }
            syn::Expr::Call(value)
                if !field
                    && value.args.len() == 1
                    && matches!(value.func.as_ref(), syn::Expr::Path(path) if path.path.is_ident("if_present")) =>
            {
                let IntegerExpression::Reference(IntegerReference::Dereference(index)) =
                    recurse(&value.args[0])?
                else {
                    return Err(invalid());
                };
                IntegerExpression::Reference(IntegerReference::OptionalDereference(index))
            }
            syn::Expr::Unary(value) => match value.op {
                syn::UnOp::Deref(_) if !field => {
                    let IntegerExpression::Reference(IntegerReference::Parameter(index)) =
                        recurse(&value.expr)?
                    else {
                        return Err(invalid());
                    };
                    IntegerExpression::Reference(IntegerReference::Dereference(index))
                }
                syn::UnOp::Neg(_) => {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Int(literal),
                        ..
                    }) = value.expr.as_ref()
                        && matches!(literal.suffix(), "" | "i32" | "i64")
                    {
                        let number = literal
                            .base10_parse::<i128>()?
                            .checked_neg()
                            .ok_or_else(invalid)?;
                        IntegerExpression::Literal(if literal.suffix() == "i64" {
                            IntegerLiteral::I64(number.try_into().map_err(|_| invalid())?)
                        } else {
                            IntegerLiteral::I32(number.try_into().map_err(|_| invalid())?)
                        })
                    } else {
                        IntegerExpression::Subtract(
                            Box::new(IntegerExpression::Literal(IntegerLiteral::I32(0))),
                            Box::new(recurse(&value.expr)?),
                        )
                    }
                }
                _ => return Err(invalid()),
            },
            syn::Expr::Binary(value) => {
                let left = Box::new(recurse(&value.left)?);
                let right = Box::new(recurse(&value.right)?);
                match value.op {
                    syn::BinOp::Add(_) => IntegerExpression::Add(left, right),
                    syn::BinOp::Sub(_) => IntegerExpression::Subtract(left, right),
                    syn::BinOp::Mul(_) => IntegerExpression::Multiply(left, right),
                    _ => return Err(invalid()),
                }
            }
            _ => return Err(invalid()),
        })
    }
    let value = parse(expression, names, field, 0)?;
    if !value.is_valid() {
        return Err(syn::Error::new_spanned(
            expression,
            "invalid integer expression",
        ));
    }
    Ok(value)
}

pub(crate) fn write_integer_expression(
    value: &IntegerExpression,
    names: &[&str],
) -> Option<TokenStream> {
    use IntegerExpression::*;
    Some(match value {
        Literal(value) => {
            let text = match value {
                IntegerLiteral::I32(value) => value.to_string(),
                IntegerLiteral::U32(value) => format!("{value}u32"),
                IntegerLiteral::I64(value) => format!("{value}i64"),
                IntegerLiteral::U64(value) => format!("{value}u64"),
            };
            text.parse().ok()?
        }
        Reference(value) => match value {
            IntegerReference::Parameter(index) | IntegerReference::Dereference(index) => {
                let name = buffer_extent::parameter_ident(*index, names)?;
                if matches!(value, IntegerReference::Dereference(_)) {
                    quote! { *#name }
                } else {
                    name
                }
            }
            IntegerReference::Field(name) => buffer_extent::extent_ident(name)?,
            IntegerReference::OptionalDereference(index) => {
                let name = buffer_extent::parameter_ident(*index, names)?;
                quote! { if_present(*#name) }
            }
            IntegerReference::Return => quote! { (return) },
        },
        Add(left, right) | Subtract(left, right) | Multiply(left, right) => {
            let mut left_tokens = write_integer_expression(left, names)?;
            let mut right_tokens = write_integer_expression(right, names)?;
            if matches!(value, Multiply(..)) && matches!(left.as_ref(), Add(..) | Subtract(..)) {
                left_tokens = quote! { (#left_tokens) };
            }
            if matches!(right.as_ref(), Add(..) | Subtract(..))
                || matches!((value, right.as_ref()), (Multiply(..), Multiply(..)))
            {
                right_tokens = quote! { (#right_tokens) };
            }
            if matches!(value, Add(..)) {
                quote! { #left_tokens + #right_tokens }
            } else if matches!(value, Subtract(..)) {
                quote! { #left_tokens - #right_tokens }
            } else {
                quote! { #left_tokens * #right_tokens }
            }
        }
    })
}
