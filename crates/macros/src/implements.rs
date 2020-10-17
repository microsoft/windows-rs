use syn::spanned::Spanned;

struct Implements(Vec<winrt_gen::Type>);

impl syn::parse::Parse for Implements {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut types = Vec::new();
        let reader = build_reader();

        loop {
            use_tree_to_types(reader, &input.parse::<syn::UseTree>()?, &mut types)?;

            if input.parse::<syn::Token!(,)>().is_err() {
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
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _input_stream = input.clone();

    let implements = syn::parse_macro_input!(attribute as Implements);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let impl_name = input.ident.clone();
    let box_name = quote::format_ident!("impl_{}", impl_name.to_string());

    let mut vtables = vec![];
    let mut vtable_ctors = vec![];
    let mut method_impls = vec![];

    for typ in implements.0 {
        if let winrt_gen::Type::Interface(typ) = typ {
            // TODO: maybe delay conversion to proc_macro2 until later in the pipeline.
            let name = typ
                .name
                .gen_binding_abi()
                .parse::<proc_macro2::TokenStream>()
                .unwrap();
            let mut initializers = vec![];

            for method in &typ.default_interface().methods {
                let method_name = quote::format_ident!("{}", &method.name);
                initializers.push(quote::quote! { #method_name: #box_name::#method_name });

                let method = method
                    .gen_binding_abi_impl(&typ.name)
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap();
                method_impls.push(quote::quote! {
                    #method { panic!(); }
                });
            }

            vtable_ctors.push(quote::quote! {
                #name {
                    inspectable: ::winrt::abi_IInspectable {
                        unknown: ::winrt::abi_IUnknown {
                            query_interface: #box_name::unknown_query_interface,
                            add_ref: #box_name::unknown_add_ref,
                            release: #box_name::unknown_release,
                        },
                        inspectable_iids: #box_name::inspectable_iids,
                        inspectable_type_name: #box_name::inspectable_type_name,
                        inspectable_trust_level: #box_name::inspectable_trust_level,
                    },
                    #(#initializers)*,
                }
            });

            vtables.push(name);
        }
    }

    let tokens = quote::quote! {
        #input
        // TODO: something like this takes the place of C++/WinRT's make/make_self functions.
        // impl ::std::convert::Into<::winrt::foundation::IStringable> for #impl_name {
        //     fn into(self) -> ::winrt::foundation::IStringable {
        //         panic!();
        //     }
        // }
        #[repr(C)]
        struct #box_name {
            vtable: (#(*const #vtables),*),
            references: ::winrt::RefCount,
            implementation: #impl_name,
        }
        impl #box_name {
            const VTABLE: (#(#vtables),*) = (#(#vtable_ctors),*);

            extern "system" fn unknown_query_interface(
                this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>,
                iid: &::winrt::Guid,
                interface: *mut ::winrt::RawPtr,
            ) -> ::winrt::ErrorCode {
                panic!();
            }
            extern "system" fn unknown_add_ref(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                panic!();
            }
            extern "system" fn unknown_release(this: ::winrt::NonNullRawComPtr<::winrt::IUnknown>) -> u32 {
                panic!();
            }
            extern "system" fn inspectable_iids(this: ::winrt::NonNullRawComPtr<::winrt::Object>, count: *mut u32, values: *mut *mut ::winrt::Guid) -> ::winrt::ErrorCode {
                panic!();
            }
            extern "system" fn inspectable_type_name(this: ::winrt::NonNullRawComPtr<::winrt::Object>, name: *mut <::winrt::HString as ::winrt::AbiTransferable>::Abi) -> ::winrt::ErrorCode {
                panic!();
            }
            extern "system" fn inspectable_trust_level(this: ::winrt::NonNullRawComPtr<::winrt::Object>, level: *mut i32) -> ::winrt::ErrorCode {
                panic!();
            }
            #(#method_impls)*
        }

        // Build the scaffolding for implementing the interfaces.
    };

    println!("{}", tokens.to_string());

    tokens.into()
}
