pub fn verify(reader: &metadata::Reader, filter: &metadata::Filter) -> crate::Result<()> {
    for def in reader.types(filter) {
        for field in reader.type_def_fields(def) {
            let ty = reader.field_type(field, Some(def));
            if let metadata::Type::TypeRef(ty) = ty {
                return Err(crate::Error::new(&format!(
                    "missing type definition `{}`",
                    reader.type_def_or_ref(ty)
                )));
            }
        }
        // for method in reader.type_def_methods(def) {
        //     let sig = reader.method_def_signature(method);

        // }
    }

    Ok(())
}
