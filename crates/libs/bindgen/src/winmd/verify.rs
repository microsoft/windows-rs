use super::*;
use metadata::RowReader;

pub fn verify(reader: &metadata::Reader, filter: &metadata::Filter) -> crate::Result<()> {
    for item in reader.items(filter) {
        // TODO: cover all variants
        let metadata::Item::Type(def) = item else {
            continue;
        };

        let generics = &metadata::type_def_generics(reader, def);

        reader.type_def_fields(def).try_for_each(|field| not_type_ref(reader, &reader.field_type(field, Some(def))))?;

        reader.type_def_methods(def).try_for_each(|method| {
            let sig = reader.method_def_signature(method, generics);
            not_type_ref(reader, &sig.return_type)?;

            sig.params.iter().try_for_each(|param| not_type_ref(reader, param))
        })?;
    }

    Ok(())
}

fn not_type_ref(reader: &metadata::Reader, ty: &metadata::Type) -> crate::Result<()> {
    if let metadata::Type::TypeRef(ty) = ty {
        return Err(crate::Error::new(&format!("missing type definition `{}`", reader.type_def_or_ref(*ty))));
    }
    Ok(())
}
