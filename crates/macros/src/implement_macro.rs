use gen::{TypeKind, TypeReader};
use std::collections::*;
use syn::parse::*;
use syn::*;

custom_keyword!(extend);

#[derive(Debug, Default)]
pub struct ImplementMacro {
    pub extend: Option<(&'static str, &'static str)>,
    pub overrides: BTreeSet<&'static str>,
    pub implement: BTreeSet<(&'static str, &'static str)>,
}

impl ImplementMacro {
    fn parse_implement(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        self.walk_implement(reader, &cursor.parse()?, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(
        &mut self,
        reader: &'static TypeReader,
        tree: &UseTree,
        namespace: &mut String,
    ) -> Result<()> {
        match tree {
            UseTree::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(reader, &*input.tree, namespace)?;
            }
            UseTree::Name(input) => {
                let name = input.ident.to_string();

                if let Some((namespace, name)) = reader.get_type_name(namespace, &name) {
                    match reader.resolve_type_def(namespace, name).kind() {
                        TypeKind::Class | TypeKind::Interface => {
                            self.implement.insert((namespace, name));
                        }
                        _ => {
                            return Err(Error::new_spanned(
                                input,
                                format!("`{}.{}` not a class or interface", namespace, name),
                            ));
                        }
                    }
                } else {
                    return Err(Error::new_spanned(
                        input,
                        format!("`{}.{}` not found in metadata", namespace, name),
                    ));
                }
            }
            UseTree::Glob(input) => {
                return Err(Error::new_spanned(input, "Glob syntax not supported"));
            }
            UseTree::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(reader, tree, namespace)?;
                }
            }
            UseTree::Rename(input) => {
                return Err(Error::new_spanned(input, "Rename syntax not supported"));
            }
        }

        Ok(())
    }

    fn parse_override(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        // Any number of methods may be overridden but only if a class is being overridden.
        if let Some((namespace, name)) = self.extend {
            if cursor.parse::<Token![override]>().is_ok() {
                let methods = reader
                    .resolve_type_def(namespace, name)
                    .overridable_methods();

                while let Ok(input) = cursor.parse::<Ident>() {
                    let name = input.to_string();

                    if let Some(name) = methods.get(name.as_str()) {
                        self.overrides.insert(name);
                    } else {
                        return Err(Error::new_spanned(
                            input,
                            format!("`{}` not an overridable method", name),
                        ));
                    }
                }

                if !cursor.is_empty() {
                    cursor.parse::<Token![,]>()?;
                }
            }
        }

        Ok(())
    }

    fn parse_extend(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        // Only one class may be extended
        if self.extend.is_none() && cursor.parse::<extend>().is_ok() {
            self.walk_extend(reader, &cursor.parse()?, &mut String::new())?;

            if !cursor.is_empty() {
                cursor.parse::<Token![,]>()?;
            }
        }

        Ok(())
    }

    fn walk_extend(
        &mut self,
        reader: &'static TypeReader,
        tree: &UseTree,
        namespace: &mut String,
    ) -> Result<()> {
        match tree {
            UseTree::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_extend(reader, &*input.tree, namespace)?;
            }
            UseTree::Name(input) => {
                let name = input.ident.to_string();

                if let Some((namespace, name)) = reader.get_type_name(namespace, &name) {
                    if reader
                        .resolve_type_def(namespace, name)
                        .is_public_composable()
                    {
                        self.extend.replace((namespace, name));
                    } else {
                        return Err(Error::new_spanned(
                            input,
                            format!("`{}.{}` not extendable", namespace, name),
                        ));
                    }
                } else {
                    return Err(Error::new_spanned(
                        input,
                        format!("`{}.{}` not found in metadata", namespace, name),
                    ));
                }
            }
            _ => {
                return Err(Error::new_spanned(tree, "Syntax not supported"));
            }
        }

        Ok(())
    }
}

impl Parse for ImplementMacro {
    fn parse(cursor: ParseStream) -> Result<Self> {
        let mut input = Self::default();
        let reader = TypeReader::get();

        while !cursor.is_empty() {
            input.parse_extend(reader, cursor)?;
            input.parse_override(reader, cursor)?;
            input.parse_implement(reader, cursor)?;
        }

        Ok(input)
    }
}
