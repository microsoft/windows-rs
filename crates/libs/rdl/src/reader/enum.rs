use super::*;

pub fn encode_enum(encoder: &mut Encoder, item: &syntax::Enum) -> Result<(), Error> {
    let value_type = encoder.output.TypeRef("System", "Enum");

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime,
    );

    let Some(attribute) = item
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("repr"))
    else {
        return encoder.err(item.token, "`repr` attribute not found");
    };

    let Ok(ty) = attribute.parse_args::<syn::Ident>() else {
        return encoder.err(attribute, "`repr` integer type attribute not found");
    };

    let ty = ty.to_string();

    let ty = match ty.as_str() {
        "u32" => metadata::Type::U32,
        "u64" => metadata::Type::U64,
        "i32" => metadata::Type::I32,
        "i64" => metadata::Type::I64,
        _ => {
            return encoder.err(attribute, "`repr` integer type not supported");
        }
    };

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

        let syn::Expr::Lit(value) = value else {
            return encoder.err(variant, "variant value not literal");
        };

        let syn::Lit::Int(value) = &value.lit else {
            return encoder.err(variant, "variant value not integer");
        };

        let value = match ty {
            metadata::Type::U32 => metadata::Value::U32(
                value
                    .base10_parse::<u32>()
                    .map_err(|_| encoder.error(variant, "variant value not valid"))?,
            ),
            metadata::Type::U64 => metadata::Value::U64(
                value
                    .base10_parse::<u64>()
                    .map_err(|_| encoder.error(variant, "variant value not valid"))?,
            ),
            metadata::Type::I32 => metadata::Value::I32(
                value
                    .base10_parse::<i32>()
                    .map_err(|_| encoder.error(variant, "variant value not valid"))?,
            ),
            metadata::Type::I64 => metadata::Value::I64(
                value
                    .base10_parse::<i64>()
                    .map_err(|_| encoder.error(variant, "variant value not valid"))?,
            ),
            _ => todo!(),
        };

        encoder
            .output
            .Constant(metadata::writer::HasConstant::Field(field), &value);
    }

    Ok(())
}
