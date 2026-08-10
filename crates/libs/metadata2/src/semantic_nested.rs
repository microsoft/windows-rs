use super::*;

impl Database {
    /// Iterates direct nested-to-enclosing type relationships.
    pub fn nested_types(&self) -> impl Iterator<Item = (TypeDefinition<'_>, TypeDefinition<'_>)> {
        self.images()
            .iter()
            .enumerate()
            .flat_map(move |(file, image)| {
                let file = FileId::new(file);
                image
                    .rows::<tables::NestedClass>()
                    .map(move |relationship| {
                        let relationship = image.view(relationship).unwrap();
                        let nested = relationship.index::<tables::TypeDef>(0).unwrap().unwrap();
                        let enclosing = relationship.index::<tables::TypeDef>(1).unwrap().unwrap();
                        (
                            TypeDefinition {
                                database: self,
                                entity: Entity::new(file, nested),
                            },
                            TypeDefinition {
                                database: self,
                                entity: Entity::new(file, enclosing),
                            },
                        )
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_relationships_match_existing_reader() {
        let database = Database::new([Image::new(windows_default::WIN32).unwrap()]).unwrap();
        let mut actual: Vec<_> = database
            .nested_types()
            .map(|(nested, enclosing)| {
                (
                    nested.name().unwrap().to_string(),
                    enclosing.name().unwrap().to_string(),
                )
            })
            .collect();

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
        ]);
        let mut expected = Vec::new();
        fn collect(
            index: &windows_metadata::reader::Index,
            enclosing: windows_metadata::reader::TypeDef<'_>,
            result: &mut Vec<(String, String)>,
        ) {
            for nested in index.nested(enclosing) {
                result.push((nested.name().to_string(), enclosing.name().to_string()));
                collect(index, nested, result);
            }
        }
        for (_, _, enclosing) in old.iter() {
            collect(&old, enclosing, &mut expected);
        }

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
