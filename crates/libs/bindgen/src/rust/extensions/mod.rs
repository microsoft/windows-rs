use super::*;

/// Helper function for `gen_mod` and `gen_impl`.
///
/// This function generates an `include!(...)` that points into the `src/includes` directory for
/// the `windows` and `windows-sys` crates. This makes it easy to inject code into WinMD namespaces.
#[allow(dead_code)]
fn include_ext(namespace: &str, relative_path: &str) -> TokenStream {
    let mut path = "../".repeat(namespace.split('.').count());
    path.push_str("includes/");
    path.push_str(relative_path);
    quote! {
        core::include!(#path);
    }
}

/// Generates extension code for a specific namespace
pub fn gen_mod(_writer: &Writer, namespace: &str) -> TokenStream {
    match namespace {
        "Windows.Win32.UI.WindowsAndMessaging" => {
            include_ext(namespace, "Win32/UI/WindowsAndMessaging/WindowLong.rs")
        }

        _ => quote!(),
    }
}
