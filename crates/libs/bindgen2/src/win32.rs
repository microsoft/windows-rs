use super::*;
use std::collections::BTreeMap;

struct Namespace {
    name: String,
    types: Vec<Entity<TypeDef>>,
    constants: Vec<Entity<Field>>,
    functions: Vec<Entity<MethodDef>>,
}

/// Typed-entity selection for Win32 `Apis` constants and functions.
pub struct Win32Items<'a> {
    database: &'a Database,
    namespaces: Vec<Namespace>,
    nested: BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    type_count: usize,
    architecture_type_count: usize,
    architecture_constant_count: usize,
    architecture_function_count: usize,
    constant_count: usize,
    function_count: usize,
}

impl Generator {
    /// Selects Win32 constants and functions from non-WinRT `Apis` containers.
    pub fn win32_items(&self) -> Result<Win32Items<'_>, Error> {
        Win32Items::new(&self.database)
    }
}

impl<'a> Win32Items<'a> {
    fn new(database: &'a Database) -> Result<Self, Error> {
        let mut namespaces = BTreeMap::<
            String,
            (
                Vec<(String, i32, Entity<TypeDef>)>,
                Vec<(String, i32, Entity<Field>)>,
                Vec<(String, i32, Entity<MethodDef>)>,
            ),
        >::new();
        for definition in database.definitions() {
            if definition.is_windows_runtime()? {
                continue;
            }
            let namespace = definition.namespace()?.to_string();
            let category = definition.category()?;
            match category {
                TypeCategory::Enum | TypeCategory::Struct => {
                    namespaces.entry(namespace).or_default().0.push((
                        definition.name()?.to_string(),
                        definition.architectures()?,
                        definition.entity(),
                    ));
                }
                TypeCategory::Class if definition.name()? == "Apis" => {
                    let entries = namespaces.entry(namespace).or_default();
                    for field in definition.fields()? {
                        entries.1.push((
                            field.name()?.to_string(),
                            field.architectures()?,
                            field.entity(),
                        ));
                    }
                    for method in definition.methods()? {
                        if let Some(import) = method.import()?
                            && (import.module() == "FORCEINLINE" || import.name().starts_with('#'))
                        {
                            continue;
                        }
                        entries.2.push((
                            method.name()?.to_string(),
                            method.architectures()?,
                            method.entity(),
                        ));
                    }
                }
                _ => continue,
            }
        }
        let mut type_count = 0;
        let mut architecture_type_count = 0;
        let mut architecture_constant_count = 0;
        let mut architecture_function_count = 0;
        let mut constant_count = 0;
        let mut function_count = 0;
        let namespaces: Vec<Namespace> = namespaces
            .into_iter()
            .map(|(name, (mut types, mut constants, mut functions))| {
                types.sort();
                constants.sort();
                functions.sort();
                architecture_type_count += types.iter().filter(|(_, bits, _)| *bits != 0).count();
                architecture_constant_count +=
                    constants.iter().filter(|(_, bits, _)| *bits != 0).count();
                architecture_function_count +=
                    functions.iter().filter(|(_, bits, _)| *bits != 0).count();
                type_count += types.len();
                constant_count += constants.len();
                function_count += functions.len();
                Namespace {
                    name,
                    types: types.into_iter().map(|(_, _, entity)| entity).collect(),
                    constants: constants.into_iter().map(|(_, _, entity)| entity).collect(),
                    functions: functions.into_iter().map(|(_, _, entity)| entity).collect(),
                }
            })
            .collect();
        let mut nested = BTreeMap::<Entity<TypeDef>, Vec<Entity<TypeDef>>>::new();
        for (child, parent) in database.nested_types() {
            if !child.is_windows_runtime()?
                && child.category()? == TypeCategory::Struct
                && parent.category()? == TypeCategory::Struct
            {
                nested
                    .entry(parent.entity())
                    .or_default()
                    .push(child.entity());
            }
        }
        Ok(Self {
            database,
            namespaces,
            nested,
            type_count,
            architecture_type_count,
            architecture_constant_count,
            architecture_function_count,
            constant_count,
            function_count,
        })
    }

    /// Returns the number of selected native type definitions.
    pub fn type_count(&self) -> usize {
        self.type_count
    }

    /// Returns the number of selected native definitions with architecture gates.
    pub fn architecture_type_count(&self) -> usize {
        self.architecture_type_count
    }

    /// Returns the number of selected constants with architecture gates.
    pub fn architecture_constant_count(&self) -> usize {
        self.architecture_constant_count
    }

    /// Returns the number of selected functions with architecture gates.
    pub fn architecture_function_count(&self) -> usize {
        self.architecture_function_count
    }

