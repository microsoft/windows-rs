use super::*;
use std::sync::Arc;

struct TypeIndexData {
    files: Vec<File>,
    types: HashMap<String, HashMap<String, Vec<(usize, usize)>>>,
    nested: HashMap<(usize, usize), Vec<usize>>,
}

/// A ref-counted, cheaply-cloneable handle to the raw winmd index.
///
/// All row types (`TypeDef`, `MethodDef`, `Field`, …) store a clone of this
/// handle, so they are lifetime-free owned values that can be stored in
/// collections, hashed, and sent across threads without any `'static` tricks.
#[derive(Clone)]
pub struct TypeIndex(Arc<TypeIndexData>);

impl TypeIndex {
    #[must_use]
    pub fn read<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        Some(Self::new(vec![File::read(path)?]))
    }

    #[must_use]
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
                let inner = file.usize(map, NestedClass::TABLE, 0) - 1;
                let outer = file.usize(map, NestedClass::TABLE, 1) - 1;
                nested.entry((file_pos, outer)).or_default().push(inner);
            }
        }

        Self(Arc::new(TypeIndexData {
            files,
            types,
            nested,
        }))
    }

    pub(crate) fn files(&self, pos: usize) -> &File {
        &self.0.files[pos]
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str, TypeDef)> + '_ {
        self.0
            .types
            .iter()
            .flat_map(|(namespace, types)| {
                types
                    .iter()
                    .map(move |(name, types)| (namespace.as_str(), name.as_str(), types))
            })
            .flat_map(|(namespace, name, types)| types.iter().map(move |ty| (namespace, name, ty)))
            .map(|(namespace, name, (file, pos))| {
                (namespace, name, TypeDef(Row::new(self.clone(), *file, *pos)))
            })
    }

    pub fn types(&self) -> impl Iterator<Item = TypeDef> + '_ {
        self.0
            .types
            .values()
            .flat_map(|types| types.values())
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self.clone(), *file, *pos)))
    }

    pub fn get(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.0
            .types
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .map(|(file, pos)| TypeDef(Row::new(self.clone(), *file, *pos)))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn contains_namespace(&self, namespace: &str) -> bool {
        self.0.types.contains_key(namespace)
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.0
            .types
            .get(namespace)
            .and_then(|types| types.get(name))
            .is_some()
    }

    pub fn assembly_name(&self, namespace: &str, name: &str) -> Option<&str> {
        self.0
            .types
            .get(namespace)
            .and_then(|types| types.get(name))
            .and_then(|types| types.first())
            .map(|(file, _)| self.files(*file))
            .and_then(|file| file.assembly_name())
    }

    #[track_caller]
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
        let (file, pos) = (ty.0.file, ty.0.pos);
        self.0
            .nested
            .get(&(file, pos))
            .into_iter()
            .flatten()
            .cloned()
            .map(move |inner_pos| TypeDef(Row::new(self.clone(), file, inner_pos)))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
