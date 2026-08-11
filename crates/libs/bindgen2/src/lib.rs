#![doc = include_str!("../readme.md")]

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Arc,
};
use windows_metadata2::{
    AnyRowId, AttributeArgument, AttributeValue, ConstantValue, Database, Entity, FileId, Image,
    MethodSignature, TypeAttributes, TypeCategory, TypeDefinition, TypeIdentity, TypeKind,
    TypeResolution,
    tables::{Field, MethodDef, TypeDef},
};

mod enum_model;
mod error;
mod external;
mod filter;
mod guid;
mod model;
mod native;
mod native_closure;
mod native_com;
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
mod winrt_class;
mod winrt_collection;
mod winrt_delegate;
mod winrt_interface;

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
    Interface,
    Class,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Projection {
    Sys,
    #[default]
    Default,
    Minimal,
}

impl Projection {
    const fn is_sys(self) -> bool {
        matches!(self, Self::Sys)
    }

    const fn is_minimal(self) -> bool {
        matches!(self, Self::Minimal)
    }
}

/// Selection policy for one generation request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Request {
    filter: Option<Filter>,
    implementations: Option<Filter>,
    projection: Projection,
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
            implementations: None,
            projection: Projection::Default,
        }
    }

    /// Selects interfaces that require implementation traits and typed ABI vtables.
    pub fn implementations(mut self, filter: Filter) -> Self {
        self.implementations = Some(filter);
        self
    }

    #[cfg(test)]
    fn projection(mut self, projection: Projection) -> Self {
        self.projection = projection;
        self
    }
}

#[derive(Clone, Copy)]
struct WinrtEntry {
    entity: Entity<TypeDef>,
    kind: WinrtKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum MemberSelection {
    All,
    Names(BTreeSet<String>),
    Shell,
}

impl MemberSelection {
    fn includes(&self, metadata_name: &str, projected_name: &str) -> bool {
        match self {
            Self::All => true,
            Self::Names(names) => names.contains(metadata_name) || names.contains(projected_name),
            Self::Shell => false,
        }
    }

    const fn emits_implementation(&self, projection: Projection) -> bool {
        matches!(self, Self::All) || (projection.is_minimal() && matches!(self, Self::Names(_)))
    }

    fn merge(&mut self, other: Self) -> bool {
        match (&mut *self, other) {
            (Self::All, _) | (Self::Names(_), Self::Shell) | (Self::Shell, Self::Shell) => false,
            (selection, Self::All) => {
                *selection = Self::All;
                true
            }
            (Self::Names(names), Self::Names(other)) => {
                let length = names.len();
                names.extend(other);
                names.len() != length
            }
            (selection @ Self::Shell, Self::Names(names)) => {
                *selection = Self::Names(names);
                true
            }
        }
    }
}

struct InterfaceBase {
    file: FileId,
    entity: Entity<TypeDef>,
    arguments: Vec<windows_metadata2::Type>,
    default: bool,
}

enum InterfaceRelationship {
    Resolved(InterfaceBase),
    Invalid {
        owner: String,
        message: &'static str,
    },
}

impl InterfaceRelationship {
    fn from_resolution(
        resolution: TypeResolution<'_>,
        file: FileId,
        arguments: Vec<windows_metadata2::Type>,
        default: bool,
        owner: String,
    ) -> Self {
        let entity = match resolution {
            TypeResolution::Definition(entity) => Some(entity),
            TypeResolution::Candidates(candidates) => candidates.first(),
            TypeResolution::Specification(_) => {
                return Self::Invalid {
                    owner,
                    message: "required interface has a nested type specification",
                };
            }
        };
        entity.map_or(
            Self::Invalid {
                owner,
                message: "required interface cannot be resolved",
            },
            |entity| {
                Self::Resolved(InterfaceBase {
                    file,
                    entity,
                    arguments,
                    default,
                })
            },
        )
    }

