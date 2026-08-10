use super::*;

impl Database {
    /// Iterates type-to-interface implementation relationships.
    pub fn interface_implementations(
        &self,
    ) -> impl Iterator<Item = Result<(TypeDefinition<'_>, TypeIdentity), Error>> {
        self.images()
            .iter()
            .enumerate()
            .flat_map(move |(file, image)| {
                let file = FileId::new(file);
                image
                    .rows::<tables::InterfaceImpl>()
                    .map(move |relationship| {
                        let relationship = image.view(relationship).unwrap();
                        let definition = relationship
                            .index::<tables::TypeDef>(0)?
                            .ok_or_else(|| Error::invalid_metadata("missing interface owner"))?;
                        let interface = relationship.coded(1)?.ok_or_else(|| {
                            Error::invalid_metadata("missing implemented interface")
                        })?;
                        Ok((
                            TypeDefinition {
                                database: self,
                                entity: Entity::new(file, definition),
                            },
                            TypeIdentity {
                                file,
                                ty: interface,
                            },
                        ))
                    })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interface_relationships_are_valid() {
        let database = Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        let relationships = database
            .interface_implementations()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(!relationships.is_empty());
        assert!(relationships.iter().all(|(definition, interface)| {
            !definition.name().unwrap().is_empty()
                && database.resolve_type(interface.file, interface.ty).is_ok()
        }));
    }
}
