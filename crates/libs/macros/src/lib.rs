mod implement;

#[proc_macro_attribute]
pub fn implement(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    implement::gen(attribute, input)
}
