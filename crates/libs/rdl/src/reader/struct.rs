use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub name: syn::Ident,
    pub fields: Vec<Field>,
    pub winrt: bool,
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token: syn::Token![struct] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(Field::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            span: token.span,
            name,
            fields,
            winrt: false,
        })
    }
}

impl Struct {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let type_def = encode_struct_or_union(
            encoder,
            &self.name.to_string(),
            self.winrt,
            false,
            &self.fields,
        )?;

        if let Some(packing_size) = read_packed(encoder, &self.attrs)? {
            encoder.output.ClassLayout(type_def, packing_size, 0);
        }

        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(type_def),
            &self.attrs,
            &["packed"],
        )
    }
}

/// Parse an optional `#[packed(N)]` attribute from `attrs`.  Returns `Some(N)` if
/// the attribute is present and well-formed, `None` if absent, or an error if the
/// attribute is malformed.
fn read_packed(encoder: &Encoder, attrs: &[syn::Attribute]) -> Result<Option<u16>, Error> {
    for attr in attrs {
        if !attr.path().is_ident("packed") {
            continue;
        }

        let Ok(size_literal) = attr.parse_args::<syn::LitInt>() else {
            return encoder.err(attr, "`packed` attribute requires an integer argument");
        };

        let Ok(size) = size_literal.base10_parse::<u16>() else {
            return encoder.err(attr, "`packed` size must be a valid u16");
        };

        return Ok(Some(size));
    }

    Ok(None)
}

/// Encode a flat struct or union type definition into the metadata output.
pub fn encode_struct_or_union(
    encoder: &mut Encoder,
    item_name: &str,
    winrt: bool,
    is_union: bool,
    fields: &[Field],
) -> Result<metadata::writer::TypeDef, Error> {
    let value_type = encoder.output.TypeRef("System", "ValueType");

    let layout_flag = if is_union {
        metadata::TypeAttributes::ExplicitLayout
    } else {
        metadata::TypeAttributes::SequentialLayout
    };

    let flags = layout_flag
        | metadata::TypeAttributes::Sealed
        | metadata::TypeAttributes::Public
        | if winrt {
            metadata::TypeAttributes::WindowsRuntime
        } else {
            metadata::TypeAttributes::default()
        };

    let type_def = encoder.output.TypeDef(
        encoder.namespace,
        item_name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags,
    );

    for field in fields {
        let field_name = field.name.to_string();
        let mt = encode_type(encoder, &field.ty)?;
        let field_id = encoder
            .output
            .Field(&field_name, &mt, metadata::FieldAttributes::Public);
        if is_union {
            encoder.output.FieldLayout(field_id, 0);
        }
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::Field(field_id),
            &field.attrs,
            &[],
        )?;
    }

    Ok(type_def)
}
