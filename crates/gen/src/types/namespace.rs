use super::*;
use crate::*;
use case::to_snake;
use squote::{quote, TokenStream};
use std::iter::FromIterator;

pub fn to_namespace_tokens(destination: &str, source: &str) -> TokenStream {
    let mut tokens = Vec::new();

    let mut source = source.split('.').peekable();
    let mut destination = destination.split('.').peekable();

    while source.peek() == destination.peek() {
        if source.next().is_none() {
            break;
        }
        destination.next();
    }

    let count = source.count();

    if count > 0 {
        tokens.resize(tokens.len() + count, quote! { super:: });
    }

    tokens.extend(destination.map(|destination| {
        let destination = format_ident(&to_snake(destination, MethodKind::Normal));
        quote! { #destination:: }
    }));

    TokenStream::from_iter(tokens)
}
