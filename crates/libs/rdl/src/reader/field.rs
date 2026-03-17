/// A field in a struct or union, which may itself contain an inline type definition.
#[derive(Debug, Clone)]
pub struct Field {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub ty: FieldType,
}

/// The type of a [`Field`].
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum FieldType {
    /// A plain type reference, e.g. `name: u32` or `name: SomeStruct`.
    Type(syn::Type),
    /// An inline struct definition, e.g. `name: struct { a: u8, b: u16 }`.
    Struct(Vec<Field>),
    /// An inline union definition, e.g. `name: union { a: u8, b: u16 }`.
    Union(Vec<Field>),
    /// A fixed-size array of an inline struct, e.g. `name: [struct { a: u8 }; 4]`.
    StructArray(Vec<Field>, usize),
    /// A fixed-size array of an inline union, e.g. `name: [union { a: u8 }; 4]`.
    UnionArray(Vec<Field>, usize),
}

impl syn::parse::Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        let ty = if input.peek(syn::Token![struct]) {
            input.parse::<syn::Token![struct]>()?;
            // Consume optional name (e.g. backward-compatible `struct Foo { ... }` syntax).
            let _: Option<syn::Ident> = input.parse()?;
            let content;
            syn::braced!(content in input);
            let fields = content
                .parse_terminated(Field::parse, syn::Token![,])?
                .into_iter()
                .collect();
            FieldType::Struct(fields)
        } else if input.peek(syn::Token![union]) {
            input.parse::<syn::Token![union]>()?;
            // Consume optional name.
            let _: Option<syn::Ident> = input.parse()?;
            let content;
            syn::braced!(content in input);
            let fields = content
                .parse_terminated(Field::parse, syn::Token![,])?
                .into_iter()
                .collect();
            FieldType::Union(fields)
        } else if input.peek(syn::token::Bracket) {
            // Fork to peek inside the brackets and check for an inline struct/union.
            let fork = input.fork();
            let bracket_peek;
            syn::bracketed!(bracket_peek in fork);

            if bracket_peek.peek(syn::Token![struct]) {
                let inner;
                syn::bracketed!(inner in input);
                inner.parse::<syn::Token![struct]>()?;
                let _: Option<syn::Ident> = inner.parse()?;
                let content;
                syn::braced!(content in inner);
                let fields = content
                    .parse_terminated(Field::parse, syn::Token![,])?
                    .into_iter()
                    .collect();
                inner.parse::<syn::Token![;]>()?;
                let len: usize = inner.parse::<syn::LitInt>()?.base10_parse()?;
                FieldType::StructArray(fields, len)
            } else if bracket_peek.peek(syn::Token![union]) {
                let inner;
                syn::bracketed!(inner in input);
                inner.parse::<syn::Token![union]>()?;
                let _: Option<syn::Ident> = inner.parse()?;
                let content;
                syn::braced!(content in inner);
                let fields = content
                    .parse_terminated(Field::parse, syn::Token![,])?
                    .into_iter()
                    .collect();
                inner.parse::<syn::Token![;]>()?;
                let len: usize = inner.parse::<syn::LitInt>()?.base10_parse()?;
                FieldType::UnionArray(fields, len)
            } else {
                FieldType::Type(input.parse()?)
            }
        } else {
            FieldType::Type(input.parse()?)
        };

        Ok(Field { attrs, name, ty })
    }
}
