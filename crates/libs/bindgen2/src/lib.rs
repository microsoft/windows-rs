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
mod winrt_delegate;

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
enum WinrtKind {
    Enum,
    Struct,
    Delegate,
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
struct WinrtEntry {
    entity: Entity<TypeDef>,
    kind: WinrtKind,
}

/// Owns a reusable validated metadata database.
pub struct Metadata {
    shared: Arc<Shared>,
}

/// Owns one deterministic generation request over shared metadata.
pub struct Generator {
    shared: Arc<Shared>,
    winrt: Vec<WinrtEntry>,
    win32: win32::Win32Selection,
}

struct Shared {
    database: Database,
    winrt_entries: Vec<(String, String, WinrtEntry)>,
    values: Values,
    win32_catalogs: Arc<win32::Win32Catalogs>,
}

/// A borrowed projected WinRT value item.
#[derive(Clone, Copy)]
struct ValueItem<'a> {
    definition: TypeDefinition<'a>,
    #[cfg(test)]
    kind: WinrtKind,
}

impl Metadata {
    /// Wraps an owned validated metadata database for reuse.
    pub fn new(database: Database) -> Result<Self, Error> {
        let winrt_entries = winrt_entries(&database)?;
        let values = Values::lower(&database, &winrt_entries)?;
        let win32_catalogs = Arc::new(win32::Win32Catalogs::new(&database)?);
        Ok(Self {
            shared: Arc::new(Shared {
                database,
                winrt_entries,
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
        let winrt = if let Some(filter) = filter {
            Self::close_winrt(&shared, filter)?
        } else {
            shared
                .winrt_entries
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
            winrt,
            win32,
        })
    }

    fn close_winrt(shared: &Shared, filter: &Filter) -> Result<Vec<WinrtEntry>, Error> {
        let mut catalog = BTreeMap::<(&str, &str), Vec<WinrtEntry>>::new();
        let mut selected = BTreeSet::new();
        let mut pending = VecDeque::new();

        for (namespace, name, entry) in &shared.winrt_entries {
            catalog.entry((namespace, name)).or_default().push(*entry);
            if filter.includes(namespace, name) && selected.insert(entry.entity) {
                pending.push_back(*entry);
            }
        }

        while let Some(entry) = pending.pop_front() {
            let definition = shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let dependencies = match entry.kind {
                WinrtKind::Struct => {
                    let Some(Value::Struct(model)) = shared.values.get(namespace, name) else {
                        continue;
                    };
                    model.dependencies()
                }
                WinrtKind::Delegate => winrt_delegate::Delegate::dependencies(
                    &shared.database,
                    definition,
                    &format!("{namespace}.{name}"),
                )?,
                WinrtKind::Enum => continue,
            };
            for (namespace, name) in dependencies {
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
            .winrt_entries
            .iter()
            .filter_map(|(_, _, entry)| selected.contains(&entry.entity).then_some(*entry))
            .collect())
    }

    /// Iterates projected values in deterministic namespace/name/entity order.
    fn values(&self) -> impl Iterator<Item = ValueItem<'_>> {
        self.winrt
            .iter()
            .filter(|entry| entry.kind != WinrtKind::Delegate)
            .map(|entry| ValueItem {
                definition: self.shared.database.definition(entry.entity).unwrap(),
                #[cfg(test)]
                kind: entry.kind,
            })
    }
}

fn winrt_entries(database: &Database) -> Result<Vec<(String, String, WinrtEntry)>, Error> {
    let mut entries = Vec::new();
    for definition in database.definitions() {
        if !definition.is_windows_runtime()? {
            continue;
        }
        let kind = match definition.category()? {
            TypeCategory::Enum => WinrtKind::Enum,
            TypeCategory::Struct => {
                if definition.has_attribute("ApiContractAttribute")? {
                    continue;
                }
                WinrtKind::Struct
            }
            TypeCategory::Delegate => WinrtKind::Delegate,
            _ => continue,
        };
        let name = definition.name()?;
        let name = if kind == WinrtKind::Delegate {
            name.split_once('`').map_or(name, |(name, _)| name)
        } else {
            name
        };
        entries.push((
            definition.namespace()?.to_string(),
            name.to_string(),
            WinrtEntry {
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
    const fn kind(self) -> WinrtKind {
        self.kind
    }
}

#[cfg(test)]
mod tests;