    fn resolve(&self) -> Result<&InterfaceBase, Error> {
        match self {
            Self::Resolved(base) => Ok(base),
            Self::Invalid { owner, message } => Err(Error::InvalidType {
                name: owner.clone(),
                message,
            }),
        }
    }
}

/// Owns reusable checked metadata and shared projection catalogs.
pub struct Metadata {
    shared: Arc<Shared>,
}

/// Owns one deterministic generation request over shared metadata.
pub struct Generator {
    shared: Arc<Shared>,
    winrt: Vec<WinrtEntry>,
    winrt_members: BTreeMap<Entity<TypeDef>, MemberSelection>,
    winrt_implementations: Option<BTreeSet<Entity<TypeDef>>>,
    win32: win32::Win32Selection,
}

struct Shared {
    database: Database,
    winrt_entries: Vec<(String, String, WinrtEntry)>,
    values: Values,
    interface_relationships: BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
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
        let interface_relationships = interface_relationships(&database)?;
        let win32_catalogs = Arc::new(win32::Win32Catalogs::new(&database)?);
        Ok(Self {
            shared: Arc::new(Shared {
                database,
                winrt_entries,
                values,
                interface_relationships,
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
        Generator::from_shared(self.shared.clone(), &request)
    }
}

impl Generator {
    fn from_shared(shared: Arc<Shared>, request: &Request) -> Result<Self, Error> {
        let filter = request.filter.as_ref();
        let (winrt, winrt_members) = if let Some(filter) = filter {
            Self::close_winrt(
                &shared,
                filter,
                request.implementations.as_ref(),
                request.projection,
            )?
        } else {
            (
                shared
                    .winrt_entries
                    .iter()
                    .map(|(_, _, entry)| *entry)
                    .collect(),
                BTreeMap::new(),
            )
        };
        let win32 = win32::Win32Selection::new_with_catalogs(
            &shared.database,
            shared.win32_catalogs.clone(),
            filter,
        )?;
        let winrt_implementations = request.implementations.as_ref().map(|implementations| {
            winrt
                .iter()
                .filter_map(|entry| {
                    let definition = shared.database.definition(entry.entity).unwrap();
                    implementations
                        .includes(
                            definition.namespace().unwrap(),
                            trim_generic_arity(definition.name().unwrap()),
                        )
                        .then_some(entry.entity)
                })
                .collect()
        });

        Ok(Self {
            shared,
            winrt,
            winrt_members,
            winrt_implementations,
            win32,
        })
    }

    fn close_winrt(
        shared: &Shared,
        filter: &Filter,
        implementations: Option<&Filter>,
        projection: Projection,
    ) -> Result<(Vec<WinrtEntry>, BTreeMap<Entity<TypeDef>, MemberSelection>), Error> {
        let mut catalog = BTreeMap::<(&str, &str), Vec<WinrtEntry>>::new();
        let mut selected = BTreeMap::<Entity<TypeDef>, MemberSelection>::new();
        let mut pending = VecDeque::new();

        for (namespace, name, entry) in &shared.winrt_entries {
            catalog.entry((namespace, name)).or_default().push(*entry);
            let members = if filter.includes(namespace, name) {
                Some(MemberSelection::All)
            } else {
                filter
                    .methods(namespace, name)
                    .cloned()
                    .map(MemberSelection::Names)
            };
            if let Some(members) = members {
                selected.insert(entry.entity, members);
                pending.push_back(*entry);
            }
        }

        while let Some(entry) = pending.pop_front() {
            let definition = shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let mut members = selected.get(&entry.entity).unwrap().clone();
            let implemented = implementations.is_some_and(|implementations| {
                implementations.includes(namespace, trim_generic_arity(name))
            });
            if implemented {
                match members {
                    MemberSelection::Names(_) => {
                        return Err(Error::InvalidType {
                            name: format!("{namespace}.{name}"),
                            message: "implemented interface has a member filter",
                        });
                    }
                    MemberSelection::Shell => {
                        members = MemberSelection::All;
                        selected.insert(entry.entity, members.clone());
                    }
                    MemberSelection::All => {}
                }
            }
            let (dependencies, relationship_members) = match entry.kind {
                WinrtKind::Struct => {
                    let Some(Value::Struct(model)) = shared.values.get(namespace, name) else {
                        continue;
                    };
                    (model.dependencies(), BTreeMap::new())
                }
                WinrtKind::Delegate => (
                    winrt_delegate::Delegate::dependencies(
                        &shared.database,
                        definition,
                        &format!("{namespace}.{name}"),
                    )?,
                    BTreeMap::new(),
                ),
                WinrtKind::Interface => {
                    let model = winrt_interface::Interface::lower(
                        &shared.database,
                        definition,
                        &shared.interface_relationships,
                        &format!("{namespace}.{name}"),
                    )?;
                    (
                        model.dependencies(&members, implementations.is_none() || implemented),
                        model.relationship_members(&members),
                    )
                }
                WinrtKind::Class => {
                    let model = winrt_class::Class::lower(
                        &shared.database,
                        definition,
                        &shared.interface_relationships,
                        &format!("{namespace}.{name}"),
                    )?;
                    (
                        model.dependencies(&members),
                        model.relationship_members(&members),
                    )
                }
                WinrtKind::Enum => continue,
            };
            for (namespace, name) in dependencies {
                if projection.is_minimal() && external::minimal_crate(&namespace, &name).is_some() {
                    continue;
                }
                if let Some(entries) = catalog.get(&(namespace.as_str(), name.as_str())) {
                    for entry in entries {
                        let members = match entry.kind {
                            WinrtKind::Enum | WinrtKind::Struct | WinrtKind::Delegate => {
                                MemberSelection::All
                            }
                            WinrtKind::Interface | WinrtKind::Class => relationship_members
                                .get(&(namespace.clone(), name.clone()))
                                .cloned()
                                .unwrap_or(MemberSelection::Shell),
                        };
                        if let Some(current) = selected.get_mut(&entry.entity) {
                            if current.merge(members) {
                                pending.push_back(*entry);
                            }
                        } else {
                            selected.insert(entry.entity, members);
                            pending.push_back(*entry);
                        }
                    }
                }
            }
        }

        let entries = shared
            .winrt_entries
            .iter()
            .filter_map(|(_, _, entry)| selected.contains_key(&entry.entity).then_some(*entry))
            .collect();
        Ok((entries, selected))
    }

    fn members(&self, entity: Entity<TypeDef>) -> &MemberSelection {
        self.winrt_members
            .get(&entity)
            .unwrap_or(&MemberSelection::All)
    }

    fn implements(&self, entity: Entity<TypeDef>) -> Option<bool> {
        self.winrt_implementations
            .as_ref()
            .map(|implementations| implementations.contains(&entity))
    }

    /// Iterates projected values in deterministic namespace/name/entity order.
    fn values(&self) -> impl Iterator<Item = ValueItem<'_>> {
        self.winrt
            .iter()
            .filter(|entry| matches!(entry.kind, WinrtKind::Enum | WinrtKind::Struct))
            .map(|entry| ValueItem {
                definition: self.shared.database.definition(entry.entity).unwrap(),
                #[cfg(test)]
                kind: entry.kind,
            })
    }
}

fn winrt_entries(database: &Database) -> Result<Vec<(String, String, WinrtEntry)>, Error> {
    let mut entries = Vec::new();
    let mut selected = BTreeSet::new();
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
            TypeCategory::Interface => WinrtKind::Interface,
            TypeCategory::Class => WinrtKind::Class,
            _ => continue,
        };
        let namespace = definition.namespace()?;
        let name = definition.name()?;
        if !selected.insert((namespace.to_string(), name.to_string())) {
            continue;
        }
        let name = if matches!(
            kind,
            WinrtKind::Delegate | WinrtKind::Interface | WinrtKind::Class
        ) {
            name.split_once('`').map_or(name, |(name, _)| name)
        } else {
            name
        };
        entries.push((
            namespace.to_string(),
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

fn interface_relationships(
    database: &Database,
) -> Result<BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>, Error> {
    let mut result = BTreeMap::<Entity<TypeDef>, Vec<InterfaceRelationship>>::new();
    for relationship in database.interface_relationships() {
        let owner = relationship.owner()?;
        let identity = relationship.interface()?;
        let default = relationship.has_attribute("DefaultAttribute")?;
        let owner_name = format!("{}.{}", owner.namespace()?, owner.name()?);
        let relationship = match database.resolve_type(identity.file, identity.ty)? {
            TypeResolution::Specification(entity) => {
                let row = database.view(entity).unwrap();
                let signature = database
                    .image(entity.file())
                    .unwrap()
                    .type_signature(row.blob_id(0)?)?;
                match signature.kind {
                    TypeKind::GenericInstance { ty, arguments, .. } => {
                        InterfaceRelationship::from_resolution(
                            database.resolve_type(identity.file, ty)?,
                            identity.file,
                            arguments,
                            default,
                            owner_name,
                        )
                    }
                    _ => InterfaceRelationship::Invalid {
                        owner: owner_name,
                        message: "required interface specification is not generic",
                    },
                }
            }
            resolution => InterfaceRelationship::from_resolution(
                resolution,
                identity.file,
                Vec::new(),
                default,
                owner_name,
            ),
        };
        result.entry(owner.entity()).or_default().push(relationship);
    }
    Ok(result)
}

fn trim_generic_arity(name: &str) -> &str {
    name.split_once('`').map_or(name, |(name, _)| name)
}

fn is_agile(definition: TypeDefinition<'_>) -> Result<bool, Error> {
    if definition.namespace()? == "Windows.Foundation"
        && matches!(
            trim_generic_arity(definition.name()?),
            "IAsyncAction"
                | "IAsyncActionWithProgress"
                | "IAsyncOperation"
                | "IAsyncOperationWithProgress"
        )
    {
        return Ok(true);
    }
    let Some(attribute) = definition.find_attribute("MarshalingBehaviorAttribute")? else {
        return Ok(false);
    };
    Ok(attribute.arguments(&())?.iter().any(|argument| {
        matches!(
            argument,
            AttributeArgument::Fixed {
                value: AttributeValue::Enum { value, .. },
                ..
            } if matches!(value.as_ref(), AttributeValue::I32(2))
        )
    }))
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
