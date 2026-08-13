use super::*;

/// Identifies an image owned by a [`Database`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FileId(usize);

impl FileId {
    pub(crate) const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based image position.
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Combines an owned image identity with a typed row identity.
pub struct Entity<T: Table> {
    file: FileId,
    row: RowId<T>,
}

impl<T: Table> Entity<T> {
    pub(crate) const fn new(file: FileId, row: RowId<T>) -> Self {
        Self { file, row }
    }

    /// Returns the owning image.
    pub const fn file(self) -> FileId {
        self.file
    }

    /// Returns the table-local row identity.
    pub const fn row(self) -> RowId<T> {
        self.row
    }
}

impl<T: Table> Clone for Entity<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Table> Copy for Entity<T> {}

impl<T: Table> std::fmt::Debug for Entity<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(T::ID.schema().name())
            .field("file", &self.file)
            .field("row", &self.row)
            .finish()
    }
}

impl<T: Table> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.file, self.row) == (other.file, other.row)
    }
}

impl<T: Table> Eq for Entity<T> {}

impl<T: Table> PartialOrd for Entity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Table> Ord for Entity<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.file, self.row).cmp(&(other.file, other.row))
    }
}

impl<T: Table> std::hash::Hash for Entity<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.file.hash(state);
        self.row.hash(state);
    }
}

/// The result of resolving an encoded signature type reference.
pub enum TypeResolution<'a> {
    /// The signature directly names a TypeDef row.
    Definition(Entity<tables::TypeDef>),
    /// The signature names a TypeRef resolved by namespace and name.
    Candidates(TypeCandidates<'a>),
    /// The signature names a TypeSpec row.
    Specification(Entity<tables::TypeSpec>),
}

/// Type definitions matching a TypeRef name and resolution scope.
#[derive(Clone, Copy)]
pub struct TypeCandidates<'a> {
    database: &'a Database,
    definitions: &'a [Entity<tables::TypeDef>],
    scope: CandidateScope<'a>,
}

#[derive(Clone, Copy)]
enum CandidateScope<'a> {
    Any,
    File(FileId),
    Assembly(&'a str),
}

impl<'a> TypeCandidates<'a> {
    /// Returns the number of matching definitions.
    pub fn len(self) -> usize {
        self.iter().count()
    }

    /// Returns whether no definition matches the resolution scope.
    pub fn is_empty(self) -> bool {
        self.iter().next().is_none()
    }

    /// Returns the first matching definition.
    pub fn first(self) -> Option<Entity<tables::TypeDef>> {
        self.iter().next()
    }

    /// Iterates definitions matching the resolution scope.
    pub fn iter(self) -> impl Iterator<Item = Entity<tables::TypeDef>> + 'a {
        let assembly_match = match self.scope {
            CandidateScope::Assembly(name) => self
                .definitions
                .iter()
                .any(|definition| self.database.assembly_name(definition.file()) == Some(name)),
            _ => false,
        };
        self.definitions
            .iter()
            .copied()
            .filter(move |definition| match self.scope {
                CandidateScope::Any => true,
                CandidateScope::File(file) => definition.file() == file,
                CandidateScope::Assembly(name) => {
                    !assembly_match || self.database.assembly_name(definition.file()) == Some(name)
                }
            })
    }
}

/// Owns metadata images and indexes cross-image identities.
pub struct Database {
    images: Vec<Image>,
    types: HashMap<String, HashMap<String, Vec<Entity<tables::TypeDef>>>>,
    pub(crate) nested: HashMap<Entity<tables::TypeDef>, Vec<Entity<tables::TypeDef>>>,
}

impl Database {
    /// Builds a database over owned metadata images.
    pub fn new(images: impl IntoIterator<Item = Image>) -> Result<Self, Error> {
        let images: Vec<_> = images.into_iter().collect();
        let mut types: HashMap<String, HashMap<String, Vec<_>>> = HashMap::new();
        let mut nested_types: HashMap<Entity<tables::TypeDef>, Vec<Entity<tables::TypeDef>>> =
            HashMap::new();

        for (file, image) in images.iter().enumerate() {
            let file = FileId::new(file);
            let mut assembly_rows = image.rows::<tables::Assembly>();
            if let Some(row) = assembly_rows.next() {
                image.view(row).unwrap().string(7)?;
            }
            if assembly_rows.next().is_some() {
                return Err(Error::invalid_metadata(
                    "image has more than one Assembly row",
                ));
            }
            let mut nested = HashSet::new();
            for row in image.rows::<tables::NestedClass>() {
                let relationship = image.view(row).unwrap();
                let child = relationship
                    .index::<tables::TypeDef>(0)?
                    .ok_or_else(|| Error::invalid(row.number() as usize, "null nested type"))?;
                let parent = relationship
                    .index::<tables::TypeDef>(1)?
                    .ok_or_else(|| Error::invalid(row.number() as usize, "null enclosing type"))?;
                nested.insert(child.number());
                nested_types
                    .entry(Entity::new(file, parent))
                    .or_default()
                    .push(Entity::new(file, child));
            }
            for row in image.rows::<tables::TypeDef>() {
                if nested.contains(&row.number()) {
                    continue;
                }
                let view = image.view(row).unwrap();
                let namespace = view.string(2)?;
                let name = view.string(1)?;
                if namespace.is_empty() && name == "<Module>" {
                    continue;
                }
                types
                    .entry(namespace.to_string())
                    .or_default()
                    .entry(name.to_string())
                    .or_default()
                    .push(Entity::new(file, row));
            }
        }

        Ok(Self {
            images,
            types,
            nested: nested_types,
        })
    }

