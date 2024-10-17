use super::*;

impl Interface {
    pub fn write_name(&self, writer: &Writer) -> TokenStream {
        let name = to_ident(self.def.name());
        let namespace = writer.write_namespace(self.def.namespace());
        let generics = writer.write_generic_names(&self.generics);
        quote! { #namespace #name #generics }
    }
}

impl Writer {
    pub fn write_interface(&self, item: &Interface) -> TokenStream {
        let name = item.write_name(self);
        let vtbl_name = self.write_vtbl_ident(item);
        let guid = self.write_guid_u128(&item.def.guid_attribute().unwrap());
        let non_exclusive = !item.def.has_attribute("ExclusiveToAttribute");
        let constraints = self.write_generic_constraints(&item.generics);

        let methods = non_exclusive.then(|| {
            let method_names = &mut MethodNames::new();

            let methods = item.def.methods().map(|method| {
                self.write_method(method, &item.generics, InterfaceKind::Default, method_names)
            });

            quote! {
                impl<#constraints> #name {
                    #(#methods)*
                }
            }
        });

        let definition = if item.generics.is_empty() {
            quote ! { 
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
            }
        } else {
            let phantoms = self.write_generic_phantoms(&item.generics);

            quote! {
                #[repr(transparent)]
                #[derive(PartialEq, Eq, Debug, Clone)]
                pub struct #name(windows_core::IUnknown, #phantoms) where #constraints;
                impl<#constraints> windows_core::imp::CanInto<windows_core::IUnknown> for #name {}
                impl<#constraints> windows_core::imp::CanInto<windows_core::IInspectable> for #name {}
                // TODO: add required interfaces here
                unsafe impl<#constraints> windows_core::Interface for #name {
                    type Vtable = #vtbl_name;
                    const IID: windows_core::GUID = windows_core::GUID::from_u128(#guid);
                }
            }
        };

        quote! {
            #definition
            impl<#constraints> core::ops::Deref for #name {
                type Target = windows_core::IInspectable;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            #methods
            impl<#constraints> windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            pub struct #vtbl_name {
                pub base__: windows_core::IInspectable_Vtbl,
            }
        }
    }

    pub fn write_vtbl_ident(&self, item: &Interface) -> TokenStream {
        format!("{}_Vtbl", item.def.name()).into()
    }
}
