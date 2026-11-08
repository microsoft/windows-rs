//! Tests for `#[implement]` attribute and input validation. Generated code is covered by the
//! behavioral tests in `test_implement` and `test_implement_core`.

use proc_macro2::TokenStream;
use quote::quote;

fn error(attributes: TokenStream, item_tokens: TokenStream) -> String {
    let output = crate::implement_core(attributes, item_tokens);
    let error: syn::ItemMacro = syn::parse2(output).unwrap();
    assert_eq!(
        error.mac.path.segments.last().unwrap().ident,
        "compile_error"
    );
    let message: syn::LitStr = syn::parse2(error.mac.tokens).unwrap();
    message.value()
}

#[test]
fn union_is_rejected() {
    let message = error(
        quote!(IFoo),
        quote! {
            union Value {
                number: u32,
            }
        },
    );
    assert_eq!(message, "expected a struct or enum");
}

#[test]
fn invalid_key_values_are_rejected() {
    let item = quote!(
        struct Foo;
    );
    assert_eq!(
        error(quote!(IFoo, Compose = yes), item.clone()),
        "`Compose` must be `true` or `false`"
    );
    assert_eq!(
        error(quote!(IFoo, Agile = yes), item.clone()),
        "`Agile` must be `true` or `false`"
    );
    assert_eq!(
        error(quote!(IFoo, Unknown = true), item),
        "Unrecognized key-value pair"
    );
}
