use super::*;
use std::collections::*;

pub struct TypeReader {
    // Nested types are stored in a BTreeMap to ensure a stable order. This impacts
    // the derived nested type names.
    nested: HashMap<Row, BTreeMap<&'static str, TypeDef>>,

    pub types: TypeTree,
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
        unsafe { &*VALUE.as_mut_ptr() }
    }

    /// Insert WinRT metadata at the given paths
    ///
    /// # Panics
    ///
    /// This function panics if the if the files where the windows metadata are stored cannot be read.
    fn new() -> Self {
        let files = workspace_winmds();
        let mut nested = HashMap::<Row, BTreeMap<&'static str, TypeDef>>::new();
        let mut types = TypeTree::from_namespace("");

        for file in files {
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def: TypeDef = Row::new(row, TableIndex::TypeDef, file).into();
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
                    namespace.insert_type(type_name.name, Type::TypeDef(def));
                } else {
                    for field in def.fields() {
                        let name = field.name();
                        namespace.insert_type(name, Type::Field(field));
                    }

                    for method in def.methods() {
                        let name = method.name();
                        namespace.insert_type(name, Type::MethodDef(method));
                    }
                }
            }

            let row_count = file.nested_class_table().row_count;

            for row in 0..row_count {
                let row = NestedClass(Row::new(row, TableIndex::NestedClass, file));
                let enclosed = row.nested_type();
                let enclosing = row.enclosing_type();
                let name = enclosed.name();

                nested.entry(enclosing.row.clone()).or_default().insert(name, enclosed);
            }
        }

        Self { nested, types }
    }

    pub fn nested_types(&'static self, enclosing: &TypeDef) -> Option<&BTreeMap<&'static str, TypeDef>> {
        self.nested.get(&enclosing.row)
    }

    pub fn get_type_entry<T: HasTypeName>(&'static self, type_name: T) -> Option<&Vec<Type>> {
        self.types.get_namespace(type_name.namespace()).and_then(|tree| tree.get_type(type_name.name()))
    }

    pub fn get_type<T: HasTypeName>(&'static self, type_name: T) -> Option<&Type> {
        self.types.get_namespace(type_name.namespace()).and_then(|tree| tree.get_type(type_name.name())).and_then(|entry| entry.first())
    }

    pub fn get_namespace(&self, namespace: &str) -> Option<&TypeTree> {
        self.types.get_namespace(namespace)
    }

    pub fn expect_type_def<T: HasTypeName>(&'static self, type_name: T) -> TypeDef {
        self.get_type(type_name).and_then(|def| if let Type::TypeDef(def) = def { Some(def.clone()) } else { None }).unwrap_or_else(|| panic!("Expected type not found `{}.{}`", type_name.namespace(), type_name.name()))
    }

    pub fn expect_type_ref(&'static self, enclosing: Option<&TypeDef>, type_ref: &TypeRef) -> TypeDef {
        let type_name = type_ref.type_name();

        if let Some(enclosing) = enclosing {
            if type_name.namespace.is_empty() {
                return self.nested[&enclosing.row].get(type_name.name).unwrap_or_else(|| panic!("Could not find nested type `{}` in `{}`", type_name.name, enclosing.type_name(),)).clone();
            }
        }

        self.expect_type_def(type_name)
    }

    pub fn type_from_code(&'static self, code: &TypeDefOrRef, enclosing: Option<&TypeDef>, generics: &[Type]) -> Type {
        if let TypeDefOrRef::TypeSpec(def) = code {
            let mut blob = def.blob();
            return self.type_from_blob_impl(&mut blob, enclosing, generics);
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

        code.resolve(enclosing).into()
    }

    pub fn type_from_blob(&'static self, blob: &mut Blob, enclosing: Option<&TypeDef>, generics: &[Type]) -> Option<Type> {
        let is_winrt_const_ref = blob.read_modifiers().iter().any(|def| def.type_name() == TypeName::IsConst);

        let is_winrt_array_ref = blob.read_expected(0x10);

        if blob.read_expected(0x01) {
            return None;
        }

        let is_winrt_array = blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        let mut kind = self.type_from_blob_impl(blob, enclosing, generics);

        if pointers > 0 {
            kind = Type::MutPtr((Box::new(kind), pointers));
        }

        Some(if is_winrt_array {
            if is_winrt_array_ref {
                Type::WinrtArrayRef(Box::new(kind))
            } else {
                Type::WinrtArray(Box::new(kind))
            }
        } else if is_winrt_const_ref {
            Type::WinrtConstRef(Box::new(kind))
        } else {
            kind
        })
    }

    fn type_from_blob_impl(&'static self, blob: &mut Blob, enclosing: Option<&TypeDef>, generics: &[Type]) -> Type {
        let code = blob.read_unsigned();

        if let Some(code) = Type::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => self.type_from_code(&TypeDefOrRef::decode(blob.file, blob.read_unsigned()), enclosing, generics),
            0x13 => generics.get(blob.read_unsigned() as usize).unwrap_or(&Type::Void).clone(),
            0x14 => {
                let kind = self.type_from_blob(blob, enclosing, generics).unwrap();
                let _rank = blob.read_unsigned();
                let _bounds_count = blob.read_unsigned();
                let bounds = blob.read_unsigned();
                Type::Win32Array((Box::new(kind), bounds))
            }
            0x15 => {
                blob.read_unsigned();

                let mut def = TypeDefOrRef::decode(blob.file, blob.read_unsigned()).resolve(enclosing);
                let args = blob.read_unsigned();

                for _ in 0..args {
                    def.generics.push(self.type_from_blob_impl(blob, enclosing, generics));
                }

                Type::TypeDef(def)
            }
            _ => unimplemented!(),
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

const WELL_KNOWN_TYPES: [(TypeName, Type); 9] = [(TypeName::GUID, Type::GUID), (TypeName::IUnknown, Type::IUnknown), (TypeName::HResult, Type::HRESULT), (TypeName::HRESULT, Type::HRESULT), (TypeName::HSTRING, Type::String), (TypeName::IInspectable, Type::IInspectable), (TypeName::LARGE_INTEGER, Type::I64), (TypeName::ULARGE_INTEGER, Type::U64), (TypeName::Type, Type::TypeName)];
