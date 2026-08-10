#![doc = include_str!("../readme.md")]

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
};
use windows_metadata2::{
    AnyRowId, AttributeArgument, AttributeValue, ConstantValue, Database, Entity, FileId, Image,
    MethodSignature, TypeAttributes, TypeCategory, TypeDefinition, TypeKind,
    tables::{Field, MethodDef, TypeDef},
};

mod enum_model;
mod error;
mod filter;
mod guid;
mod model;
mod native;
mod native_closure;
mod native_constant;
mod native_default;
mod native_delegate;
mod native_function;
mod native_interface;
mod native_signature;
mod native_type;
mod output;
mod struct_model;
mod tokens;
mod ty;
mod win32;

use enum_model::Enum;
pub use error::Error;
pub use filter::Filter;
use model::{Value, Values};
use native_constant::Constant;
use native_delegate::Delegate;
use native_function::Function;
use native_interface::NativeInterface;
use native_type::NativeType;
#[cfg(test)]
use native_type::NativeTypeKind;
use struct_model::Struct;

/// A projected WinRT value category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValueKind {
    Enum,
    Struct,
}

/// Generated Rust output layout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Layout {
    /// Emit nested Rust modules for metadata namespaces.
    #[default]
    Modules,
    /// Emit one flat list of items.
    Flat,
}

/// Selection policy for one generation request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Request {
    filter: Option<Filter>,
}

impl Request {
    /// Selects all currently supported metadata items.
    pub fn all() -> Self {
        Self::default()
    }

    /// Selects items matched by the supplied filter and their dependencies.
    pub fn filtered(filter: Filter) -> Self {
        Self {
            filter: Some(filter),
        }
    }
}

#[derive(Clone, Copy)]
struct ValueEntry {
    entity: Entity<TypeDef>,
    kind: ValueKind,
}

/// Owns a reusable validated metadata database.
pub struct Metadata {
    shared: Arc<Shared>,
}

/// Owns one deterministic generation request over shared metadata.
pub struct Generator {
    shared: Arc<Shared>,
    values: Vec<ValueEntry>,
    win32: win32::Win32Selection,
}

struct Shared {
    database: Database,
    value_entries: Vec<(String, String, ValueEntry)>,
    values: Values,
    win32_catalogs: Arc<win32::Win32Catalogs>,
}

/// A borrowed projected WinRT value item.
#[derive(Clone, Copy)]
struct ValueItem<'a> {
    definition: TypeDefinition<'a>,
    #[cfg(test)]
    kind: ValueKind,
}

impl Metadata {
    /// Wraps an owned validated metadata database for reuse.
    pub fn new(database: Database) -> Result<Self, Error> {
        let value_entries = value_entries(&database)?;
        let values = Values::lower(&database, &value_entries)?;
        let win32_catalogs = Arc::new(win32::Win32Catalogs::new(&database)?);
        Ok(Self {
            shared: Arc::new(Shared {
                database,
                value_entries,
                values,
                win32_catalogs,
            }),
        })
    }

    /// Builds a reusable database from owned metadata images.
    pub fn from_images(images: impl IntoIterator<Item = Image>) -> Result<Self, Error> {
        Self::new(Database::new(images)?)
    }

    /// Creates an independent generation request sharing this metadata.
    pub fn generator(&self, request: Request) -> Result<Generator, Error> {
        Generator::from_shared(self.shared.clone(), request.filter.as_ref())
    }
}

impl Generator {
    fn from_shared(shared: Arc<Shared>, filter: Option<&Filter>) -> Result<Self, Error> {
        let values = if let Some(filter) = filter {
            Self::close_values(&shared, filter)?
        } else {
            shared
                .value_entries
                .iter()
                .map(|(_, _, entry)| *entry)
                .collect()
        };
        let win32 = win32::Win32Selection::new_with_catalogs(
            &shared.database,
            shared.win32_catalogs.clone(),
            filter,
        )?;

        Ok(Self {
            shared,
            values,
            win32,
        })
    }

    fn close_values(shared: &Shared, filter: &Filter) -> Result<Vec<ValueEntry>, Error> {
        let mut catalog = BTreeMap::<(&str, &str), Vec<ValueEntry>>::new();
        let mut selected = BTreeSet::new();
        let mut pending = VecDeque::new();

        for (namespace, name, entry) in &shared.value_entries {
            catalog.entry((namespace, name)).or_default().push(*entry);
            if filter.includes(namespace, name) && selected.insert(entry.entity) {
                pending.push_back(*entry);
            }
        }

        while let Some(entry) = pending.pop_front() {
            if entry.kind != ValueKind::Struct {
                continue;
            }
            let definition = shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let Some(Value::Struct(model)) = shared.values.get(namespace, name) else {
                continue;
            };
            for (namespace, name) in model.dependencies() {
                if let Some(entries) = catalog.get(&(namespace.as_str(), name.as_str())) {
                    for entry in entries {
                        if selected.insert(entry.entity) {
                            pending.push_back(*entry);
                        }
                    }
                }
            }
        }

        Ok(shared
            .value_entries
            .iter()
            .filter_map(|(_, _, entry)| selected.contains(&entry.entity).then_some(*entry))
            .collect())
    }

    /// Iterates projected values in deterministic namespace/name/entity order.
    fn values(&self) -> impl ExactSizeIterator<Item = ValueItem<'_>> {
        self.values.iter().map(|entry| ValueItem {
            definition: self.shared.database.definition(entry.entity).unwrap(),
            #[cfg(test)]
            kind: entry.kind,
        })
    }
}

fn value_entries(database: &Database) -> Result<Vec<(String, String, ValueEntry)>, Error> {
    let mut entries = Vec::new();
    for definition in database.definitions() {
        if !definition.is_windows_runtime()? {
            continue;
        }
        let kind = match definition.category()? {
            TypeCategory::Enum => ValueKind::Enum,
            TypeCategory::Struct => {
                if definition.has_attribute("ApiContractAttribute")? {
                    continue;
                }
                ValueKind::Struct
            }
            _ => continue,
        };
        entries.push((
            definition.namespace()?.to_string(),
            definition.name()?.to_string(),
            ValueEntry {
                entity: definition.entity(),
                kind,
            },
        ));
    }
    entries.sort_by(|left, right| {
        (&left.0, &left.1, left.2.entity).cmp(&(&right.0, &right.1, right.2.entity))
    });
    Ok(entries)
}

impl<'a> ValueItem<'a> {
    /// Returns the metadata definition.
    const fn definition(self) -> TypeDefinition<'a> {
        self.definition
    }

    /// Returns the projected value category.
    #[cfg(test)]
    const fn kind(self) -> ValueKind {
        self.kind
    }
}

#[cfg(test)]
mod tests;
