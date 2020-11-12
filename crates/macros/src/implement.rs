use squote::{format_ident, quote, Literal, TokenStream};
use syn::spanned::Spanned;

// TODO: all the format_ident! macro calls should go through the version in winrt-gen.

// TODO: distinguish between COM and WinRT interfaces
struct Implements(Vec<winrt_gen::Type>);

impl syn::parse::Parse for Implements {
    fn parse(inner_type: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut types = Vec::new();
        let reader = build_reader();

        loop {
            use_tree_to_types(reader, &inner_type.parse::<syn::UseTree>()?, &mut types)?;

            if inner_type.parse::<syn::Token!(,)>().is_err() {
                break;
            }
        }

        Ok(Self(types))
    }
}

fn build_reader() -> &'static winmd::TypeReader {
    use std::{mem::MaybeUninit, sync::Once};
    static ONCE: Once = Once::new();
    static mut VALUE: MaybeUninit<winmd::TypeReader> = MaybeUninit::uninit();

    ONCE.call_once(|| {
        // This is safe because `Once` provides thread-safe one-time initialization
        unsafe {
            VALUE = MaybeUninit::new(
                // TODO: load all the winmd files from the target/nuget folder
                winmd::TypeReader::from_os(),
            )
        }
    });

    // This is safe because `call_once` has already been called.
    unsafe { &*VALUE.as_ptr() }
}

