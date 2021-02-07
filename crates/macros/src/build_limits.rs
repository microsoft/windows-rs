use super::*;
use gen::{NamespaceTypes, TypeLimit, TypeLimits, TypeTree};
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;

pub struct BuildLimits(pub std::collections::BTreeSet<TypesDeclaration>);

impl BuildLimits {
    pub fn to_tokens_string(self) -> Result<String, proc_macro2::TokenStream> {
        let is_foundation = self.0.is_empty();

        let reader = winmd::TypeReader::get();

        let mut limits = TypeLimits::new(reader);

        let foundation_namespaces = &[
            "Windows.Foundation",
            "Windows.Foundation.Collections",
            "Windows.Foundation.Diagnostics",
            "Windows.Foundation.Numerics",
        ];

        if is_foundation {
            for namespace in foundation_namespaces {
                limits
                    .insert(NamespaceTypes {
                        namespace: &namespace,
                        limit: TypeLimit::All,
                    })
                    .unwrap();
            }
        }

        for limit in self.0 {
            let types = limit.types;
            let syntax = limit.syntax;
            limits.insert(types).map_err(|ns| {
                syn::Error::new_spanned(syntax, format!("'{}' is not a known namespace", ns))
                    .to_compile_error()
            })?;
        }

        let mut tree = TypeTree::from_limits(reader, &limits);

        if !is_foundation {
            for namespace in foundation_namespaces {
                tree.remove(namespace);
            }

            tree.reexport();
        }

        let ts = tree.gen().fold(squote::TokenStream::new(), |mut accum, n| {
            accum.combine(&n);
            accum
        });

        Ok(ts.into_string())
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
    let reader = winmd::TypeReader::get();
    fn recurse(
        reader: &'static winmd::TypeReader,
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
            syn::UseTree::Glob(g) => {
                let namespace = find_namespace(reader, &current.clone(), g.span())?;
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            syn::UseTree::Group(g) => {
                let namespace = find_namespace(reader, &current.clone(), g.span())?;

                let mut types = Vec::with_capacity(g.items.len());
                for tree in &g.items {
                    match tree {
                        syn::UseTree::Name(n) => {
                            types.push(n.ident.to_string());
                        }
                        syn::UseTree::Rename(_) => {
                            return Err(syn::Error::new(
                                tree.span(),
                                "Renaming syntax is not supported",
                            ))
                        }
                        _ => return Err(syn::Error::new(tree.span(), "Nested paths not allowed")),
                    }
                }
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(types),
                })
            }
            syn::UseTree::Name(n) => {
                let namespace = find_namespace(reader, &current.clone(), n.span())?;
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

fn find_namespace(
    reader: &'static winmd::TypeReader,
    namespace: &str,
    span: proc_macro2::Span,
) -> syn::parse::Result<&'static str> {
    let namespace = namespace_literal_to_rough_namespace(namespace);
    if let Some(namespace) = reader.find_lowercase_namespace(&namespace) {
        Ok(namespace)
    } else {
        Err(syn::Error::new(span, "Module not found"))
    }
}
