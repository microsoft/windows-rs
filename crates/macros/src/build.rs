use crate::namespace_literal_to_rough_namespace;
use proc_macro2::Span;
use rayon::iter::ParallelIterator;
use syn::parse::{self, Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, UseTree};

use winrt_deps::cargo;
use winrt_gen::{
    dependencies, NamespaceTypes, TypeLimit, TypeLimits, TypeReader, TypeStage, WinmdFile,
};

use std::convert::{TryFrom, TryInto};
use std::{collections::BTreeSet, path::PathBuf};

/// A parsed `build!` macro
pub struct BuildMacro {
    foundation: bool,
    dependencies: Dependencies,
    types: TypesDeclarations,
}

impl BuildMacro {
    pub fn winmd_paths(&self) -> &BTreeSet<PathBuf> {
        &self.dependencies.0
    }

    pub fn to_tokens_string(self) -> Result<String, proc_macro2::TokenStream> {
        let dependencies = self.dependencies.0.iter().map(WinmdFile::new).collect();

        let reader = &TypeReader::new(dependencies);
        let mut limits = TypeLimits::new(reader);

        let foundation_namespaces = &[
            "Windows.Foundation",
            "Windows.Foundation.Collections",
            "Windows.Foundation.Diagnostics",
            "Windows.Foundation.Numerics",
        ];

        if self.foundation {
            for namespace in foundation_namespaces {
                limits
                    .insert(NamespaceTypes {
                        namespace: namespace.to_string(),
                        limit: TypeLimit::All,
                    })
                    .unwrap();
            }
        }

        for limit in self.types.0 {
            let types = limit.types;
            let syntax = limit.syntax;
            if let Err(e) = limits.insert(types).map_err(|ns| {
                syn::Error::new_spanned(syntax, format!("'{}' is not a known namespace", ns))
            }) {
                return Err(e.to_compile_error());
            };
        }

        let stage = TypeStage::from_limits(reader, &limits);
        let mut tree = stage.into_tree();

        if !self.foundation {
            for namespace in foundation_namespaces {
                tree.remove(namespace);
            }

            tree.reexport();
        }

        let ts = tree
            .to_tokens()
            .reduce(squote::TokenStream::new, |mut accum, n| {
                accum.combine(&n);
                accum
            });

        Ok(ts.into_string())
    }
}

impl Parse for BuildMacro {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let dependencies = Dependencies::parse()
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("{}", e)))?;
        let foundation = input.parse::<keywords::foundation>().is_ok();
        let _ = input.parse::<keywords::types>()?;
        let types: TypesDeclarations = input.parse()?;

        Ok(BuildMacro {
            foundation,
            dependencies,
            types,
        })
    }
}

/// keywords used in the `build!` macro
mod keywords {
    syn::custom_keyword!(nuget);
    syn::custom_keyword!(dependencies);
    syn::custom_keyword!(types);
    syn::custom_keyword!(foundation);
}

/// A parsed `dependencies` section of the `build!` macro
#[derive(Debug)]
struct Dependencies(BTreeSet<PathBuf>);

impl Dependencies {
    fn parse() -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let mut dependencies = BTreeSet::new();
        let deps = cargo::package_manifest()?.get_dependencies()?;
        for dep in deps {
            let nuget_path = std::fs::read_dir(dependencies::nuget_root())?;
            let name = dep.name();
            let mut dependency_path_iter = nuget_path
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .filter(|entry| {
                    if let Some(entry) = entry.file_name().to_str().map(str::to_owned) {
                        entry.starts_with(&name)
                    } else {
                        false
                    }
                })
                .map(|entry| entry.path());

            let path = dependency_path_iter
                .next()
                .ok_or_else(|| format!("No directory for dependency '{}'", dep.name()))?;
            // Check for multiple versions
            if dependency_path_iter.next().is_some() {
                return Err(format!("multiple nuget package versions found for '{}'", name).into());
            }

            dependencies::expand_paths(path, &mut dependencies, true)
                .map_err(|e| format!("could not read dependency: {}", e))?;
        }
        Ok(Dependencies(dependencies))
    }
}

/// A parsed `types` section of the `build!` macro
struct TypesDeclarations(BTreeSet<TypesDeclaration>);

struct TypesDeclaration {
    types: NamespaceTypes,
    syntax: syn::UseTree,
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
    fn try_from(tree: UseTree) -> Result<Self, Self::Error> {
        Ok(Self {
            types: use_tree_to_namespace_types(&tree)?,
            syntax: tree,
        })
    }
}

impl Parse for TypesDeclarations {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut limits = BTreeSet::new();
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

fn use_tree_to_namespace_types(use_tree: &syn::UseTree) -> parse::Result<NamespaceTypes> {
    fn recurse(tree: &UseTree, current: &mut String) -> parse::Result<NamespaceTypes> {
        fn check_for_module_instead_of_type(name: &str, span: Span) -> parse::Result<()> {
            let error = Err(Error::new(
                span,
                "Expected `*` or type name, but found what appears to be a module",
            ));
            if name.to_lowercase() == name {
                return error;
            }
            Ok(())
        }

        match tree {
            UseTree::Path(p) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&p.ident.to_string());

                recurse(&*p.tree, current)
            }
            UseTree::Glob(_) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            UseTree::Group(g) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());

                let mut types = Vec::with_capacity(g.items.len());
                for tree in &g.items {
                    match tree {
                        UseTree::Name(n) => {
                            let name = n.ident.to_string();
                            check_for_module_instead_of_type(&name, n.span())?;
                            types.push(name);
                        }
                        UseTree::Rename(_) => {
                            return Err(Error::new(tree.span(), "Renaming syntax is not supported"))
                        }
                        _ => return Err(Error::new(tree.span(), "Nested paths not allowed")),
                    }
                }
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(types),
                })
            }
            UseTree::Name(n) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                let name = n.ident.to_string();
                check_for_module_instead_of_type(&name, n.span())?;
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(vec![name]),
                })
            }
            UseTree::Rename(r) => Err(Error::new(r.span(), "Renaming syntax is not supported")),
        }
    }

    recurse(use_tree, &mut String::new())
}
