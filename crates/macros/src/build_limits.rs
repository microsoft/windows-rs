use crate::*;
use rayon::iter::ParallelIterator;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use winrt_gen::{NamespaceTypes, TypeLimit, TypeLimits, TypeTree};

pub struct BuildLimits(pub std::collections::BTreeSet<TypesDeclaration>);

impl BuildLimits {
    pub fn to_tokens_string(self) -> Result<String, proc_macro2::TokenStream> {
        let is_foundation = self.0.is_empty();

        let reader = winmd::TypeReader::from_build();

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
                        namespace: namespace.to_string(),
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

        let ts = tree.gen().reduce(squote::TokenStream::new, |mut accum, n| {
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
        }
        Ok(Self(limits))
    }
}

fn use_tree_to_namespace_types(use_tree: &syn::UseTree) -> syn::parse::Result<NamespaceTypes> {
    fn recurse(tree: &syn::UseTree, current: &mut String) -> syn::parse::Result<NamespaceTypes> {
        fn check_for_module_instead_of_type(
            name: &str,
            span: proc_macro2::Span,
        ) -> syn::parse::Result<()> {
            let error = Err(syn::Error::new(
                span,
                "Expected `*` or type name, but found what appears to be a module",
            ));
            if name.to_lowercase() == name {
                return error;
            }
            Ok(())
        }

        match tree {
            syn::UseTree::Path(p) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&p.ident.to_string());

                recurse(&*p.tree, current)
            }
            syn::UseTree::Glob(_) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            syn::UseTree::Group(g) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());

                let mut types = Vec::with_capacity(g.items.len());
                for tree in &g.items {
                    match tree {
                        syn::UseTree::Name(n) => {
                            let name = n.ident.to_string();
                            check_for_module_instead_of_type(&name, n.span())?;
                            types.push(name);
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
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                let name = n.ident.to_string();
                check_for_module_instead_of_type(&name, n.span())?;
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

    recurse(use_tree, &mut String::new())
}
