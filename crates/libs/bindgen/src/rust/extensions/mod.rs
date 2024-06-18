use super::*;

/// Helper function for `gen_mod` and `gen_impl`.
///
/// This function generates an `include!(...)` that points into the `src/includes` directory for
/// the `windows` and `windows-sys` crates. This makes it easy to inject code into WinMD namespaces.
#[allow(dead_code)]
fn include_ext(relative_path: &str) -> TokenStream {
    quote! {
        core::include!(
            core::concat!(
                core::env!("CARGO_MANIFEST_DIR"),
                "/src/includes/",
                #relative_path
            )
        );
    }
}

/// Generates extension code for a specific namespace
pub fn gen_mod(_writer: &Writer, namespace: &str) -> TokenStream {
    match namespace {
        "Windows.Win32.UI.WindowsAndMessaging" => {
            include_ext("Win32/UI/WindowsAndMessaging/WindowLong.rs")
        }

        _ => quote!(),
    }
}

/// Generates extension code that is subject to the `implement` feature for a specific namespace
pub fn gen_impl(_namespace: &str) -> TokenStream {
    TokenStream::new()
}
