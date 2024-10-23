use super::*;

// This is WinRT methods only
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method {
    pub def: MethodDef,
    pub generics: Vec<Type>,
    pub signature: Signature,
    pub dependencies: Dependencies,
}

impl Method {
    pub fn new(def: MethodDef, generics: &[Type]) -> Self {
        let signature = def.signature(generics);

        let mut dependencies = Dependencies::new();
        signature.dependencies(&mut dependencies);

        Self {
            def,
            generics: generics.to_vec(),
            signature,
            dependencies,
        }
    }

    pub fn included(&self, filter: &Filter) -> bool {
        self.dependencies.included(filter)
    }

    pub fn write(
        &self,
        _writer: &Writer,
        _interface_name: TokenStream,
        kind: InterfaceKind,
        method_names: &mut MethodNames,
        virtual_names: &mut MethodNames,
    ) -> TokenStream {

        // TODO: here's where we decide whether to "include" this method based on filter info...


        // TODO: for config.package the dependencies need to be used to generate [cfg] and for
        // whether to include the method at all based on filtering/exclusions

        let params = if kind == InterfaceKind::Composable {
            &self.signature.params[..self.signature.params.len() - 2]
        } else {
            &self.signature.params
        };

        let name = if kind == InterfaceKind::Composable && params.is_empty() {
            quote!(new)
        } else {
            method_names.add(self.def)
        };

        let _vname = virtual_names.add(self.def);

        quote! {
            pub fn #name(&self) {

            }
        }
    }
}
