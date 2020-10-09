use squote::{quote, TokenStream};
use std::iter::FromIterator;

pub fn gen_namespace(destination: &str, source: &str) -> TokenStream {
    if destination == source {
        return TokenStream::new();
    }

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
        let destination =
            crate::format_ident(&crate::to_snake(destination, crate::MethodKind::Normal));
        quote! { #destination:: }
    }));

    TokenStream::from_iter(tokens)
}

pub fn gen_binding_namespace(destination: &str) -> TokenStream {
    let mut tokens = vec![quote! { ::winrt_bindings:: }];
    let destination = destination.split('.').peekable();

    tokens.extend(destination.map(|destination| {
        let destination =
            crate::format_ident(&crate::to_snake(destination, crate::MethodKind::Normal));
        quote! { #destination:: }
    }));

    TokenStream::from_iter(tokens)
}
