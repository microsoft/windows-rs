use super::*;

/// Indexes metadata as raw types and as Win32 `Apis`-expanded items.
pub struct Index {
    files: Vec<File>,
    types: HashMap<String, HashMap<String, Vec<(usize, usize)>>>,
    items: HashMap<String, HashMap<String, Vec<RawItem>>>,
    nested: HashMap<(usize, usize), Vec<usize>>,
    architecture: i32,
}

#[derive(Copy, Clone)]
enum RawItem {
    Type(usize, usize),  // (file_pos, typedef_pos)
    Fn(usize, usize),    // (file_pos, methoddef_pos)
    Const(usize, usize), // (file_pos, field_pos)
}

/// An individually-addressable item exposed by [`Index::iter_items`] and friends.
#[derive(Copy, Clone, Debug)]
pub enum Item<'a> {
    Type(TypeDef<'a>),
    Fn(MethodDef<'a>),
    Const(Field<'a>),
}

impl Index {
    /// Reads a single metadata file from disk and builds an `Index` for it.
    #[must_use]
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        Some(Self::new(vec![File::read(path)?]))
    }

    /// Builds an `Index` over the given metadata files.
    #[must_use]
    pub fn new(files: Vec<File>) -> Self {
        Self::new_for_architecture(files, 0)
    }

    /// Builds an `Index` filtered to one architecture bit (1=X86, 2=X64, 4=Arm64). A zero bit
    /// preserves all architecture-specific rows.
    #[must_use]
    pub fn new_for_architecture(files: Vec<File>, architecture: i32) -> Self {
        // Build types first so `Apis` detection can inspect synthesized `TypeDef` values.
        let mut types: HashMap<String, HashMap<String, Vec<(usize, usize)>>> = HashMap::new();
        let mut nested: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (file_pos, file) in files.iter().enumerate() {
            for def_pos in file.TypeDef() {
                let namespace = file.str(def_pos, TypeDef::TABLE, 2);

                if namespace.is_empty() {
                    continue;
                }

                let name = file.str(def_pos, TypeDef::TABLE, 1);

                types
                    .entry(namespace.to_string())
                    .or_default()
                    .entry(trim_tick(name).to_string())
                    .or_default()
                    .push((file_pos, def_pos));
            }

            for map in file.NestedClass() {
                let inner = file.usize(map, NestedClass::TABLE, 0) - 1;
                let outer = file.usize(map, NestedClass::TABLE, 1) - 1;
                nested.entry((file_pos, outer)).or_default().push(inner);
            }
        }

        let mut index = Self {
            files,
            types,
            items: HashMap::new(),
            nested,
            architecture,
        };

        // Expand Win32 `Apis` into function/constant items.
        let mut items: HashMap<String, HashMap<String, Vec<RawItem>>> = HashMap::new();

        for (namespace, name, ty) in index.iter() {
            let apis = !ty.flags().contains(TypeAttributes::WindowsRuntime)
                && ty.category() == TypeCategory::Class
                && name == "Apis";

            if apis {
                for method in ty
                    .methods()
                    .filter(|method| index.supports_architecture(*method))
                {
                    items
                        .entry(namespace.to_string())
                        .or_default()
                        .entry(method.name().to_string())
                        .or_default()
                        .push(RawItem::Fn(ty.0.file, method.0.pos));
                }
                for field in ty
                    .fields()
                    .filter(|field| index.supports_architecture(*field))
                {
                    items
                        .entry(namespace.to_string())
                        .or_default()
                        .entry(field.name().to_string())
                        .or_default()
                        .push(RawItem::Const(ty.0.file, field.0.pos));
                }
            } else {
                items
                    .entry(namespace.to_string())
                    .or_default()
                    .entry(name.to_string())
                    .or_default()
                    .push(RawItem::Type(ty.0.file, ty.0.pos));
            }
        }

        index.items = items;
        index
    }

    /// Leaks `self` for consumers that need `'static` metadata rows.
    #[must_use]
    pub fn leak(self) -> &'static Self {
        Box::leak(Box::new(self))
    }

    /// Reads one metadata file and leaks the resulting `Index`.
    #[must_use]
    pub fn read_static<P: AsRef<std::path::Path>>(path: P) -> Option<&'static Self> {
        Some(Self::new(vec![File::read(path)?]).leak())
    }

    pub(crate) fn files(&self, pos: usize) -> &File {
        &self.files[pos]
    }

    /// Iterates `(namespace, name, TypeDef)` triples over every type in the index.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, TypeDef<'_>)> + '_ {
        self.types
            .iter()
            .flat_map(|(namespace, types)| {
                types
                    .iter()
                    .map(move |(name, types)| (namespace.as_str(), name.as_str(), types))
            })
            .flat_map(|(namespace, name, types)| types.iter().map(move |ty| (namespace, name, ty)))
            .map(|(namespace, name, (file, pos))| {
                (namespace, name, TypeDef(Row::new(self, *file, *pos)))
            })
            .filter(|(_, _, def)| self.supports_architecture(*def))
    }

    /// Iterates every `TypeDef` in the index.
    pub fn types(&self) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.types
            .values()
            .flat_map(|types| types.values())
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
            .filter(|def| self.supports_architecture(*def))
    }

    /// Iterates the `TypeDef`s matching `(namespace, name)`.
    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
            .filter(|def| self.supports_architecture(*def))
    }

    /// Returns whether any type lives in `namespace`.
    pub fn contains_namespace(&self, namespace: &str) -> bool {
        self.types.get(namespace).is_some_and(|types| {
            types
                .values()
                .flatten()
                .any(|(file, pos)| self.supports_architecture(TypeDef(Row::new(self, *file, *pos))))
        })
    }

    /// Returns whether the `(namespace, name)` type exists.
    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.get(namespace, name).next().is_some()
    }

    /// Returns the assembly name of the first file in which `(namespace, name)` was defined.
    pub fn assembly_name(&self, namespace: &str, name: &str) -> Option<&str> {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .and_then(|types| {
                types.iter().find(|(file, pos)| {
                    self.supports_architecture(TypeDef(Row::new(self, *file, *pos)))
                })
            })
            .map(|(file, _)| self.files(*file))
            .and_then(|file| file.assembly_name())
    }

    /// Returns the single `TypeDef` matching `(namespace, name)`, panicking if there are zero
    /// or more than one.
    #[track_caller]
    pub fn expect(&self, namespace: &str, name: &str) -> TypeDef<'_> {
        let mut iter = self.get(namespace, name);

        if let Some(def) = iter.next() {
            if iter.next().is_none() {
                def
            } else {
                panic!("more than one type found: {namespace}.{name}");
            }
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }

    /// Iterates the types directly nested inside `ty`. Use [`Self::nested_recursive`] to recurse.
    pub fn nested(&self, ty: TypeDef) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.nested
            .get(&(ty.0.file, ty.0.pos))
            .into_iter()
            .flatten()
            .copied()
            .map(move |pos| {
                TypeDef(Row {
                    index: self,
                    file: ty.0.file,
                    pos,
                })
            })
            .filter(|def| self.supports_architecture(*def))
    }

    /// Depth-first walk of every type nested directly or transitively inside `ty`.
    pub fn nested_recursive<'a>(&'a self, ty: TypeDef<'a>) -> Vec<TypeDef<'a>> {
        let mut out = Vec::new();
        self.collect_nested(ty, &mut out);
        out
    }

    fn collect_nested<'a>(&'a self, ty: TypeDef<'a>, out: &mut Vec<TypeDef<'a>>) {
        for inner in self.nested(ty) {
            out.push(inner);
            self.collect_nested(inner, out);
        }
    }

    fn supports_architecture<'a, R: HasAttributes<'a>>(&self, row: R) -> bool {
        let arches = row.arches();
        self.architecture == 0 || arches == 0 || arches & self.architecture != 0
    }

    /// Iterates every namespace that contains at least one item.
    pub fn namespaces(&self) -> impl Iterator<Item = &str> + '_ {
        self.items.keys().map(String::as_str)
    }

    /// Iterates over every custom attribute row in the indexed files.
    pub fn attributes(&self) -> impl Iterator<Item = Attribute<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every property map row in the indexed files.
    pub fn property_maps(&self) -> impl Iterator<Item = PropertyMap<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every property row in the indexed files.
    pub fn properties(&self) -> impl Iterator<Item = Property<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every event map row in the indexed files.
    pub fn event_maps(&self) -> impl Iterator<Item = EventMap<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every event row in the indexed files.
    pub fn events(&self) -> impl Iterator<Item = Event<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every class layout row in the indexed files.
    pub fn class_layouts(&self) -> impl Iterator<Item = ClassLayout<'_>> + '_ {
        self.table_rows()
    }

    /// Iterates over every field layout row in the indexed files.
    pub fn field_layouts(&self) -> impl Iterator<Item = FieldLayout<'_>> + '_ {
        self.table_rows()
    }

    fn table_rows<'a, R: AsRow<'a> + 'a>(&'a self) -> impl Iterator<Item = R> + 'a {
        self.files
            .iter()
            .enumerate()
            .flat_map(move |(file, metadata)| {
                metadata
                    .rows(R::TABLE)
                    .map(move |pos| R::from_row(Row::new(self, file, pos)))
            })
    }

    /// Iterates `(namespace, name, Item)` triples over every item in the index.
    pub fn iter_items(&self) -> impl Iterator<Item = (&str, &str, Item<'_>)> + '_ {
        self.items
            .iter()
            .flat_map(|(namespace, items)| {
                items
                    .iter()
                    .map(move |(name, items)| (namespace.as_str(), name.as_str(), items))
            })
            .flat_map(move |(namespace, name, items)| {
                items
                    .iter()
                    .map(move |raw| (namespace, name, self.item(*raw)))
            })
    }

    /// Iterates every `Item` in the index.
    pub fn items(&self) -> impl Iterator<Item = Item<'_>> + '_ {
        self.items
            .values()
            .flat_map(|items| items.values())
            .flatten()
            .map(move |raw| self.item(*raw))
    }

    /// Iterates `(name, Item)` pairs in `namespace`.
    pub fn namespace_items(&self, namespace: &str) -> impl Iterator<Item = (&str, Item<'_>)> + '_ {
        self.items
            .get(namespace)
            .into_iter()
            .flatten()
            .flat_map(move |(name, items)| {
                items
                    .iter()
                    .map(move |raw| (name.as_str(), self.item(*raw)))
            })
    }

    /// Iterates the `Item`s matching `(namespace, name)`.
    pub fn get_item(&self, namespace: &str, name: &str) -> impl Iterator<Item = Item<'_>> + '_ {
        self.items
            .get(namespace)
            .and_then(|items| items.get(name))
            .into_iter()
            .flatten()
            .map(move |raw| self.item(*raw))
    }

    /// Returns the single `Item` matching `(namespace, name)`, panicking if there are zero
    /// or more than one.
    #[track_caller]
    pub fn expect_item(&self, namespace: &str, name: &str) -> Item<'_> {
        let mut iter = self.get_item(namespace, name);

        if let Some(item) = iter.next() {
            if iter.next().is_none() {
                item
            } else {
                panic!("more than one item found: {namespace}.{name}");
            }
        } else {
            panic!("item not found: {namespace}.{name}")
        }
    }

    fn item(&self, raw: RawItem) -> Item<'_> {
        match raw {
            RawItem::Type(file, pos) => Item::Type(TypeDef(Row::new(self, file, pos))),
            RawItem::Fn(file, pos) => Item::Fn(MethodDef(Row::new(self, file, pos))),
            RawItem::Const(file, pos) => Item::Const(Field(Row::new(self, file, pos))),
        }
    }
}
