use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Class {
    pub fn write(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());

        let runtime_name = format!("{}.{}", self.def.namespace(), self.def.name(),);

        let runtime_name = quote! {
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
        };

        if let Some(default_interface) = self.default_interface(&self.generics) {
            let default_interface = default_interface.write(writer);

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
                //windows_core::imp::required_hierarchy!(#name, IClosable);
                impl #name {
                    //#(#methods)*
                }
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, #default_interface>();
                }
                unsafe impl windows_core::Interface for #name {
                    type Vtable = <#default_interface as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <#default_interface as windows_core::Interface>::IID;
                }
                #runtime_name
            }
        } else {
            quote! {
                pub struct #name;
                impl #name {
                    //#(#methods)*
                }
                #runtime_name
            }
        }
    }

    pub fn default_interface(&self, generics: &[Type]) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(generics))
    }

    //pub fn required_interfaces(&self, )

    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.default_interface(&self.generics)
                .unwrap()
                .runtime_signature()
        )
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            if let Some(default_interface) = self.default_interface(&self.generics) {
                default_interface.dependencies(dependencies, config);
            }
        }
    }
}
