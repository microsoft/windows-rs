use super::*;
use std::collections::*;

pub struct TypeReader {
    // Nested types are stored in a BTreeMap to ensure a stable order. This impacts
    // the derived nested type names.
    nested: HashMap<Row, BTreeMap<&'static str, tables::TypeDef>>,

    pub types: TypeTree,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TypeRow {
    TypeDef(tables::TypeDef),
    MethodDef(tables::MethodDef),
    Field(tables::Field),
}

impl TypeRow {
    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        match self {
            Self::TypeDef(def) => def.dependencies(include),
            Self::MethodDef(def) => def.dependencies(),
            Self::Field(def) => def.dependencies(include),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::TypeDef(def) => def.name(),
            Self::MethodDef(def) => def.name(),
            Self::Field(def) => def.name(),
        }
    }

    pub fn namespace(&self) -> &str {
        match self {
            Self::TypeDef(def) => def.namespace(),
            Self::MethodDef(def) => def.parent().namespace(),
            Self::Field(def) => def.parent().namespace(),
        }
    }
}

impl From<&TypeRow> for ElementType {
    fn from(from: &TypeRow) -> Self {
        match from {
            TypeRow::TypeDef(row) => row.clone().into(),
            TypeRow::MethodDef(row) => Self::MethodDef(row.clone()),
            TypeRow::Field(row) => Self::Field(row.clone()),
        }
    }
}

