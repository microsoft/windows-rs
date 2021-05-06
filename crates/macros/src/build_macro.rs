use super::*;

use gen::TypeReader;
use std::collections::*;
use syn::parse::*;
use syn::*;

#[derive(Debug, Default)]
pub struct BuildMacro {
    imports: BTreeMap<&'static str, ImportLimit>,
    // TODO: add exports
}

impl BuildMacro {
    fn import(&mut self, namespace: &'static str, name: &'static str) {
        // Just skip the import if `All` is already requested.
        if let ImportLimit::Some(names) = self
            .imports
            .entry(namespace)
            .or_insert_with(|| ImportLimit::none())
        {
            names.insert(name);
        }
    }

    fn import_namespace(&mut self, namespace: &'static str) {
        // Just replace with `All`.
        self.imports.insert(namespace, ImportLimit::All);
    }

    pub fn into_tokens_string(&self) -> String {
        let reader = TypeReader::get();
        let tree = TypeTree::from_imports(reader, &self.imports);

        let ts = tree.gen(&tree).fold(TokenStream::new(), |mut accum, n| {
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
            let reader = TypeReader::get();
            let tree: UseTree = input.parse()?;

            fn walk(
                reader: &'static TypeReader,
                tree: &UseTree,
                namespace: &mut String,
                build: &mut BuildMacro,
            ) -> Result<()> {
                match tree {
                    UseTree::Path(input) => {
                        if !namespace.is_empty() {
                            namespace.push('.');
                        }

                        namespace.push_str(&input.ident.to_string());
                        walk(reader, &*input.tree, namespace, build)?;
                    }
                    UseTree::Name(input) => {
                        let name = input.ident.to_string();

                        if let Some((namespace, name)) = reader.get_type_name(namespace, &name) {
                            build.import(namespace, name);
                        } else {
                            return Err(Error::new_spanned(
                                input,
                                format!("`{}.{}` not found in metadata", namespace, name),
                            ));
                        }
                    }
                    UseTree::Glob(input) => {
                        if let Some(namespace) = reader.get_namespace(namespace) {
                            build.import_namespace(namespace);
                        } else {
                            return Err(Error::new_spanned(
                                input,
                                format!("`{}` not found in metadata", namespace),
                            ));
                        }
                    }
                    UseTree::Group(input) => {
                        for tree in &input.items {
                            walk(reader, tree, namespace, build)?;
                        }
                    }
                    UseTree::Rename(input) => {
                        return Err(Error::new_spanned(input, "Rename syntax not supported"));
                    }
                }

                Ok(())
            }

            walk(reader, &tree, &mut String::new(), &mut build)?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(build)
    }
}
