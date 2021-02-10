use super::*;
use std::iter::FromIterator;

pub enum Gen {
    Absolute,
    Relative(&'static str),
}

impl Gen {
    pub(crate) fn windows(&self) -> TokenStream {
        match self {
            Self::Absolute => quote! { windows:: },
            Self::Relative(_) => quote! { ::windows:: },
        }
    }

    pub fn namespace(&self, namespace: &str) -> TokenStream {
        match self {
            Self::Absolute => {
                let mut tokens = TokenStream::new();

                for namespace in namespace.split('.') {
                    let namespace = to_ident(&crate::to_snake(namespace));

                    tokens.combine(&quote! { #namespace:: });
                }

                tokens
            }
            Self::Relative(relative) => {
                if namespace == *relative {
                    return TokenStream::new();
                }

                let mut tokens = Vec::new();
                let mut relative = relative.split('.').peekable();
                let mut namespace = namespace.split('.').peekable();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }
                    namespace.next();
                }

                let count = relative.count();

                if count > 0 {
                    tokens.resize(tokens.len() + count, quote! { super:: });
                }

                tokens.extend(namespace.map(|namespace| {
                    let namespace = to_ident(&crate::to_snake(namespace));
                    quote! { #namespace:: }
                }));

                TokenStream::from_iter(tokens)
            }
        }
    }
}