impl TypeReader {
    pub fn gen<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.types.gen()
    }

    pub fn get_mut() -> &'static mut Self {
        use std::{mem::MaybeUninit, sync::Once};
        static ONCE: Once = Once::new();
        static mut VALUE: MaybeUninit<TypeReader> = MaybeUninit::uninit();

        ONCE.call_once(|| {
            // This is safe because `Once` provides thread-safe one-time initialization
            unsafe { VALUE = MaybeUninit::new(Self::new()) }
        });

        // This is safe because `call_once` has already been called.
        unsafe { &mut *VALUE.as_mut_ptr() }
    }

    pub fn get() -> &'static Self {
        Self::get_mut()
    }

    /// Insert WinRT metadata at the given paths
    ///
    /// # Panics
    ///
    /// This function panics if the if the files where the windows metadata are stored cannot be read.
    fn new() -> Self {
        let files = workspace_winmds();
        let mut nested = HashMap::<Row, BTreeMap<&'static str, tables::TypeDef>>::new();
        let mut types = TypeTree::from_namespace("");
        types.include = true;

        for file in files {
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def: tables::TypeDef = Row::new(row, TableIndex::TypeDef, file).into();
                let namespace = def.namespace();
                let name = trim_tick(def.name());

                if namespace.is_empty() {
                    continue;
                }

                if is_well_known(namespace, name) {
                    continue;
                }

                let extends = def.extends();

                if extends == ("System", "Attribute") {
                    continue;
                }

                let namespace = types.insert_namespace(namespace, 0);

                if def.flags().windows_runtime() {
                    namespace.insert_type(name, TypeRow::TypeDef(def));
                } else {
                    if extends != ("System", "Object") {
                        namespace.insert_type(name, TypeRow::TypeDef(def));
                    } else {
                        for field in def.fields() {
                            let name = field.name();
                            namespace.insert_type(name, TypeRow::Field(field));
                        }

                        for method in def.methods() {
                            let name = method.name();
                            namespace.insert_type(name, TypeRow::MethodDef(method));
                        }
                    }
                }
            }

            let row_count = file.nested_class_table().row_count;

            for row in 0..row_count {
                let row = tables::NestedClass(Row::new(row, TableIndex::NestedClass, file));
                let enclosed = row.nested_type();
                let enclosing = row.enclosing_type();
                let name = enclosed.name();

                nested
                    .entry(enclosing.row.clone())
                    .or_default()
                    .insert(name, enclosed);
            }
        }

        Self {
            nested,
            types,
        }
    }

    /// Get all the namespace names that the [`TypeReader`] knows about
    pub fn namespaces(&'static self) -> Vec<&'static str> {
        //self.types.keys().copied()
        self.types.namespaces()
    }

    pub fn import_namespace(&mut self, namespace: &str) -> bool {
        // TODO: borrow hackery going on here...
        if let Some(namespace) = Self::get().types.get_namespace(namespace) {
            for name in namespace.types.keys() {
                self.import_type_include(namespace.namespace, name, TypeInclude::Full);
            }

            true
        } else {
            false
        }
    }

    pub fn get_type_name(
        &'static self,
        namespace: &str,
        name: &str,
    ) -> Option<(&'static str, &'static str)> {
        if let Some(tree) = self.types.get_namespace(namespace) {
            if let Some((key, _)) = tree.types.get_key_value(name) {
                return Some((tree.namespace, key));
            }
        }

        None
    }

    pub fn import_type(&mut self, namespace: &str, name: &str) -> bool {
        self.import_type_include(namespace, name, TypeInclude::Full)
    }

    fn import_type_dependencies(&mut self, def: &TypeRow, include: TypeInclude) {
        for entry in def.dependencies(include) {
            let namespace = entry.def.namespace();

            // If def.namespace is empty it means its a nested type and we need to find its dependencies to avoid type slicing.
            if namespace.is_empty() {
                self.import_type_dependencies(&entry.def, TypeInclude::Minimal);
            } else {
                self.import_type_include(namespace, trim_tick(entry.def.name()), entry.include);
            }
        }
    }

    fn import_type_include(&mut self, namespace: &str, name: &str, include: TypeInclude) -> bool {
        assert!(!namespace.is_empty());
        if let Some(entry) = self
            .types
            .get_namespace_mut(namespace)
            .and_then(|tree| tree.get_type_mut(name))
        {
            let copy = entry.def.clone();

            if include == TypeInclude::Full {
                if entry.include != TypeInclude::Full {
                    entry.include = TypeInclude::Full;
                    self.import_type_dependencies(&copy, include);
                }
            } else {
                if entry.include == TypeInclude::None {
                    entry.include = TypeInclude::Minimal;
                    self.import_type_dependencies(&copy, include);
                }
            }

            true
        } else {
            false
        }
    }

    pub fn nested_types(
        &'static self,
        enclosing: &tables::TypeDef,
    ) -> Option<&BTreeMap<&'static str, tables::TypeDef>> {
        self.nested.get(&enclosing.row)
    }

    pub fn resolve_type(&'static self, namespace: &str, name: &str) -> ElementType {
        if let Some(def) = self
            .types
            .get_namespace(namespace)
            .and_then(|tree| tree.get_type(trim_tick(name)))
        {
            return (&def.def).into();
        }

        panic!("Could not find type `{}.{}`", namespace, name);
    }

    pub fn resolve_type_row(&'static self, namespace: &str, name: &str) -> TypeRow {
        if let Some(def) = self
            .types
            .get_namespace(namespace)
            .and_then(|tree| tree.get_type(trim_tick(name)))
        {
            return def.def.clone();
        }

        panic!("Could not find type row `{}.{}`", namespace, name);
    }

    pub fn resolve_type_def(&'static self, namespace: &str, name: &str) -> tables::TypeDef {
        if let Some(def) = self
            .types
            .get_namespace(namespace)
            .and_then(|tree| tree.get_type(trim_tick(name)))
        {
            if let TypeRow::TypeDef(row) = &def.def {
                return row.clone();
            }
        }

        panic!("Could not find type def `{}.{}`", namespace, name);
    }

    pub fn resolve_type_ref(&'static self, type_ref: &tables::TypeRef) -> tables::TypeDef {
        if let ResolutionScope::TypeRef(scope) = type_ref.scope() {
            self.nested[&scope.resolve().row]
                .get(type_ref.name())
                .unwrap_or_else(|| {
                    panic!(
                        "Could not find nested type `{}` in `{}.{}`",
                        type_ref.name(),
                        scope.namespace(),
                        scope.name()
                    )
                })
                .clone()
        } else {
            self.resolve_type_def(type_ref.namespace(), type_ref.name())
        }
    }

    pub fn signature_from_blob(
        &'static self,
        blob: &mut Blob,
        generics: &[ElementType],
    ) -> Option<Signature> {
        let is_const = blob
            .read_modifiers()
            .iter()
            .any(|def| def.full_name() == ("System.Runtime.CompilerServices", "IsConst"));

        let by_ref = blob.read_expected(0x10);

        if blob.read_expected(0x01) {
            return None;
        }

        let is_array = blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        let kind = self.type_from_blob(blob, generics);

        Some(Signature {
            kind,
            pointers,
            by_ref,
            is_const,
            is_array,
        })
    }

    pub fn type_from_code(
        &'static self,
        code: &TypeDefOrRef,
        generics: &[ElementType],
    ) -> ElementType {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob();
            return self.type_from_blob(&mut blob, generics);
        }

        let full_name = code.full_name();

        for (namespace, name, kind) in &WELL_KNOWN_TYPES {
            if full_name == (namespace, name) {
                return kind.clone();
            }
        }

        code.resolve().into()
    }

    pub fn type_from_blob(&'static self, blob: &mut Blob, generics: &[ElementType]) -> ElementType {
        let code = blob.read_unsigned();

        if let Some(code) = ElementType::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => self.type_from_code(
                &TypeDefOrRef::decode(blob.file, blob.read_unsigned()),
                generics,
            ),
            0x13 => generics
                .get(blob.read_unsigned() as usize)
                .unwrap_or_else(|| &ElementType::Void)
                .clone(),
            0x14 => {
                let kind = self.signature_from_blob(blob, generics).unwrap();
                let _rank = blob.read_unsigned();
                let _bounds_count = blob.read_unsigned();
                let bounds = blob.read_unsigned();
                ElementType::Array((Box::new(kind), bounds))
            }
            0x15 => {
                blob.read_unsigned();

                let mut def = TypeDefOrRef::decode(blob.file, blob.read_unsigned()).resolve();
                let args = blob.read_unsigned();

                for _ in 0..args {
                    def.generics.push(self.type_from_blob(blob, generics));
                }

                ElementType::TypeDef(def)
            }
            _ => unexpected!(),
        }
    }
}

