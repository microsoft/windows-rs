use super::*;

#[derive(Debug, Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: FieldType,
    pub bitfields: Vec<BitfieldMember>,
}

#[derive(Debug, Clone)]
pub struct BitfieldMember {
    pub name: Option<String>,
    pub ty: Option<syn::Type>,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Type(Box<syn::Type>),
    Nested(Box<NestedRecord>),
}

#[derive(Debug, Clone)]
pub struct NestedRecord {
    pub attrs: Vec<syn::Attribute>,
    pub name: Option<syn::Ident>,
    pub is_union: bool,
    pub body: RecordBody,
}

#[derive(Debug, Clone, Default)]
pub struct RecordBody {
    pub fields: Vec<Field>,
    pub records: Vec<NestedRecord>,
    pub constants: Vec<Const>,
    pub enums: Vec<Enum>,
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        // The type position may be an inline anonymous nested struct/union,
        // optionally preceded by its own attributes.
        let inner_attrs = input.call(syn::Attribute::parse_outer)?;
        let (ty, bitfields) = if input.peek(syn::Token![struct]) || input.peek(syn::Token![union]) {
            (
                FieldType::Nested(Box::new(NestedRecord::parse(input, inner_attrs, false)?)),
                vec![],
            )
        } else if inner_attrs.is_empty() {
            let ty = FieldType::Type(Box::new(input.parse()?));
            // A backing integer field may be followed by a `{ ... }` block listing its
            // bit-field members in the concise C-like form.
            let bitfields = if input.peek(syn::token::Brace) {
                parse_bitfield_block(input)?
            } else {
                vec![]
            };
            (ty, bitfields)
        } else {
            return Err(input
                .error("attributes are only allowed on an inline nested struct/union field type"));
        };

        Ok(Self {
            attrs,
            name,
            ty,
            bitfields,
        })
    }
}

fn parse_bitfield_block(input: syn::parse::ParseStream) -> syn::Result<Vec<BitfieldMember>> {
    let content;
    syn::braced!(content in input);
    let members = content
        .parse_terminated(<BitfieldMember as syn::parse::Parse>::parse, syn::Token![,])?
        .into_iter()
        .collect();
    Ok(members)
}

impl syn::parse::Parse for BitfieldMember {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = if input.peek(syn::Token![_]) {
            input.parse::<syn::Token![_]>()?;
            None
        } else if input.peek(syn::LitStr) {
            Some(input.parse::<syn::LitStr>()?.value())
        } else {
            Some(input.parse::<syn::Ident>()?.unraw_to_string())
        };
        input.parse::<syn::Token![:]>()?;
        let ty = if input.peek(syn::LitInt) {
            None
        } else {
            if name.is_none() {
                return Err(input.error("anonymous bitfield padding cannot have a member type"));
            }
            let ty = input.parse()?;
            input.parse::<syn::Token![:]>()?;
            Some(ty)
        };
        let width: syn::LitInt = input.parse()?;
        Ok(Self {
            name,
            ty,
            width: width.base10_parse()?,
        })
    }
}

impl NestedRecord {
    fn parse(
        input: syn::parse::ParseStream,
        attrs: Vec<syn::Attribute>,
        named: bool,
    ) -> syn::Result<Self> {
        let is_union = input.peek(syn::Token![union]);
        if is_union {
            input.parse::<syn::Token![union]>()?;
        } else {
            input.parse::<syn::Token![struct]>()?;
        }
        let name = if named { Some(input.parse()?) } else { None };

        let content;
        syn::braced!(content in input);
        let body = parse_record_body(&content)?;

        Ok(Self {
            attrs,
            name,
            is_union,
            body,
        })
    }
}

pub fn parse_record_body(input: syn::parse::ParseStream) -> syn::Result<RecordBody> {
    let mut body = RecordBody::default();
    while !input.is_empty() {
        let fork = input.fork();
        fork.call(syn::Attribute::parse_outer)?;
        if fork.peek(syn::Token![struct]) || fork.peek(syn::Token![union]) {
            let attrs = input.call(syn::Attribute::parse_outer)?;
            body.records.push(NestedRecord::parse(input, attrs, true)?);
        } else if fork.peek(syn::Token![const]) {
            body.constants.push(input.parse()?);
        } else if fork.peek(syn::Token![enum]) {
            body.enums.push(input.parse()?);
        } else {
            body.fields.push(input.parse()?);
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
    }
    Ok(body)
}
