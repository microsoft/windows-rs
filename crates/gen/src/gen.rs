use super::*;

pub enum Gen {
    Absolute,
    Relative(&'static str),
}

impl Gen {
    pub fn namespace(&self, namespace: &str) -> TokenStream {
        match self {
            Self::Absolute => {
                let mut tokens = TokenStream::new();

                for namespace in namespace.split('.') {
                    let namespace = to_ident(namespace);

                    tokens.combine(&quote! { #namespace:: });
                }

                tokens
            }
            Self::Relative(relative) => {
                if namespace == *relative {
                    return TokenStream::new();
                }

                let mut relative = relative.split('.').peekable();
                let mut namespace = namespace.split('.').peekable();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }
                    namespace.next();
                }

                let mut tokens = TokenStream::new();

                for _ in 0..relative.count() {
                    tokens.push_str("super::");
                }

                for namespace in namespace {
                    tokens.push_str(namespace);
                    tokens.push_str("::");
                }

                tokens
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(
            Gen::Absolute.namespace("Windows.Foundation").as_str(),
            "Windows :: Foundation ::"
        );

        assert_eq!(
            Gen::Relative("Windows")
                .namespace("Windows.Foundation")
                .as_str(),
            "Foundation::"
        );

        assert_eq!(
            Gen::Relative("Windows.Foundation")
                .namespace("Windows.Foundation")
                .as_str(),
            ""
        );

        assert_eq!(
            Gen::Relative("Windows.Foundation.Collections")
                .namespace("Windows.Foundation")
                .as_str(),
            "super::"
        );
    }
}
