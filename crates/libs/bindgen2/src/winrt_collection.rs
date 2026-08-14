use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) struct Conveniences {
    pub(super) before_runtime_name: TokenStream,
    pub(super) after_implementation: TokenStream,
}

pub(super) fn write(
    interface: &winrt_interface::Interface,
    namespace: &str,
    layout: Layout,
    projection: Projection,
    members: &MemberSelection,
    artifact_cfg: &TokenStream,
) -> Result<Conveniences, Error> {
    if projection.is_minimal() {
        return Ok(Conveniences {
            before_runtime_name: TokenStream::new(),
            after_implementation: TokenStream::new(),
        });
    }

    let name = tokens::ident(&interface.name);
    let generic_names = interface
        .generics
        .iter()
        .map(|name| tokens::ident(name))
        .collect::<Vec<_>>();
    let constraints = generic_names
        .iter()
        .map(|name| quote! { #name: windows_core::RuntimeType + 'static })
        .collect::<Vec<_>>();
    let constrained_generics = if constraints.is_empty() {
        quote! {}
    } else {
        quote! { <#(#constraints),*> }
    };
    let type_arguments = if generic_names.is_empty() {
        quote! {}
    } else {
        quote! { <#(#generic_names),*> }
    };

    let direct_iterable = (interface.namespace == "Windows.Foundation.Collections"
        && interface.name == "IIterable"
        && generic_names.len() == 1
        && has_method(interface, members, "First"))
    .then(|| {
        let item = &generic_names[0];
        iterable(
            &name,
            &quote! { <#item: windows_core::RuntimeType> },
            &quote! { <#item> },
            item,
            artifact_cfg,
        )
    });
    let direct_iterator = (interface.namespace == "Windows.Foundation.Collections"
        && interface.name == "IIterator"
        && generic_names.len() == 1
        && ["Current", "HasCurrent", "MoveNext"]
            .iter()
            .all(|name| has_method(interface, members, name)))
    .then(|| {
        let item = &generic_names[0];
        quote! {
            #artifact_cfg
            impl<#item: windows_core::RuntimeType> Iterator for #name<#item> {
                type Item = #item;
                fn next(&mut self) -> Option<Self::Item> {
                    let result = if self.HasCurrent().unwrap_or(false) {
                        self.Current().ok()
                    } else {
                        None
                    };
                    if result.is_some() {
                        let _ = self.MoveNext();
                    }
                    result
                }
            }
        }
    });
    let required_iterable = interface
        .required
        .iter()
        .find(|required| {
            required.namespace == "Windows.Foundation.Collections"
                && required.name == "IIterable"
                && required.arguments.len() == 1
                && required
                    .methods
                    .iter()
                    .any(|method| method.name == "First" && method.selected(members))
        })
        .map(|required| {
            let item = required.arguments[0].write_name(namespace, layout, &interface.generics)?;
            Ok::<_, Error>(iterable(
                &name,
                &constrained_generics,
                &type_arguments,
                &item,
                artifact_cfg,
            ))
        })
        .transpose()?;

    Ok(Conveniences {
        before_runtime_name: quote! { #required_iterable },
        after_implementation: quote! { #direct_iterable #direct_iterator },
    })
}

fn has_method(
    interface: &winrt_interface::Interface,
    members: &MemberSelection,
    name: &str,
) -> bool {
    interface
        .methods
        .iter()
        .any(|method| method.name == name && method.selected(members))
}

pub(super) fn iterable(
    name: &TokenStream,
    generics: &TokenStream,
    type_arguments: &TokenStream,
    item: &TokenStream,
    cfg: &TokenStream,
) -> TokenStream {
    quote! {
        #cfg
        impl #generics IntoIterator for #name #type_arguments {
            type Item = #item;
            type IntoIter = windows_collections::BufferedIterator<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                IntoIterator::into_iter(&self)
            }
        }
        #cfg
        impl #generics IntoIterator for &#name #type_arguments {
            type Item = #item;
            type IntoIter = windows_collections::BufferedIterator<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                windows_collections::BufferedIterator::new(self.First().unwrap())
            }
        }
    }
}
