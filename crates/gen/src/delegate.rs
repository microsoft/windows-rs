use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub method: Method,
}

impl Delegate {
    pub fn from_type_name(name: TypeName) -> Self {
        let method = name
            .def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap();
        let method = Method::from_method_def(&method, 3, &name.generics, &name.namespace);
        let guid = TypeGuid::from_type_def(&name.def);
        Self { name, method, guid }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.method.dependencies()
    }

    pub fn gen(&self) -> TokenStream {
        let definition = self.name.gen_definition();
        let vtable_definition = self.name.gen_abi_definition();
        let fn_constraint = self.gen_fn_constraint();
        let box_definition = self.gen_box_definition(&fn_constraint);
        let name = self.name.gen();
        let vtable_name = self.name.gen_abi();
        let box_name = self.gen_box_name();
        let phantoms = self.name.phantoms();
        let constraints = self.name.gen_constraint();
        let method = self.method.gen_method(&self.name, InterfaceKind::Default);
        let abi_signature = self.method.gen_abi();

        let guid = self.name.gen_guid(&self.guid);

        let signature = if self.name.generics.is_empty() {
            self.name
                .gen_signature(&format!("delegate({{{:#?}}})", &self.guid))
        } else {
            self.name.gen_signature(&format!("{{{:#?}}}", &self.guid))
        };

        let invoke_upcall = self.method.gen_upcall(quote! { ((*this).invoke) }, true);

        quote! {
            #[repr(transparent)]
            pub struct #definition(::winrt::IUnknown, #phantoms) where #constraints;
            impl<#constraints> ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone(), #phantoms)
                }
            }
            impl<#constraints> ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl<#constraints> ::std::cmp::Eq for #name {}
            impl<#constraints> ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            unsafe impl<#constraints> ::winrt::Interface for #name {
                type Vtable = #vtable_definition;
                const IID: ::winrt::Guid = #guid;
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::winrt::ConstBuffer = { #signature };
            }
            #[repr(C)]
            pub struct #vtable_definition(
                pub unsafe extern "system" fn(this: ::winrt::RawPtr, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                pub unsafe extern "system" fn(this: ::winrt::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::winrt::RawPtr) -> u32,
                pub unsafe extern "system" fn #abi_signature,
                #phantoms
            ) where #constraints;
            impl<#constraints> #name {
                #method
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    let com = #box_name {
                        vtable: &#box_name::VTABLE,
                        count: ::winrt::RefCount::new(),
                        invoke,
                    };
                    unsafe {
                        std::mem::transmute(::std::boxed::Box::new(com))
                    }
                }
            }
            #[repr(C)]
            struct #box_definition where #constraints {
                vtable: *const #vtable_definition,
                invoke: F,
                count: ::winrt::RefCount,
            }
            #[allow(non_snake_case)]
            impl<#constraints #fn_constraint> #box_name {
                const VTABLE: #vtable_definition = #vtable_name(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    #phantoms
                );
                unsafe extern "system" fn QueryInterface(this: ::winrt::RawPtr, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode {
                    let this = this as *mut ::winrt::RawPtr as *mut Self;

                    *interface = if iid == &<#name as ::winrt::Interface>::IID ||
                        iid == &<::winrt::IUnknown as ::winrt::Interface>::IID ||
                        iid == &<::winrt::IAgileObject as ::winrt::Interface>::IID {
                            &mut (*this).vtable as *mut _ as _
                        } else {
                            ::std::ptr::null_mut()
                        };

                    if (*interface).is_null() {
                        ::winrt::ErrorCode::E_NOINTERFACE
                    } else {
                        (*this).count.add_ref();
                        ::winrt::ErrorCode::S_OK
                    }
                }
                unsafe extern "system" fn AddRef(this: ::winrt::RawPtr) -> u32 {
                    let this = this as *mut ::winrt::RawPtr as *mut Self;
                    (*this).count.add_ref()
                }
                unsafe extern "system" fn Release(this: ::winrt::RawPtr) -> u32 {
                    let this = this as *mut ::winrt::RawPtr as *mut Self;
                    let remaining = (*this).count.release();

                    if remaining == 0 {
                        Box::from_raw(this);
                    }

                    remaining
                }
                unsafe extern "system" fn Invoke #abi_signature {
                    let this = this as *mut ::winrt::RawPtr as *mut Self;
                    #invoke_upcall
                }
            }
        }
    }

    fn gen_fn_constraint(&self) -> TokenStream {
        let params = self.method.params.iter().map(|param| param.gen_fn());

        let return_type = if let Some(return_type) = &self.method.return_type {
            return_type.gen_return()
        } else {
            quote! { () }
        };

        quote! { F: FnMut(#(#params)*) -> ::winrt::Result<#return_type> + 'static }
    }

    fn gen_box_definition(&self, fn_constraint: &TokenStream) -> TokenStream {
        if self.name.generics.is_empty() {
            let name = format_impl_ident(&self.name.name);
            quote! { #name<#fn_constraint> }
        } else {
            let name = format_impl_ident(&self.name.name[..self.name.name.len() - 2]);
            let generics = self.name.generics.iter().map(|g| g.gen());
            quote! { #name<#(#generics,)* #fn_constraint> }
        }
    }

    fn gen_box_name(&self) -> TokenStream {
        if self.name.generics.is_empty() {
            let name = format_impl_ident(&self.name.name);
            quote! { #name::<F> }
        } else {
            let name = format_impl_ident(&self.name.name[..self.name.name.len() - 2]);
            let generics = self.name.generics.iter().map(|g| g.gen());
            quote! { #name::<#(#generics,)* F> }
        }
    }
}

fn format_impl_ident(name: &str) -> squote::Ident {
    squote::format_ident!("{}_box", name)
}
