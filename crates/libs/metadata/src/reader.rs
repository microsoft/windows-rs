use super::*;

#[derive(Clone)]
pub enum Item {
    Type(TypeDef),
    Const(Field),
    // TODO: get rid of the trailing String - that's just a hack to get around a silly Win32 metadata deficiency where parsing method signatures
    // requires knowing which namespace the method's surrounding interface was defined in.
    Fn(MethodDef, &'static str),
}

pub struct Reader {
    // TODO: get rid of inner Vec - that's just a hack to support multi-arch structs in Win32 metadata.
    items: BTreeMap<&'static str, BTreeMap<&'static str, Vec<Item>>>,

    // TODO: riddle should just avoid nested structs
    nested: HashMap<TypeDef, BTreeMap<&'static str, TypeDef>>,

    // The reader needs to store the filter since standalone code generation needs more than just the filtered items
    // in order to chase dependencies automatically. This is why `Reader::filter` can't just filter everything up front.
    filter: Filter,

    sys: bool,
}

impl Reader {
    pub fn new(files: Vec<File>) -> &'static Self {
        let mut config = BTreeMap::new();
        config.insert("sys", "");
        Self::filter(files, &[], &[], &config)
    }

    pub fn filter(files: Vec<File>, include: &[&str], exclude: &[&str], config: &BTreeMap<&str, &str>) -> &'static Self {
        let reader: &'static mut Reader = Box::leak(Box::new(Self { items: Default::default(), nested: Default::default(), filter: Filter::new(include, exclude), sys: config.contains_key("sys") }));

        for mut file in files {
            file.reader = reader as *mut Reader;
            let file = Box::leak(Box::new(file));

            for def in file.table::<TypeDef>() {
                let namespace = def.namespace();

                if namespace.is_empty() {
                    continue;
                }

                let namespace_items = reader.items.entry(namespace).or_default();
                let name = def.name();

                if name == "Apis" {
                    for method in def.methods() {
                        namespace_items.entry(method.name()).or_default().push(Item::Fn(method, namespace));
                    }

                    for field in def.fields() {
                        namespace_items.entry(field.name()).or_default().push(Item::Const(field));
                    }
                } else {
                    namespace_items.entry(name).or_default().push(Item::Type(def));

                    // TODO: these should all be fields on the Apis class so we don't have to go looking for all of these as well.
                    if def.extends() == Some(TypeName::Enum) && !def.flags().contains(TypeAttributes::WindowsRuntime) && !def.has_attribute("ScopedEnumAttribute") {
                        for field in def.fields().filter(|field| field.flags().contains(FieldAttributes::Literal)) {
                            namespace_items.entry(field.name()).or_default().push(Item::Const(field));
                        }
                    }
                }
            }

            for key in file.table::<NestedClass>() {
                let inner = key.inner();
                reader.nested.entry(key.outer()).or_default().insert(inner.name(), inner);
            }
        }

