use super::*;
use std::sync::*;
use windows_metadata::reader::{Item, TypeCategory};

fn resolve(name: TypeName, item: Item<'static>, legacy: bool) -> Type {
    match item {
        Item::Type(def) => {
            if def.flags().contains(TypeAttributes::WindowsRuntime) {
                match def.category() {
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
                    TypeCategory::Struct => Type::Struct(Struct { def }),
                    TypeCategory::Attribute => unreachable!(),
                }
            } else {
                match def.category() {
                    TypeCategory::Delegate => Type::CppDelegate(CppDelegate { def }),
                    TypeCategory::Enum => Type::CppEnum(CppEnum { def }),
                    TypeCategory::Interface => Type::CppInterface(CppInterface { def }),
                    TypeCategory::Struct => {
                        Type::CppStruct(make_struct(def, name.name(), None, legacy))
                    }
                    _ => unreachable!(),
                }
            }
        }
        Item::Fn(method) => Type::CppFn(CppFn {
            namespace: name.namespace(),
            method,
        }),
        Item::Const(field) => Type::CppConst(CppConst {
            namespace: name.namespace(),
            field,
            arches: field.arches(),
            is_enum_member: false,
        }),
    }
}

/// Legacy metadata names nested structs by their position among all nested types, as the
/// `windows` 0.62 releases did.
fn make_struct(
    def: TypeDef,
    name: &'static str,
    enclosing: Option<TypeName>,
    legacy: bool,
) -> CppStruct {
    let index = def.index();
    let mut ty = CppStruct {
        def,
        name,
        enclosing,
        nested: BTreeMap::new(),
    };
    let mut anonymous = BTreeMap::new();
    for child in index.nested(def) {
        if let Some(position) = child.anonymous_index().unwrap() {
            assert!(
                anonymous.insert(position, child).is_none(),
                "duplicate anonymous field position in `{}`",
                def.name()
            );
        }
    }
    let anonymous: HashMap<_, _> = anonymous
        .into_values()
        .enumerate()
        .map(|(index, child)| (child, index))
        .collect();

    for (position, nested_def) in index.nested(def).enumerate() {
        if nested_def.category() == TypeCategory::Struct {
            let name = if legacy {
                format!("{}_{position}", ty.name)
            } else {
                match anonymous.get(&nested_def) {
                    Some(index) => format!("{}_{index}", ty.name),
                    None => format!("{}_{}", ty.name, nested_def.name()),
                }
            };
            ty.nested.insert(
                nested_def.name(),
                make_struct(
                    nested_def,
                    name.leak(),
                    Some(enclosing.unwrap_or(ty.type_name())),
                    legacy,
                ),
            );
        }
    }
    ty
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
    for (name, item) in index.namespace_items("Windows.Foundation.Numerics") {
        if let Item::Type(def) = item
            && def.category() == TypeCategory::Struct
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

type CachedTypes = Arc<OnceLock<Arc<[Type]>>>;

pub struct Reader {
    index: &'static windows_metadata::reader::Index,
    namespaces: HashSet<&'static str>,
    enum_members: HashMap<&'static str, HashMap<&'static str, Vec<(Field, i32)>>>,
    cache: RwLock<HashMap<TypeName, CachedTypes>>,
    project_numerics: bool,
    legacy: bool,
}

impl Reader {
    pub fn new(files: Vec<File>) -> Self {
        Self::from_index(windows_metadata::reader::Index::new(files).leak(), true)
    }

    pub fn new_sys(files: Vec<File>) -> Self {
        Self::from_index(windows_metadata::reader::Index::new(files).leak(), false)
    }

    /// Package generation also interprets the Win32 metadata vocabulary
    /// (`Windows.Win32.Foundation.Metadata`) used by the `windows` 0.62 releases, when the inputs
    /// use it.
    pub fn new_package(files: Vec<File>, sys: bool) -> Self {
        let index = windows_metadata::reader::Index::new(files).leak();
        let legacy = index
            .get_item(
                "Windows.Win32.Foundation.Metadata",
                "NativeTypedefAttribute",
            )
            .next()
            .is_some();
        Self::build(index, !sys, legacy)
    }

    pub fn from_index(
        index: &'static windows_metadata::reader::Index,
        project_numerics: bool,
    ) -> Self {
        Self::build(index, project_numerics, false)
    }

    fn build(
        index: &'static windows_metadata::reader::Index,
        project_numerics: bool,
        legacy: bool,
    ) -> Self {
        let project_numerics = project_numerics && has_numerics(index);

        let mut reader = Self {
            index,
            namespaces: HashSet::new(),
            enum_members: HashMap::new(),
            cache: RwLock::new(HashMap::new()),
            project_numerics,
            legacy,
        };

        for namespace in index.namespaces() {
            if index
                .namespace_items(namespace)
                .any(|(name, item)| match item {
                    Item::Type(_) => reader.remap(namespace, name) == Remap::None,
                    _ => reader.item_name(item).is_some(),
                })
            {
                reader.namespaces.insert(namespace);
            }
        }

        for (namespace, name, def) in index.iter() {
            if reader.remap(namespace, name) != Remap::None
                || def.flags().contains(TypeAttributes::WindowsRuntime)
                || def.category() != TypeCategory::Enum
                || def.has_attribute("ScopedEnumAttribute")
            {
                continue;
            }
            let arches = def.arches();
            let members = reader.enum_members.entry(namespace).or_default();
            for field in def.fields() {
                if field.flags().contains(FieldAttributes::Literal) {
                    members
                        .entry(field.name())
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push((
                            field,
                            match field.arches() {
                                0 => arches,
                                arches => arches,
                            },
                        ));
                }
            }
        }

        reader
    }

    /// Whether the inputs use the legacy Win32 metadata vocabulary, projected as in the `windows`
    /// 0.62 releases. See [`Reader::new_package`].
    pub fn legacy(&self) -> bool {
        self.legacy
    }

    /// Maps metadata names to core types or other names; see [`Type::remap`]. Legacy metadata
    /// follows the `windows` 0.62 mapping: the `BOOLEAN` and `CHAR` typedefs are Rust primitives
    /// (`BOOLEAN` is `u8` rather than `bool`, which has a different ABI),
    /// `D2D_VECTOR_3F` is not a numerics type, and the status types stay at their metadata paths
    /// (see [`Reader::legacy_core_type`]).
    pub fn remap(&self, namespace: &str, name: &str) -> Remap {
        if self.legacy {
            match (namespace, name) {
                ("Windows.Win32.Foundation", "BOOLEAN") => return Remap::Type(Type::U8),
                ("Windows.Win32.Foundation", "CHAR") => return Remap::Type(Type::I8),
                ("Windows.Win32.Foundation", "NTSTATUS")
                | ("Windows.Win32.System.Rpc", "RPC_STATUS")
                | ("Windows.Win32.Graphics.Direct2D.Common", "D2D_VECTOR_3F") => {
                    return Remap::None;
                }
                _ => {}
            }
        }
        Type::remap(namespace, name, self.project_numerics)
    }

    /// Returns the core type that a legacy metadata status definition re-exports, so the
    /// `windows` 0.62 paths name the same type as `windows-core`.
    pub fn legacy_core_type(&self, def: TypeDef) -> Option<Type> {
        if !self.legacy {
            return None;
        }
        match (def.namespace(), def.name()) {
            ("Windows.Win32.Foundation", "NTSTATUS") => Some(Type::NTSTATUS),
            ("Windows.Win32.System.Rpc", "RPC_STATUS") => Some(Type::RPC_STATUS),
            _ => None,
        }
    }

    fn item_name(&self, item: Item<'static>) -> Option<&'static str> {
        match item {
            Item::Type(def) => {
                let name = windows_metadata::trim_tick(def.name());
                if self.remap(def.namespace(), name) != Remap::None {
                    return None;
                }
                let category = def.category();
                if category == TypeCategory::Attribute
                    || (!def.flags().contains(TypeAttributes::WindowsRuntime)
                        && category == TypeCategory::Class)
                    || (def.flags().contains(TypeAttributes::WindowsRuntime)
                        && category == TypeCategory::Struct
                        && def.has_attribute("ApiContractAttribute"))
                {
                    return None;
                }
                Some(name)
            }
            Item::Fn(method) => {
                if let Some(map) = method.impl_map() {
                    // Legacy packages also omit functions imported by ordinal.
                    if map.import_scope().name() == "FORCEINLINE"
                        || (self.legacy && map.import_name().starts_with('#'))
                    {
                        return None;
                    }
                }
                Some(method.name())
            }
            Item::Const(field) => Some(field.name()),
        }
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.namespaces.iter().copied()
    }

    pub fn contains_namespace(&self, namespace: &str) -> bool {
        self.namespaces.contains(namespace)
    }

    pub fn find_name(&self, namespace: &str, name: &str) -> Option<TypeName> {
        let namespace = *self.namespaces.get(namespace)?;
        if let Some(name) = self
            .index
            .get_item(namespace, name)
            .find_map(|item| self.item_name(item))
        {
            return Some(TypeName(namespace, name));
        }
        self.enum_members
            .get(namespace)?
            .get_key_value(name)
            .map(|(name, _)| TypeName(namespace, name))
    }

    pub fn contains_name(&self, namespace: &str, name: &str) -> bool {
        self.find_name(namespace, name).is_some()
    }

    pub fn type_names(&self, namespace: &str) -> impl Iterator<Item = TypeName> {
        let mut names = HashSet::new();
        if let Some(namespace) = self.namespaces.get(namespace) {
            for (_, item) in self.index.namespace_items(namespace) {
                if let Some(name) = self.item_name(item) {
                    names.insert(TypeName(namespace, name));
                }
            }
            if let Some(members) = self.enum_members.get(namespace) {
                names.extend(members.keys().map(|name| TypeName(namespace, name)));
            }
        }
        names.into_iter()
    }

    pub fn types(&self, name: TypeName) -> Arc<[Type]> {
        let cached = self.cache.read().unwrap().get(&name).cloned();
        let cached =
            cached.unwrap_or_else(|| self.cache.write().unwrap().entry(name).or_default().clone());
        // Decode outside the map lock so failures do not poison unrelated lookups.
        cached
            .get_or_init(|| {
                let mut types = Vec::with_capacity(1);
                types.extend(
                    self.index
                        .get_item(name.namespace(), name.name())
                        .filter(|item| self.item_name(*item).is_some())
                        .map(|item| resolve(name, item, self.legacy)),
                );
                if let Some(members) = self
                    .enum_members
                    .get(name.namespace())
                    .and_then(|members| members.get(name.name()))
                {
                    types.extend(members.iter().map(|(field, arches)| {
                        Type::CppConst(CppConst {
                            namespace: name.namespace(),
                            field: *field,
                            arches: *arches,
                            is_enum_member: true,
                        })
                    }));
                }
                // Explicit constants shadow unscoped enum members only on overlapping targets.
                let explicit_arches = types.iter().fold(0, |arches, ty| {
                    if let Type::CppConst(constant) = ty
                        && !constant.is_enum_member
                    {
                        arches | arch_mask(constant.arches)
                    } else {
                        arches
                    }
                });
                if explicit_arches != 0 {
                    types.retain_mut(|ty| {
                        if let Type::CppConst(constant) = ty
                            && constant.is_enum_member
                        {
                            constant.arches = arch_mask(constant.arches) & !explicit_arches;
                            return constant.arches != 0;
                        }
                        true
                    });
                }
                types.into()
            })
            .clone()
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
    pub fn with_full_name<'a>(
        &'a self,
        namespace: &str,
        name: &str,
    ) -> impl Iterator<Item = Type> + use<'a> {
        let (name, nested) = name.split_once('/').map_or((name, None), |(root, nested)| {
            (root, Some(nested.to_string()))
        });
        let name = windows_metadata::trim_tick(name);
        let types = self
            .find_name(namespace, name)
            .map(|name| self.types(name))
            .unwrap_or_default();
        (0..types.len()).filter_map(move |index| {
            let mut ty = types[index].clone();
            if let Some(nested) = &nested {
                for name in nested.split('/') {
                    let Type::CppStruct(outer) = ty else {
                        return None;
                    };
                    ty = outer.nested_type(name)?;
                }
            }
            Some(ty)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_metadata::writer;

    fn constants() -> &'static Reader {
        static READER: OnceLock<Reader> = OnceLock::new();
        READER.get_or_init(|| {
            let mut file = writer::File::new("reader_resolution");
            let base = file.TypeRef("System", "Object");
            file.TypeDef(
                "Test",
                "Apis",
                writer::TypeDefOrRef::TypeRef(base),
                TypeAttributes::Public,
            );
            let attribute =
                file.TypeRef("Windows.Win32.Metadata", "SupportedArchitectureAttribute");
            let constructor = file.MemberRef(
                ".ctor",
                &windows_metadata::Signature {
                    flags: MethodCallAttributes::HASTHIS,
                    types: vec![windows_metadata::Type::I32],
                    ..Default::default()
                },
                writer::MemberRefParent::TypeRef(attribute),
            );
            for (name, value) in [
                ("Valid", Value::I32(2)),
                ("Concurrent", Value::I32(2)),
                ("Malformed", Value::U16(2)),
            ] {
                let field = file.Field(
                    name,
                    &windows_metadata::Type::I32,
                    FieldAttributes::Public | FieldAttributes::Static | FieldAttributes::Literal,
                );
                file.Constant(writer::HasConstant::Field(field), &Value::I32(0));
                // The malformed value is shorter than the constructor's i32 argument.
                file.Attribute(
                    writer::HasAttribute::Field(field),
                    writer::AttributeType::MemberRef(constructor),
                    &[(String::new(), value)],
                );
            }
            Reader::new(vec![File::new(file.into_stream()).unwrap()])
        })
    }

    #[test]
    fn resolution_is_cached_and_failures_do_not_poison_the_reader() {
        let reader = constants();
        assert!(reader.contains_name("Test", "Malformed"));
        for _ in 0..2 {
            assert!(
                std::panic::catch_unwind(|| reader.unwrap_full_name("Test", "Malformed")).is_err()
            );
            assert_eq!(reader.unwrap_full_name("Test", "Valid").arches(), 2);
        }
        assert!(Arc::ptr_eq(
            &reader.types(TypeName("Test", "Valid")),
            &reader.types(TypeName("Test", "Valid")),
        ));
    }

    #[test]
    fn concurrent_first_lookup_shares_the_resolution() {
        let reader = constants();
        let name = TypeName("Test", "Concurrent");
        assert!(!reader.cache.read().unwrap().contains_key(&name));
        let barrier = Barrier::new(4);
        let results = std::thread::scope(|scope| {
            let threads: Vec<_> = (0..4)
                .map(|_| {
                    scope.spawn(|| {
                        barrier.wait();
                        reader.types(name)
                    })
                })
                .collect();
            threads
                .into_iter()
                .map(|thread| thread.join().unwrap())
                .collect::<Vec<_>>()
        });
        for result in &results {
            assert!(Arc::ptr_eq(&results[0], result));
            assert_eq!(result[0].arches(), 2);
        }
    }

    #[test]
    fn malformed_nested_record_fails_when_reached_as_a_dependency() {
        static READER: OnceLock<Reader> = OnceLock::new();
        let reader = READER.get_or_init(|| {
            let mut file = writer::File::new("nested_dependency");
            let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
            let flags = TypeAttributes::Public | TypeAttributes::SequentialLayout;
            file.TypeDef("Test", "Valid", base, flags);
            file.Field(
                "value",
                &windows_metadata::Type::I32,
                FieldAttributes::Public,
            );
            file.TypeDef("Test", "Consumer", base, flags);
            file.Field(
                "value",
                &windows_metadata::Type::value_named("Test", "Broken"),
                FieldAttributes::Public,
            );
            let broken = file.TypeDef("Test", "Broken", base, flags);
            file.Field(
                "value",
                &windows_metadata::Type::I32,
                FieldAttributes::Public,
            );
            let attribute = file.TypeRef("Windows.Win32.Metadata", "NativeAnonymousAttribute");
            let constructor = file.MemberRef(
                ".ctor",
                &windows_metadata::Signature {
                    flags: MethodCallAttributes::HASTHIS,
                    types: vec![windows_metadata::Type::I32],
                    ..Default::default()
                },
                writer::MemberRefParent::TypeRef(attribute),
            );
            for name in ["First", "Second"] {
                let child = file.TypeDef(
                    "",
                    name,
                    base,
                    TypeAttributes::NestedPublic | TypeAttributes::SequentialLayout,
                );
                file.NestedClass(child, broken);
                file.Attribute(
                    writer::HasAttribute::TypeDef(child),
                    writer::AttributeType::MemberRef(constructor),
                    &[(String::new(), Value::I32(0))],
                );
            }
            Reader::new(vec![File::new(file.into_stream()).unwrap()])
        });
        reader
            .unwrap_type_name("Test", "Valid")
            .dependencies(reader);
        let consumer = reader.unwrap_type_name("Test", "Consumer");
        let error = std::panic::catch_unwind(|| consumer.dependencies(reader)).unwrap_err();
        assert_eq!(
            error.downcast_ref::<String>().unwrap(),
            "duplicate anonymous field position in `Broken`"
        );
        reader
            .unwrap_type_name("Test", "Valid")
            .dependencies(reader);
    }
}
