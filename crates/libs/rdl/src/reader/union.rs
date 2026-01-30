use super::*;

pub fn encode_union(encoder: &mut Encoder, item: &syntax::Union) -> Result<(), Error> {
    let value_type = encoder.output.TypeRef("System", "ValueType");

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::ExplicitLayout
            | metadata::TypeAttributes::Sealed,
    );

    for field in &item.fields {
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = encode_type(encoder, &field.ty)?;
        let field_id = encoder
            .output
            .Field(&name, &ty, metadata::FieldAttributes::Public);

        encoder.output.FieldLayout(field_id, 0);
    }

    Ok(())
}
