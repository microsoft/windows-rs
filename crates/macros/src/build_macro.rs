use super::*;

use gen::TypeReader;
use std::collections::*;
use syn::parse::*;
use syn::*;

#[derive(Debug, Default)]
pub struct BuildMacro {
    // TODO: add exports
}

impl BuildMacro {
    // TODO: doesn't need to be member of BuildMacro
    pub fn into_tokens_string(&self) -> String {
        let reader = TypeReader::get();

        let ts = reader.gen().fold(TokenStream::new(), |mut accum, n| {
            accum.combine(&n);
            accum
        });

        ts.into_string()
    }
}

impl Parse for BuildMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut build = Self::default();

        while !input.is_empty() {
            let tree: UseTree = input.parse()?;

            fn walk(
                
                tree: &UseTree,
                mut namespace: String,
                build: &mut BuildMacro,
            ) -> Result<()> {

                fn render_namespace(namespace: &str) -> &str {
                    if namespace.is_empty() {
                        "(global namespace)"
                    } else {
                        namespace
                    }
                }

                match tree {
                    UseTree::Path(input) => {
                        if !namespace.is_empty() {
                            namespace.push('.');
                        }

                        namespace.push_str(&input.ident.to_string());
                        walk(&*input.tree, namespace, build)?;
                    }
                    UseTree::Name(input) => {
                        let reader = TypeReader::get_mut();
                        let name = input.ident.to_string();

                        if !reader.import_type(&namespace, &name) {
                            return Err(Error::new_spanned(
                                input,
                                format!(
                                    "`{}.{}` not found in metadata",
                                    render_namespace(&namespace),
                                    name
                                ),
                            ));
                        }
                    }
                    UseTree::Glob(input) => {
                        let reader = TypeReader::get_mut();

                        if !reader.import_namespace(&namespace) {
                            return Err(Error::new_spanned(
                                input,
                                format!("`{}` not found in metadata", render_namespace(&namespace)),
                            ));
                        }
                    }
                    UseTree::Group(input) => {
                        for tree in &input.items {
                            walk( tree, namespace.clone(), build)?;
                        }
                    }
                    UseTree::Rename(input) => {
                        return Err(Error::new_spanned(input, "Rename syntax not supported"));
                    }
                }

                Ok(())
            }

            walk( &tree, String::new(), &mut build)?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(build)
    }
}
