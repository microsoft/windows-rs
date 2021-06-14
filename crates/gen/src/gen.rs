use super::*;

pub struct Gen<'a> {
    relation: GenRelation,
    tree: &'a TypeTree,
}

pub enum GenRelation {
    Absolute,
    Relative(&'static str),
}

impl<'a> Gen<'a> {
    pub fn relative(namespace: &'static str, tree: &'a TypeTree) -> Self {
        Self {
            relation: GenRelation::Relative(namespace),
            tree,
        }
    }

    pub fn absolute(tree: &'a TypeTree) -> Self {
        Self {
            relation: GenRelation::Absolute,
            tree,
        }
    }

    // TODO: This method is very slow - remove in favor of dependency tracking retaining all methods #592
    pub fn include_method(&self, signature: &MethodSignature) -> bool {
        if let GenRelation::Absolute = self.relation {
            return true;
        }

        self.tree.include_method(signature)
    }

    pub fn namespace(&self, namespace: &str) -> TokenStream {
        match self.relation {
            GenRelation::Absolute => {
                let mut tokens = TokenStream::new();

                for namespace in namespace.split('.') {
                    let namespace = to_ident(namespace);

                    tokens.combine(&quote! { #namespace:: });
                }

                tokens
            }
            GenRelation::Relative(relative) => {
                if namespace == relative {
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
            t.gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "Windows :: Foundation :: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative("Windows", &TypeTree::from_namespace("")))
                .as_str(),
            "Foundation:: IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative(
                "Windows.Foundation",
                &TypeTree::from_namespace("")
            ))
            .as_str(),
            "IStringable"
        );

        assert_eq!(
            t.gen_name(&Gen::relative(
                "Windows.Foundation.Collections",
                &TypeTree::from_namespace("")
            ))
            .as_str(),
            "super:: IStringable"
        );
    }
}
