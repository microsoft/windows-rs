use super::*;

use syn::parse::*;
use syn::*;

// TODO: can we do this without requiring a type to "parse" while preserving compiler error reporting,
// since the parsing code below simplify updates the static TypeReader directly?
pub struct BuildMacro();

impl Parse for BuildMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        while !input.is_empty() {
            let tree: UseTree = input.parse()?;

            fn walk(tree: &UseTree, mut namespace: String) -> Result<()> {
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
                        walk(&*input.tree, namespace)?;
                    }
                    UseTree::Name(input) => {
                        let reader = TypeReader::get_mut();
                        let name = input.ident.to_string();

                        if !reader.import_type(&namespace, &name) {
                            return Err(Error::new_spanned(input, format!("`{}.{}` not found in metadata", render_namespace(&namespace), name)));
                        }
                    }
                    UseTree::Glob(input) => {
                        let reader = TypeReader::get_mut();

                        if !reader.import_namespace(&namespace) {
                            return Err(Error::new_spanned(input, format!("`{}` not found in metadata", render_namespace(&namespace))));
                        }
                    }
                    UseTree::Group(input) => {
                        for tree in &input.items {
                            walk(tree, namespace.clone())?;
                        }
                    }
                    UseTree::Rename(input) => {
                        return Err(Error::new_spanned(input, "Rename syntax not supported"));
                    }
                }

                Ok(())
            }

            walk(&tree, String::new())?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(BuildMacro())
    }
}
