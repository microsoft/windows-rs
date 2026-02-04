use super::*;

pub fn encode_struct(encoder: &mut Encoder, item: &syntax::Struct) -> Result<(), Error> {
    let value_type = encoder.output.TypeRef("System", "ValueType");

            let flags = if item.winrt {
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::SequentialLayout
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime
    } else {
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::SequentialLayout
            | metadata::TypeAttributes::Sealed
    };

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags
    );

    for field in &item.fields {
        // Note: syntax parsing requires named fields so the following `unwrap` should not panic.
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = encode_type(encoder, &field.ty)?;

        encoder
            .output
            .Field(&name, &ty, metadata::FieldAttributes::Public);
    }

    Ok(())
}
