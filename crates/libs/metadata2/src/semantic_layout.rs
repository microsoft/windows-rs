use super::*;

/// A database-backed semantic view of a ClassLayout row.
#[derive(Clone, Copy)]
pub struct ClassLayoutDefinition<'a> {
    database: &'a Database,
    entity: Entity<tables::ClassLayout>,
}

impl<'a> TypeDefinition<'a> {
    /// Returns the class layout attached to this type, when present.
    pub fn layout(self) -> Result<Option<ClassLayoutDefinition<'a>>, Error> {
        let file = self.entity.file();
        let image = self
            .database
            .image(file)
            .ok_or_else(|| Error::invalid_metadata("invalid file identity"))?;
        let mut rows = image.matching_rows::<tables::ClassLayout>(2, self.entity.row().number())?;
        let layout = rows.next().map(|row| ClassLayoutDefinition {
            database: self.database,
            entity: Entity::new(file, row),
        });
        if rows.next().is_some() {
            return Err(Error::invalid_metadata(
                "type definition has more than one class layout",
            ));
        }
        Ok(layout)
    }
}

impl<'a> ClassLayoutDefinition<'a> {
    /// Returns the field packing size.
    pub fn packing_size(self) -> Result<u16, Error> {
        self.row()?.u16(0)
    }

    /// Returns the encoded class size.
    pub fn class_size(self) -> Result<u32, Error> {
        self.row()?.u32(1)
    }

    fn row(self) -> Result<Row<'a, tables::ClassLayout>, Error> {
        self.database
            .view(self.entity)
            .ok_or_else(|| Error::invalid_metadata("invalid class layout identity"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_layouts_match_existing_reader() {
        let database = Database::new([
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ])
        .unwrap();
        let mut actual: Vec<_> = database
            .definitions()
            .filter_map(|definition| {
                definition.layout().unwrap().map(|layout| {
                    (
                        definition.namespace().unwrap().to_string(),
                        definition.name().unwrap().to_string(),
                        layout.packing_size().unwrap(),
                        layout.class_size().unwrap(),
                    )
                })
            })
            .collect();

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
            windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
        ]);
        let mut expected: Vec<_> = old
            .iter()
            .filter_map(|(_, _, definition)| {
                definition.class_layout().map(|layout| {
                    (
                        definition.namespace().to_string(),
                        definition.name().to_string(),
                        layout.packing_size(),
                        layout.class_size(),
                    )
                })
            })
            .collect();

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
}
