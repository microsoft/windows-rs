use super::*;

pub struct Index {
    pub(crate) files: Vec<File>,
    pub(crate) types: HashMap<String, HashMap<String, Vec<(usize, usize)>>>,
    pub(crate) nested: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Index {
    pub fn new(files: Vec<File>) -> Self {
        let mut types: HashMap<String, HashMap<String, Vec<(usize, usize)>>> = HashMap::new();
        let mut nested: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

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
                nested
                    .entry((file_pos, outer))
                    .or_default()
                    .push((file_pos, inner));
            }
        }

        Self {
            files,
            types,
            nested,
        }
    }

    pub fn file(&self, pos: usize) -> &File {
        &self.files[pos]
    }

    pub fn all(&self) -> impl Iterator<Item = TypeDef> + '_ {
        self.types
            .values()
            .flat_map(|types| types.values())
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self, *file, *pos)))
    }

    pub fn expect(&self, namespace: &str, name: &str) -> TypeDef {
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

    pub fn nested(&self, ty: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested
            .get(&(ty.0.file, ty.0.pos))
            .into_iter()
            .flatten()
            .cloned()
            .map(|(file, pos)| {
                TypeDef(Row {
                    index: self,
                    file,
                    pos,
                })
            })
    }
}
