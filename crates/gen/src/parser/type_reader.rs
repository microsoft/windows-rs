use super::*;
use std::collections::BTreeMap;

/// A reader of type information from Windows Metadata
pub struct TypeReader {
    types: BTreeMap<&'static str, BTreeMap<&'static str, TypeRow>>,
    nested: BTreeMap<tables::TypeDef, BTreeMap<&'static str, tables::TypeDef>>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum TypeRow {
    TypeDef(tables::TypeDef),
    Function(tables::MethodDef),
    Constant(tables::Field),
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

        let mut types = BTreeMap::<&'static str, BTreeMap<&'static str, TypeRow>>::default();

        let mut nested =
            BTreeMap::<tables::TypeDef, BTreeMap<&'static str, tables::TypeDef>>::new();

        for file in files {
            let row_count = file.type_def_table().row_count;

            for row in 0..row_count {
                let def = tables::TypeDef(Row::new(row, TableIndex::TypeDef, file));
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

                types
                    .entry(namespace)
                    .or_default()
                    .entry(name)
                    .or_insert_with(|| TypeRow::TypeDef(def));

                if flags.interface() || flags.windows_runtime() {
                    continue;
                }

                match extends {
                    ("System", "Object") => {
                        for field in def.fields() {
                            let name = field.name();
        
                            types
                                .entry(namespace)
                                .or_default()
                                .entry(name)
                                .or_insert_with(|| TypeRow::Constant(field));
                        }
        
                        for method in def.methods() {
                            let name = method.name();
        
                            types
                                .entry(namespace)
                                .or_default()
                                .entry(name)
                                .or_insert_with(|| TypeRow::Function(method));
                        }
                    }
                    ("System", "Enum") => {
                        for field in def.fields() {
                            let name = field.name();
        
                            types
                                .entry(namespace)
                                .or_default()
                                .entry(name)
                                .or_insert_with(|| TypeRow::Constant(field));
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

                nested.entry(enclosing).or_default().insert(name, enclosed);
            }
        }

        let exclude = &[
            ("Windows.Foundation", "HResult"),
            ("Windows.Win32.System.Com", "HRESULT"),
            ("Windows.Win32.System.Com", "IUnknown"),
            ("Windows.Win32.System.WinRT", "HSTRING"),
            ("Windows.Win32.System.WinRT", "IActivationFactory"),
            ("Windows.Win32.Graphics.Direct2D", "D2D_MATRIX_3X2_F"),
            ("Windows.Win32.System.SystemServices", "LARGE_INTEGER"),
            ("Windows.Win32.System.SystemServices", "ULARGE_INTEGER"),
        ];

        for (namespace, name) in exclude {
            if let Some(value) = types.get_mut(*namespace) {
                value.remove(*name);
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
    pub fn namespace_types(
        &'static self,
        namespace: &str,
    ) -> impl Iterator<Item = ElementType> + '_ {
        self.types[namespace]
            .values()
            .map(move |row| self.to_element_type(row))
    }

    // TODO: how to make this return an iterator?
    pub fn nested_types(&'static self, enclosing: &tables::TypeDef) -> Vec<tables::TypeDef> {
        self.nested
            .get(enclosing)
            .iter()
            .flat_map(|t| t.values())
            .copied()
            .collect()
    }

    pub fn resolve_type(&'static self, namespace: &str, name: &str) -> ElementType {
        if let Some(types) = self.types.get(namespace) {
            if let Some(row) = types.get(trim_tick(name)) {
                return self.to_element_type(row);
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

    fn to_element_type(&'static self, row: &TypeRow) -> ElementType {
        match row {
            TypeRow::TypeDef(row) => ElementType::from_type_def(*row, Vec::new()),
            TypeRow::Function(row) => ElementType::Function(types::Function(*row)),
            TypeRow::Constant(row) => ElementType::Constant(types::Constant(*row)),
        }
    }

    pub fn resolve_type_def(&'static self, namespace: &str, name: &str) -> tables::TypeDef {
        if let Some(types) = self.types.get(namespace) {
            if let Some(TypeRow::TypeDef(row)) = types.get(trim_tick(name)) {
                return *row;
            }
        }

        panic!("Could not find type def `{}.{}`", namespace, name);
    }

    pub fn resolve_type_ref(&'static self, type_ref: &tables::TypeRef) -> tables::TypeDef {
        if let ResolutionScope::TypeRef(scope) = type_ref.scope() {
            self.nested[&scope.resolve()][type_ref.name()]
        } else {
            self.resolve_type_def(type_ref.namespace(), type_ref.name())
        }
    }

    #[cfg(test)]
    pub fn get_class(namespace: &str, name: &str) -> types::Class {
        if let ElementType::Class(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_struct(namespace: &str, name: &str) -> types::Struct {
        if let ElementType::Struct(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_enum(namespace: &str, name: &str) -> types::Enum {
        if let ElementType::Enum(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }

    #[cfg(test)]
    pub fn get_interface(namespace: &str, name: &str) -> types::Interface {
        if let ElementType::Interface(value) = Self::get().resolve_type(namespace, name) {
            value.clone()
        } else {
            unexpected!();
        }
    }
}

fn trim_tick(name: &str) -> &str {
    match name.as_bytes().get(name.len() - 2) {
        Some(c) if *c == b'`' => &name[..name.len() - 2],
        _ => name,
    }
}
