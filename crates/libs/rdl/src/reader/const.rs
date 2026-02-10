use super::*;

pub fn encode_const(encoder: &mut Encoder, item: &syntax::Const) -> Result<(), Error> {
    let name = item.name.to_string();
    let ty = encode_type(encoder, &item.ty)?;
    let value = encode_value(encoder, &ty, &item.expr)?;

    let field = encoder.output.Field(
        &name,
        &ty,
        metadata::FieldAttributes::Public
            | metadata::FieldAttributes::Static
            | metadata::FieldAttributes::Literal,
    );

    encoder
        .output
        .Constant(metadata::writer::HasConstant::Field(field), &value);

    Ok(())
}