    /// Returns the number of nested native structs retained for future attachment.
    pub fn nested_type_count(&self) -> usize {
        self.nested.values().map(Vec::len).sum()
    }

    /// Returns the number of selected constants.
    pub fn constant_count(&self) -> usize {
        self.constant_count
    }

    /// Returns the number of selected functions.
    pub fn function_count(&self) -> usize {
        self.function_count
    }

    /// Lowers native type definitions in deterministic namespace and name order.
    pub fn native_types(&self) -> impl Iterator<Item = Result<NativeType, Error>> + '_ {
        self.namespaces.iter().flat_map(|namespace| {
            namespace.types.iter().map(|entity| {
                NativeType::lower(
                    self.database,
                    self.database.definition(*entity).unwrap(),
                    &self.nested,
                )
            })
        })
    }

    /// Lowers constants in deterministic namespace and name order.
    pub fn constants(&self) -> impl Iterator<Item = Result<Constant, Error>> + '_ {
        self.namespaces.iter().flat_map(|namespace| {
            namespace.constants.iter().map(|entity| {
                let field = self.database.field(*entity).unwrap();
                Constant::lower(self.database, field, &namespace.name, field.name().unwrap())
            })
        })
    }

    /// Lowers functions in deterministic namespace and name order.
    pub fn functions(&self) -> impl Iterator<Item = Result<Function, Error>> + '_ {
        self.namespaces.iter().flat_map(|namespace| {
            namespace.functions.iter().map(|entity| {
                let method = self.database.method(*entity).unwrap();
                Function::lower(
                    self.database,
                    method,
                    &namespace.name,
                    method.name().unwrap(),
                )
            })
        })
    }

    /// Lowers a uniquely named constant.
    pub fn constant(&self, namespace: &str, name: &str) -> Result<Constant, Error> {
        let entity = self.constant_entity(namespace, name)?;
        Constant::lower(
            self.database,
            self.database.field(entity).unwrap(),
            namespace,
            name,
        )
    }

    /// Lowers a uniquely named function.
    pub fn function(&self, namespace: &str, name: &str) -> Result<Function, Error> {
        let entity = self.function_entity(namespace, name)?;
        Function::lower(
            self.database,
            self.database.method(entity).unwrap(),
            namespace,
            name,
        )
    }

    /// Lowers a uniquely named native type definition.
    pub fn native_type(&self, namespace: &str, name: &str) -> Result<NativeType, Error> {
        let entity = self.type_entity(namespace, name)?;
        NativeType::lower(
            self.database,
            self.database.definition(entity).unwrap(),
            &self.nested,
        )
    }

    pub(super) fn render(
        &self,
        mut add: impl FnMut(&str, &str, u8, proc_macro2::TokenStream),
    ) -> Result<(), Error> {
        for namespace in &self.namespaces {
            for entity in &namespace.types {
                let definition = self.database.definition(*entity).unwrap();
                let ty = NativeType::lower(self.database, definition, &self.nested)?;
                for (name, kind, tokens) in ty.write_sys_items() {
                    add(&namespace.name, name, kind, tokens);
                }
            }
            for entity in &namespace.constants {
                let field = self.database.field(*entity).unwrap();
                let name = field.name()?;
                add(
                    &namespace.name,
                    name,
                    2,
                    Constant::lower(self.database, field, &namespace.name, name)?.write_sys(),
                );
            }
            for entity in &namespace.functions {
                let method = self.database.method(*entity).unwrap();
                let name = method.name()?;
                add(
                    &namespace.name,
                    name,
                    3,
                    Function::lower(self.database, method, &namespace.name, name)?.write_sys(),
                );
            }
        }
        Ok(())
    }

    fn type_entity(&self, namespace: &str, name: &str) -> Result<Entity<TypeDef>, Error> {
        let Some(namespace) = self.namespaces.iter().find(|item| item.name == namespace) else {
            return Err(missing(namespace, name));
        };
        unique_entity(
            namespace.types.iter().copied().filter(|entity| {
                self.database.definition(*entity).unwrap().name().unwrap() == name
            }),
            &namespace.name,
            name,
        )
    }

    fn constant_entity(&self, namespace: &str, name: &str) -> Result<Entity<Field>, Error> {
        let Some(namespace) = self.namespaces.iter().find(|item| item.name == namespace) else {
            return Err(missing(namespace, name));
        };
        unique_entity(
            namespace
                .constants
                .iter()
                .copied()
                .filter(|entity| self.database.field(*entity).unwrap().name().unwrap() == name),
            &namespace.name,
            name,
        )
    }

    fn function_entity(&self, namespace: &str, name: &str) -> Result<Entity<MethodDef>, Error> {
        let Some(namespace) = self.namespaces.iter().find(|item| item.name == namespace) else {
            return Err(missing(namespace, name));
        };
        unique_entity(
            namespace
                .functions
                .iter()
                .copied()
                .filter(|entity| self.database.method(*entity).unwrap().name().unwrap() == name),
            &namespace.name,
            name,
        )
    }
}

