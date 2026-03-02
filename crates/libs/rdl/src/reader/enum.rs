use super::*;

pub fn encode_enum(encoder: &mut Encoder, item: &syntax::Enum) -> Result<(), Error> {
    let value_type = encoder.output.TypeRef("System", "Enum");

    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags,
    );

    let Some(attribute) = item
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("repr"))
    else {
        return encoder.err(item.token, "`repr` attribute not found");
    };

    let Ok(ty) = attribute.parse_args::<syn::Path>() else {
        return encoder.err(attribute, "`repr` integer type attribute not found");
    };

    let ty = encode_path(encoder, &ty)?;

    encoder.output.Field(
        "value__",
        &ty,
        metadata::FieldAttributes::Private
            | metadata::FieldAttributes::SpecialName
            | metadata::FieldAttributes::RTSpecialName,
    );

    let type_name = metadata::Type::named(encoder.namespace, encoder.name);

    for variant in &item.variants {
        let name = variant.ident.to_string();

        let field = encoder.output.Field(
            &name,
            &type_name,
            metadata::FieldAttributes::Public
                | metadata::FieldAttributes::Static
                | metadata::FieldAttributes::Literal,
        );

        let Some((_, value)) = &variant.discriminant else {
            return encoder.err(variant, "variant value not found");
        };

        let value = encode_value(encoder, &ty, value)?;

        encoder
            .output
            .Constant(metadata::writer::HasConstant::Field(field), &value);
    }

    Ok(())
}
