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

mod canonical;
mod enum_model;
mod error;
mod external;
mod filter;
mod format;
mod guid;
mod model;
mod native;
mod native_closure;
mod native_com;
mod native_com_producer;
mod native_constant;
mod native_default;
mod native_delegate;
mod native_function;
mod native_function_call;
mod native_interface;
mod native_signature;
mod native_type;
mod output;
mod struct_model;
mod tokens;
mod ty;
mod win32;
mod win32_catalog;
#[cfg(test)]
mod win32_test;
mod winrt_catalog;
mod winrt_class;
mod winrt_class_type;
mod winrt_collection;
mod winrt_delegate;
mod winrt_dependency;
mod winrt_interface;
mod winrt_method;

pub use build::{Bindgen, builder, command_file};
use enum_model::Enum;
pub use error::Error;
pub use filter::Filter;
pub use format::format;
use model::{Value, Values};
use native_constant::Constant;
use native_delegate::Delegate;
use native_function::Function;
use native_interface::NativeInterface;
use native_type::NativeType;
#[cfg(test)]
use native_type::NativeTypeKind;
use struct_model::Struct;

mod build;

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
    /// Emit one file per metadata namespace for a package crate.
    Package,
}

impl Layout {
    const fn is_flat(self) -> bool {
        matches!(self, Self::Flat)
    }

    const fn is_package(self) -> bool {
        matches!(self, Self::Package)
    }

