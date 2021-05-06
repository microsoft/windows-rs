use super::*;

#[derive(Debug)]
pub struct TypeTree {
    pub namespace: &'static str,
    pub types: Vec<ElementType>,
    pub namespaces: BTreeMap<&'static str, TypeTree>,
}

impl TypeTree {
    pub fn from_namespace(namespace: &'static str) -> Self {
        Self {
            namespace,
            types: Vec::new(),
            namespaces: BTreeMap::new(),
        }
    }

    pub fn from_imports(
        reader: &'static TypeReader,
        imports: &BTreeMap<&'static str, ImportLimit>,
    ) -> Self {
        let mut root = Self::from_namespace("");
        let mut set = BTreeSet::new();

        for (namespace, limit) in imports {
            match limit {
                ImportLimit::All => {
                    for def in reader.namespace_types(namespace) {
                        root.insert_if(reader, namespace, &mut set, &def);
                    }
                }
                ImportLimit::Some(names) => {
                    for name in names {
                        root.insert_if(
                            reader,
                            namespace,
                            &mut set,
                            &reader.resolve_type(namespace, name),
                        );
                    }
                }
            }
        }

        root
    }

    fn insert_if(
        &mut self,
        reader: &TypeReader,
        namespace: &'static str,
        set: &mut BTreeSet<Row>,
        t: &ElementType,
    ) {
        if set.insert(t.row()) {
            for def in t.dependencies() {
                self.insert_if(reader, def.namespace(), set, &def);
            }

            if !namespace.is_empty() {
                self.insert(namespace, 0, t);
            }
        }
    }

    fn insert(&mut self, namespace: &'static str, pos: usize, t: &ElementType) {
        if let Some(next) = namespace[pos..].find('.') {
            let next = pos + next;
            self.namespaces
                .entry(&namespace[pos..next])
                .or_insert_with(|| Self::from_namespace(&namespace[..next]))
                .insert(namespace, next + 1, t);
        } else {
            self.namespaces
                .entry(&namespace[pos..])
                .or_insert_with(|| Self::from_namespace(namespace))
                .types
                .push(t.clone());
        }
    }

    pub fn include_method(&self, signature: &MethodSignature) -> bool {
        for kind in signature.dependencies() {
            if !self.includes(&kind) {
                return false;
            }
        }

        true
    }

    pub fn includes(&self, kind: &ElementType) -> bool {
        let namespace = kind.namespace();

        if namespace.is_empty() {
            return true;
        }

        self.includes_namespace_type(namespace, kind)
    }

    fn includes_namespace_type(&self, namespace: &str, kind: &ElementType) -> bool {
        if let Some(pos) = namespace.find('.') {
            if let Some(tree) = self.namespaces.get(&namespace[..pos]) {
                return tree.includes_namespace_type(&namespace[pos + 1..], kind);
            }
        } else if let Some(tree) = self.namespaces.get(namespace) {
            return tree.types.iter().any(|t| t.row() == kind.row());
        }

        false
    }

    pub fn gen<'a>(&'a self, root: &'a Self) -> impl Iterator<Item = TokenStream> + 'a {
        let gen = Gen::relative(self.namespace, root);

        self.types
            .iter()
            .map(move |t| t.gen(&gen))
            .chain(gen_namespaces(&self.namespaces, root))
    }
}

fn gen_namespaces<'a>(
    namespaces: &'a BTreeMap<&'static str, TypeTree>,
    root: &'a TypeTree,
) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        let name = to_ident(name);

        let tokens = tree.gen(root);

        quote! {
            // TODO: https://github.com/microsoft/windows-rs/issues/212
            // TODO: https://github.com/microsoft/win32metadata/issues/380
            #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
            pub mod #name {
                #(#tokens)*
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let reader = TypeReader::get();
        let mut imports = BTreeMap::new();

        let mut single = BTreeSet::new();
        single.insert("FILE_ACCESS_FLAGS");

        imports.insert("Windows.Win32.FileSystem", ImportLimit::Some(single));

        let tree = TypeTree::from_imports(reader, &imports);

        assert_eq!(tree.namespace, "");
        assert_eq!(tree.types.len(), 0);
        assert_eq!(tree.namespaces.len(), 1);

        let tree = &tree.namespaces["Windows"];

        assert_eq!(tree.namespace, "Windows");
        assert_eq!(tree.types.len(), 0);
        assert_eq!(tree.namespaces.len(), 1);

        let tree = &tree.namespaces["Win32"];

        assert_eq!(tree.namespace, "Windows.Win32");
        assert_eq!(tree.types.len(), 0);
        assert_eq!(tree.namespaces.len(), 1);

        let tree = &tree.namespaces["FileSystem"];

        assert_eq!(tree.namespace, "Windows.Win32.FileSystem");
        assert_eq!(tree.types.len(), 1);
        assert_eq!(tree.namespaces.len(), 0);

        let t = &tree.types[0];
        assert_eq!(
            t.gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "Windows :: Win32 :: FileSystem :: FILE_ACCESS_FLAGS"
        );
    }
}
