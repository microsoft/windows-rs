use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub guid: TypeGuid,
    pub method: Method,
}

impl Delegate {
    pub fn from_type_name(reader: &winmd::TypeReader, name: TypeName) -> Self {
        let method = name
            .def
            .methods(reader)
            .find(|method| method.name(reader) == "Invoke")
            .unwrap();
        let method = Method::from_method_def(reader, method, &name.generics, &name.namespace);
        let guid = TypeGuid::from_type_def(reader, name.def);
        Self { name, method, guid }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.method.dependencies()
    }

    pub fn gen(&self) -> TokenStream {
        let definition = self.name.gen_definition();
        let abi_definition = self.name.gen_abi_definition();
        let fn_constraint = self.gen_fn_constraint();
        let impl_definition = self.gen_impl_definition(&fn_constraint);
        let name = self.name.gen();
        let abi_name = self.name.gen_abi();
        let impl_name = self.gen_impl_name();
        let phantoms = self.name.phantoms();
        let constraints = self.name.gen_constraint();
        let method = self.method.gen_default();
        let abi_method = self.method.gen_abi(&self.name);

        let guid = self.name.gen_guid(&self.guid);

        let signature = if self.name.generics.is_empty() {
            self.name
                .gen_signature(&format!("delegate({{{:#?}}})", &self.guid))
        } else {
            self.name.gen_signature(&format!("{{{:#?}}}", &self.guid))
        };

        let invoke_sig = self.method.gen_abi_impl(&self.name);
        let invoke_args = self
            .method
            .params
            .iter()
            .map(|param| param.gen_invoke_arg());
        let debug = default_gen_debug(&self.name);

        let invoke_upcall = if let Some(return_type) = &self.method.return_type {
            if return_type.array {
                let result = format_ident(&return_type.name);
                let result_size = squote::format_ident!("array_size_{}", &return_type.name);

                quote! {
                    match ((*this).invoke)(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            *#result = ok_data__;
                            *#result_size = ok_data_len__;
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            } else {
                let return_name = format_ident(&return_type.name);
                let return_kind = return_type.kind.gen();

                quote! {
                    match ((*this).invoke)(#(#invoke_args,)*) {
                        ::std::result::Result::Ok(ok__) => {
                            *#return_name = <#return_kind as ::winrt::AbiTransferable>::into_abi(ok__);
                            ::winrt::ErrorCode(0)
                        }
                        ::std::result::Result::Err(err) => err.into()
                    }
                }
            }
        } else {
            quote! {
                ((*this).invoke)(#(#invoke_args,)*).into()
            }
        };

        quote! {
            #[repr(transparent)]
            #[derive(::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq)]
            pub struct #definition where #constraints {
                ptr: ::winrt::ComPtr<#name>,
                #phantoms
            }
            impl<#constraints> #name {
                #method
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    #impl_name::make(invoke)
                }
            }
            unsafe impl<#constraints> ::winrt::ComInterface for #name {
                type VTable = #abi_definition;
                const IID: ::winrt::Guid = #guid;
            }
            #[repr(C)]
            pub struct #abi_definition where #constraints {
                pub unknown: ::winrt::abi_IUnknown,
                #abi_method
                #phantoms
            }
            unsafe impl<#constraints> ::winrt::RuntimeType for #name {
                const SIGNATURE: ::winrt::ConstBuffer = { #signature };
            }
            unsafe impl<#constraints> ::winrt::AbiTransferable for #name {
                type Abi = ::winrt::RawComPtr<Self>;
                fn get_abi(&self) -> Self::Abi {
                    <::winrt::ComPtr<#name> as ::winrt::AbiTransferable>::get_abi(&self.ptr)
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    <::winrt::ComPtr<#name> as ::winrt::AbiTransferable>::set_abi(&mut self.ptr)
                }
            }
            #debug
            #[repr(C)]
            struct #impl_definition where #constraints {
                vtable: *const #abi_definition,
                count: ::winrt::RefCount,
                invoke: F,
            }
            impl<#constraints #fn_constraint> #impl_name {
                const VTABLE: #abi_definition = #abi_name {
                    unknown: ::winrt::abi_IUnknown {
                        query_interface: #impl_name::query_interface,
                        add_ref: #impl_name::add_ref,
                        release: #impl_name::release,
                    },
                    invoke: #impl_name::invoke,
                    #phantoms
                };
                pub fn make(invoke: F) -> #name {
                    let value = Self {
                        vtable: &Self::VTABLE,
                        count: ::winrt::RefCount::new(),
                        invoke,
                    };
                    unsafe {
                        let mut result: #name = std::mem::zeroed();
                        let ptr: ::std::ptr::NonNull<Self> = ::std::ptr::NonNull::new_unchecked(::std::boxed::Box::into_raw(::std::boxed::Box::new(value)));
                        *<#name as ::winrt::AbiTransferable>::set_abi(&mut result) = Some(::winrt::NonNullRawComPtr::new(ptr.cast()));
                        result
                    }
                }
                extern "system" fn query_interface(
                    this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>,
                    iid: &::winrt::Guid,
                    interface: *mut ::winrt::RawPtr,
                ) -> ::winrt::ErrorCode {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;

                        if iid == &<#name as ::winrt::ComInterface>::IID
                            || iid == &<::winrt::IUnknown as ::winrt::ComInterface>::IID
                            || iid == &<::winrt::IAgileObject as ::winrt::ComInterface>::IID
                        {
                            *interface = this as ::winrt::RawPtr;
                            (*this).count.add_ref();
                            return ::winrt::ErrorCode(0);
                        }

                        *interface = std::ptr::null_mut();
                        ::winrt::ErrorCode::E_NOINTERFACE
                    }
                }
                extern "system" fn add_ref(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;
                        (*this).count.add_ref()
                    }
                }
                extern "system" fn release(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                    unsafe {
                        let this: *mut Self = this.as_raw() as _;
                        let remaining = (*this).count.release();

                        if remaining == 0 {
                            Box::from_raw(this);
                        }

                        remaining
                    }
                }
                #invoke_sig {
                    let this: *mut Self = this.as_raw() as _;
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

    fn gen_impl_definition(&self, fn_constraint: &TokenStream) -> TokenStream {
        if self.name.generics.is_empty() {
            let name = format_impl_ident(&self.name.name);
            quote! { #name<#fn_constraint> }
        } else {
            let name = format_impl_ident(&self.name.name[..self.name.name.len() - 2]);
            let generics = self.name.generics.iter().map(|g| g.gen());
            quote! { #name<#(#generics,)* #fn_constraint> }
        }
    }

    fn gen_impl_name(&self) -> TokenStream {
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
    squote::format_ident!("impl_{}", name)
}
