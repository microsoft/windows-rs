use syn::spanned::Spanned;

struct Implements(Vec<winrt_gen::Type>);

impl syn::parse::Parse for Implements {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut types = Vec::new();
        let reader = winrt_gen::TypeReader::from_build();

        loop {
            use_tree_to_types(reader, &input.parse::<syn::UseTree>()?, &mut types)?;

            if input.parse::<syn::Token!(,)>().is_err() {
                break;
            }
        }

        Ok(Self(types))
    }
}

fn use_tree_to_types(reader: &winrt_gen::TypeReader, tree: &syn::UseTree, types: &mut Vec<winrt_gen::Type>) -> syn::parse::Result<()> {
    fn recurse(
        reader: &winrt_gen::TypeReader,
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
                    None => return Err(syn::parse::Error::new(name.span(), "Metadata not found for type name")),
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
                return Err(syn::parse::Error::new(glob.span(), "Glob syntax is not supported"))
            }
            syn::UseTree::Rename(rename) => {
                return Err(syn::parse::Error::new(rename.span(), "Rename syntax is not supported"))
            }
        }

        Ok(())
    }

    recurse(reader, tree, types, &mut String::new())
}

pub fn to_tokens(attribute: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _input_stream = input.clone();

    let implements = syn::parse_macro_input!(attribute as Implements);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let impl_name = input.ident.clone();
    let box_name = quote::format_ident!("impl_{}", impl_name.to_string());

    let vtable_pointers = implements.0.iter().map(|typ| {
        //let typ: proc_macro::TokenStream = typ.to_tokens().into();
        quote::quote! {
            *const i32 //#typ
        }
    });

    let tokens = quote::quote! {
        #input
        impl ::std::convert::Into<::winrt::foundation::IStringable> for #impl_name {
            fn into(self) -> ::winrt::foundation::IStringable {
                panic!();
            }
        }
        #[repr(C)]
        struct #box_name {
            vtables: (#(#vtable_pointers),*),
            references: ::winrt::RefCount,
            implementation: #impl_name,
        }
        impl #box_name {

        }

        // Build the scaffolding for implementing the interfaces.
    };

    tokens.into()
}
