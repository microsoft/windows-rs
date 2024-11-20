use super::*;

// TODO: maybe use LazyLock to "expand"  the items in place including things like dependencies
// and required interfaces. This could avoid all the staging steps like Reader > Filter > Includes > ItemTree

type ItemMap = HashMap<&'static str, Vec<Type>>;
type ReaderMap = HashMap<&'static str, ItemMap>;

fn insert(items: &mut ItemMap, name: &'static str, item: Type) {
    // TODO: debug assert that we're only getting arch dupes on CppStruct and CppFn?
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
                let type_name = def.type_name();

                if type_name.namespace().is_empty() {
                    // This skips the nested types as we've already retrieved them.
                    continue;
                }

                if Type::remap(type_name) != Remap::None {
                    continue;
                }

                let items = reader.0.entry(type_name.0).or_default();
                let category = Category::new(def);

                if def.flags().contains(TypeAttributes::WindowsRuntime) {
                    let item = match category {
                        Category::Attribute => continue,
                        Category::Class => Type::Class(Class { def }),
                        Category::Delegate => Type::Delegate(Delegate {
                            def,
                            generics: def.generics(),
                        }),
                        Category::Enum => Type::Enum(Enum { def }),
                        Category::Interface => Type::Interface(Interface {
                            def,
                            generics: def.generics(),
                            kind: InterfaceKind::None,
                        }),
                        Category::Struct => {
                            // Skip marker types representing API contracts.
                            if def.has_attribute("ApiContractAttribute") {
                                continue;
                            }

                            Type::Struct(Struct { def })
                        }
                    };

                    insert(items, type_name.1, item);
                } else {
                    match category {
                        Category::Attribute => continue,
                        Category::Class => {
                            if type_name.1 == "Apis" {
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
                                    insert(
                                        items,
                                        name,
                                        Type::CppFn(CppFn {
                                            namespace: def.namespace(),
                                            method,
                                        }),
                                    );
                                }

                                for field in def.fields() {
                                    let name = field.name();
                                    insert(
                                        items,
                                        name,
                                        Type::CppConst(CppConst {
                                            namespace: def.namespace(),
                                            field,
                                        }),
                                    );
                                }
                            }
                        }
                        Category::Delegate => {
                            insert(items, type_name.1, Type::CppDelegate(CppDelegate { def }));
                        }
                        Category::Enum => {
                            insert(items, type_name.1, Type::CppEnum(CppEnum { def }));

                            if !def.has_attribute("ScopedEnumAttribute") {
                                for field in def.fields() {
                                    if field.flags().contains(FieldAttributes::Literal) {
                                        let name = field.name();
                                        insert(
                                            items,
                                            name,
                                            Type::CppConst(CppConst {
                                                namespace: def.namespace(),
                                                field,
                                            }),
                                        );
                                    }
                                }
                            }
                        }
                        Category::Interface => {
                            insert(items, type_name.1, Type::CppInterface(CppInterface { def }));
                        }
                        Category::Struct => {
                            fn make(
                                def: TypeDef,
                                name: String,
                                nested: &HashMap<TypeDef, Vec<TypeDef>>,
                            ) -> CppStruct {
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

                                item
                            }

                            insert(
                                items,
                                type_name.1,
                                Type::CppStruct(make(def, String::new(), &nested)),
                            );
                        }
                    };
                }
            }
        }

        reader
    }

    /// Gets all items matching the given namespace and name.
    pub fn with_full_name(&self, name: TypeName<'_>) -> impl Iterator<Item = Type> + '_ {
        self.get(name.namespace())
            .and_then(|items| items.get(name.name()))
            .into_iter()
            .flatten()
            .cloned()
    }

    // Gets all items with the given name regardless of namespace.
    // pub fn with_name(&self, name: &str) -> Vec<Type> {
    //     // This doesn't return an iterator as that would require `name` to be a static reference.
    //     self.values()
    //         .flatten()
    //         .filter_map(|(key, value)| (*key == name).then_some(value))
    //         .flatten()
    //         .cloned()
    //         .collect()
    // }

    /// Gets all items from the given namespace.
    pub fn with_namespace(&self, namespace: &str) -> impl Iterator<Item = Type> + '_ {
        self.get(namespace)
            .into_iter()
            .flat_map(|map| map.values())
            .flatten()
            .cloned()
    }

    pub fn get_type_name(&self, path: &str) -> TypeName<'static> {
        if let Some((namespace, name)) = path.rsplit_once('.') {
            if let Some((namespace, items)) = self.get_key_value(namespace) {
                if let Some((name, _)) = items.get_key_value(name) {
                    return TypeName(namespace, name);
                }
            }
        } else {
            for (namespace, items) in self.iter() {
                if let Some((name, _)) = items.get_key_value(path) {
                    return TypeName(namespace, name);
                }
            }
        }

        panic!("Type not found: `{path}`");
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
