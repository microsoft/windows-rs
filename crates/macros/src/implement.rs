use super::*;
use gen::format_ident;
use squote::{format_ident, quote, Literal, TokenStream};

// TODO: distinguish between COM and WinRT interfaces
struct Implements(Vec<gen::TypeDefinition>);

impl syn::parse::Parse for Implements {
    fn parse(inner_type: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut types = Vec::new();
        let reader = winmd::TypeReader::get();

        loop {
            use_tree_to_types(reader, &inner_type.parse::<ImplementTree>()?, &mut types)?;

            if inner_type.parse::<syn::Token!(,)>().is_err() {
                break;
            }
        }

        Ok(Self(types))
    }
}

fn use_tree_to_types(
    reader: &'static winmd::TypeReader,
    tree: &ImplementTree,
    types: &mut Vec<gen::TypeDefinition>,
) -> syn::parse::Result<()> {
    fn recurse(
        reader: &'static winmd::TypeReader,
        tree: &ImplementTree,
        types: &mut Vec<gen::TypeDefinition>,
        current: &mut String,
    ) -> syn::parse::Result<()> {
        match tree {
            ImplementTree::Path(path) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&path.ident.to_string());
                recurse(reader, &*path.tree, types, current)?;
            }
            ImplementTree::Group(group) => {
                let prev = current.clone();

                for tree in &group.items {
                    recurse(reader, tree, types, current)?;
                    *current = prev.clone();
                }
            }
            ImplementTree::Name(name) => {
                let namespace = crate::namespace_literal_to_rough_namespace(&current.clone());
                let namespace = reader.find_lowercase_namespace(&namespace).unwrap(); // TODO: handle

                let mut meta_name = name.ident.to_string();
                let generic_count = name.generics.params.len();

                if generic_count > 0 {
                    meta_name.push('`');
                    meta_name.push_str(&generic_count.to_string());
                }

                let def = reader.expect_type_def((namespace, &meta_name));

                types.push(gen::TypeDefinition::from_type_def(&def));

                // TODO
                // If type is a class, add any required interfaces.
                // If type is an interface, add any required interfaces.
                // If any other kind of type, return an error.
                // If more than one class, return an error.
                // If dupe interface, produce warning but continue,
                //   unless warning is unavoidable (same interface required by different mentioned interfaces)
                // Finally, remove any dupes (TypeName can be used as key for set container)
            }
        }

        Ok(())
    }

    recurse(reader, tree, types, &mut String::new())
}

