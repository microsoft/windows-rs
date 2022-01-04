//use super::*;
use syn::parse::*;
use syn::Ident;
use syn::*;

// New traits-based implement macro that doesn't rely on metadata
// Also no support for overrides (Xaml) but developers can still implement overrides directly by implementning the necessary override interface
// Maybe gen up override traits to have a default implementation to make this simpler { Ok() }
// And don't make the overrides required constraints.
pub fn gen(_attribute: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    original_type
}

pub enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
}

pub struct UsePath2 {
    pub ident: Ident,
    pub colon2_token: Token![::],
    pub tree: Box<UseTree2>,
}

pub struct UseName2 {
    pub ident: Ident,
    pub generics: Vec<UseTree2>,
}

pub struct UseGroup2 {
    pub brace_token: token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, Token![,]>,
}

impl Parse for UseTree2 {
    fn parse(input: ParseStream) -> Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(Ident::parse_any)?;
            if input.peek(Token![::]) {
                Ok(UseTree2::Path(UsePath2 { ident, colon2_token: input.parse()?, tree: Box::new(input.parse()?) }))
            } else {
                let generics = if input.peek(Token![<]) {
                    input.parse::<Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse)?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}