fn use_tree_to_types(
    reader: &winmd::TypeReader,
    tree: &syn::UseTree,
    types: &mut Vec<winrt_gen::Type>,
) -> syn::parse::Result<()> {
    fn recurse(
        reader: &winmd::TypeReader,
        tree: &syn::UseTree,
        types: &mut Vec<winrt_gen::Type>,
        current: &mut String,
    ) -> syn::parse::Result<()> {
        match tree {
            syn::UseTree::Path(path) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&path.ident.to_string());
                recurse(reader, &*path.tree, types, current)?;
            }
            syn::UseTree::Group(group) => {
                let prev = current.clone();

                for tree in &group.items {
                    recurse(reader, tree, types, current)?;
                    *current = prev.clone();
                }
            }
            syn::UseTree::Name(name) => {
                let namespace = crate::namespace_literal_to_rough_namespace(&current.clone());

                let namespace_types = match reader
                    .types
                    .iter()
                    .find(|(name, _)| name.to_lowercase() == namespace)
                {
                    Some((_, types)) => types,
                    None => {
                        return Err(syn::parse::Error::new(
                            name.span(),
                            "Metadata not found for type namespace",
                        ))
                    }
                };

                let def = match namespace_types.get(&name.ident.to_string()) {
                    Some(def) => def,
                    None => {
                        return Err(syn::parse::Error::new(
                            name.span(),
                            "Metadata not found for type name",
                        ))
                    }
                };

                types.push(winrt_gen::Type::from_type_def(reader, *def));

                // TODO
                // If type is a class, add any required interfaces.
                // If type is an interface, add any required interfaces.
                // If any other kind of type, return an error.
                // If more than one class, return an error.
                // If dupe interface, produce warning but continue,
                //   unless warning is unavoidable (same interface required by different mentioned interfaces)
                // Finally, remove any dupes (TypeName can be used as key for set container)

                //println!("implement: {}.{}", def.name(reader).0, def.name(reader).1);
            }
            syn::UseTree::Glob(glob) => {
                return Err(syn::parse::Error::new(
                    glob.span(),
                    "Glob syntax is not supported",
                ))
            }
            syn::UseTree::Rename(rename) => {
                return Err(syn::parse::Error::new(
                    rename.span(),
                    "Rename syntax is not supported",
                ))
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

    // TODO:
    // 1. get squote/TokenStream working here
    // 2. gen the inner_type
    // 3. gen the vtables
    // 3. gen the boxed type

    let mut tokens = TokenStream::new();
    let mut vtable_idents = vec![];
    let mut vtable_ordinals = vec![];
    let mut vtable_ctors = TokenStream::new();
    let mut shims = TokenStream::new();
    let mut queries = TokenStream::new();

    for (interface_count, implement) in implements.0.iter().enumerate() {
        if let winrt_gen::Type::Interface(t) = implement {
            let vtable_ident = format_ident!("{}_vtable{}", inner_name, interface_count);
            vtable_ordinals.push(Literal::u32_unsuffixed(interface_count as u32));

            let query_interface = format_ident!("vtable{}_QueryInterface", interface_count);
            let add_ref = format_ident!("vtable{}_AddRef", interface_count);
            let release = format_ident!("vtable{}_Release", interface_count);

            let mut vtable_ptrs = quote! {
                Self::#query_interface,
                Self::#add_ref,
                Self::#release,
                Self::GetIids,
                Self::GetRuntimeClassName,
                Self::GetTrustLevel,
            };

            shims.combine(&quote! {
                extern "system" fn #query_interface(this: ::winrt::RawPtr, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode {
                    unsafe {
                        let this = (this as *mut ::winrt::RawPtr).sub(#interface_count) as *mut Self;
                        (*this).QueryInterface(iid, interface)
                    }
                }
                extern "system" fn #add_ref(this: ::winrt::RawPtr) -> u32 {
                    unsafe {
                        let this = (this as *mut ::winrt::RawPtr).sub(#interface_count) as *mut Self;
                        (*this).AddRef()
                    }
                }
                extern "system" fn #release(this: ::winrt::RawPtr) -> u32 {
                    unsafe {
                        let this = (this as *mut ::winrt::RawPtr).sub(#interface_count) as *mut Self;
                        (*this).Release()
                    }
                }
            });

            let interface_ident = t.name.gen_full();
            let interface_literal = Literal::u32_unsuffixed(interface_count as u32);

            let externs = t.default_interface().methods.iter().map(|method| {
                let method_ident = format_ident!("{}", method.name);
                let vcall_ident = format_ident!("vtable{}_{}", interface_count, method.ordinal);

                vtable_ptrs.combine(&quote! {
                    Self::#vcall_ident,
                });

                let signature = method.gen_abi();
                let upcall = method.gen_upcall(quote! { (*this).inner.#method_ident });

                shims.combine(&quote! {
                    extern "system" fn #vcall_ident #signature {
                        unsafe {
                            let this = (this as *mut ::winrt::RawPtr).sub(#interface_count) as *mut Self;
                            #upcall
                        }
                    }
                });

                queries.combine(&quote! {
                    &<#interface_ident as ::winrt::Interface>::IID => {
                        &mut self.vtable.#interface_literal as *mut _ as _
                    }
                });

                quote! {
                    extern "system" fn #signature
                }
            });

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
                struct #vtable_ident(
                    extern "system" fn(this: ::winrt::RawPtr, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                    extern "system" fn(this: ::winrt::RawPtr) -> u32,
                    extern "system" fn(this: ::winrt::RawPtr) -> u32,
                    extern "system" fn(this: ::winrt::RawPtr, count: *mut u32, values: *mut *mut ::winrt::Guid) -> ::winrt::ErrorCode,
                    extern "system" fn(this: ::winrt::RawPtr, value: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode,
                    extern "system" fn(this: ::winrt::RawPtr, value: *mut i32) -> ::winrt::ErrorCode,
                    #(#externs,)*
                );
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
            count: ::winrt::RefCount,
        }
        impl #box_ident {
            const VTABLE: (#(#vtable_idents,)*) = (
                #vtable_ctors
            );

            fn new(inner: #inner_ident) -> Self {
                Self {
                    vtable: (#(&Self::VTABLE.#vtable_ordinals,)*),
                    inner,
                    count: ::winrt::RefCount::new()
                }
            }

            fn QueryInterface(&mut self, iid: &::winrt::Guid, interface: *mut ::winrt::RawPtr) -> ::winrt::ErrorCode {
                unsafe {
                    *interface = match iid {
                        #queries
                        &<::winrt::IUnknown as ::winrt::Interface>::IID
                        | &<::winrt::Object as ::winrt::Interface>::IID
                        | &<::winrt::IAgileObject as ::winrt::Interface>::IID => {
                            &mut self.vtable.0 as *mut _ as _
                        }
                        _ => ::std::ptr::null_mut(),
                    };
        
                    if (*interface).is_null() {
                        ::winrt::ErrorCode::E_NOINTERFACE
                    } else {
                        self.count.add_ref();
                        ::winrt::ErrorCode::S_OK
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

            extern "system" fn GetIids(
                _: ::winrt::RawPtr,
                count: *mut u32,
                values: *mut *mut ::winrt::Guid,
            ) -> ::winrt::ErrorCode {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                unsafe {
                    *count = 0;
                    *values = ::std::ptr::null_mut();
                }
                ::winrt::ErrorCode(0)
            }

            extern "system" fn GetRuntimeClassName(
                _: ::winrt::RawPtr,
                value: *mut ::winrt::RawPtr,
            ) -> ::winrt::ErrorCode {
                unsafe {
                    let h: ::winrt::HString = "Thing".into(); // TODO: replace with class name or first interface
                    *value = ::std::mem::transmute(h);
                }
                ::winrt::ErrorCode::S_OK
            }

            extern "system" fn GetTrustLevel(_: ::winrt::RawPtr, value: *mut i32) -> ::winrt::ErrorCode {
                // Note: even if we end up implementing this in future, it still doesn't need a this pointer
                // since the data to be returned is type- not instance-specific so can be shared for all
                // interfaces.
                unsafe {
                    *value = 0;
                }
                ::winrt::ErrorCode(0)
            }
            #shims
        }
    });

    // TODO: there's a lot of friction when using squote with quote and syn...
    // Here I'm turning the results of squote (preferred) into proc_macro2 and
    // then from there into proc_macro. This seems inefficient but I'm not
    // sure yet. But it's certainly tedious.

    let tokens = tokens.parse::<proc_macro2::TokenStream>().unwrap();

    let tokens = quote::quote! {
        #inner_type
        #tokens
    };

    //println!("{}", tokens.to_string());
    tokens.into()
}