        reader
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        self.filter.includes_namespace(namespace)
    }

    pub fn namespaces(&self) -> impl Iterator<Item = &str> + '_ {
        self.items.keys().copied()
    }

    pub fn items(&self) -> impl Iterator<Item = Item> + '_ {
        self.items.iter().filter(move |(namespace, _)| self.filter.includes_namespace(namespace)).flat_map(move |(namespace, items)| items.iter().filter(move |(name, _)| self.filter.includes_type_name(namespace, name))).flat_map(move |(_, items)| items).cloned()
    }

    pub fn namespace_items(&self, namespace: &str) -> impl Iterator<Item = Item> + '_ {
        self.items.get_key_value(namespace).into_iter().flat_map(move |(namespace, items)| items.iter().filter(move |(name, _)| self.filter.includes_type_name(namespace, name))).flat_map(move |(_, items)| items).cloned()
    }

    pub fn unused(&self) -> impl Iterator<Item = &str> + '_ {
        self.filter.0.iter().filter_map(|(name, _)| if self.is_unused(name) { Some(name.as_str()) } else { None })
    }

    fn is_unused(&self, filter: &str) -> bool {
        // Match namespaces
        if self.items.contains_key(filter) {
            return false;
        }

        // Match type names
        if let Some((namespace, name)) = filter.rsplit_once('.') {
            if self.items.get(namespace).is_some_and(|items| items.contains_key(name)) {
                return false;
            }
        }

        // Match empty parent namespaces
        for namespace in self.items.keys() {
            if namespace.len() > filter.len() && namespace.starts_with(filter) && namespace.as_bytes()[filter.len()] == b'.' {
                return false;
            }
        }

        true
    }

    fn get_item(&self, namespace: &str, name: &str) -> impl Iterator<Item = Item> + '_ {
        if let Some(items) = self.items.get(namespace) {
            if let Some(items) = items.get(name) {
                return Some(items.iter().cloned()).into_iter().flatten();
            }
        }
        None.into_iter().flatten()
    }

    pub fn get_type_def(&self, namespace: &str, name: &str) -> impl Iterator<Item = TypeDef> + '_ {
        self.get_item(namespace, name).filter_map(|item| if let Item::Type(def) = item { Some(def) } else { None })
    }

    pub fn get_method_def(&self, namespace: &str, name: &str) -> impl Iterator<Item = (MethodDef, &'static str)> + '_ {
        self.get_item(namespace, name).filter_map(|item| if let Item::Fn(def, namespace) = item { Some((def, namespace)) } else { None })
    }

    pub fn nested_types(&self, type_def: TypeDef) -> impl Iterator<Item = TypeDef> + '_ {
        self.nested.get(&type_def).map(|map| map.values().copied()).into_iter().flatten()
    }

    pub fn remap_types(&self) -> impl Iterator<Item = &(TypeName, TypeName)> + '_ {
        if self.sys {
            [].iter()
        } else {
            REMAP_TYPES.iter()
        }
    }

    pub fn core_types(&self) -> impl Iterator<Item = &(TypeName, Type)> + '_ {
        if self.sys {
            SYS_CORE_TYPES.iter()
        } else {
            CORE_TYPES.iter()
        }
    }

    pub fn type_from_ref(&self, code: TypeDefOrRef, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob(0);
            return self.type_from_blob_impl(&mut blob, None, generics);
        }

        let mut full_name = code.type_name();

        for (known_name, kind) in self.core_types() {
            if full_name == *known_name {
                return kind.clone();
            }
        }

        for (from, to) in self.remap_types() {
            if full_name == *from {
                full_name = *to;
                break;
            }
        }

        if let Some(outer) = enclosing {
            if full_name.namespace.is_empty() {
                let nested = &self.nested[&outer];
                let Some(inner) = nested.get(full_name.name) else {
                    panic!("Nested type not found: {}.{}", outer.type_name(), full_name.name);
                };
                return Type::TypeDef(*inner, Vec::new());
            }
        }

        if let Some(def) = self.get_type_def(full_name.namespace, full_name.name).next() {
            Type::TypeDef(def, Vec::new())
        } else {
            Type::TypeRef(full_name)
        }
    }

    pub fn type_from_blob(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        // Used by WinRT to indicate that a struct input parameter is passed by reference rather than by value on the ABI.
        let is_const = blob.read_modifiers().iter().any(|def| def.type_name() == TypeName::IsConst);

        // Used by WinRT to indicate an output parameter, but there are other ways to determine this direction so here
        // it is only used to distinguish between slices and heap-allocated arrays.
        let is_ref = blob.read_expected(ELEMENT_TYPE_BYREF as usize);

        if blob.read_expected(ELEMENT_TYPE_VOID as usize) {
            return Type::Void;
        }

        let is_array = blob.read_expected(ELEMENT_TYPE_SZARRAY as usize); // Used by WinRT to indicate an array

        let mut pointers = 0;

        while blob.read_expected(ELEMENT_TYPE_PTR as usize) {
            pointers += 1;
        }

        let kind = self.type_from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            Type::MutPtr(Box::new(kind), pointers)
        } else if is_const {
            Type::ConstRef(Box::new(kind))
        } else if is_array {
            if is_ref {
                Type::WinrtArrayRef(Box::new(kind))
            } else {
                Type::WinrtArray(Box::new(kind))
            }
        } else {
            kind
        }
    }

    fn type_from_blob_impl(&self, blob: &mut Blob, enclosing: Option<TypeDef>, generics: &[Type]) -> Type {
        let code = blob.read_usize();

        if let Some(code) = Type::from_code(code) {
            return code;
        }

        match code as u8 {
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => self.type_from_ref(TypeDefOrRef::decode(blob.file, blob.read_usize()), enclosing, generics),
            ELEMENT_TYPE_VAR => generics.get(blob.read_usize()).unwrap_or(&Type::Void).clone(),
            ELEMENT_TYPE_ARRAY => {
                let kind = self.type_from_blob(blob, enclosing, generics);
                let _rank = blob.read_usize();
                let _count = blob.read_usize();
                let bounds = blob.read_usize();
                Type::Win32Array(Box::new(kind), bounds)
            }
            ELEMENT_TYPE_GENERICINST => {
                blob.read_usize(); // ELEMENT_TYPE_VALUETYPE or ELEMENT_TYPE_CLASS

                let type_name = TypeDefOrRef::decode(blob.file, blob.read_usize()).type_name();
                let def = self.get_type_def(type_name.namespace, type_name.name).next().unwrap_or_else(|| panic!("Type not found: {}", type_name));
                let mut args = Vec::with_capacity(blob.read_usize());

                for _ in 0..args.capacity() {
                    args.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef(def, args)
            }
            rest => unimplemented!("{rest:?}"),
        }
    }
}

// TODO: this should be in riddle's Rust generator if at all - perhaps as convertible types rather than remapped types since there's already some precedent for that.
const REMAP_TYPES: [(TypeName, TypeName); 2] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2), (TypeName::D3DMATRIX, TypeName::Matrix4x4)];

// TODO: get rid of at least the second tuple if not the whole thing.
const CORE_TYPES: [(TypeName, Type); 13] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::BSTR, Type::BSTR), (TypeName::IInspectable, Type::IInspectable), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR), (TypeName::Type, Type::Type), (TypeName::CHAR, Type::I8), (TypeName::VARIANT, Type::VARIANT), (TypeName::PROPVARIANT, Type::PROPVARIANT)];
const SYS_CORE_TYPES: [(TypeName, Type); 11] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::BSTR, Type::BSTR), (TypeName::IInspectable, Type::IInspectable), (TypeName::PSTR, Type::PSTR), (TypeName::PWSTR, Type::PWSTR), (TypeName::Type, Type::Type), (TypeName::CHAR, Type::I8)];
