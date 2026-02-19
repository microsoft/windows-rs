use super::*;
use windows_metadata::AsRow;

pub fn write_struct(item: &metadata::reader::TypeDef) -> TokenStream {
    if is_nested_type(item) {
        return quote! {};
    }

    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());

    let fields = item.fields().map(|field| write_field(namespace, &field));

    let keyword = if item
        .flags()
        .contains(metadata::TypeAttributes::ExplicitLayout)
    {
        quote! { union }
    } else {
        quote! { struct }
    };

    quote! {
        #keyword #name {
            #(#fields)*
        }
    }
}

fn write_field(namespace: &str, item: &metadata::reader::Field) -> TokenStream {
    let name = format_ident!("{}", item.name());

    let ty = match item.ty() {
        metadata::Type::Name(ty_name) => {
            if let Some(resolved_type) = item.index().get(namespace, &ty_name.name).next() {
                if is_nested_type(&resolved_type) {
                    let fields = resolved_type
                        .fields()
                        .map(|field| write_field(namespace, &field));
                    quote! { struct { #(#fields)* } }
                } else {
                    write_type(
                        namespace,
                        &metadata::Type::named(&ty_name.namespace, &ty_name.name),
                    )
                }
            } else {
                write_type(
                    namespace,
                    &metadata::Type::named(&ty_name.namespace, &ty_name.name),
                )
            }
        }
        _ => write_type(namespace, &item.ty()),
    };

    quote! {
        #name: #ty,
    }
}

fn is_nested_type(item: &metadata::reader::TypeDef) -> bool {
    item.flags()
        .contains(metadata::TypeAttributes::NestedPublic)
}