    fn package_crate(self, namespace: &str, name: &str) -> Option<&'static str> {
        self.is_package()
            .then(|| external::package_crate_name(namespace, name))
            .flatten()
    }

    fn winrt_crate(self, current: &str, namespace: &str, name: &str) -> Option<&'static str> {
        if self.is_package() {
            external::package_crate_name(namespace, name)
        } else if current != namespace {
            external::winrt_crate(namespace, name)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ProjectionStyle {
    Sys,
    #[default]
    Default,
    Minimal,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Visibility {
    #[default]
    Public,
    Crate,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Projection {
    style: ProjectionStyle,
    visibility: Visibility,
}

#[allow(non_upper_case_globals)]
impl Projection {
    const Sys: Self = Self {
        style: ProjectionStyle::Sys,
        visibility: Visibility::Public,
    };
    const Default: Self = Self {
        style: ProjectionStyle::Default,
        visibility: Visibility::Public,
    };
    const Minimal: Self = Self {
        style: ProjectionStyle::Minimal,
        visibility: Visibility::Crate,
    };
    const fn minimal_public() -> Self {
        Self {
            style: ProjectionStyle::Minimal,
            visibility: Visibility::Public,
        }
    }

    const fn is_sys(self) -> bool {
        matches!(self.style, ProjectionStyle::Sys)
    }

    const fn is_minimal(self) -> bool {
        matches!(self.style, ProjectionStyle::Minimal)
    }

    const fn has_public_methods(self) -> bool {
        matches!(self.visibility, Visibility::Public)
    }
}

/// Selection policy for one generation request.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Request {
    filter: Option<Filter>,
    winrt_implementations: Option<Filter>,
    native_implementations: Option<Filter>,
    derives: BTreeMap<String, BTreeSet<String>>,
    preserve_field_names: bool,
    implement_all: bool,
    package: bool,
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
            winrt_implementations: None,
            native_implementations: None,
            derives: BTreeMap::new(),
            preserve_field_names: false,
            implement_all: false,
            package: false,
            projection: Projection::Default,
        }
    }

    /// Selects interfaces that require implementation traits and typed ABI vtables.
    pub fn implementations(mut self, filter: Filter) -> Self {
        self.winrt_implementations = Some(filter.clone());
        self.native_implementations = Some(filter);
        self.implement_all = false;
        self
    }

    /// Preserves WinRT metadata field names instead of converting them to Rust style.
    pub fn preserve_field_names(mut self) -> Self {
        self.preserve_field_names = true;
        self
    }

    /// Adds a derived trait to a generated native type.
    pub fn derive(mut self, name: impl Into<String>, derive: impl Into<String>) -> Self {
        self.derives
            .entry(name.into())
            .or_default()
            .insert(derive.into());
        self
    }

    /// Emits implementation traits for every selected interface.
    pub fn implement_all(mut self) -> Self {
        self.winrt_implementations = None;
        self.native_implementations = None;
        self.implement_all = true;
        self
    }

    /// Selects raw ABI-oriented output.
    pub fn sys(mut self) -> Self {
        self.projection = Projection::Sys;
        self
    }

    /// Selects the minimal projection used by focused library crates.
    pub fn minimal(mut self) -> Self {
        self.projection = Projection::Minimal;
        self
    }

    fn minimal_public(mut self) -> Self {
        self.projection = Projection::minimal_public();
        self
    }

    fn package(mut self) -> Self {
        self.package = true;
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

    fn emits_implementation(&self, projection: Projection) -> bool {
        let _ = projection;
        matches!(self, Self::All)
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
    winrt_explicit_items: BTreeSet<Entity<TypeDef>>,
    winrt_implementations: Option<BTreeSet<Entity<TypeDef>>>,
    win32: win32::Win32Selection,
    derives: BTreeMap<String, BTreeSet<String>>,
    preserve_field_names: bool,
    projection: Projection,
}

struct Shared {
    database: Database,
    winrt_entries: Vec<(String, String, WinrtEntry)>,
    values: Values,
    winrt_catalogs: winrt_catalog::Catalogs,
    winrt_artifacts: winrt_dependency::ArtifactGraph,
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
        let winrt_catalogs =
            winrt_catalog::Catalogs::new(&database, &winrt_entries, &interface_relationships)?;
        let winrt_artifacts =
            winrt_dependency::ArtifactGraph::new(&winrt_entries, &values, &winrt_catalogs)?;
        let win32_catalogs = Arc::new(win32::Win32Catalogs::new(&database)?);
        Ok(Self {
            shared: Arc::new(Shared {
                database,
                winrt_entries,
                values,
                winrt_catalogs,
                winrt_artifacts,
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
        let (mut winrt, winrt_members) = if let Some(filter) = filter {
            Self::close_winrt(
                &shared,
                filter,
                request.winrt_implementations.as_ref(),
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
        if request.package {
            winrt.retain(|entry| {
                let definition = shared.database.definition(entry.entity).unwrap();
                external::package_crate_name(
                    definition.namespace().unwrap(),
                    trim_generic_arity(definition.name().unwrap()),
                )
                .is_none()
            });
        }
        let win32 = win32::Win32Selection::new_with_catalogs(
            &shared.database,
            shared.win32_catalogs.clone(),
            filter,
            request.native_implementations.as_ref(),
            request.implement_all,
            request.package,
        )?;
        let winrt_implementations = if request.implement_all {
            Some(
                winrt
                    .iter()
                    .filter_map(|entry| {
                        if !matches!(entry.kind, WinrtKind::Interface) {
                            return None;
                        }
                        let definition = shared.database.definition(entry.entity).unwrap();
                        request
                            .filter
                            .as_ref()
                            .is_none_or(|filter| {
                                filter.includes(
                                    definition.namespace().unwrap(),
                                    trim_generic_arity(definition.name().unwrap()),
                                )
                            })
                            .then_some(entry.entity)
                    })
                    .collect(),
            )
        } else {
            request
                .winrt_implementations
                .as_ref()
                .map(|implementations| {
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
                })
        };
        let winrt_explicit_items = filter
            .map(|filter| {
                shared
                    .winrt_entries
                    .iter()
                    .filter_map(|(namespace, name, entry)| {
                        filter
                            .includes_exact_type(namespace, name)
                            .then_some(entry.entity)
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            shared,
            winrt,
            winrt_members,
            winrt_explicit_items,
            winrt_implementations,
            win32,
            derives: request.derives.clone(),
            preserve_field_names: request.preserve_field_names,
            projection: request.projection,
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
                if matches!(entry.kind, WinrtKind::Class)
                    && filter.includes_exact_item(namespace, name)
                {
                    Some(MemberSelection::Names(
                        filter.methods(namespace, name).cloned().unwrap_or_default(),
                    ))
                } else {
                    Some(MemberSelection::All)
                }
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
                    shared
                        .winrt_catalogs
                        .delegate(entry.entity)
                        .direct_selection_dependencies(),
                    BTreeMap::new(),
                ),
                WinrtKind::Interface => {
                    let model = shared.winrt_catalogs.interface(entry.entity);
                    let implementation_dependencies = implemented
                        || (implementations.is_none()
                            && model.implicitly_implements(&members, projection));
                    (
                        model.selection_dependencies(&members, implementation_dependencies),
                        model.relationship_members(&members),
                    )
                }
                WinrtKind::Class => {
                    let model = shared.winrt_catalogs.class(entry.entity);
                    (
                        model.selection_dependencies(&members, implementations, projection),
                        model.relationship_members(&members, implementations, projection),
                    )
                }
                WinrtKind::Enum => continue,
            };
            for (namespace, name) in dependencies {
                if filter.excludes(&namespace, &name) {
                    continue;
                }
                if !filter.includes(&namespace, &name)
                    && (external::winrt_crate(&namespace, &name).is_some()
                        || (projection.is_minimal()
                            && external::minimal_crate(&namespace, &name).is_some()))
                {
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
                            let preserve_explicit_members =
                                filter.methods(&namespace, &name).is_some()
                                    && matches!(current, MemberSelection::Names(_))
                                    && matches!(members, MemberSelection::All);
                            if !preserve_explicit_members && current.merge(members) {
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
