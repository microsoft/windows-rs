use super::*;
use std::collections::*;

/// A reader of type information from Windows Metadata
pub struct TypeReader {
    types: HashMap<&'static str, HashMap<&'static str, TypeRow>>,
    // Nested types are stored in a BTreeMap to ensure a stable order. This impacts
    // the derived nested type names.
    nested: HashMap<Row, BTreeMap<&'static str, tables::TypeDef>>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum TypeRow {
    TypeDef(tables::TypeDef),
    MethodDef(tables::MethodDef),
    Field(tables::Field),
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
    pub fn get() -> &'static Self {
        use std::{mem::MaybeUninit, sync::Once};
        static ONCE: Once = Once::new();
        static mut VALUE: MaybeUninit<TypeReader> = MaybeUninit::uninit();

        ONCE.call_once(|| {
            // This is safe because `Once` provides thread-safe one-time initialization
            unsafe { VALUE = MaybeUninit::new(Self::new()) }
        });

        // This is safe because `call_once` has already been called.
        unsafe { &*VALUE.as_ptr() }
    }

    /// Insert WinRT metadata at the given paths
    ///
    /// # Panics
    ///
    /// This function panics if the if the files where the windows metadata are stored cannot be read.
    fn new() -> Self {
        let files = workspace_winmds();

        let mut types = HashMap::<&'static str, HashMap<&'static str, TypeRow>>::default();

        let mut nested = HashMap::<Row, BTreeMap<&'static str, tables::TypeDef>>::new();

        for file in files {
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def: tables::TypeDef = Row::new(row, TableIndex::TypeDef, file).into();
                let namespace = def.namespace();
                let name = trim_tick(def.name());

                if namespace.is_empty() {
                    continue;
                }

                let flags = def.flags();
                let extends = def.extends();

                if extends == ("System", "Attribute") {
                    continue;
                }

                let values = types.entry(namespace).or_default();

                values
                    .entry(name)
                    .or_insert_with(|| TypeRow::TypeDef(def.clone()));

                if flags.interface() || flags.windows_runtime() {
                    continue;
                }

                match extends {
                    ("System", "Object") => {
                        for field in def.fields() {
                            let name = field.name();

                            values.entry(name).or_insert_with(|| TypeRow::Field(field));
                        }

                        for method in def.methods() {
                            let name = method.name();

                            values
                                .entry(name)
                                .or_insert_with(|| TypeRow::MethodDef(method));
                        }
                    }
                    _ => {}
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

        for (namespace, name, _) in WELL_KNOWN_TYPES {
            if let Some(value) = types.get_mut(namespace) {
                value.remove(name);
            }
        }

        Self { types, nested }
    }

    pub fn resolve_namespace(&'static self, find: &str) -> &'static str {
        self.types
            .keys()
            .find(|namespace| *namespace == &find)
            .unwrap_or_else(|| panic!("Could not find namespace `{}`", find))
    }

    /// Get all the namespace names that the [`TypeReader`] knows about
    pub fn namespaces(&'static self) -> impl Iterator<Item = &'static str> {
        self.types.keys().copied()
    }

    /// Get all types for a given namespace
    ///
    /// # Panics
    ///
    /// Panics if the namespace does not exist
    pub fn namespace_types(&'static self, namespace: &str) -> impl Iterator<Item = ElementType> {
        self.types[namespace].values().map(move |row| row.into())
    }

    pub fn nested_types(
        &'static self,
        enclosing: &tables::TypeDef,
    ) -> Option<&BTreeMap<&'static str, tables::TypeDef>> {
        self.nested.get(&enclosing.row)
    }

    pub fn resolve_type(&'static self, namespace: &str, name: &str) -> ElementType {
        if let Some(types) = self.types.get(namespace) {
            if let Some(row) = types.get(trim_tick(name)) {
                return row.into();
            }
        }

        panic!("Could not find type `{}.{}`", namespace, name);
    }

    pub fn get_namespace(&'static self, namespace: &str) -> Option<&'static str> {
        if let Some((namespace, _)) = self.types.get_key_value(namespace) {
            Some(namespace)
        } else {
            None
        }
    }

    pub fn get_type_name(
        &'static self,
        namespace: &str,
        name: &str,
    ) -> Option<(&'static str, &'static str)> {
        if let Some((namespace, types)) = self.types.get_key_value(namespace) {
            if let Some((name, _)) = types.get_key_value(trim_tick(name)) {
                return Some((namespace, name));
            }
        }

        None
    }

    pub fn resolve_type_def(&'static self, namespace: &str, name: &str) -> tables::TypeDef {
        if let Some(types) = self.types.get(namespace) {
            if let Some(TypeRow::TypeDef(row)) = types.get(trim_tick(name)) {
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

        for (namespace, name, kind) in WELL_KNOWN_TYPES {
            if full_name == (namespace, name) {
                return kind;
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
            0x13 => generics[blob.read_unsigned() as usize].clone(),
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
    match name.as_bytes().get(name.len() - 2) {
        Some(c) if *c == b'`' => &name[..name.len() - 2],
        _ => name,
    }
}

const WELL_KNOWN_TYPES: [(&'static str, &'static str, ElementType); 10] = [
    ("System", "Guid", ElementType::Guid),
    (
        "Windows.Win32.System.Com",
        "IUnknown",
        ElementType::IUnknown,
    ),
    ("Windows.Foundation", "HResult", ElementType::HRESULT),
    ("Windows.Win32.System.Com", "HRESULT", ElementType::HRESULT),
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
