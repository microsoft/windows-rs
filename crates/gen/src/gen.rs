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
        let reader = TypeReader::get();
        let t = reader.resolve_type("Windows.Foundation", "IStringable");

        assert_eq!(
            t.gen_name(&Gen::Absolute).as_str(),
            "Windows :: Foundation :: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::Relative("Windows")).as_str(),
            "Foundation:: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::Relative("Windows.Foundation")).as_str(),
            "IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::Relative("Windows.Foundation.Collections"))
                .as_str(),
            "super:: IStringable"
        );
    }
}
