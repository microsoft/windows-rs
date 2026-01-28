use super::*;

pub fn write_struct(item: &metadata::reader::TypeDef) -> TokenStream {
    let namespace = item.namespace();
    let name = format_ident!("{}", item.name());
    let fields = item.fields().map(|field| write_field(namespace, &field));

    quote! {
        struct #name {
            #(#fields)*
        }
    }
}

fn write_field(namespace: &str, item: &metadata::reader::Field) -> TokenStream {
    let name = format_ident!("{}", item.name());
    let ty = write_type(namespace, &item.ty());

    quote! {
        #name: #ty,
    }
}
