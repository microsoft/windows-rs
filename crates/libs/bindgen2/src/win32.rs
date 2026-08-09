use super::*;
use std::collections::BTreeMap;

struct Namespace {
    name: String,
    constants: Vec<Entity<Field>>,
    functions: Vec<Entity<MethodDef>>,
}

/// Typed-entity selection for Win32 `Apis` constants and functions.
pub struct Win32Items<'a> {
    database: &'a Database,
    namespaces: Vec<Namespace>,
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
                Vec<(String, Entity<Field>)>,
                Vec<(String, Entity<MethodDef>)>,
            ),
        >::new();
        for definition in database.definitions() {
            if definition.is_windows_runtime()?
                || definition.category()? != TypeCategory::Class
                || definition.name()? != "Apis"
            {
                continue;
            }
            let namespace = definition.namespace()?.to_string();
            let entries = namespaces.entry(namespace).or_default();
            for field in definition.fields()? {
                entries.0.push((field.name()?.to_string(), field.entity()));
            }
            for method in definition.methods()? {
                if let Some(import) = method.import()?
                    && (import.module() == "FORCEINLINE" || import.name().starts_with('#'))
                {
                    continue;
                }
                entries
                    .1
                    .push((method.name()?.to_string(), method.entity()));
            }
        }
        let mut constant_count = 0;
        let mut function_count = 0;
        let namespaces = namespaces
            .into_iter()
            .map(|(name, (mut constants, mut functions))| {
                constants.sort();
                functions.sort();
                constant_count += constants.len();
                function_count += functions.len();
                Namespace {
                    name,
                    constants: constants.into_iter().map(|(_, entity)| entity).collect(),
                    functions: functions.into_iter().map(|(_, entity)| entity).collect(),
                }
            })
            .collect();
        Ok(Self {
            database,
            namespaces,
            constant_count,
            function_count,
        })
    }

    /// Returns the number of selected constants.
    pub fn constant_count(&self) -> usize {
        self.constant_count
    }

    /// Returns the number of selected functions.
    pub fn function_count(&self) -> usize {
        self.function_count
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
        let mut supported = [0; 2];
        let mut unsupported = BTreeMap::<String, usize>::new();

        for namespace in &items.namespaces {
            for entity in &namespace.constants {
                let field = database.field(*entity).unwrap();
                match Constant::lower(&database, field, &namespace.name, field.name().unwrap()) {
                    Ok(constant) => {
                        constant.write_sys();
                        supported[0] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.functions {
                let method = database.method(*entity).unwrap();
                match Function::lower(&database, method, &namespace.name, method.name().unwrap()) {
                    Ok(function) => {
                        function.write_sys();
                        supported[1] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
        }

        assert_eq!(supported, [83_641, 14_559]);
        assert!(unsupported.is_empty(), "{unsupported:#?}");
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
