use super::*;

pub struct TypeIndex {
    files: Vec<File>,
    types: HashMap<String, HashMap<String, Vec<(usize, usize)>>>,
    nested: HashMap<(usize, usize), Vec<usize>>,
}

impl TypeIndex {
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        Some(Self::new(vec![File::read(path)?]))
    }

    pub fn new(files: Vec<File>) -> Self {
        let mut types: HashMap<String, HashMap<String, Vec<(usize, usize)>>> = HashMap::new();
        let mut nested: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (file_pos, file) in files.iter().enumerate() {
            for def_pos in file.TypeDef() {
                let namespace = file.str(def_pos, TypeDef::TABLE, 2);

                if namespace.is_empty() {
                    // Skips `<Module>` as well as nested types.
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
                let inner = file.usize(map, TypeDef::TABLE, 0);
                let outer = file.usize(map, TypeDef::TABLE, 1);
                nested.entry((file_pos, outer)).or_default().push(inner);
            }
        }

        Self {
            files,
            types,
            nested,
        }
    }

    pub(crate) fn files(&self, pos: usize) -> &File {
        &self.files[pos]
    }

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
    }

    pub fn types(&self) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.types
            .values()
            .flat_map(|types| types.values())
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
    }

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

    pub fn nested(&self, ty: TypeDef) -> impl Iterator<Item = TypeDef<'_>> + '_ {
        self.nested
            .get(&(ty.0.file, ty.0.pos))
            .into_iter()
            .flatten()
            .cloned()
            .map(move |pos| {
                TypeDef(Row {
                    index: self,
                    file: ty.0.file,
                    pos,
                })
            })
    }
}
