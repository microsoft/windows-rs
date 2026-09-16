use super::*;

fn insert(types: &mut HashMap<&'static str, Vec<Type>>, name: &'static str, ty: Type) {
    types.entry(name).or_default().push(ty);
}

fn has_numerics(index: &windows_metadata::reader::Index) -> bool {
    const TYPES: [(&str, usize); 5] = [
        ("Matrix3x2", 6),
        ("Matrix4x4", 16),
        ("Vector2", 2),
        ("Vector3", 3),
        ("Vector4", 4),
    ];
    let mut found = [false; TYPES.len()];
    for (namespace, name, item) in index.iter_items() {
        if namespace == "Windows.Foundation.Numerics"
            && let windows_metadata::reader::Item::Type(def) = item
            && def.category() == windows_metadata::reader::TypeCategory::Struct
            && let Some(index) = TYPES.iter().position(|candidate| candidate.0 == name)
            && def.fields().len() == TYPES[index].1
            && def
                .fields()
                .all(|field| field.ty() == windows_metadata::Type::F32)
        {
            found[index] = true;
            if found.into_iter().all(|found| found) {
                return true;
            }
        }
    }
    false
}

pub struct Reader {
    map: HashMap<&'static str, HashMap<&'static str, Vec<Type>>>,
    project_numerics: bool,
}

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> Self {
        Self::from_index(windows_metadata::reader::Index::new(files).leak(), true)
    }

    pub fn new_sys(files: Vec<File>) -> Self {
        Self::from_index(windows_metadata::reader::Index::new(files).leak(), false)
    }

    pub fn from_index(
        index: &'static windows_metadata::reader::Index,
        project_numerics: bool,
    ) -> Self {
        let project_numerics = project_numerics && has_numerics(index);

        let mut reader = Self {
            map: HashMap::new(),
            project_numerics,
        };

        for (namespace, name, item) in index.iter_items() {
            match item {
                windows_metadata::reader::Item::Type(def) => {
                    if Type::remap(namespace, name, project_numerics) != Remap::None {
                        continue;
                    }

                    let flags = def.flags();
                    let category = def.category();
                    let types = reader.map.entry(namespace).or_default();

                    if flags.contains(TypeAttributes::WindowsRuntime) {
                        let ty = match category {
                            windows_metadata::reader::TypeCategory::Attribute => continue,
                            windows_metadata::reader::TypeCategory::Class => {
                                Type::Class(Class { def })
                            }
                            windows_metadata::reader::TypeCategory::Delegate => {
                                Type::Delegate(Delegate {
                                    def,
                                    generics: def.generics(),
                                })
                            }
                            windows_metadata::reader::TypeCategory::Enum => {
                                Type::Enum(Enum { def })
                            }
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
                            // Non-WinRT classes (other than `Apis`, already expanded into
                            // `Item::Fn`/`Item::Const` by the Index) do not contribute types.
                            windows_metadata::reader::TypeCategory::Class => {}
                            windows_metadata::reader::TypeCategory::Delegate => {
                                insert(types, name, Type::CppDelegate(CppDelegate { def }));
                            }
                            windows_metadata::reader::TypeCategory::Enum => {
                                insert(types, name, Type::CppEnum(CppEnum { def }));

                                if !def.has_attribute("ScopedEnumAttribute") {
                                    let enum_arches = def.arches();
                                    for field in def.fields() {
                                        if field.flags().contains(FieldAttributes::Literal) {
                                            let field_name = field.name();
                                            insert(
                                                types,
                                                field_name,
                                                Type::CppConst(CppConst {
                                                    namespace,
                                                    field,
                                                    enum_arches,
                                                    is_enum_member: true,
                                                }),
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
                                    index: &'static windows_metadata::reader::Index,
                                    def: TypeDef,
                                    name: &'static str,
                                ) -> CppStruct {
                                    let mut ty = CppStruct {
                                        def,
                                        name,
                                        nested: BTreeMap::new(),
                                    };

                                    for (idx, nested_def) in index.nested(def).enumerate() {
                                        if nested_def.category()
                                            == windows_metadata::reader::TypeCategory::Struct
                                        {
                                            ty.nested.insert(
                                                nested_def.name(),
                                                make(
                                                    index,
                                                    nested_def,
                                                    format!("{}_{idx}", ty.name).leak(),
                                                ),
                                            );
                                        }
                                    }

                                    ty
                                }

                                insert(types, name, Type::CppStruct(make(index, def, name)));
                            }
                        }
                    }
                }
                windows_metadata::reader::Item::Fn(method) => {
                    if let Some(map) = method.impl_map()
                        && (map.import_scope().name() == "FORCEINLINE"
                            || map.import_name().starts_with('#'))
                    {
                        continue;
                    }

                    let types = reader.map.entry(namespace).or_default();
                    insert(types, name, Type::CppFn(CppFn { namespace, method }));
                }
                windows_metadata::reader::Item::Const(field) => {
                    let types = reader.map.entry(namespace).or_default();
                    insert(
                        types,
                        name,
                        Type::CppConst(CppConst {
                            namespace,
                            field,
                            enum_arches: 0,
                            is_enum_member: false,
                        }),
                    );
                }
            }
        }

        reader
    }

    pub fn project_numerics(&self) -> bool {
        self.project_numerics
    }

    #[track_caller]
    pub fn unwrap_full_name(&self, namespace: &str, name: &str) -> Type {
        if let Some(ty) = self.with_full_name(namespace, name).next() {
            ty
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }

    #[track_caller]
    pub fn unwrap_type_name(&self, namespace: &str, name: &str) -> Type {
        if let Some(ty) = self
            .with_full_name(namespace, name)
            .find(|ty| !matches!(ty, Type::CppConst(_) | Type::CppFn(_)))
        {
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
