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

                let name = def.name();

                if Type::remap(namespace, name).is_some() {
                    continue;
                }

                let items = reader.0.entry(namespace).or_default();
                let category = Category::new(def);

                if def.flags().contains(TypeAttributes::WindowsRuntime) {
                    let item = match category {
                        Category::Attribute => continue,
                        Category::Class => Item::Class(Class { def }),
                        Category::Delegate => Item::Delegate(Delegate { def }),
                        Category::Enum => Item::Enum(Enum { def }),
                        Category::Interface => Item::Interface(Interface { def }),
                        Category::Struct => {
                            // Skip marker types representing API contracts.
                            if def.has_attribute("ApiContractAttribute") {
                                continue;
                            }

                            Item::Struct(Struct { def })
                        }
                    };

                    insert(items, name, item);
                } else {
                    match category {
                        Category::Attribute => continue,
                        Category::Class => {
                            if name == "Apis" {
                                for method in def.methods() {
                                    if let Some(map) = method.impl_map() {
                                        // Skip inline and ordinal functions.
                                        if map.scope().name() == "FORCEINLINE"
                                            || map.import_name().starts_with("#")
                                        {
                                            continue;
                                        }
                                    }

                                    let name = method.name();
                                    insert(items, name, Item::CppFn(CppFn { def, method }));
                                }

                                for field in def.fields() {
                                    let name = field.name();
                                    insert(items, name, Item::CppConst(CppConst { def, field }));
                                }
                            }
                        }
                        Category::Delegate => {
                            insert(items, name, Item::CppDelegate(CppDelegate { def }));
                        }
                        Category::Enum => {
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
                        Category::Interface => {
                            insert(items, name, Item::CppInterface(CppInterface { def }));
                        }
                        Category::Struct => {
                            fn make(
                                def: TypeDef,
                                name: String,
                                nested: &HashMap<TypeDef, Vec<TypeDef>>,
                            ) -> Item {
                                let mut item = CppStruct {
                                    def,
                                    name,
                                    nested: BTreeMap::new(),
                                };

                                for (index, def) in
                                    nested.get(&def).into_iter().flatten().enumerate()
                                {
                                    item.nested.insert(
                                        def.name(),
                                        make(*def, format!("{}_{index}", item.name()), nested),
                                    );
                                }

                                Item::CppStruct(item)
                            }

                            insert(items, name, make(def, String::new(), &nested));
                        }
                    };
                }
            }
        }

        reader
    }

    /// Gets all items matching the given namespace and name.
    pub fn with_full_name(&self, namespace: &str, name: &str) -> impl Iterator<Item = &Item> + '_ {
        self.get(namespace)
            .and_then(|items| items.get(name))
            .into_iter()
            .flatten()
    }

    /// Gets all items with the given name regardless of namespace.
    pub fn with_name(&self, name: &str) -> Vec<&Item> {
        // This doesn't return an iterator as that would require `name` to be a static reference.
        self.values()
            .flatten()
            .filter_map(|(key, value)| (*key == name).then_some(value))
            .flatten()
            .collect()
    }

    /// Gets all items from the given namespace.
    pub fn with_namespace(&self, namespace: &str) -> impl Iterator<Item = &Item> + '_ {
        self.get(namespace)
            .into_iter()
            .flat_map(|map| map.values())
            .flatten()
    }

    pub fn dependencies(&'static self, namespace: &str, name: &str, dependencies: &mut Dependencies, minimal: bool) {
        for item in &self[namespace][name] {
            item.dependencies( dependencies, minimal);
        }
    }
}

enum Category {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
}

impl Category {
    fn new(def: TypeDef) -> Self {
        if let Some(extends) = def.extends() {
            if extends.namespace() == "System" {
                match extends.name() {
                    "Enum" => Self::Enum,
                    "MulticastDelegate" => Self::Delegate,
                    "ValueType" => Self::Struct,
                    "Attribute" => Self::Attribute,
                    _ => Self::Class,
                }
            } else {
                Self::Class
            }
        } else {
            Self::Interface
        }
    }
}
