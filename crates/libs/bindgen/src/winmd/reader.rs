use super::*;

fn insert(types: &mut HashMap<Arc<str>, Vec<Type>>, name: &str, ty: Type) {
    types.entry(Arc::from(name)).or_default().push(ty);
}

pub struct Reader {
    map: HashMap<Arc<str>, HashMap<Arc<str>, Vec<Type>>>,
}

impl std::ops::Deref for Reader {
    type Target = HashMap<Arc<str>, HashMap<Arc<str>, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> Reader {
        let index = windows_metadata::reader::TypeIndex::new(files);

        let mut reader = Self {
            map: HashMap::new(),
        };

        // Build a nested-class map: outer TypeDef -> Vec<inner TypeDef>, including recursively
        // nested types (e.g. VARIANT -> VARIANT_0 -> _Anonymous_e__Struct).
        let mut nested: HashMap<TypeDef, Vec<TypeDef>> = HashMap::new();
        fn collect_nested(
            index: &windows_metadata::reader::TypeIndex,
            def: TypeDef,
            nested: &mut HashMap<TypeDef, Vec<TypeDef>>,
        ) {
            for inner in index.nested(def.clone()) {
                nested.entry(def.clone()).or_default().push(inner.clone());
                collect_nested(index, inner, nested);
            }
        }
        for (_, _, def) in index.iter() {
            collect_nested(&index, def, &mut nested);
        }

        for (namespace, name, def) in index.iter() {
            let flags = def.flags();

            if Type::remap(namespace, name) != Remap::None {
                continue;
            }

            let types = reader.map.entry(Arc::from(namespace)).or_default();
            let category = def.category();

            if flags.contains(TypeAttributes::WindowsRuntime) {
                let ty = match category {
                    windows_metadata::reader::TypeCategory::Attribute => continue,
                    windows_metadata::reader::TypeCategory::Class => {
                        Type::Class(Class { def: def.clone() })
                    }
                    windows_metadata::reader::TypeCategory::Delegate => {
                        let generics = def.generics();
                        Type::Delegate(Delegate {
                            def: def.clone(),
                            generics,
                        })
                    }
                    windows_metadata::reader::TypeCategory::Enum => {
                        Type::Enum(Enum { def: def.clone() })
                    }
                    windows_metadata::reader::TypeCategory::Interface => {
                        let generics = def.generics();
                        Type::Interface(Interface {
                            def: def.clone(),
                            generics,
                            kind: InterfaceKind::None,
                        })
                    }
                    windows_metadata::reader::TypeCategory::Struct => {
                        if def.has_attribute("ApiContractAttribute") {
                            continue;
                        }
                        Type::Struct(Struct { def: def.clone() })
                    }
                };

                insert(types, name, ty);
            } else {
                match category {
                    windows_metadata::reader::TypeCategory::Attribute => continue,
                    windows_metadata::reader::TypeCategory::Class => {
                        if name == "Apis" {
                            let ns: Arc<str> = Arc::from(namespace);
                            for method in def.methods() {
                                if let Some(map) = method.impl_map() {
                                    if map.import_scope().name() == "FORCEINLINE"
                                        || map.import_name().starts_with('#')
                                    {
                                        continue;
                                    }
                                }

                                let method_name = method.name().to_string();
                                insert(
                                    types,
                                    &method_name,
                                    Type::CppFn(CppFn {
                                        namespace: Arc::clone(&ns),
                                        method,
                                    }),
                                );
                            }

                            for field in def.fields() {
                                let field_name = field.name().to_string();
                                insert(
                                    types,
                                    &field_name,
                                    Type::CppConst(CppConst {
                                        namespace: Arc::clone(&ns),
                                        field,
                                    }),
                                );
                            }
                        }
                    }
                    windows_metadata::reader::TypeCategory::Delegate => {
                        insert(
                            types,
                            name,
                            Type::CppDelegate(CppDelegate { def: def.clone() }),
                        );
                    }
                    windows_metadata::reader::TypeCategory::Enum => {
                        insert(types, name, Type::CppEnum(CppEnum { def: def.clone() }));

                        if !def.has_attribute("ScopedEnumAttribute") {
                            let ns: Arc<str> = Arc::from(namespace);
                            for field in def.fields() {
                                if field.flags().contains(FieldAttributes::Literal) {
                                    let field_name = field.name().to_string();
                                    insert(
                                        types,
                                        &field_name,
                                        Type::CppConst(CppConst {
                                            namespace: Arc::clone(&ns),
                                            field,
                                        }),
                                    );
                                }
                            }
                        }
                    }
                    windows_metadata::reader::TypeCategory::Interface => {
                        insert(
                            types,
                            name,
                            Type::CppInterface(CppInterface { def: def.clone() }),
                        );
                    }
                    windows_metadata::reader::TypeCategory::Struct => {
                        fn make(
                            def: TypeDef,
                            name: Arc<str>,
                            nested: &HashMap<TypeDef, Vec<TypeDef>>,
                        ) -> CppStruct {
                            let mut ty = CppStruct {
                                def: def.clone(),
                                name: Arc::clone(&name),
                                nested: BTreeMap::new(),
                            };

                            for (index, nested_def) in
                                nested.get(&def).into_iter().flatten().enumerate()
                            {
                                if nested_def.category()
                                    == windows_metadata::reader::TypeCategory::Struct
                                {
                                    let nested_name: Arc<str> =
                                        Arc::from(format!("{}_{index}", name).as_str());
                                    let nested_key: Arc<str> = Arc::from(nested_def.name());
                                    ty.nested.insert(
                                        nested_key,
                                        make(nested_def.clone(), nested_name, nested),
                                    );
                                }
                            }

                            ty
                        }

                        let struct_name: Arc<str> = Arc::from(name);
                        insert(
                            types,
                            name,
                            Type::CppStruct(make(def.clone(), struct_name, &nested)),
                        );
                    }
                }
            }
        }

        reader
    }

    #[track_caller]
    pub fn unwrap_full_name(&self, namespace: &str, name: &str) -> Type {
        if let Some(ty) = self.with_full_name(namespace, name).next() {
            ty
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }

    /// Gets all types matching the given namespace and name.
    /// Trims any generic arity suffix (e.g. "`1") so callers may pass raw metadata names.
    pub fn with_full_name(&self, namespace: &str, name: &str) -> impl Iterator<Item = Type> + '_ {
        let name = windows_metadata::trim_tick(name);
        self.map
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .cloned()
    }
}
