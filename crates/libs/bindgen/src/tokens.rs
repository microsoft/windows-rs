//! Thin shim around `proc_macro2` and `quote` for the bindgen emitter.
//!
//! Previously this module reimplemented the `quote!` macro on top of a
//! `TokenStream(pub String)` (~1050 LOC). It now delegates to the upstream
//! `quote` crate so the emitter operates on real token trees.

pub use proc_macro2::{Literal, TokenStream};
pub use quote::{quote, ToTokens};

/// Extension methods on `TokenStream` that match the API of the previous
/// hand-rolled `TokenStream(String)` wrapper.
pub trait TokenStreamExt {
    /// Append another stream of tokens to this one.
    ///
    /// Accepts anything that implements [`ToTokens`], including
    /// `&TokenStream`, `TokenStream`, `Ident`, etc.
    fn combine<T: ToTokens>(&mut self, other: T);

    /// Append `suffix` to the textual form of `self` and re-parse, producing a
    /// `TokenStream` that names `<self><suffix>`. Typically used to derive
    /// identifiers like `Foo_Vtbl` from a parent name.
    fn join(&self, suffix: &str) -> TokenStream;

    /// Convert into an owned `String`. Equivalent to [`ToString::to_string`],
    /// preserved as an alias for legacy call sites.
    fn into_string(self) -> String;
}

impl TokenStreamExt for TokenStream {
    fn combine<T: ToTokens>(&mut self, other: T) {
        other.to_tokens(self);
    }

    fn join(&self, suffix: &str) -> TokenStream {
        format!("{self}{suffix}")
            .parse()
            .expect("invalid Rust tokens")
    }

    fn into_string(self) -> String {
        self.to_string()
    }
}

/// Make a Rust identifier from a name, escaping keywords and reserved names.
///
/// The returned `TokenStream` always wraps exactly one identifier token, so it
/// can be interpolated wherever an identifier is expected.
pub fn to_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => format!("r#{name}").parse().unwrap(),
        "Self" | "self" => format!("{name}_").parse().unwrap(),
        "_" => {
            proc_macro2::Ident::new("unused", proc_macro2::Span::call_site()).into_token_stream()
        }
        other => proc_macro2::Ident::new(other, proc_macro2::Span::call_site()).into_token_stream(),
    }
}
