use super::*;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;

pub struct BuildLimits(pub std::collections::BTreeSet<TypesDeclaration>);

impl BuildLimits {
    pub fn into_tokens_string(self) -> String {
        let reader = TypeReader::get();
        let mut limits = TypeLimits::new(reader);

        for limit in self.0 {
            limits.insert(limit.types);
        }

        let tree = TypeTree::from_limits(reader, &limits);

        let ts = tree.gen(&tree).fold(TokenStream::new(), |mut accum, n| {
            accum.combine(&n);
            accum
        });

        ts.into_string()
    }
}

pub struct TypesDeclaration {
    pub types: NamespaceTypes,
    pub syntax: syn::UseTree,
}

impl std::cmp::PartialOrd for TypesDeclaration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for TypesDeclaration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.types.cmp(&other.types)
    }
}

impl PartialEq for TypesDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.types == other.types
    }
}

impl Eq for TypesDeclaration {}

impl TryFrom<syn::UseTree> for TypesDeclaration {
    type Error = syn::Error;
    fn try_from(tree: syn::UseTree) -> Result<Self, Self::Error> {
        Ok(Self {
            types: use_tree_to_namespace_types(&tree)?,
            syntax: tree,
        })
    }
}

impl syn::parse::Parse for BuildLimits {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut limits = std::collections::BTreeSet::new();
        loop {
            if input.is_empty() {
                break;
            }

            let use_tree: syn::UseTree = input.parse()?;
            let limit: TypesDeclaration = use_tree.try_into()?;

            limits.insert(limit);

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self(limits))
    }
}

fn use_tree_to_namespace_types(use_tree: &syn::UseTree) -> syn::parse::Result<NamespaceTypes> {
    let reader = TypeReader::get();
    fn recurse(
        reader: &'static TypeReader,
        tree: &syn::UseTree,
        current: &mut String,
    ) -> syn::parse::Result<NamespaceTypes> {
        match tree {
            syn::UseTree::Path(p) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&p.ident.to_string());

                recurse(reader, &*p.tree, current)
            }
            syn::UseTree::Glob(_) => {
                let namespace = reader.resolve_namespace(current.trim_matches('"'));
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            syn::UseTree::Group(g) => {
                let namespace = reader.resolve_namespace(current.trim_matches('"'));

                let mut types = Vec::with_capacity(g.items.len());
                for tree in &g.items {
                    match tree {
                        syn::UseTree::Name(n) => {
                            types.push(n.ident.to_string());
                        }
                        _ => return Err(syn::Error::new(tree.span(), "Unsupported syntax")),
                    }
                }
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(types),
                })
            }
            syn::UseTree::Name(n) => {
                let namespace = reader.resolve_namespace(current.trim_matches('"'));
                let name = n.ident.to_string();
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(vec![name]),
                })
            }
            syn::UseTree::Rename(r) => Err(syn::Error::new(
                r.span(),
                "Renaming syntax is not supported",
            )),
        }
    }

    recurse(reader, use_tree, &mut String::new())
}
