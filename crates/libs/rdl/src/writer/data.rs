use super::*;

pub fn write_data(namespace: &str, item: &metadata::reader::Field) -> Result<TokenStream, Error> {
    let import = item
        .data_import()
        .map_err(|error| writer_err!("{error}"))?
        .ok_or_else(|| writer_err!("data `{}` has no import descriptor", item.name()))?;
    let name = write_ident(item.name());
    let ty = write_type(namespace, &item.ty());
    let dll = import.dll;
    let target = match import.target {
        metadata::reader::NativeDataImportTarget::Name(name) => quote! { name = #name },
        metadata::reader::NativeDataImportTarget::Ordinal(ordinal) => {
            let ordinal = Literal::u16_unsuffixed(ordinal);
            quote! { ordinal = #ordinal }
        }
    };
    let kind = match import.kind {
        metadata::reader::NativeDataImportKind::Data => quote! { data },
        metadata::reader::NativeDataImportKind::Constant => quote! { constant },
    };
    let arch_attr = write_arch_attr(item.arches());
    let attrs = write_custom_attributes_except(
        item.attributes(),
        namespace,
        item.index(),
        &[
            "SupportedArchitectureAttribute",
            "NativeDataImportAttribute",
        ],
    )?;
    Ok(quote! {
        #arch_attr
        #(#attrs)*
        #[import(#dll, #target, kind = #kind)]
        static #name: #ty;
    })
}