fn unique_entity<T: windows_metadata2::Table>(
    mut matches: impl Iterator<Item = Entity<T>>,
    namespace: &str,
    name: &str,
) -> Result<Entity<T>, Error> {
    let Some(result) = matches.next() else {
        return Err(missing(namespace, name));
    };
    if matches.next().is_some() {
        return Err(Error::InvalidValue {
            name: format!("{namespace}.{name}"),
            message: "Win32 item is not unique",
        });
    }
    Ok(result)
}

fn missing(namespace: &str, name: &str) -> Error {
    Error::MissingWin32Item {
        namespace: namespace.to_string(),
        name: name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use windows_metadata2::Image;

    #[test]
    fn inventory_current_win32_lowering() {
        let database = Database::new([Image::new(windows_default::WIN32).unwrap()]).unwrap();
        let items = Win32Items::new(&database).unwrap();
        let mut supported = [0; 5];
        let mut unsupported = BTreeMap::<String, usize>::new();

        for namespace in &items.namespaces {
            for entity in &namespace.types {
                let definition = database.definition(*entity).unwrap();
                match NativeType::lower(&database, definition, &items.nested) {
                    Ok(ty) => {
                        ty.write_sys();
                        supported[match ty.kind() {
                            NativeTypeKind::Alias => 0,
                            NativeTypeKind::Enum => 1,
                            NativeTypeKind::Struct => 2,
                        }] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.constants {
                let field = database.field(*entity).unwrap();
                match Constant::lower(&database, field, &namespace.name, field.name().unwrap()) {
                    Ok(constant) => {
                        constant.write_sys();
                        supported[3] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.functions {
                let method = database.method(*entity).unwrap();
                match Function::lower(&database, method, &namespace.name, method.name().unwrap()) {
                    Ok(function) => {
                        function.write_sys();
                        supported[4] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
        }

        assert_eq!(supported[..3], [12_666, 4_728, 12_715]);
        assert_eq!(supported[3..], [83_641, 14_559]);
        assert_eq!(items.type_count, 30_109);
        assert!(unsupported.is_empty(), "{unsupported:#?}");
    }

    #[test]
    fn inventory_architecture_variants_and_nested_types() {
        let database = Database::new([Image::new(windows_default::WIN32).unwrap()]).unwrap();
        let items = Win32Items::new(&database).unwrap();
        let image = &database.images()[0];
        let mut architecture_rows = 0;
        let mut architecture_groups = BTreeMap::<(String, String), Vec<i32>>::new();
        for definition in database.definitions() {
            if definition.is_windows_runtime().unwrap() {
                continue;
            }
            let architectures = definition.architectures().unwrap();
            if architectures != 0 {
                architecture_rows += 1;
                architecture_groups
                    .entry((
                        definition.namespace().unwrap().to_string(),
                        definition.name().unwrap().to_string(),
                    ))
                    .or_default()
                    .push(architectures);
            }
        }
        let variant_groups = architecture_groups
            .values()
            .filter(|architectures| architectures.len() > 1)
            .count();
        let nested_rows = image.rows::<windows_metadata2::tables::NestedClass>().len();
        assert_eq!(
            (
                architecture_rows,
                architecture_groups.len(),
                variant_groups,
                nested_rows,
                items.architecture_type_count(),
                items.architecture_constant_count(),
                items.architecture_function_count(),
                items.nested_type_count(),
            ),
            (1_054, 671, 374, 2_633, 997, 512, 261, 2_633)
        );
        assert_eq!(
            items.nested.values().map(Vec::len).sum::<usize>(),
            nested_rows
        );
        assert_eq!(items.nested.len(), 1_925);
    }

    fn classify(error: Error) -> String {
        match error {
            Error::UnsupportedType { shape, .. } if shape.starts_with("typed constant ") => {
                "typed constant".to_string()
            }
            Error::UnsupportedType { shape, .. } => {
                shape.split(['(', ' ', '<']).next().unwrap().to_string()
            }
            Error::InvalidValue { message, .. } => message.to_string(),
            Error::Metadata(error) => format!("metadata: {error}"),
            other => other.to_string(),
        }
    }
}
