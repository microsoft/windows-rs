use super::*;

pub fn encode_const(encoder: &mut Encoder, item: &syntax::Const) -> Result<(), Error> {
    let name = item.name.to_string();
    let ty = encode_type(encoder, &item.ty)?;

    match &ty {
        windows_metadata::Type::Name(tn) if tn == ("System", "Guid") => {
            encode_const_guid(encoder, &ty, item, &name)?;
        }
        _ => encode_const_value(encoder, &ty, item, &name)?,
    }

    Ok(())
}

fn encode_const_value(
    encoder: &mut Encoder,
    ty: &windows_metadata::Type,
    item: &syntax::Const,
    name: &str,
) -> Result<(), Error> {
    let value = encode_value(encoder, ty, &item.expr)?;

    let field = encoder.output.Field(
        name,
        ty,
        metadata::FieldAttributes::Public
            | metadata::FieldAttributes::Static
            | metadata::FieldAttributes::Literal,
    );

    encoder
        .output
        .Constant(metadata::writer::HasConstant::Field(field), &value);

    Ok(())
}

fn encode_const_guid(
    encoder: &mut Encoder,
    ty: &windows_metadata::Type,
    item: &syntax::Const,
    name: &str,
) -> Result<(), Error> {
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(lit),
        ..
    }) = &item.expr
    else {
        panic!("expected integer literal");
    };

    let value: u128 = lit.base10_parse().expect("invalid guid literal");
    let field = encoder.output.Field(
        name,
        ty,
        metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
    );

    let guid_typeref = encoder
        .output
        .TypeRef("Windows.Win32.Foundation.Metadata", "GuidAttribute");

    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::HASTHIS,
        return_type: metadata::Type::Void,
        types: vec![
            metadata::Type::U32,
            metadata::Type::U16,
            metadata::Type::U16,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
            metadata::Type::U8,
        ],
    };

    let ctor = encoder.output.MemberRef(
        ".ctor",
        &signature,
        metadata::writer::MemberRefParent::TypeRef(guid_typeref),
    );

    let val = |val: metadata::Value| (String::new(), val);
    encoder.output.Attribute(
        metadata::writer::HasAttribute::Field(field),
        metadata::writer::AttributeType::MemberRef(ctor),
        &[
            val(metadata::Value::U32((value >> 96) as u32)),
            val(metadata::Value::U16((value >> 80) as u16)),
            val(metadata::Value::U16((value >> 64) as u16)),
            val(metadata::Value::U8((value >> 56) as u8)),
            val(metadata::Value::U8((value >> 48) as u8)),
            val(metadata::Value::U8((value >> 40) as u8)),
            val(metadata::Value::U8((value >> 32) as u8)),
            val(metadata::Value::U8((value >> 24) as u8)),
            val(metadata::Value::U8((value >> 16) as u8)),
            val(metadata::Value::U8((value >> 8) as u8)),
            val(metadata::Value::U8(value as u8)),
        ],
    );

    Ok(())
}
