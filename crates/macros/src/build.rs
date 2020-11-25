use crate::*;
use rayon::iter::ParallelIterator;
use syn::parse::{self, Parse, ParseStream};
use winrt_deps::cargo;
use winrt_gen::{NamespaceTypes, TypeLimit, TypeLimits, TypeTree};

/// A parsed `build!` macro
pub struct BuildMacro {
    types: BuildLimits,
}

impl BuildMacro {
    pub fn to_tokens_string(self) -> Result<String, proc_macro2::TokenStream> {
        let foundation = cargo::package_manifest().unwrap().package_name() == "winrt";

        let reader = &windows::reader();

        let mut limits = TypeLimits::new(reader);

        let foundation_namespaces = &[
            "Windows.Foundation",
            "Windows.Foundation.Collections",
            "Windows.Foundation.Diagnostics",
            "Windows.Foundation.Numerics",
        ];

        if foundation {
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

        let mut tree = TypeTree::from_limits(reader, &limits);

        if !foundation {
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

impl Parse for BuildMacro {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let types: BuildLimits = input.parse()?;

        Ok(BuildMacro {
            types,
        })
    }
}
