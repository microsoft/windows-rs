use super::*;

// Thread-local pointer to the active Reader. Set in Reader::new and cleared in Reader::drop.
std::thread_local! {
    static CURRENT_READER: std::cell::Cell<*const Reader> = std::cell::Cell::new(std::ptr::null());
}

/// Returns the current Reader from the thread-local. Panics if not set.
pub fn current_reader() -> &'static Reader {
    CURRENT_READER.with(|r| {
        let ptr = r.get();
        assert!(!ptr.is_null(), "Reader thread-local not set");
        unsafe { &*ptr }
    })
}

fn insert(types: &mut HashMap<&'static str, Vec<Type>>, name: &'static str, ty: Type) {
    types.entry(name).or_default().push(ty);
}

pub struct Reader {
    map: HashMap<&'static str, HashMap<&'static str, Vec<Type>>>,
    index: *mut windows_metadata::reader::TypeIndex,
}

// Safety: The `index` raw pointer is owned exclusively by this Reader (created via
// Box::into_raw and freed in Drop). The `map` contains only 'static data derived from
// the heap-pinned TypeIndex. There is no shared mutable state.
unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        CURRENT_READER.with(|r| r.set(std::ptr::null()));
        unsafe {
            _ = Box::from_raw(self.index);
        }
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> Box<Self> {
        let index = Box::new(windows_metadata::reader::TypeIndex::new(files));
        let index_ptr: *mut windows_metadata::reader::TypeIndex = Box::into_raw(index);

        // Safety: TypeIndex is heap-allocated and never moved. We extend its lifetime to 'static
        // so that TypeDef<'static>, Field<'static>, etc. can be stored in the map.
        let index_ref: &'static windows_metadata::reader::TypeIndex = unsafe { &*index_ptr };

        let mut reader = Box::new(Self {
            map: HashMap::new(),
            index: index_ptr,
        });

        // Build a nested-class map: outer TypeDef -> Vec<inner TypeDef>
        let mut nested: HashMap<TypeDef, Vec<TypeDef>> = HashMap::new();
        for (_, _, def) in index_ref.iter() {
            for inner in index_ref.nested(def) {
                nested.entry(def).or_default().push(inner);
            }
        }

        for (namespace, name, def) in index_ref.iter() {
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
                        insert(types, name, Type::CppEnum(CppEnum { def }));

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
                                        make(
                                            *nested_def,
                                            format!("{}_{index}", ty.name).leak(),
                                            nested,
                                        ),
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

        // Set the thread-local after the map is fully built.
        let reader_ptr: *const Reader = &*reader;
        CURRENT_READER.with(|r| r.set(reader_ptr));

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
    pub fn with_full_name(&self, namespace: &str, name: &str) -> impl Iterator<Item = Type> + '_ {
        self.map
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .cloned()
    }
}
