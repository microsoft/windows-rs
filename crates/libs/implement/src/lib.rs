mod implement;

#[proc_macro_attribute]
pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}

// TODO: add "interface" macro to define COM/WinRT interfaces directly in Rust.
// Also need a way to turn those definitions into a winmd. Perhaps back to a build macro
// that both spits out a winmd and adds the generated definitions to the OUT_DIR for inclusion.
