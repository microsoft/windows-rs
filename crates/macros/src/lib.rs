extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::parse::{self, Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, Ident, Token, UseTree};

use winmd::{dependencies, NamespaceLimit, TypeLimit, TypeLimits, TypeReader, TypeStage};

use std::collections::BTreeSet;
use std::path::PathBuf;

/// A macro for generating WinRT modules into the current module
#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let import = parse_macro_input!(stream as ImportMacro);

    let dependencies = import
        .dependencies
        .0
        .into_iter()
        .map(|p| winmd::WinmdFile::new(p))
        .collect();
    let reader = &TypeReader::new(dependencies);

    let mut limits = TypeLimits::new(reader);

    for limit in import.namespace_limits.0 {
        limits.insert(limit);
    }

    let stage = TypeStage::from_limits(reader, &limits);
    let tree = stage.into_tree();
    let stream = tree.to_tokens();
    stream.into()
}

#[derive(Debug)]
struct ImportMacro {
    dependencies: Dependencies,
    namespace_limits: NamespaceLimits,
}

impl Parse for ImportMacro {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let category: ImportCategory = input.parse()?;
        let (dependencies, namespace_limits) = match category {
            ImportCategory::Namespace => {
                let namespace_limits: NamespaceLimits = input.parse()?;
                let category: ImportCategory = input.parse()?;
                // TODO: don't panic here
                assert!(matches!(category, ImportCategory::Dependency));
                let dependencies: Dependencies = input.parse()?;
                (dependencies, namespace_limits)
            }
            ImportCategory::Dependency => {
                let dependencies: Dependencies = input.parse()?;
                let category: ImportCategory = input.parse()?;
                // TODO: don't panic here
                assert!(matches!(category, ImportCategory::Namespace));
                let namespace_limits: NamespaceLimits = input.parse()?;
                (dependencies, namespace_limits)
            }
        };

        Ok(ImportMacro {
            dependencies,
            namespace_limits,
        })
    }
}

#[derive(PartialEq, Debug)]
enum ImportCategory {
    Dependency,
    Namespace,
}

impl Parse for ImportCategory {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let name: Ident = input.parse()?;
        let result = match name.to_string().as_str() {
            "dependencies" => Ok(ImportCategory::Dependency),
            "modules" => Ok(ImportCategory::Namespace),
            _ => {
                return Err(Error::new(
                    name.span(),
                    "expected `dependency` or `modules`",
                ))
            }
        };
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![:]) {
            let _ = input.parse::<Token![:]>()?;
        }

        result
    }
}

#[derive(Debug)]
struct Dependencies(BTreeSet<PathBuf>);
mod keywords {
    syn::custom_keyword!(os);
    syn::custom_keyword!(nuget);
}

impl Parse for Dependencies {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut dependencies = BTreeSet::new();
        loop {
            let is_os = input.peek(keywords::os);
            let is_nuget = input.peek(keywords::os);

            if is_os {
                let _ = input.parse::<keywords::os>();
                let path = winmd::dependencies::system_metadata_root();

                dependencies::expand_paths(path, &mut dependencies, false);
            } else if is_nuget {
                let _ = input.parse::<keywords::nuget>();
                input.parse::<Token![:]>()?;
                let mut package = vec![];
                loop {
                    let ident: Ident = input.parse()?;
                    package.push(ident);
                    let peek = input.peek(Token![,]);
                    if peek {
                        let _: Token![,] = input.parse()?;
                    } else {
                        break;
                    }
                }

                let name = package
                    .iter()
                    .map(|ident| ident.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                let mut path = winmd::dependencies::nuget_root();
                path.push(name);

                dependencies::expand_paths(path, &mut dependencies, true);
            } else {
                break;
            }
        }
        Ok(Dependencies(dependencies))
    }
}

#[derive(Debug)]
struct NamespaceLimits(BTreeSet<NamespaceLimit>);

impl Parse for NamespaceLimits {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut limits = BTreeSet::<NamespaceLimit>::new();
        loop {
            if input.is_empty() {
                break;
            }

            let use_tree: syn::UseTree = input.parse()?;

            limits.insert(use_tree_to_namespace_limit(use_tree)?);
        }
        Ok(NamespaceLimits(limits))
    }
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

fn use_tree_to_namespace_limit(use_tree: syn::UseTree) -> parse::Result<NamespaceLimit> {
    fn recurse(tree: UseTree, current: &mut String) -> parse::Result<NamespaceLimit> {
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

                recurse(*p.tree, current)
            }
            UseTree::Glob(_) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                Ok(NamespaceLimit {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            UseTree::Group(g) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());

                let mut types = Vec::with_capacity(g.items.len());
                for tree in g.items {
                    match &tree {
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
                Ok(NamespaceLimit {
                    namespace,
                    limit: TypeLimit::Some(types),
                })
            }
            UseTree::Name(n) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                let name = n.ident.to_string();
                check_for_module_instead_of_type(&name, n.span())?;
                Ok(NamespaceLimit {
                    namespace,
                    limit: TypeLimit::Some(vec![name]),
                })
            }
            UseTree::Rename(r) => {
                return Err(Error::new(r.span(), "Renaming syntax is not supported"))
            }
        }
    }

    recurse(use_tree, &mut String::new())
}
