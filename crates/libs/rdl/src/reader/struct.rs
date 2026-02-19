use super::*;

pub fn encode_struct(encoder: &mut Encoder, item: &syntax::Struct) -> Result<(), Error> {
    let mut depth = 0;
    encode_struct_inner(encoder, item, None, encoder.name, &mut depth)
}

fn encode_struct_inner(
    encoder: &mut Encoder,
    item: &syntax::Struct,
    outer: Option<metadata::writer::TypeDef>,
    name: &str,
    depth: &mut usize,
) -> Result<(), Error> {
    let nested = nested(item, name, depth);

    let value_type = encoder.output.TypeRef("System", "ValueType");
    let mut flags = metadata::TypeAttributes::SequentialLayout | metadata::TypeAttributes::Sealed;

    flags |= if outer.is_some() {
        metadata::TypeAttributes::NestedPublic
    } else {
        metadata::TypeAttributes::Public
    };

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    let ty = encoder.output.TypeDef(
        encoder.namespace,
        name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags,
    );

    fields(encoder, item, &nested)?;

    if let Some(outer) = outer {
        encoder.output.NestedClass(ty, outer);
    }

    for (nested_type_name, def) in nested.values() {
        *depth = 0;
        encode_struct_inner(encoder, def, Some(ty), nested_type_name, depth)?;
    }

    Ok(())
}

fn fields(
    encoder: &mut Encoder,
    item: &syntax::Struct,
    nested: &BTreeMap<String, (String, &syntax::Struct)>,
) -> Result<(), Error> {
    for field in &item.fields {
        match field {
            syntax::StructField::Regular(f) => {
                let name = f.ident.as_ref().unwrap().to_string();
                let ty = encode_type(encoder, &f.ty)?;
                encoder
                    .output
                    .Field(&name, &ty, metadata::FieldAttributes::Public);
            }
            syntax::StructField::Nested { name, .. } => {
                let field_name = name.to_string();
                let (nested_type_name, _) = &nested[&field_name];
                let ty = metadata::Type::named(encoder.namespace, nested_type_name.as_str());
                encoder
                    .output
                    .Field(&field_name, &ty, metadata::FieldAttributes::Public);
            }
        }
    }

    Ok(())
}

fn nested<'a>(
    item: &'a syntax::Struct,
    name: &str,
    depth: &mut usize,
) -> BTreeMap<String, (String, &'a syntax::Struct)> {
    item.fields
        .iter()
        .filter_map(|field| {
            if let syntax::StructField::Nested {
                name: ident_name,
                def,
                ..
            } = field
            {
                let field_name = ident_name.to_string();
                let nested_name = format!("{}_{}", name, *depth);
                *depth += 1;
                Some((field_name, (nested_name, def)))
            } else {
                None
            }
        })
        .collect()
}
