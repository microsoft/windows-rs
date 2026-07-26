#[derive(Debug, Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: FieldType,
    pub bitfields: Vec<BitfieldMember>,
}

#[derive(Debug, Clone)]
pub struct BitfieldMember {
    pub name: Option<syn::Ident>,
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
    pub is_union: bool,
    pub fields: Vec<Field>,
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
                FieldType::Nested(Box::new(NestedRecord::parse(input, inner_attrs)?)),
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
        } else {
            Some(input.parse::<syn::Ident>()?)
        };
        input.parse::<syn::Token![:]>()?;
        let width: syn::LitInt = input.parse()?;
        Ok(Self {
            name,
            width: width.base10_parse()?,
        })
    }
}

impl NestedRecord {
    fn parse(input: syn::parse::ParseStream, attrs: Vec<syn::Attribute>) -> syn::Result<Self> {
        let is_union = input.peek(syn::Token![union]);
        if is_union {
            input.parse::<syn::Token![union]>()?;
        } else {
            input.parse::<syn::Token![struct]>()?;
        }

        let content;
        syn::braced!(content in input);
        let fields = content
            .parse_terminated(<Field as syn::parse::Parse>::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            is_union,
            fields,
        })
    }
}
