use super::*;

fn insert(types: &mut HashMap<&'static str, Vec<Type>>, name: &'static str, ty: Type) {
    types.entry(name).or_default().push(ty);
}

pub struct Reader {
    map: HashMap<&'static str, HashMap<&'static str, Vec<Type>>>,
}

// Safety: all Type values stored in the map reference data from a 'static TypeIndex (Box::leaked),
// so they remain valid for the lifetime of the Reader, and Reader can be sent/shared across threads.
unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> Reader {
        // Leak the TypeIndex so all TypeDef<'static>, Field<'static>, etc. remain valid forever.
        let index: &'static windows_metadata::reader::TypeIndex =
            Box::leak(Box::new(windows_metadata::reader::TypeIndex::new(files)));

        let mut reader = Self {
            map: HashMap::new(),
        };

        // Build a nested-class map: outer TypeDef -> Vec<inner TypeDef>, including recursively
        // nested types (e.g. VARIANT -> VARIANT_0 -> _Anonymous_e__Struct).
        let mut nested: HashMap<TypeDef, Vec<TypeDef>> = HashMap::new();
        fn collect_nested(
            index: &'static windows_metadata::reader::TypeIndex,
            def: TypeDef,
            nested: &mut HashMap<TypeDef, Vec<TypeDef>>,
        ) {
            for inner in index.nested(def) {
                nested.entry(def).or_default().push(inner);
                collect_nested(index, inner, nested);
            }
        }
        for (_, _, def) in index.iter() {
            collect_nested(index, def, &mut nested);
        }

        let nested_inner: HashSet<TypeDef> = nested.values().flatten().cloned().collect();

        for (namespace, name, def) in index.iter() {
            if nested_inner.contains(&def) {
                continue;
            }

            let flags = def.flags();

            let type_name = TypeName(namespace, name);

            if Type::remap(type_name) != Remap::None {
                continue;
            }

            let types = reader.map.entry(namespace).or_default();
            let category = def.category();

            if flags.contains(TypeAttributes::WindowsRuntime) {
                let ty = match category {
                    windows_metadata::reader::TypeCategory::Attribute => continue,
                    windows_metadata::reader::TypeCategory::Class => Type::Class(Class { def }),
                    windows_metadata::reader::TypeCategory::Delegate => Type::Delegate(Delegate {
                        def,
                        generics: def.generics(),
                    }),
                    windows_metadata::reader::TypeCategory::Enum => Type::Enum(Enum { def }),
                    windows_metadata::reader::TypeCategory::Interface => {
                        Type::Interface(Interface {
                            def,
                            generics: def.generics(),
                            kind: InterfaceKind::None,
                        })
                    }
                    windows_metadata::reader::TypeCategory::Struct => {
                        if def.has_attribute("ApiContractAttribute") {
                            continue;
                        }
                        Type::Struct(Struct { def })
                    }
                };

                insert(types, name, ty);
            } else {
                match category {
                    windows_metadata::reader::TypeCategory::Attribute => continue,
                    windows_metadata::reader::TypeCategory::Class => {
                        if name == "Apis" {
                            for method in def.methods() {
                                if let Some(map) = method.impl_map() {
                                    if map.import_scope().name() == "FORCEINLINE"
                                        || map.import_name().starts_with('#')
                                    {
                                        continue;
                                    }
                                }

                                let method_name = method.name();
                                insert(
                                    types,
                                    method_name,
                                    Type::CppFn(CppFn { namespace, method }),
                                );
                            }

                            for field in def.fields() {
                                let field_name = field.name();
                                insert(
                                    types,
                                    field_name,
                                    Type::CppConst(CppConst { namespace, field }),
                                );
                            }
                        }
                    }
                    windows_metadata::reader::TypeCategory::Delegate => {
                        insert(types, name, Type::CppDelegate(CppDelegate { def }));
                    }
                    windows_metadata::reader::TypeCategory::Enum => {
                        insert(types, name, Type::CppEnum(CppEnum { def, name }));

                        if !def.has_attribute("ScopedEnumAttribute") {
                            for field in def.fields() {
                                if field.flags().contains(FieldAttributes::Literal) {
                                    let field_name = field.name();
                                    insert(
                                        types,
                                        field_name,
                                        Type::CppConst(CppConst { namespace, field }),
                                    );
                                }
                            }
                        }
                    }
                    windows_metadata::reader::TypeCategory::Interface => {
                        insert(types, name, Type::CppInterface(CppInterface { def }));
                    }
                    windows_metadata::reader::TypeCategory::Struct => {
                        fn make(
                            def: TypeDef,
                            name: &'static str,
                            nested: &HashMap<TypeDef, Vec<TypeDef>>,
                        ) -> CppStruct {
                            let mut ty = CppStruct {
                                def,
                                name,
                                nested: BTreeMap::new(),
                            };

                            for (index, nested_def) in
                                nested.get(&def).into_iter().flatten().enumerate()
                            {
                                if nested_def.category()
                                    == windows_metadata::reader::TypeCategory::Struct
                                {
                                    ty.nested.insert(
                                        nested_def.name(),
                                        Type::CppStruct(make(
                                            *nested_def,
                                            format!("{}_{index}", ty.name).leak(),
                                            nested,
                                        )),
                                    );
                                } else if nested_def.category()
                                    == windows_metadata::reader::TypeCategory::Enum
                                {
                                    ty.nested.insert(
                                        nested_def.name(),
                                        Type::CppEnum(CppEnum {
                                            def: *nested_def,
                                            name: format!("{}_{}", ty.name, nested_def.name())
                                                .leak(),
                                        }),
                                    );
                                }
                            }

                            ty
                        }

                        insert(types, name, Type::CppStruct(make(def, name, &nested)));
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
