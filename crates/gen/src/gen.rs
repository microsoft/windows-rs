use super::*;

pub struct Gen {
    pub relative: &'static str,
    pub features: bool,
    // TODO features should be a feature field of string type (not bool) to hold the root feature to check against
}

impl Gen {
    pub fn absolute() -> Self {
        Gen{ relative : "", features: false }
    }

    pub fn relative(namespace: &'static str) -> Self {
        Gen{ relative : namespace, features: false }
    }

    pub fn namespace(&self, namespace: &str) -> TokenStream {
        if self.relative.is_empty() {
                let mut tokens = TokenStream::with_capacity();

                for namespace in namespace.split('.') {
                    tokens.push_str(namespace);
                    tokens.push_str("::");
                }

                tokens
            } else {
                if namespace == self.relative {
                    return TokenStream::new();
                }

                let mut relative = self.relative.split('.').peekable();
                let mut namespace = namespace.split('.').peekable();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    namespace.next();
                }

                let mut tokens = TokenStream::with_capacity();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(
            Gen::absolute().namespace("Windows.Foundation").as_str(),
            "Windows::Foundation::"
        );

        assert_eq!(
            Gen::relative("Windows")
                .namespace("Windows.Foundation")
                .as_str(),
            "Foundation::"
        );

        assert_eq!(
            Gen::relative("Windows.Foundation")
                .namespace("Windows.Foundation")
                .as_str(),
            ""
        );

        assert_eq!(
            Gen::relative("Windows.Foundation.Collections")
                .namespace("Windows.Foundation")
                .as_str(),
            "super::"
        );
    }
}
