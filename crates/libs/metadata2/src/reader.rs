use super::*;

type ItemMap = HashMap<&'static str, Vec<Item>>;
type ReaderMap = HashMap<&'static str, ItemMap>;

fn insert(items: &mut ItemMap, name: &'static str, item: Item) {
    items.entry(name).or_default().push(item);
}

// TODO: this is the index for general type queries e.g. when there's a TypeRef or Blob that refers to a type by name
// then there's transmformations to flatten or filter this but the reader remains immutable to support the queries.
// This filtering process can then maybe remove actual duplicates?
pub struct Reader(ReaderMap);

impl std::ops::Deref for Reader {
    type Target = ReaderMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> &'static Self {
        let reader = Box::leak(Box::new(Self(HashMap::new())));

        for mut file in files {
            file.reader = reader;
            let file = Box::leak(Box::new(file));
            let mut nested = HashMap::<TypeDef, Vec<TypeDef>>::new();

            for key in file.table::<NestedClass>() {
                let inner = key.inner();
                nested.entry(key.outer()).or_default().push(inner);
            }

            for def in file.table::<TypeDef>() {
                let namespace = def.namespace();

                if namespace.is_empty() {
                    // This skips the nested types as we've already retrieved them.
                    continue;
                }

                let items = reader.0.entry(namespace).or_default();
                let name = def.name();
                let kind = def.kind();

                if def.flags().contains(TypeAttributes::WindowsRuntime) {
                    let item = match kind {
                        TypeKind::Class => Item::Class(Class { def }),
                        TypeKind::Delegate => Item::Delegate(Delegate { def }),
                        TypeKind::Enum => Item::Enum(Enum { def }),
                        TypeKind::Interface => Item::Interface(Interface { def }),
                        TypeKind::Struct => Item::Struct(Struct { def }),
                    };

                    insert(items, name, item);
                } else {
                    match kind {
                        TypeKind::Class => {
                            if name == "Apis" {
                                for method in def.methods() {
                                    let name = method.name();
                                    insert(items, name, Item::CppFn(CppFn { def, method }));
                                }

                                for field in def.fields() {
                                    let name = field.name();
                                    insert(items, name, Item::CppConst(CppConst { def, field }));
                                }
                            }
                        }
                        TypeKind::Delegate => {
                            insert(items, name, Item::CppDelegate(CppDelegate { def }));
                        }
                        TypeKind::Enum => {
                            insert(items, name, Item::CppEnum(CppEnum { def }));

                            if !def.has_attribute("ScopedEnumAttribute") {
                                for field in def.fields() {
                                    if field.flags().contains(FieldAttributes::Literal) {
                                        let name = field.name();
                                        insert(
                                            items,
                                            name,
                                            Item::CppConst(CppConst { def, field }),
                                        );
                                    }
                                }
                            }
                        }
                        TypeKind::Interface => {
                            insert(items, name, Item::CppInterface(CppInterface { def }));
                        }
                        TypeKind::Struct => {
                            fn make(
                                def: TypeDef,
                                nested: &HashMap<TypeDef, Vec<TypeDef>>,
                            ) -> CppStruct {
                                let mut item = CppStruct {
                                    def,
                                    nested: HashMap::new(),
                                };

                                for def in nested.get(&def).into_iter().flatten() {
                                    item.nested.insert(def.name(), make(*def, nested));
                                }

                                item
                            }

                            insert(items, name, Item::CppStruct(make(def, &nested)));
                        }
                    };
                }
            }
        }

        reader
    }

    pub fn get(
        &'static self,
        namespace: &str,
        name: &str,
    ) -> impl Iterator<Item = &'static Item> + 'static {
        self.0
            .get(namespace)
            .and_then(|items| items.get(name))
            .into_iter()
            .flatten()
    }

    // This doesn't return an iterator as that would require `name` to be a static reference.
    pub fn flat_get(&self, name: &str) -> Vec<&Item> {
        self.0
            .values()
            .flatten()
            .filter_map(|(key, value)| (*key == name).then_some(value))
            .flatten()
            .collect()
    }
}