pub fn gen(
    attribute: proc_macro::TokenStream,
    inner_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let implements = syn::parse_macro_input!(attribute as Implements);
    let inner_type = syn::parse_macro_input!(inner_type as syn::ItemStruct);
    let inner_name = inner_type.ident.to_string();
    let inner_ident = format_ident!("{}", inner_name); // because squote doesn't know how to deal with syn::*
    let box_ident = format_ident!("{}_box", inner_name);

    let mut tokens = TokenStream::new();
    let mut vtable_idents = vec![];
    let mut vtable_ordinals = vec![];
    let mut vtable_ctors = TokenStream::new();
    let mut shims = TokenStream::new();
    let mut queries = TokenStream::new();

    for (interface_count, implement) in implements.0.iter().enumerate() {
        if let gen::TypeDefinition::Interface(t) = implement {
            vtable_ordinals.push(Literal::u32_unsuffixed(interface_count as u32));

            let query_interface = format_ident!("QueryInterface_abi{}", interface_count);
            let add_ref = format_ident!("AddRef_abi{}", interface_count);
            let release = format_ident!("Release_abi{}", interface_count);

            let mut vtable_ptrs = quote! {
                Self::#query_interface,
                Self::#add_ref,
                Self::#release,
                Self::GetIids,
                Self::GetRuntimeClassName,
                Self::GetTrustLevel,
            };

            shims.combine(&quote! {
                unsafe extern "system" fn #query_interface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode {
                    let this = (this as *mut ::windows::RawPtr).sub(#interface_count) as *mut Self;
                    (*this).QueryInterface(iid, interface)
                }
                unsafe extern "system" fn #add_ref(this: ::windows::RawPtr) -> u32 {
                    let this = (this as *mut ::windows::RawPtr).sub(#interface_count) as *mut Self;
                    (*this).AddRef()
                }
                unsafe extern "system" fn #release(this: ::windows::RawPtr) -> u32 {
                    let this = (this as *mut ::windows::RawPtr).sub(#interface_count) as *mut Self;
                    (*this).Release()
                }
            });

            let vtable_ident = t.name.gen_full_abi();
            let interface_ident = t.name.gen_full();
            let interface_literal = Literal::u32_unsuffixed(interface_count as u32);

            for method in &t.default_interface().methods {
                let method_ident = format_ident(&method.name);
                let vcall_ident = format_ident!("abi{}_{}", interface_count, method.vtable_offset);

                vtable_ptrs.combine(&quote! {
                    Self::#vcall_ident,
                });

                let signature = method.gen_full_abi();
                let upcall = method.gen_upcall(quote! { (*this).inner.#method_ident }, false);

                shims.combine(&quote! {
                    unsafe extern "system" fn #vcall_ident #signature {
                        let this = (this as *mut ::windows::RawPtr).sub(#interface_count) as *mut Self;
                        #upcall
                    }
                });

                queries.combine(&quote! {
                    &<#interface_ident as ::windows::Interface>::IID => {
                        &mut self.vtable.#interface_literal as *mut _ as _
                    }
                });
            }

            tokens.combine(&quote! {
                impl ::std::convert::From<#inner_ident> for #interface_ident {
                    fn from(inner: #inner_ident) -> Self {
                        let com = #box_ident::new(inner);

                        unsafe {
                            let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                            ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).vtable.#interface_literal as *mut _ as _))
                        }
                    }
                }
            });

            vtable_ctors.combine(&quote! {
                #vtable_ident(
                    #vtable_ptrs
                ),
            });

            vtable_idents.push(vtable_ident);
        }
    }

    tokens.combine(&quote! {
        #[repr(C)]
        struct #box_ident {
            vtable: (#(*const #vtable_idents,)*),
            inner: #inner_ident,
            count: ::windows::RefCount,
        }
        impl #box_ident {
            const VTABLE: (#(#vtable_idents,)*) = (
                #vtable_ctors
            );
            fn new(inner: #inner_ident) -> Self {
                Self {
                    vtable: (#(&Self::VTABLE.#vtable_ordinals,)*),
                    inner,
                    count: ::windows::RefCount::new()
                }
            }
            fn QueryInterface(&mut self, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode {
                unsafe {
                    *interface = match iid {
                        #queries
                        &<::windows::IUnknown as ::windows::Interface>::IID
                        | &<::windows::Object as ::windows::Interface>::IID
                        | &<::windows::IAgileObject as ::windows::Interface>::IID => {
                            &mut self.vtable.0 as *mut _ as _
                        }
                        _ => ::std::ptr::null_mut(),
                    };

                    if (*interface).is_null() {
                        ::windows::ErrorCode::E_NOINTERFACE
                    } else {
                        self.count.add_ref();
                        ::windows::ErrorCode::S_OK
                    }
                }
            }
            fn AddRef(&mut self) -> u32 {
                self.count.add_ref()
            }
            fn Release(&mut self) -> u32 {
                let remaining = self.count.release();
                if remaining == 0 {
                    unsafe {
                        ::std::boxed::Box::from_raw(self);
                    }
                }
                remaining
            }
            unsafe extern "system" fn GetIids(
                _: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::ErrorCode {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *count = 0;
                *values = ::std::ptr::null_mut();
                ::windows::ErrorCode(0)
            }
            unsafe extern "system" fn GetRuntimeClassName(
                _: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::ErrorCode {
                let h: ::windows::HString = "Thing".into(); // TODO: replace with class name or first interface
                *value = ::std::mem::transmute(h);
                ::windows::ErrorCode::S_OK
            }
            unsafe extern "system" fn GetTrustLevel(_: ::windows::RawPtr, value: *mut i32) -> ::windows::ErrorCode {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *value = 0;
                ::windows::ErrorCode(0)
            }
            #shims
        }
    });

    let tokens = tokens.parse::<proc_macro2::TokenStream>().unwrap();

    let tokens = quote::quote! {
        #inner_type
        #tokens
    };

    // println!("{}", tokens.to_string());
    tokens.into()
}
