use super::*;

#[derive(Debug)]
pub struct Method {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
    pub return_attrs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;

        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let (inputs, variadic) = parse_fn_inputs(&content)?;

        let (output, return_attrs) = parse_return_type_with_attrs(input)?;

        input.parse::<syn::Token![;]>()?;

        let sig = syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        };

        Ok(Self {
            attrs,
            sig,
            return_attrs,
        })
    }
}
