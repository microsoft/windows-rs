use super::*;
use windows_metadata::AsRow;

pub fn write_struct(item: &metadata::reader::TypeDef) -> TokenStream {
    if is_nested_type(item) {
        return quote! {};
    }

    let namespace = item.namespace();
    let name = write_ident(item.name());

    let fields = item
        .fields()
        .map(|field| write_field(namespace, item, &field));

    let keyword = if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    };

    let custom_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! {
        #(#custom_attrs)*
        #keyword #name {
            #(#fields)*
        }
    }
}

fn write_field(
    namespace: &str,
    parent: &metadata::reader::TypeDef,
    item: &metadata::reader::Field,
) -> TokenStream {
    let name = write_ident(item.name());

    let ty = match item.ty() {
        metadata::Type::Name(ty_name) => {
            if let Some(_resolved) = item.index().get(namespace, &ty_name.name).next() {
                write_type(
                    namespace,
                    &metadata::Type::named(&ty_name.namespace, &ty_name.name),
                )
            } else if ty_name.namespace.is_empty() && !ty_name.name.contains('/') {
                let nested = item
                    .index()
                    .nested(*parent)
                    .find(|t| t.name() == ty_name.name)
                    .unwrap_or_else(|| panic!("Could not resolve nested type: {}", ty_name.name));

                let keyword = nested_keyword(&nested);
                let fields = nested
                    .fields()
                    .map(|f| write_field(namespace, &nested, &f))
                    .collect::<Vec<_>>();
                quote! { #keyword { #(#fields)* } }
            } else if ty_name.namespace.is_empty() {
                let mut segments = ty_name.name.split('/');
                let first = segments.next().unwrap();

                let mut resolved = item
                    .index()
                    .nested(*parent)
                    .find(|t| t.name() == first)
                    .unwrap_or_else(|| panic!("Could not resolve nested type: {}", first));

                for segment in segments {
                    resolved = item
                        .index()
                        .nested(resolved)
                        .find(|t| t.name() == segment)
                        .unwrap_or_else(|| panic!("Could not resolve nested type: {}", segment));
                }

                let keyword = nested_keyword(&resolved);
                let fields = resolved
                    .fields()
                    .map(|f| write_field(namespace, &resolved, &f))
                    .collect::<Vec<_>>();
                quote! { #keyword { #(#fields)* } }
            } else {
                write_type(namespace, &metadata::Type::Name(ty_name.clone()))
            }
        }
        _ => write_type(namespace, &item.ty()),
    };

    let field_attrs = write_custom_attributes(item.attributes(), namespace, item.index());

    quote! { #(#field_attrs)* #name: #ty, }
}

fn is_nested_type(item: &metadata::reader::TypeDef) -> bool {
    item.flags()
        .contains(metadata::TypeAttributes::NestedPublic)
}

fn nested_keyword(item: &metadata::reader::TypeDef) -> TokenStream {
    if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    }
}
