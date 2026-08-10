use quote::{ToTokens, quote};

pub(super) fn architectures(value: i32) -> proc_macro2::TokenStream {
    match value & 7 {
        0 => quote! {},
        1 => quote! { #[cfg(target_arch = "x86")] },
        2 => quote! { #[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))] },
        3 => quote! {
            #[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
        },
        4 => quote! { #[cfg(target_arch = "aarch64")] },
        5 => quote! { #[cfg(any(target_arch = "aarch64", target_arch = "x86"))] },
        6 => quote! {
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86_64"
            ))]
        },
        7 => quote! {
            #[cfg(any(
                target_arch = "aarch64",
                target_arch = "arm64ec",
                target_arch = "x86",
                target_arch = "x86_64"
            ))]
        },
        _ => unreachable!(),
    }
}

pub(super) fn ident(name: &str) -> proc_macro2::TokenStream {
    match name {
        "Self" | "self" => {
            proc_macro2::Ident::new(&format!("{name}_"), proc_macro2::Span::call_site())
                .into_token_stream()
        }
        "_" => {
            proc_macro2::Ident::new("unused", proc_macro2::Span::call_site()).into_token_stream()
        }
        "crate" | "super" => {
            proc_macro2::Ident::new(&format!("{name}_"), proc_macro2::Span::call_site())
                .into_token_stream()
        }
        name if is_keyword(name) => {
            proc_macro2::Ident::new_raw(name, proc_macro2::Span::call_site()).into_token_stream()
        }
        name => proc_macro2::Ident::new(name, proc_macro2::Span::call_site()).into_token_stream(),
    }
}

pub(super) fn namespace(current: &str, target: &str) -> proc_macro2::TokenStream {
    if target.is_empty() || target == current {
        return quote! {};
    }

    let mut current = current.split('.').peekable();
    let mut target = target.split('.').peekable();
    while current.peek() == target.peek() {
        current.next();
        target.next();
    }

    let mut path = String::new();
    for _ in current {
        path.push_str("super::");
    }
    for part in target {
        path.push_str(part);
        path.push_str("::");
    }
    path.parse().unwrap()
}

fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "abstract"
            | "as"
            | "async"
            | "await"
            | "become"
            | "box"
            | "break"
            | "const"
            | "continue"
            | "do"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "final"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "macro"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "override"
            | "priv"
            | "pub"
            | "ref"
            | "return"
            | "static"
            | "struct"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "typeof"
            | "unsafe"
            | "unsized"
            | "use"
            | "virtual"
            | "where"
            | "while"
            | "yield"
    )
}
