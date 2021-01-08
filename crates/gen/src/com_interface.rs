use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct ComInterface {
    pub name: TypeName,
    pub methods: Vec<NativeMethod>,
}

impl ComInterface {
    pub fn from_type_name(name: TypeName) -> Self {
        let methods = name
            .def
            .methods()
            .map(|method| NativeMethod::new(&method, &name.namespace))
            .collect();

        Self { name, methods }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let abi_name = self.name.gen_abi_definition();
        let guid = TypeGuid::from_type_def(&self.name.def);
        let guid = self.name.gen_guid(&guid);

        let abi_methods = self.methods.iter().map(|method| {
            let return_type = if let Some(t) = &method.return_type {
                let tokens = t.gen_field();
                quote! { -> #tokens }
            } else {
                TokenStream::new()
            };

            let params = method.params.iter().map(|(name, t)| {
                let name = format_ident(name);
                let tokens = t.gen_field();
                quote! { #name: #tokens }
            });

            quote! {
                pub unsafe extern "system" fn (this: ::winrt::RawPtr, #(#params),*) #return_type
            }
        });

        quote! {
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #name(::winrt::IUnknown);
            impl ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for #name {}
            unsafe impl ::winrt::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::winrt::Guid = #guid;
            }
            #[repr(C)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::winrt::RawPtr, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                pub unsafe extern "system" fn(this: ::winrt::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::winrt::RawPtr) -> u32,
                #(#abi_methods,)*
            );
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.methods
            .iter()
            .map(|method| method.dependencies())
            .flatten()
            .collect()
    }
}
