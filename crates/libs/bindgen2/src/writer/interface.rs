use super::*;

impl Writer {
    pub fn write_interface(&self, item: &Interface) -> TokenStream {
        let generics = item.def.generics();
        let name = to_ident(item.def.name());
        let vtbl_name = self.write_vtbl_ident(item);
        let guid = self.write_guid_u128(&item.def.guid_attribute().unwrap());
        let non_exclusive = !item.def.has_attribute("ExclusiveToAttribute");
        
        let methods = non_exclusive.then(||{
            let method_names = &mut MethodNames::new();

            let methods = item.def.methods().map(|method|
                self.write_method(method, &generics, InterfaceKind::Default, method_names)
            );

            quote! {
                impl #name {
                    #(#methods)*
                }
            }
        });
        
        quote! {
            windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
            impl core::ops::Deref for #name {
                type Target = windows_core::IInspectable;
                fn deref(&self) -> &Self::Target {
                    unsafe { core::mem::transmute(self) }
                }
            }
            windows_core::imp::interface_hierarchy!(#name, windows_core::IUnknown, windows_core::IInspectable);
            #methods
            impl windows_core::RuntimeType for #name {
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
