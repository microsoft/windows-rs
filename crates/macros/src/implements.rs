use syn::{
    parse::{Error, Parse, ParseStream, Result},
    spanned::Spanned,
    Token, UseTree,
};

use crate::namespace_literal_to_rough_namespace;
use winrt_gen::{Type, TypeReader};

pub struct Implements(Vec<Type>);

impl Parse for Implements {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("{:?}", input);
        let mut types = Vec::new();

        let reader = TypeReader::from_build();

        loop {
            use_tree_to_types(reader, &input.parse::<UseTree>()?, &mut types);

            if input.parse::<Token!(,)>().is_err() {
                break;
            }
        }

        Ok(Self(types))
    }
}

pub struct ImplementsClass {}

impl Parse for ImplementsClass {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {})
    }
}

fn use_tree_to_types(reader: &TypeReader, tree: &UseTree, types: &mut Vec<Type>) -> Result<()> {
    fn recurse(
        reader: &TypeReader,
        tree: &UseTree,
        types: &mut Vec<Type>,
        current: &mut String,
    ) -> Result<()> {
        match tree {
            UseTree::Path(path) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&path.ident.to_string());
                recurse(reader, &*path.tree, types, current)?;
            }
            UseTree::Group(group) => {
                let prev = current.clone();

                for tree in &group.items {
                    recurse(reader, tree, types, current)?;
                    *current = prev.clone();
                }
            }
            UseTree::Name(name) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());

                let namespace_types = match reader
                    .types
                    .iter()
                    .find(|(name, _)| name.to_lowercase() == namespace)
                {
                    Some((_, types)) => types,
                    None => {
                        return Err(Error::new(
                            name.span(),
                            "Metadata not found for type namespace",
                        ))
                    }
                };

                let def = match namespace_types.get(&name.ident.to_string()) {
                    Some(def) => def,
                    Non => return Err(Error::new(name.span(), "Metadata not found for type name")),
                };

                types.push(Type::from_type_def(reader, *def));

                // TODO
                // If type is a class, add any required interfaces.
                // If type is an interface, add any required interfaces.
                // If any other kind of type, return an error.
                // There may also be at most one class being implemented
                // but any number of interfaces.
                //

                println!("implement: {}.{}", def.name(reader).0, def.name(reader).1);
            }
            UseTree::Glob(glob) => {
                return Err(Error::new(glob.span(), "Glob syntax is not supported"))
            }
            UseTree::Rename(rename) => {
                return Err(Error::new(rename.span(), "Rename syntax is not supported"))
            }
        }

        Ok(())
    }

    recurse(reader, tree, types, &mut String::new())
}
