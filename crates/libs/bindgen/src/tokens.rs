//! Thin shim around `proc_macro2` and `quote` for the bindgen emitter.
//!
//! It delegates to the upstream `quote` crate so the emitter operates on real
//! token trees.

pub use proc_macro2::{Literal, TokenStream};
pub use quote::{ToTokens, quote};

/// Extension methods on `TokenStream` used by the emitter.
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

    /// Convert into an owned `String`. Equivalent to [`ToString::to_string`].
    fn into_string(self) -> String;
}

impl TokenStreamExt for TokenStream {
    fn combine<T: ToTokens>(&mut self, other: T) {
        other.to_tokens(self);
    }

    fn join(&self, suffix: &str) -> TokenStream {
        let joined = format!("{self}{suffix}");
        joined.parse().unwrap_or_else(|e| {
            panic!("invalid Rust tokens when joining {self:?} with {suffix:?}: {e}")
        })
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

/// Convert a PascalCase or camelCase name to snake_case.
///
/// Used in minimal mode to emit struct field names that conform to Rust
/// naming conventions (e.g. `TopLeft` -> `top_left`, `A` -> `a`).
pub fn to_snake_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
