use super::*;
use std::collections::*;

pub struct TypeReader {
    // Nested types are stored in a BTreeMap to ensure a stable order. This impacts
    // the derived nested type names.
    nested: HashMap<Row, BTreeMap<&'static str, tables::TypeDef>>,

    pub types: TypeTree,
}

impl TypeReader {
    pub fn gen(&'static self) -> impl Iterator<Item = TokenStream> {
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
        let files = crate_winmds();
        let mut nested = HashMap::<Row, BTreeMap<&'static str, tables::TypeDef>>::new();
        let mut types = TypeTree::from_namespace("");
        types.include = true;

        for file in files {
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def: tables::TypeDef = Row::new(row, TableIndex::TypeDef, file).into();
                let type_name = def.type_name();

                if type_name.namespace.is_empty() {
                    continue;
                }

                if is_well_known(type_name) {
                    continue;
                }

                let extends = def.extends();

                if extends == TypeName::Attribute {
                    continue;
                }

                let namespace = types.insert_namespace(type_name.namespace, 0);

                if def.is_winrt() || extends != TypeName::Object {
                    namespace.insert_type(type_name.name, ElementType::TypeDef(def));
                } else {
                    for field in def.fields() {
                        let name = field.name();
                        namespace.insert_type(name, ElementType::Field(field));
                    }

                    for method in def.methods() {
                        let name = method.name();
                        namespace.insert_type(name, ElementType::MethodDef(method));
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

        Self { nested, types }
    }

    /// Get all the namespace names that the [`TypeReader`] knows about
    pub fn namespaces(&'static self) -> Vec<&'static str> {
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

    pub fn import_type(&mut self, namespace: &str, name: &str) -> bool {
        self.import_type_include(namespace, name, TypeInclude::Full)
    }

    fn import_type_dependencies(&mut self, def: &ElementType, include: TypeInclude) {
        for entry in def.dependencies(include) {
            let type_name = entry.def.type_name();

            // If def.namespace is empty it means its a nested type and we need to find its dependencies to avoid type slicing.
            if type_name.namespace.is_empty() {
                self.import_type_dependencies(&entry.def, TypeInclude::Minimal);
            } else {
                self.import_type_include(type_name.namespace, type_name.name, entry.include);
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
            } else if entry.include == TypeInclude::None {
                entry.include = TypeInclude::Minimal;
                self.import_type_dependencies(&copy, include);
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

    pub fn get_type<T: HasTypeName>(&'static self, type_name: T) -> Option<ElementType> {
        self.types
            .get_namespace(type_name.namespace())
            .and_then(|tree| tree.get_type(type_name.name()))
            .map(|entry| entry.def.clone())
    }

    pub fn expect_type<T: HasTypeName>(&'static self, type_name: T) -> ElementType {
        self.get_type(type_name).unwrap_or_else(|| {
            panic!(
                "Expected type not found `{}.{}`",
                type_name.namespace(),
                type_name.name()
            )
        })
    }

    pub fn expect_type_def(&'static self, type_name: TypeName) -> tables::TypeDef {
        self.get_type(type_name)
            .and_then(|def| {
                if let ElementType::TypeDef(def) = def {
                    Some(def)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "Expected type not found `{}.{}`",
                    type_name.namespace(),
                    type_name.name()
                )
            })
    }

    pub fn expect_type_ref(&'static self, type_ref: &tables::TypeRef) -> tables::TypeDef {
        if let ResolutionScope::TypeRef(scope) = type_ref.scope() {
            self.nested[&scope.resolve().row]
                .get(type_ref.name())
                .unwrap_or_else(|| {
                    panic!(
                        "Could not find nested type `{}` in `{}`",
                        type_ref.name(),
                        scope.type_name()
                    )
                })
                .clone()
        } else {
            self.expect_type_def(type_ref.type_name())
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
            .any(|def| def.type_name() == TypeName::IsConst);

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

        let full_name = code.type_name();

        for (known_name, kind) in WELL_KNOWN_TYPES {
            if full_name == known_name {
                return kind;
            }
        }

        for (from, to) in REMAP_TYPES {
            if full_name == from {
                return TypeReader::get().expect_type_def(to).into();
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
                .unwrap_or(&ElementType::Void)
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

fn is_well_known(type_name: TypeName) -> bool {
    for (known_name, _) in WELL_KNOWN_TYPES {
        if type_name == known_name {
            return true;
        }
    }

    false
}

const REMAP_TYPES: [(TypeName, TypeName); 1] = [(TypeName::D2D_MATRIX_3X2_F, TypeName::Matrix3x2)];

const WELL_KNOWN_TYPES: [(TypeName, ElementType); 9] = [
    (TypeName::Guid, ElementType::Guid),
    (TypeName::IUnknown, ElementType::IUnknown),
    (TypeName::HResult, ElementType::HRESULT),
    (TypeName::HRESULT, ElementType::HRESULT),
    (TypeName::HSTRING, ElementType::String),
    (TypeName::IInspectable, ElementType::IInspectable),
    (TypeName::LARGE_INTEGER, ElementType::I64),
    (TypeName::ULARGE_INTEGER, ElementType::U64),
    (TypeName::Type, ElementType::TypeName),
];
