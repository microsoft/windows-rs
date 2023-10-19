use super::*;

pub fn verify(reader: &metadata::Reader) -> crate::Result<()> {
    let unused: Vec<&str> = reader.unused().collect();

    if !unused.is_empty() {
        let mut message = "unused filters".to_string();

        for unused in unused {
            message.push_str(&format!("\n  {unused}"));
        }

        return Err(crate::Error::new(&message));
    }

    for item in reader.items() {
        // TODO: cover all variants
        let metadata::Item::Type(def) = item else {
            continue;
        };

        let generics = &metadata::type_def_generics(def);

        def.fields().try_for_each(|field| not_type_ref(&field.ty(Some(def))))?;

        def.methods().try_for_each(|method| {
            let sig = method.signature(generics);
            not_type_ref(&sig.return_type)?;

            sig.params.iter().try_for_each(not_type_ref)
        })?;
    }

    Ok(())
}

fn not_type_ref(ty: &metadata::Type) -> crate::Result<()> {
    if let metadata::Type::TypeRef(ty) = ty {
        return Err(crate::Error::new(&format!("missing type definition `{}`", ty)));
    }
    Ok(())
}
