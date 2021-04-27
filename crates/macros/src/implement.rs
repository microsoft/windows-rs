use super::*;

// TODO: distinguish between COM and WinRT interfaces
struct Implements(Vec<gen::ElementType>);

impl syn::parse::Parse for Implements {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut types = Vec::new();
        let reader = gen::TypeReader::get();

        loop {
            if input.is_empty() {
                break;
            }

            use_tree_to_types(reader, &input.parse::<ImplementTree>()?, &mut types)?;

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self(types))
    }
}

fn use_tree_to_types(
    reader: &'static gen::TypeReader,
    tree: &ImplementTree,
    types: &mut Vec<gen::ElementType>,
) -> syn::parse::Result<()> {
    fn recurse(
        reader: &'static gen::TypeReader,
        tree: &ImplementTree,
        types: &mut Vec<gen::ElementType>,
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
                let namespace = current.trim_matches('"');

                let mut meta_name = name.ident.to_string();
                let generic_count = name.generics.params.len();

                if generic_count > 0 {
                    meta_name.push('`');
                    meta_name.push_str(&generic_count.to_string());
                }

                types.push(reader.resolve_type(namespace, &meta_name));

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
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let inner_type = original_type.clone();

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
        if let gen::ElementType::Interface(t) = implement {
            vtable_ordinals.push(Literal::usize_unsuffixed(interface_count));

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
                unsafe extern "system" fn #query_interface(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
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

            let empty = gen::TypeTree::from_namespace("");
            let gen = gen::Gen::absolute(&empty);

            let vtable_ident = t.0.gen_abi_name(&gen);
            let interface_ident = t.0.gen_name(&gen);
            let interface_literal = Literal::usize_unsuffixed(interface_count);

            for (vtable_offset, method) in t.0.def.methods().enumerate() {
                let method_ident = gen::to_ident(&method.rust_name());
                let vcall_ident = format_ident!("abi{}_{}", interface_count, vtable_offset + 6);

                vtable_ptrs.combine(&quote! {
                    Self::#vcall_ident,
                });

                let signature = method.signature(&[]);
                let abi_signature = signature.gen_winrt_abi(&gen);
                let upcall =
                    signature.gen_winrt_upcall(quote! { (*this).inner.#method_ident }, &gen);

                shims.combine(&quote! {
                    unsafe extern "system" fn #vcall_ident #abi_signature {
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
            count: ::windows::WeakRefCount,
        }
        impl #box_ident {
            const VTABLE: (#(#vtable_idents,)*) = (
                #vtable_ctors
            );
            fn new(inner: #inner_ident) -> Self {
                Self {
                    vtable: (#(&Self::VTABLE.#vtable_ordinals,)*),
                    inner,
                    count: ::windows::WeakRefCount::new()
                }
            }
            fn QueryInterface(&mut self, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::HRESULT {
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

                    if !(*interface).is_null() {
                        self.count.add_ref();
                        return ::windows::HRESULT(0);
                    }

                    *interface = ::std::mem::transmute(self.count.query(iid, &mut self.vtable.0 as *mut _ as _));

                    if (*interface).is_null() {
                        ::windows::HRESULT(0x8000_4002) // E_NOINTERFACE
                    } else {
                        ::windows::HRESULT(0)
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
            ) -> ::windows::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *count = 0;
                *values = ::std::ptr::null_mut();
                ::windows::HRESULT(0)
            }
            unsafe extern "system" fn GetRuntimeClassName(
                _: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let h: ::windows::HString = "Thing".into(); // TODO: replace with class name or first interface
                *value = ::std::mem::transmute(h);
                ::windows::HRESULT(0)
            }
            unsafe extern "system" fn GetTrustLevel(_: ::windows::RawPtr, value: *mut i32) -> ::windows::HRESULT {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                *value = 0;
                ::windows::HRESULT(0)
            }
            #shims
        }
    });

    let mut tokens = tokens.parse::<proc_macro::TokenStream>().unwrap();
    tokens.extend(std::iter::once(original_type));
    tokens
}
