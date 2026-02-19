use super::*;
use windows_metadata::AsRow;

pub fn write_struct(item: &metadata::reader::TypeDef) -> TokenStream {
    if item
        .flags()
        .contains(metadata::TypeAttributes::NestedPublic)
    {
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
            let nested_ty = item
                .index()
                .get(&ty_name.namespace, &ty_name.name)
                .next()
                .unwrap();

            let fields = nested_ty
                .fields()
                .map(|field| write_field(namespace, &field));
            quote! {
                struct {
                    #(#fields)*
                }
            }
        }
        _ => write_type(namespace, &item.ty()),
    };

    quote! {
        #name: #ty,
    }
}
