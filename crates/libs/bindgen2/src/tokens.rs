use crate::{Layout, canonical};
use quote::{ToTokens, quote};
use std::collections::BTreeSet;

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

pub(super) fn to_snake_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (index, character) in name.chars().enumerate() {
        if character.is_uppercase() {
            if index > 0 {
                result.push('_');
            }
            result.extend(character.to_lowercase());
        } else {
            result.push(character);
        }
    }
    result
}

pub(super) fn namespace(current: &str, target: &str, layout: Layout) -> proc_macro2::TokenStream {
    if layout.is_flat() || target.is_empty() || target == current {
        return quote! {};
    }

    let mut current = current.split('.').peekable();
    let target = if layout.is_package() {
        flat_module_namespace(target)
    } else {
        target
    };
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

fn flat_module_namespace(namespace: &str) -> &str {
    if namespace
        .strip_prefix("Windows.Win32")
        .is_some_and(|suffix| suffix.starts_with('.'))
    {
        "Windows.Win32"
    } else {
        namespace
    }
}

pub(super) fn feature_cfg<'a>(
    current: &str,
    layout: Layout,
    dependencies: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> proc_macro2::TokenStream {
    feature_cfg_set(&feature_names(current, layout, dependencies), false)
}

pub(super) fn feature_names<'a>(
    current: &str,
    layout: Layout,
    dependencies: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> BTreeSet<String> {
    if !layout.is_package() {
        return BTreeSet::new();
    }
    let mut features = BTreeSet::new();
    for (namespace, name) in dependencies {
        if namespace.is_empty()
            || namespace == "System"
            || namespace == "Windows.Foundation"
            || canonical::type_from_name(namespace, name).is_some()
            || crate::external::package_crate_name(namespace, name).is_some()
            || (current.starts_with("Windows.Win32") && !namespace.starts_with("Windows.Win32"))
            || namespace == current
            || current
                .strip_prefix(namespace)
                .is_some_and(|suffix| suffix.starts_with('.'))
        {
            continue;
        }
        if (namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32."))
            && matches!(
                name,
                "BOOL"
                    | "PSTR"
                    | "PWSTR"
                    | "PCSTR"
                    | "PCWSTR"
                    | "BSTR"
                    | "HSTRING"
                    | "IUnknown"
                    | "IInspectable"
                    | "NTSTATUS"
                    | "RPC_STATUS"
            )
        {
            continue;
        }
        let feature = if let Some(stem) = namespace.strip_prefix("Windows.Win32.") {
            stem.replace('.', "_")
        } else if let Some((_, rest)) = namespace.split_once('.') {
            rest.replace('.', "_")
        } else {
            continue;
        };
        features.insert(feature);
    }
    features
}

pub(super) fn feature_cfg_set(features: &BTreeSet<String>, not: bool) -> proc_macro2::TokenStream {
    match (features.len(), not) {
        (0, _) => quote! {},
        (1, false) => quote! { #[cfg(#(feature = #features)*)] },
        (1, true) => quote! { #[cfg(not(#(feature = #features)*))] },
        (_, false) => quote! { #[cfg(all( #(feature = #features),* ))] },
        (_, true) => quote! { #[cfg(not(all( #(feature = #features),* )))] },
    }
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
