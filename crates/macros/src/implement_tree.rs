use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{braced, Generics, Ident, Token};

pub enum ImplementTree {
    Path(ImplementPath),
    Name(ImplementName),
    Group(ImplementGroup),
}

pub struct ImplementPath {
    pub ident: Ident,
    pub colon2_token: Token![::],
    pub tree: Box<ImplementTree>,
}

pub struct ImplementName {
    pub ident: Ident,
    pub generics: Generics,
}

pub struct ImplementGroup {
    pub brace_token: Brace,
    pub items: Punctuated<ImplementTree, Token![,]>,
}

impl Parse for ImplementTree {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident = input.call(Ident::parse_any)?;
            if input.peek(Token![::]) {
                Ok(ImplementTree::Path(ImplementPath {
                    ident,
                    colon2_token: input.parse()?,
                    tree: Box::new(input.parse()?),
                }))
            } else {
                Ok(ImplementTree::Name(ImplementName {
                    ident,
                    generics: input.parse()?,
                }))
            }
        } else if lookahead.peek(Brace) {
            let content;
            Ok(ImplementTree::Group(ImplementGroup {
                brace_token: braced!(content in input),
                items: content.parse_terminated(ImplementTree::parse)?,
            }))
        } else {
            Err(lookahead.error())
        }
    }
}
