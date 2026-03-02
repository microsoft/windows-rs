use super::*;

// Thread-local pointer to the active Reader. Set in Reader::new and cleared in Reader::drop.
// On Windows, `for_each` dispatches work to thread-pool threads that do not inherit
// thread-locals; the `for_each` wrapper in config/mod.rs propagates this pointer to each
// worker thread via `reader_ptr` / `set_reader_ptr` before invoking user closures.
std::thread_local! {
    static CURRENT_READER: std::cell::Cell<*const Reader> = const { std::cell::Cell::new(std::ptr::null()) };
}

/// Returns the current Reader from the thread-local. Panics if not set.
pub fn current_reader() -> &'static Reader {
    CURRENT_READER.with(|r| {
        let ptr = r.get();
        assert!(!ptr.is_null(), "Reader thread-local not set");
        unsafe { &*ptr }
    })
}

/// Returns the raw reader pointer for the calling thread (may be null).
/// Used by the `for_each` wrapper to propagate the reader to worker threads.
pub(crate) fn reader_ptr() -> *const Reader {
    CURRENT_READER.with(|r| r.get())
}

/// Sets the reader pointer on the calling thread.
///
/// # Safety
/// `ptr` must point to a valid `Reader` that outlives all subsequent calls to
/// `current_reader()` on this thread.
pub(crate) unsafe fn set_reader_ptr(ptr: *const Reader) {
    CURRENT_READER.with(|r| r.set(ptr));
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
        for (_, _, def) in index_ref.iter() {
            collect_nested(index_ref, def, &mut nested);
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