    /// Returns the owned images.
    pub fn images(&self) -> &[Image] {
        &self.images
    }

    /// Returns an image by identity.
    pub fn image(&self, file: FileId) -> Option<&Image> {
        self.images.get(file.index())
    }

    fn assembly_name(&self, file: FileId) -> Option<&str> {
        let image = self.image(file)?;
        let row = image.rows::<tables::Assembly>().next()?;
        Some(image.view(row).unwrap().string(7).unwrap())
    }

    /// Resolves a typed entity to a row view.
    pub fn view<T: Table>(&self, entity: Entity<T>) -> Option<Row<'_, T>> {
        self.image(entity.file())?.view(entity.row())
    }

    /// Returns every non-nested type definition with the given full name.
    pub fn type_definitions(&self, namespace: &str, name: &str) -> &[Entity<tables::TypeDef>] {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .map_or(&[], Vec::as_slice)
    }

    /// Iterates indexed type names and all matching definitions.
    pub fn type_names(&self) -> impl Iterator<Item = (&str, &str, &[Entity<tables::TypeDef>])> {
        self.types.iter().flat_map(|(namespace, types)| {
            types.iter().map(move |(name, definitions)| {
                (namespace.as_str(), name.as_str(), definitions.as_slice())
            })
        })
    }

    /// Resolves a signature type reference from one image.
    pub fn resolve_type(&self, file: FileId, ty: AnyRowId) -> Result<TypeResolution<'_>, Error> {
        let image = self
            .image(file)
            .ok_or_else(|| Error::invalid(file.index(), "file identity is outside the database"))?;
        match ty.table() {
            TableId::TypeDef => {
                let row = image
                    .row::<tables::TypeDef>(ty.number())
                    .ok_or_else(|| Error::invalid(ty.number() as usize, "type row is invalid"))?;
                Ok(TypeResolution::Definition(Entity::new(file, row)))
            }
            TableId::TypeRef => {
                let row = image
                    .row::<tables::TypeRef>(ty.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid(ty.number() as usize, "type row is invalid"))?;
                let namespace = row.string(2)?;
                let name = row.string(1)?;
                let scope = match row.coded(0)? {
                    Some(scope) if scope.table() == TableId::AssemblyRef => {
                        let assembly = image
                            .row::<tables::AssemblyRef>(scope.number())
                            .and_then(|row| image.view(row))
                            .ok_or_else(|| {
                                Error::invalid_metadata("invalid TypeRef assembly scope")
                            })?;
                        CandidateScope::Assembly(assembly.string(6)?)
                    }
                    Some(scope)
                        if scope.table() == TableId::Module
                            || scope.table() == TableId::ModuleRef =>
                    {
                        CandidateScope::File(file)
                    }
                    Some(scope) if scope.table() == TableId::TypeRef => CandidateScope::Any,
                    Some(_) => {
                        return Err(Error::invalid_metadata("invalid TypeRef resolution scope"));
                    }
                    None => CandidateScope::File(file),
                };
                Ok(TypeResolution::Candidates(TypeCandidates {
                    database: self,
                    definitions: self.type_definitions(namespace, name),
                    scope,
                }))
            }
            TableId::TypeSpec => {
                let row = image
                    .row::<tables::TypeSpec>(ty.number())
                    .ok_or_else(|| Error::invalid(ty.number() as usize, "type row is invalid"))?;
                Ok(TypeResolution::Specification(Entity::new(file, row)))
            }
            _ => Err(Error::invalid(
                ty.number() as usize,
                "row is not a signature type reference",
            )),
        }
    }

    /// Returns the namespace and name encoded by a TypeDef or TypeRef identity.
    ///
    /// TypeSpec identities return `None`; callers must inspect their signature instead.
    pub fn type_name(&self, file: FileId, ty: AnyRowId) -> Result<Option<(&str, &str)>, Error> {
        let image = self
            .image(file)
            .ok_or_else(|| Error::invalid(file.index(), "file identity is outside the database"))?;
        let name = match ty.table() {
            TableId::TypeDef => {
                let row = image
                    .row::<tables::TypeDef>(ty.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid(ty.number() as usize, "type row is invalid"))?;
                Some((row.string(2)?, row.string(1)?))
            }
            TableId::TypeRef => {
                let row = image
                    .row::<tables::TypeRef>(ty.number())
                    .and_then(|row| image.view(row))
                    .ok_or_else(|| Error::invalid(ty.number() as usize, "type row is invalid"))?;
                Some((row.string(2)?, row.string(1)?))
            }
            TableId::TypeSpec => None,
            _ => {
                return Err(Error::invalid(
                    ty.number() as usize,
                    "row is not a signature type reference",
                ));
            }
        };
        Ok(name)
    }

    pub(crate) fn type_members<T: Table>(
        &self,
        definition: Entity<tables::TypeDef>,
        column: usize,
    ) -> Result<Rows<T>, Error> {
        self.list_members(definition, column)
    }

    pub(crate) fn method_parameters(
        &self,
        method: Entity<tables::MethodDef>,
    ) -> Result<Rows<tables::Param>, Error> {
        self.list_members(method, 5)
    }

    fn list_members<O: Table, T: Table>(
        &self,
        owner: Entity<O>,
        column: usize,
    ) -> Result<Rows<T>, Error> {
        let image = self
            .image(owner.file())
            .ok_or_else(|| Error::invalid(owner.file().index(), "invalid file identity"))?;
        let row = image
            .view(owner.row())
            .ok_or_else(|| Error::invalid(owner.row().number() as usize, "invalid owner row"))?;
        let start = row.list::<T>(column)?;
        let end = owner
            .row()
            .number()
            .checked_add(1)
            .and_then(|number| image.row::<O>(number))
            .map_or_else(
                || {
                    image
                        .table(T::ID)
                        .rows()
                        .checked_add(1)
                        .and_then(ListIndex::new)
                },
                |next| image.view(next).unwrap().list::<T>(column).ok(),
            )
            .ok_or_else(|| Error::invalid(row.id().number() as usize, "invalid member list"))?;
        image
            .list_range(start, end)
            .ok_or_else(|| Error::invalid(row.id().number() as usize, "invalid member range"))
    }

    pub(crate) fn fields(
        &self,
        definition: Entity<tables::TypeDef>,
    ) -> Result<Rows<tables::Field>, Error> {
        self.type_members(definition, 4)
    }

    pub(crate) fn method_owner(
        &self,
        file: FileId,
        method: RowId<tables::MethodDef>,
    ) -> Result<Entity<tables::TypeDef>, Error> {
        let image = self
            .image(file)
            .ok_or_else(|| Error::invalid_metadata("invalid file identity"))?;
        let mut low = 1;
        let mut high = image
            .table(TableId::TypeDef)
            .rows()
            .checked_add(1)
            .ok_or_else(|| Error::invalid_metadata("type definition range overflow"))?;
        while low < high {
            let middle = low + (high - low) / 2;
            let definition = image.row::<tables::TypeDef>(middle).unwrap();
            let start = image
                .view(definition)
                .unwrap()
                .list::<tables::MethodDef>(5)?
                .number();
            if start <= method.number() {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        let owner = low
            .checked_sub(1)
            .and_then(|number| image.row::<tables::TypeDef>(number))
            .ok_or_else(|| Error::invalid_metadata("method has no declaring type"))?;
        Ok(Entity::new(file, owner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_name_multiplicities_match_existing_index() {
        let images = [
            Image::new(windows_default::WINRT).unwrap(),
            Image::new(windows_default::WIN32).unwrap(),
        ];
        let database = Database::new(images).unwrap();
        let mut actual: Vec<_> = database
            .type_names()
            .map(|(namespace, name, definitions)| {
                (namespace.to_string(), name.to_string(), definitions.len())
            })
            .collect();

        let old = windows_metadata::reader::Index::new(vec![
            windows_metadata::reader::File::new(windows_default::WINRT.to_vec()).unwrap(),
            windows_metadata::reader::File::new(windows_default::WIN32.to_vec()).unwrap(),
        ]);
        let mut counts = HashMap::new();
        for (namespace, _, definition) in old.iter() {
            *counts
                .entry((namespace.to_string(), definition.name().to_string()))
                .or_insert(0usize) += 1;
        }
        let mut expected: Vec<_> = counts
            .into_iter()
            .map(|((namespace, name), count)| (namespace, name, count))
            .collect();

        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn resolves_direct_and_referenced_types() {
        let database = Database::new([Image::new(windows_default::WINRT).unwrap()]).unwrap();
        let point = database.type_definitions("Windows.Foundation", "Point")[0];
        let image = database.image(point.file()).unwrap();
        let direct = AnyRowId::new(TableId::TypeDef, point.row().number()).unwrap();
        assert!(matches!(
            database.resolve_type(point.file(), direct).unwrap(),
            TypeResolution::Definition(definition) if definition == point
        ));

        let referenced = image
            .rows::<tables::TypeRef>()
            .find_map(|id| {
                let row = image.view(id).unwrap();
                (!database
                    .type_definitions(row.string(2).unwrap(), row.string(1).unwrap())
                    .is_empty())
                .then(|| AnyRowId::new(TableId::TypeRef, id.number()).unwrap())
            })
            .unwrap();

        match database.resolve_type(point.file(), referenced).unwrap() {
            TypeResolution::Candidates(candidates) => {
                assert!(!candidates.is_empty());
            }
            _ => panic!("expected a resolved TypeRef"),
        }
    }
}
