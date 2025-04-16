pub struct ItemStruct {
    pub attrs: Vec<syn::Attribute>,
    pub struct_token: syn::Token![struct],
    pub ident: syn::Ident,
    pub brace_token: syn::token::Brace,
    pub fields: syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
}

impl syn::parse::Parse for ItemStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let struct_token = input.parse::<syn::Token![struct]>()?;
        let ident = input.parse::<syn::Ident>()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let fields = content.parse_terminated(syn::Field::parse_named, syn::Token![,])?;

        Ok(Self {
            attrs,
            struct_token,
            ident,
            brace_token,
            fields,
        })
    }
}
