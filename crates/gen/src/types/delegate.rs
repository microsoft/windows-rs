use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Delegate(pub GenericType);

impl Delegate {
    pub fn type_signature(&self) -> String {
        if self.0.generics.is_empty() {
            format!("delegate({})", self.0.interface_signature())
        } else {
            self.0.interface_signature()
        }
    }

    pub fn dependencies(&self) -> Vec<ElementType> {
        self.method().dependencies(&self.0.generics)
    }

    pub fn definition(&self) -> Vec<ElementType> {
        self.0.definition()
    }

    fn method(&self) -> tables::MethodDef {
        self.0
            .def
            .methods()
            .find(|m| m.name() == "Invoke")
            .expect("Callback")
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let abi_name = self.0.gen_abi_name(gen);
        let turbo_abi_name = self.0.gen_turbo_abi_name(gen);
        let signature = self.method().signature(&self.0.generics);
        let abi_signature = signature.gen_winrt_abi(gen);
        let fn_constraint = signature.gen_winrt_constraint(gen);
        let guid = self.0.gen_guid(gen);
        let phantoms = self.0.gen_phantoms();
        let constraints = self.0.gen_constraints();

        let method = MethodInfo {
            name: "invoke".to_string(),
            vtable_offset: 3,
            overload: 0,
        };

        let interface = InterfaceInfo {
            def: self.0.clone(),
            kind: InterfaceKind::Default,
            is_base: false,
            version: (0, 0),
        };

        let invoke = signature.gen_winrt_method(&method, &interface, gen);

        let type_signature = if self.0.generics.is_empty() {
            self.0
                .gen_signature(&format!("delegate({{{:#?}}})", &self.0.def.guid()))
        } else {
            self.0
                .gen_signature(&format!("{{{:#?}}}", &self.0.def.guid()))
        };

        let (box_name, box_definition) = if self.0.generics.is_empty() {
            let name = format_ident!("{}_box", self.0.def.name());
            (quote! { #name::<F> }, quote! { #name<#fn_constraint> })
        } else {
            let name = self.0.def.name();
            let name = format_ident!("{}_box", &name[..name.len() - 2]);
            let generics = self.0.generics.iter().map(|g| g.gen_name(gen));
            let generics = quote! { #(#generics,)* };
            (
                quote! { #name::<#generics F> },
                quote! { #name<#generics #fn_constraint> },
            )
        };

        let invoke_upcall = signature.gen_winrt_upcall(quote! { ((*this).invoke) }, gen);

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::IUnknown, #phantoms) where #constraints;
            impl<#constraints> #name {
                pub fn new<#fn_constraint>(invoke: F) -> Self {
                    let com = #box_name {
                        vtable: &#box_name::VTABLE,
                        count: ::windows::RefCount::new(),
                        invoke,
                    };
                    unsafe {
                        std::mem::transmute(::std::boxed::Box::new(com))
                    }
                }
                #invoke
            }
            unsafe impl<#constraints> ::windows::RuntimeType for #name {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = #type_signature;
            }
            unsafe impl<#constraints> ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn #abi_signature,
                #phantoms
            ) where #constraints;
            #[repr(C)]
            struct #box_definition where #constraints {
                vtable: *const #abi_name,
                invoke: F,
                count: ::windows::RefCount,
            }
            #[allow(non_snake_case)]
            impl<#constraints #fn_constraint> #box_name {
                const VTABLE: #abi_name = #turbo_abi_name(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    #phantoms
                );
                unsafe extern "system" fn QueryInterface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode {
                    let this = this as *mut ::windows::RawPtr as *mut Self;

                    *interface = if iid == &<#name as ::windows::Interface>::IID ||
                        iid == &<::windows::IUnknown as ::windows::Interface>::IID ||
                        iid == &<::windows::IAgileObject as ::windows::Interface>::IID {
                            &mut (*this).vtable as *mut _ as _
                        } else {
                            ::std::ptr::null_mut()
                        };

                    if (*interface).is_null() {
                        ::windows::ErrorCode::E_NOINTERFACE
                    } else {
                        (*this).count.add_ref();
                        ::windows::ErrorCode::S_OK
                    }
                }
                unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    (*this).count.add_ref()
                }
                unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    let remaining = (*this).count.release();

                    if remaining == 0 {
                        Box::from_raw(this);
                    }

                    remaining
                }
                unsafe extern "system" fn Invoke #abi_signature {
                    let this = this as *mut ::windows::RawPtr as *mut Self;
                    #invoke_upcall
                }
            }
        }
    }
}
