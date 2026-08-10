use super::*;

/// A database-backed semantic view of an InterfaceImpl row.
#[derive(Clone, Copy)]
pub struct InterfaceImplementation<'a> {
    database: &'a Database,
    entity: Entity<tables::InterfaceImpl>,
}

impl Database {
    /// Iterates interface implementation relationship rows.
    pub fn interface_relationships(&self) -> impl Iterator<Item = InterfaceImplementation<'_>> {
        self.images()
            .iter()
            .enumerate()
            .flat_map(move |(file, image)| {
                let file = FileId::new(file);
                image
                    .rows::<tables::InterfaceImpl>()
                    .map(move |row| InterfaceImplementation {
                        database: self,
                        entity: Entity::new(file, row),
                    })
            })
    }

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

impl<'a> InterfaceImplementation<'a> {
    /// Returns the database identity.
    pub const fn entity(self) -> Entity<tables::InterfaceImpl> {
        self.entity
    }

    /// Returns the type that owns this relationship.
    pub fn owner(self) -> Result<TypeDefinition<'a>, Error> {
        let definition = self
            .row()?
            .index::<tables::TypeDef>(0)?
            .ok_or_else(|| Error::invalid_metadata("missing interface owner"))?;
        Ok(TypeDefinition {
            database: self.database,
            entity: Entity::new(self.entity.file(), definition),
        })
    }

    /// Returns the implemented interface identity.
    pub fn interface(self) -> Result<TypeIdentity, Error> {
        let ty = self
            .row()?
            .coded(1)?
            .ok_or_else(|| Error::invalid_metadata("missing implemented interface"))?;
        Ok(TypeIdentity {
            file: self.entity.file(),
            ty,
        })
    }

    /// Iterates custom attributes attached to this relationship.
    pub fn attributes(
        self,
    ) -> Result<impl ExactSizeIterator<Item = AttributeDefinition<'a>>, Error> {
        custom_attributes(self.database, self.entity)
    }

    /// Returns the first custom attribute with the given type name.
    pub fn find_attribute(self, name: &str) -> Result<Option<AttributeDefinition<'a>>, Error> {
        find_custom_attribute(self.attributes()?, name)
    }

    /// Returns whether a custom attribute with the given type name is present.
    pub fn has_attribute(self, name: &str) -> Result<bool, Error> {
        Ok(self.find_attribute(name)?.is_some())
    }

    fn row(self) -> Result<Row<'a, tables::InterfaceImpl>, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid interface relationship identity"))
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

    #[test]
    fn relationship_attributes_are_available() {
        let database = Database::new([Image::new(windows_default::WINRT).unwrap()]).unwrap();
        let mut relationships = database.interface_relationships();
        let relationship = relationships.next().unwrap();

        assert!(!relationship.owner().unwrap().name().unwrap().is_empty());
        let interface = relationship.interface().unwrap();
        assert!(database.resolve_type(interface.file, interface.ty).is_ok());
        relationship.attributes().unwrap().for_each(drop);
    }
}
