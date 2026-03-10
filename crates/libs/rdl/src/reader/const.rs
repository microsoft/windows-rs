use super::*;

#[derive(Debug)]
pub struct Const {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![const],
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub expr: Option<syn::Expr>,
}

impl syn::parse::Parse for Const {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        let expr = if input.peek(syn::Token![=]) {
            input.parse::<syn::Token![=]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            token,
            name,
            ty,
            expr,
        })
    }
}

impl Const {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let name = self.name.to_string();
        let ty = encode_type(encoder, &self.ty)?;

        match &ty {
            windows_metadata::Type::Name(tn) if tn == ("System", "Guid") => {
                encode_const_guid(encoder, &ty, self, &name)?;
            }
            _ => encode_const_value(encoder, &ty, self, &name)?,
        }

        Ok(())
    }
}

fn encode_const_value(
    encoder: &mut Encoder,
    ty: &windows_metadata::Type,
    item: &Const,
    name: &str,
) -> Result<(), Error> {
    let field = encoder.output.Field(
        name,
        ty,
        metadata::FieldAttributes::Public
            | metadata::FieldAttributes::Static
            | metadata::FieldAttributes::Literal,
    );

    if let Some(expr) = &item.expr {
        let value = encode_value(encoder, ty, expr)?;
        encoder
            .output
            .Constant(metadata::writer::HasConstant::Field(field), &value);
    }

    encode_attrs(
        encoder,
        metadata::writer::HasAttribute::Field(field),
        &item.attrs,
        &[],
    )?;

    Ok(())
}

fn encode_const_guid(
    encoder: &mut Encoder,
    ty: &windows_metadata::Type,
    item: &Const,
    name: &str,
) -> Result<(), Error> {
    let expr = item.expr.as_ref().expect("GUID const missing value");
    let value: u128 = encode_lit_int(encoder, expr)?;
    let field = encoder.output.Field(
        name,
        ty,
        metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
    );

    let guid_typeref = encoder
        .output
        .TypeRef("Windows.Foundation.Metadata", "GuidAttribute");

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
