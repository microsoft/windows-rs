use super::*;

impl Writer {
    // This is only for WinRT methods - use write_cpp_method for nono-WinRT methods
    pub fn write_method(&self, method: MethodDef, generics: &[Type], kind: InterfaceKind, method_names: &mut MethodNames,) -> TokenStream {
         let signature = method.signature(generics);

        let params = if kind == InterfaceKind::Composable {
            &signature.params[..signature.params.len() - 2]
        } else {
            &signature.params
        };

        let name = if kind == InterfaceKind::Composable && params.is_empty() {
            quote!(new)
        } else {
            method_names.add(method)
        };

        quote! { 
            pub fn #name(&self) {

            }
        }
    }
}