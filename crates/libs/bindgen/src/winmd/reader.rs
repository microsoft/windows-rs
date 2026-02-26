use super::*;

fn insert(types: &mut HashMap<&'static str, Vec<Type>>, name: &'static str, ty: Type) {
    types.entry(name).or_default().push(ty);
}

#[repr(C)]
pub(crate) struct BindgenContext {
    pub(crate) index: windows_metadata::reader::TypeIndex, // MUST be first field
    reader: *const Reader,
}

unsafe impl Send for BindgenContext {}
unsafe impl Sync for BindgenContext {}

pub(crate) fn reader_from_index(
    index: &'static windows_metadata::reader::TypeIndex,
) -> &'static Reader {
    // Safety: BindgenContext has #[repr(C)] with TypeIndex as first field, so
    // a pointer to TypeIndex equals a pointer to BindgenContext.
    unsafe {
        let ctx = index as *const windows_metadata::reader::TypeIndex as *const BindgenContext;
        &*(*ctx).reader
    }
}

pub struct Reader {
    types: HashMap<&'static str, HashMap<&'static str, Vec<Type>>>,
    context: *mut BindgenContext,
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.types
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        unsafe {
            _ = Box::from_raw(self.context);
        }
    }
}

impl Reader {
    pub fn new(files: Vec<File>) -> Box<Self> {
        let wm_files: Vec<windows_metadata::reader::File> =
            files.into_iter().map(|f| f.0).collect();

        let ctx_ptr = Box::into_raw(Box::new(BindgenContext {
            index: windows_metadata::reader::TypeIndex::new(wm_files),
            reader: std::ptr::null(),
        }));

        let mut reader = Box::new(Self {
            types: HashMap::new(),
            context: ctx_ptr,
        });

        // Safety: ctx_ptr is valid (heap-allocated, not yet moved), and reader is on the heap.
        // We set the back-pointer before any table iteration.
        unsafe {
            (*ctx_ptr).reader = &*reader;
        }

        // Safety: ctx_ptr is valid and the TypeIndex inside will remain stable for the
        // lifetime of the Reader (via the Box on the heap).
        let static_index: &'static windows_metadata::reader::TypeIndex =
            unsafe { &(*ctx_ptr).index };

        for (_, _, def) in static_index.iter() {
            let flags = def.flags();
            let type_name = def.type_name();

            if Type::remap(type_name) != Remap::None {
                continue;
            }

            let types = reader.types.entry(type_name.namespace()).or_default();
            let category = def.category();

            if flags.contains(TypeAttributes::WindowsRuntime) {
                let ty = match category {
                    TypeCategory::Attribute => continue,
                    TypeCategory::Class => Type::Class(Class { def }),
                    TypeCategory::Delegate => Type::Delegate(Delegate {
                        def,
                        generics: def.generics(),
                    }),
                    TypeCategory::Enum => Type::Enum(Enum { def }),
                    TypeCategory::Interface => Type::Interface(Interface {
                        def,
                        generics: def.generics(),
                        kind: InterfaceKind::None,
                    }),
                    TypeCategory::Struct => {
                        if def.has_attribute("ApiContractAttribute") {
                            continue;
                        }
                        Type::Struct(Struct { def })
                    }
                };

                insert(types, type_name.name(), ty);
            } else {
                match category {
                    TypeCategory::Attribute => continue,
                    TypeCategory::Class => {
                        if type_name.name() == "Apis" {
                            for method in def.methods() {
                                if let Some(map) = method.impl_map() {
                                    if map.scope().name() == "FORCEINLINE"
                                        || map.import_name().starts_with("#")
                                    {
                                        continue;
                                    }
                                }

                                let name = method.name();
                                insert(
                                    types,
                                    name,
                                    Type::CppFn(CppFn {
                                        namespace: type_name.namespace(),
                                        method,
                                    }),
                                );
                            }

                            for field in def.fields() {
                                let name = field.name();
                                insert(
                                    types,
                                    name,
                                    Type::CppConst(CppConst {
                                        namespace: type_name.namespace(),
                                        field,
                                    }),
                                );
                            }
                        }
                    }
                    TypeCategory::Delegate => {
                        insert(
                            types,
                            type_name.name(),
                            Type::CppDelegate(CppDelegate { def }),
                        );
                    }
                    TypeCategory::Enum => {
                        insert(types, type_name.name(), Type::CppEnum(CppEnum { def }));

                        if !def.has_attribute("ScopedEnumAttribute") {
                            for field in def.fields() {
                                if field.flags().contains(FieldAttributes::Literal) {
                                    let name = field.name();
                                    insert(
                                        types,
                                        name,
                                        Type::CppConst(CppConst {
                                            namespace: type_name.namespace(),
                                            field,
                                        }),
                                    );
                                }
                            }
                        }
                    }
                    TypeCategory::Interface => {
                        insert(
                            types,
                            type_name.name(),
                            Type::CppInterface(CppInterface { def }),
                        );
                    }
                    TypeCategory::Struct => {
                        fn make(
                            def: TypeDef,
                            name: &'static str,
                            index: &'static windows_metadata::reader::TypeIndex,
                        ) -> CppStruct {
                            let mut ty = CppStruct {
                                def,
                                name,
                                nested: BTreeMap::new(),
                            };

                            for (nested_idx, nested_def) in
                                index.nested(def).enumerate()
                            {
                                if nested_def.category() == TypeCategory::Struct {
                                    ty.nested.insert(
                                        nested_def.name(),
                                        make(
                                            nested_def,
                                            format!("{}_{nested_idx}", ty.name).leak(),
                                            index,
                                        ),
                                    );
                                }
                            }

                            ty
                        }

                        insert(
                            types,
                            type_name.name(),
                            Type::CppStruct(make(def, type_name.name(), static_index)),
                        );
                    }
                };
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
    pub fn with_full_name(&self, namespace: &str, name: &str) -> impl Iterator<Item = Type> + '_ {
        self.types
            .get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .cloned()
    }
}

