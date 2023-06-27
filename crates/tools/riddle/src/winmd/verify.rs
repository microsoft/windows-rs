pub fn verify(reader: &metadata::Reader, filter: &metadata::Filter) -> crate::Result<()> {
    for def in reader.types(filter) {
        let generics = &reader.type_def_generics(def);

        reader
            .type_def_fields(def)
            .try_for_each(|field| not_type_ref(reader, &reader.field_type(field, Some(def))))?;

        reader.type_def_methods(def).try_for_each(|method| {
            let sig = reader.method_def_signature(method, generics);
            not_type_ref(reader, &sig.return_type)?;

            sig.params
                .iter()
                .try_for_each(|param| not_type_ref(reader, &param.ty))
        })?;
    }

    Ok(())
}

fn not_type_ref(reader: &metadata::Reader, ty: &metadata::Type) -> crate::Result<()> {
    if let metadata::Type::TypeRef(ty) = ty {
        return Err(crate::Error::new(&format!(
            "missing type definition `{}`",
            reader.type_def_or_ref(*ty)
        )));
    }
    Ok(())
}
