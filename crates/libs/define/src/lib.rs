// The define macro should be called by a build script to define types and 
// generate a corresponding OUT_DIR/crate_name.rs and OUT_DIR/crate_name.winmd

#[proc_macro]
pub fn define(_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}
