/// A field in a struct or union.
#[derive(Debug, Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: FieldType,
}

/// The type of a field: either a normal named type, or an inline anonymous
/// nested struct/union declared directly at the field's type position.
///
/// The nested form is the faithful representation of a C anonymous aggregate
/// member, e.g. `Anonymous: struct { x: u32, y: u32 }`.
#[derive(Debug, Clone)]
pub enum FieldType {
    Type(Box<syn::Type>),
    Nested(Box<NestedRecord>),
}

/// An inline anonymous nested struct or union. It may carry its own
/// `#[packed(N)]` / `#[align(N)]` / `#[arch(...)]` attributes, written between
/// the `:` and the `struct`/`union` keyword.
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
        let ty = if input.peek(syn::Token![struct]) || input.peek(syn::Token![union]) {
            FieldType::Nested(Box::new(NestedRecord::parse(input, inner_attrs)?))
        } else if inner_attrs.is_empty() {
            FieldType::Type(Box::new(input.parse()?))
        } else {
            return Err(input
                .error("attributes are only allowed on an inline nested struct/union field type"));
        };

        Ok(Self { attrs, name, ty })
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