fn trim_tick(name: &str) -> &str {
    let len = name.len() - 2;
    match name.as_bytes().get(len) {
        Some(c) if *c == b'`' => &name[..len],
        _ => name,
    }
}

fn is_well_known(namespace: &'static str, name: &'static str) -> bool {
    for entry in &WELL_KNOWN_TYPES {
        if name == entry.1 && namespace == entry.0 {
            return true;
        }
    }

    false
}

const WELL_KNOWN_TYPES: [(&'static str, &'static str, ElementType); 10] = [
    ("System", "Guid", ElementType::Guid),
    (
        "Windows.Win32.System.Com",
        "IUnknown",
        ElementType::IUnknown,
    ),
    ("Windows.Foundation", "HResult", ElementType::HRESULT),
    ("Windows.Win32.Foundation", "HRESULT", ElementType::HRESULT),
    ("Windows.Win32.System.WinRT", "HSTRING", ElementType::String),
    (
        "Windows.Win32.System.WinRT",
        "IInspectable",
        ElementType::IInspectable,
    ),
    (
        "Windows.Win32.System.SystemServices",
        "LARGE_INTEGER",
        ElementType::I64,
    ),
    (
        "Windows.Win32.System.SystemServices",
        "ULARGE_INTEGER",
        ElementType::U64,
    ),
    (
        "Windows.Win32.Graphics.Direct2D",
        "D2D_MATRIX_3X2_F",
        ElementType::Matrix3x2,
    ),
    ("System", "Type", ElementType::TypeName),
];
