use super::*;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

struct Namespace {
    name: String,
    types: Vec<Entity<TypeDef>>,
    delegates: Vec<Entity<TypeDef>>,
    interfaces: Vec<(Entity<TypeDef>, MemberSelection)>,
    constants: Vec<Entity<Field>>,
    functions: Vec<Entity<MethodDef>>,
}

type NamedEntity<T> = (String, i32, Entity<T>);

#[derive(Default)]
struct NamespaceSelection {
    types: Vec<NamedEntity<TypeDef>>,
    delegates: Vec<NamedEntity<TypeDef>>,
    interfaces: Vec<(String, i32, Entity<TypeDef>, MemberSelection)>,
    constants: Vec<NamedEntity<Field>>,
    functions: Vec<NamedEntity<MethodDef>>,
}

type NamespaceSelections = BTreeMap<String, NamespaceSelection>;

enum EnumVariants {
    All,
    Names(BTreeSet<String>),
}

pub(crate) struct Win32Selection {
    namespaces: Vec<Namespace>,
    enum_variants: BTreeMap<Entity<TypeDef>, EnumVariants>,
    implementations: Option<BTreeSet<Entity<TypeDef>>>,
}

pub(crate) struct Win32Catalogs {
    definitions: Vec<NativeDefinition>,
    apis: Vec<NativeApis>,
    nested: BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    interface_bases: BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    dependencies: native::DependencyCache,
}

struct NativeDefinition {
    namespace: String,
    name: String,
    architectures: i32,
    entity: Entity<TypeDef>,
    kind: NativeKind,
}

enum NativeKind {
    Enum(Vec<String>),
    Struct,
    Delegate,
    Interface,
}

struct NativeApis {
    namespace: String,
    constants: Vec<NamedEntity<Field>>,
    functions: Vec<NamedEntity<MethodDef>>,
}

pub(crate) struct Win32Items<'a> {
    database: &'a Database,
    catalogs: &'a Win32Catalogs,
    selection: &'a Win32Selection,
}

impl Generator {
    pub(crate) fn win32_items(&self) -> Win32Items<'_> {
        Win32Items {
            database: &self.shared.database,
            catalogs: &self.shared.win32_catalogs,
            selection: &self.win32,
        }
    }
}

impl Win32Selection {
    pub(crate) fn new_with_catalogs(
        database: &Database,
        catalogs: Arc<Win32Catalogs>,
        filter: Option<&Filter>,
        implementations: Option<&Filter>,
        package: bool,
    ) -> Result<Self, Error> {
        let mut closure = filter.map(|_| {
            native_closure::Closure::new(
                database,
                &catalogs.dependencies,
                &catalogs.interface_bases,
                &catalogs.nested,
            )
        });
        let mut namespaces = NamespaceSelections::new();
        let mut enum_variants = BTreeMap::<Entity<TypeDef>, EnumVariants>::new();
        let mut selected_implementations = implementations.map(|_| BTreeSet::new());
        for definition in &catalogs.definitions {
            match &definition.kind {
                NativeKind::Enum(variants) => {
                    if filter.is_none_or(|filter| {
                        filter.includes(&definition.namespace, &definition.name)
                    }) {
                        if let Some(closure) = &mut closure {
                            closure.include_definition(definition.entity)?;
                        } else {
                            namespaces
                                .entry(definition.namespace.clone())
                                .or_default()
                                .types
                                .push((
                                    definition.name.clone(),
                                    definition.architectures,
                                    definition.entity,
                                ));
                        }
                        enum_variants.insert(definition.entity, EnumVariants::All);
                    } else if let Some(filter) = filter {
                        let names = variants
                            .iter()
                            .filter(|name| filter.includes(&definition.namespace, name))
                            .cloned()
                            .collect::<BTreeSet<_>>();
                        if !names.is_empty() {
                            closure
                                .as_mut()
                                .unwrap()
                                .include_definition(definition.entity)?;
                            enum_variants.insert(definition.entity, EnumVariants::Names(names));
                        }
                    }
                }
                NativeKind::Struct | NativeKind::Delegate | NativeKind::Interface => {
                    if matches!(&definition.kind, NativeKind::Interface)
                        && implementations.is_some_and(|filter| {
                            filter.includes(&definition.namespace, &definition.name)
                        })
                    {
                        selected_implementations
                            .as_mut()
                            .unwrap()
                            .insert(definition.entity);
                        if let Some(closure) = &mut closure {
                            closure.include_implementation(definition.entity);
                        }
                    }
                    if filter.is_none_or(|filter| {
                        filter.includes(&definition.namespace, &definition.name)
                    }) {
                        if let Some(closure) = &mut closure {
                            closure.include_definition(definition.entity)?;
                        } else {
                            let namespace =
                                namespaces.entry(definition.namespace.clone()).or_default();
                            let item = (
                                definition.name.clone(),
                                definition.architectures,
                                definition.entity,
                            );
                            match &definition.kind {
                                NativeKind::Struct => namespace.types.push(item),
                                NativeKind::Delegate => namespace.delegates.push(item),
                                NativeKind::Interface => namespace.interfaces.push((
                                    item.0,
                                    item.1,
                                    item.2,
                                    MemberSelection::All,
                                )),
                                NativeKind::Enum(_) => unreachable!(),
                            }
                        }
                    } else if matches!(&definition.kind, NativeKind::Interface)
                        && let Some(methods) = filter.and_then(|filter| {
                            filter.methods(&definition.namespace, &definition.name)
                        })
                    {
                        closure.as_mut().unwrap().include_interface(
                            definition.entity,
                            MemberSelection::Names(methods.clone()),
                        );
                    }
                }
            }
        }
        for apis in &catalogs.apis {
            for (name, architectures, entity) in &apis.constants {
                if filter.is_none_or(|filter| filter.includes(&apis.namespace, name)) {
                    if let Some(closure) = &mut closure {
                        closure.include_field(database.field(*entity).unwrap(), &apis.namespace)?;
                    }
                    namespaces
                        .entry(apis.namespace.clone())
                        .or_default()
                        .constants
                        .push((name.clone(), *architectures, *entity));
                }
            }
            for (name, architectures, entity) in &apis.functions {
                if filter.is_none_or(|filter| {
                    filter.includes(&apis.namespace, name)
                        || native_function::window_long_alias(name)
                            .is_some_and(|alias| filter.includes(&apis.namespace, alias))
                }) {
                    if let Some(closure) = &mut closure {
                        closure
                            .include_method(database.method(*entity).unwrap(), &apis.namespace)?;
                    }
                    namespaces
                        .entry(apis.namespace.clone())
                        .or_default()
                        .functions
                        .push((name.clone(), *architectures, *entity));
                }
            }
        }
        if let Some(closure) = closure {
            let (entities, interface_members) = closure.finish()?;
            for entity in entities {
                let definition = database.definition(entity).unwrap();
                if definition.category()? == TypeCategory::Enum {
                    enum_variants
                        .entry(entity)
                        .or_insert_with(|| EnumVariants::Names(BTreeSet::new()));
                }
                if definition.category()? == TypeCategory::Interface {
                    add_interface(
                        &mut namespaces,
                        definition,
                        interface_members.get(&entity).unwrap().clone(),
                    )?;
                } else {
                    add_definition(&mut namespaces, definition)?;
                }
            }
        }
        let namespaces: Vec<Namespace> = namespaces
            .into_iter()
            .map(|(name, mut selection)| {
                selection.types.sort();
                if package {
                    selection
                        .types
                        .dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
                }
                selection.delegates.sort();
                if package {
                    selection
                        .delegates
                        .dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
                }
                selection.interfaces.sort_by(|left, right| {
                    (&left.0, left.1, left.2).cmp(&(&right.0, right.1, right.2))
                });
                if package {
                    selection
                        .interfaces
                        .dedup_by(|left, right| left.0 == right.0);
                }
                selection.constants.sort();
                if package {
                    selection
                        .constants
                        .dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
                }
                selection.functions.sort();
                if package {
                    selection
                        .functions
                        .dedup_by(|left, right| left.0 == right.0 && left.1 == right.1);
                }
                let types = selection.types;
                let delegates = selection.delegates;
                let interfaces = selection.interfaces;
                let constants = selection.constants;
                let functions = selection.functions;
                Namespace {
                    name,
                    types: types.into_iter().map(|(_, _, entity)| entity).collect(),
                    delegates: delegates.into_iter().map(|(_, _, entity)| entity).collect(),
                    interfaces: interfaces
                        .into_iter()
                        .map(|(_, _, entity, members)| (entity, members))
                        .collect(),
                    constants: constants.into_iter().map(|(_, _, entity)| entity).collect(),
                    functions: functions.into_iter().map(|(_, _, entity)| entity).collect(),
                }
            })
            .collect();
        Ok(Self {
            namespaces,
            enum_variants,
            implementations: selected_implementations,
        })
    }

    fn implements(&self, entity: Entity<TypeDef>) -> Option<bool> {
        self.implementations
            .as_ref()
            .map(|implementations| implementations.contains(&entity))
    }
}

impl Win32Catalogs {
    pub(crate) fn new(database: &Database) -> Result<Self, Error> {
        let mut definitions = Vec::new();
        let mut apis = Vec::new();
        let mut sys_namespaces = BTreeSet::new();
        for definition in database.definitions() {
            if definition.is_windows_runtime()? {
                continue;
            }
            let namespace = definition.namespace()?.to_string();
            let name = definition.name()?.to_string();
            let entity = definition.entity();
            let architectures = definition.architectures()?;
            let kind = match definition.category()? {
                TypeCategory::Enum => {
                    let mut variants = Vec::new();
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            variants.push(field.name()?.to_string());
                        }
                    }
                    NativeKind::Enum(variants)
                }
                TypeCategory::Struct => NativeKind::Struct,
                TypeCategory::Delegate => NativeKind::Delegate,
                TypeCategory::Interface => NativeKind::Interface,
                TypeCategory::Class if name == "Apis" => {
                    let constants = definition
                        .fields()?
                        .map(|field| {
                            Ok((
                                field.name()?.to_string(),
                                field.architectures()?,
                                field.entity(),
                            ))
                        })
                        .collect::<Result<_, Error>>()?;
                    let mut functions = Vec::new();
                    for method in definition.methods()? {
                        if let Some(import) = method.import()?
                            && (import.module() == "FORCEINLINE" || import.name().starts_with('#'))
                        {
                            continue;
                        }
                        functions.push((
                            method.name()?.to_string(),
                            method.architectures()?,
                            method.entity(),
                        ));
                    }
                    apis.push(NativeApis {
                        namespace: namespace.clone(),
                        constants,
                        functions,
                    });
                    sys_namespaces.insert(namespace);
                    continue;
                }
                _ => continue,
            };
            if !matches!(kind, NativeKind::Interface) {
                sys_namespaces.insert(namespace.clone());
            }
            definitions.push(NativeDefinition {
                namespace,
                name,
                architectures,
                entity,
                kind,
            });
        }
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
        let interface_bases = interface_bases(database)?;
        let dependencies =
            native::DependencyCache::new(database, &interface_bases, sys_namespaces)?;
        Ok(Self {
            definitions,
            apis,
            nested,
            interface_bases,
            dependencies,
        })
    }

    #[cfg(test)]
    pub(crate) fn nested_type_count(&self) -> usize {
        self.nested.values().map(Vec::len).sum()
    }
}

fn add_definition(
    namespaces: &mut NamespaceSelections,
    definition: TypeDefinition<'_>,
) -> Result<(), Error> {
    let item = (
        definition.name()?.to_string(),
        definition.architectures()?,
        definition.entity(),
    );
    let namespace = namespaces
        .entry(definition.namespace()?.to_string())
        .or_default();
    match definition.category()? {
        TypeCategory::Enum | TypeCategory::Struct => namespace.types.push(item),
        TypeCategory::Delegate => namespace.delegates.push(item),
        TypeCategory::Interface => {
            namespace
                .interfaces
                .push((item.0, item.1, item.2, MemberSelection::All));
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn add_interface(
    namespaces: &mut NamespaceSelections,
    definition: TypeDefinition<'_>,
    members: MemberSelection,
) -> Result<(), Error> {
    namespaces
        .entry(definition.namespace()?.to_string())
        .or_default()
        .interfaces
        .push((
            definition.name()?.to_string(),
            definition.architectures()?,
            definition.entity(),
            members,
        ));
    Ok(())
}

fn interface_bases(
    database: &Database,
) -> Result<BTreeMap<Entity<TypeDef>, Vec<(String, String)>>, Error> {
    let mut result = BTreeMap::<Entity<TypeDef>, Vec<(String, String)>>::new();
    for relationship in database.interface_implementations() {
        let (definition, interface) = relationship?;
        if definition.is_windows_runtime()? {
            continue;
        }
        let Some((namespace, name)) = database.type_name(interface.file, interface.ty)? else {
            return Err(Error::UnsupportedType {
                name: format!("{}.{}", definition.namespace()?, definition.name()?),
                shape: "generic native interface base".to_string(),
            });
        };
        result
            .entry(definition.entity())
            .or_default()
            .push((namespace.to_string(), name.to_string()));
    }
    Ok(result)
}

impl<'a> Win32Items<'a> {
    #[cfg(test)]
    pub(crate) fn native_types(&self) -> impl Iterator<Item = Result<NativeType, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.types.iter().map(|entity| {
                NativeType::lower_filtered(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                    &self.catalogs.nested,
                    self.enum_variants(*entity),
                )
            })
        })
    }

    /// Lowers constants in deterministic namespace and name order.
    #[cfg(test)]
    pub(crate) fn constants(&self) -> impl Iterator<Item = Result<Constant, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.constants.iter().map(|entity| {
                let field = self.database.field(*entity).unwrap();
                Constant::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    field,
                    &namespace.name,
                    field.name().unwrap(),
                )
            })
        })
    }

    /// Lowers functions in deterministic namespace and name order.
    #[cfg(test)]
    pub(crate) fn functions(&self) -> impl Iterator<Item = Result<Function, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.functions.iter().map(|entity| {
                let method = self.database.method(*entity).unwrap();
                Function::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    method,
                    &namespace.name,
                    method.name().unwrap(),
                )
            })
        })
    }

    /// Lowers native delegates in deterministic namespace and name order.
    #[cfg(test)]
    pub(crate) fn delegates(&self) -> impl Iterator<Item = Result<Delegate, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.delegates.iter().map(|entity| {
                Delegate::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                )
            })
        })
    }

    /// Lowers native interfaces in deterministic namespace and name order.
    #[cfg(test)]
    pub(crate) fn interfaces(&self) -> impl Iterator<Item = Result<NativeInterface, Error>> + '_ {
        self.selection.namespaces.iter().flat_map(|namespace| {
            namespace.interfaces.iter().map(|(entity, _)| {
                NativeInterface::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    self.database.definition(*entity).unwrap(),
                    &self.catalogs.interface_bases,
                )
            })
        })
    }

    /// Lowers a uniquely named constant.
    #[cfg(test)]
    pub(crate) fn constant(&self, namespace: &str, name: &str) -> Result<Constant, Error> {
        let entity = self.constant_entity(namespace, name)?;
        Constant::lower(
            self.database,
            &self.catalogs.dependencies,
            self.database.field(entity).unwrap(),
            namespace,
            name,
        )
    }

    /// Lowers a uniquely named function.
    #[cfg(test)]
    pub(crate) fn function(&self, namespace: &str, name: &str) -> Result<Function, Error> {
        let entity = self.function_entity(namespace, name)?;
        Function::lower(
            self.database,
            &self.catalogs.dependencies,
            self.database.method(entity).unwrap(),
            namespace,
            name,
        )
    }

    /// Lowers a uniquely named native type definition.
    #[cfg(test)]
    pub(crate) fn native_type(&self, namespace: &str, name: &str) -> Result<NativeType, Error> {
        let entity = self.type_entity(namespace, name)?;
        NativeType::lower_filtered(
            self.database,
            &self.catalogs.dependencies,
            self.database.definition(entity).unwrap(),
            &self.catalogs.nested,
            self.enum_variants(entity),
        )
    }

    pub(super) fn render(
        &self,
        layout: Layout,
        projection: Projection,
        derives: &BTreeMap<String, BTreeSet<String>>,
        mut add: impl FnMut(&str, &str, u8, i32, proc_macro2::TokenStream, BTreeSet<String>),
    ) -> Result<(), Error> {
        let mut implementable_interfaces = BTreeSet::new();
        loop {
            let mut changed = false;
            for namespace in &self.selection.namespaces {
                for (entity, members) in &namespace.interfaces {
                    let definition = self.database.definition(*entity).unwrap();
                    let interface = NativeInterface::lower(
                        self.database,
                        &self.catalogs.dependencies,
                        definition,
                        &self.catalogs.interface_bases,
                    )?;
                    let base_selected = interface.base_name().is_some_and(|(namespace, name)| {
                        implementable_interfaces
                            .contains(&(namespace.to_string(), name.to_string()))
                    });
                    if if layout.is_package() {
                        interface.can_implement_package(members, base_selected)
                    } else {
                        interface.can_implement(members, base_selected)
                    } {
                        changed |= implementable_interfaces
                            .insert((namespace.name.clone(), definition.name()?.to_string()));
                    }
                }
            }
            if !changed {
                break;
            }
        }
        for namespace in &self.selection.namespaces {
            for entity in &namespace.types {
                let definition = self.database.definition(*entity).unwrap();
                let ty = NativeType::lower_filtered(
                    self.database,
                    &self.catalogs.dependencies,
                    definition,
                    &self.catalogs.nested,
                    self.enum_variants(*entity),
                )?;
                if layout.is_package() && projection.is_sys() && !ty.supports_package_sys() {
                    continue;
                }
                let derives = derives
                    .get(definition.name()?)
                    .map(|derives| derives.iter().cloned().collect::<Vec<_>>())
                    .unwrap_or_default();
                let features = ty.package_features(layout, projection);
                for (name, kind, tokens) in ty.write_items_context(layout, projection, &derives) {
                    add(
                        &namespace.name,
                        name,
                        kind,
                        definition.architectures()?,
                        tokens,
                        features.clone(),
                    );
                }
            }
            for entity in &namespace.delegates {
                let definition = self.database.definition(*entity).unwrap();
                let delegate =
                    Delegate::lower(self.database, &self.catalogs.dependencies, definition)?;
                if layout.is_package() && projection.is_sys() && !delegate.supports_package_sys() {
                    continue;
                }
                add(
                    &namespace.name,
                    definition.name()?,
                    1,
                    definition.architectures()?,
                    delegate.write_context(layout, projection),
                    delegate.package_features(layout, projection),
                );
            }
            for (entity, members) in &namespace.interfaces {
                let definition = self.database.definition(*entity).unwrap();
                if layout.is_package() && projection.is_sys() {
                    continue;
                }
                if !projection.is_sys()
                    && native::is_core_projection(&namespace.name, definition.name()?)
                {
                    continue;
                }
                let interface = NativeInterface::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    definition,
                    &self.catalogs.interface_bases,
                )?;
                let base_selected = interface.base_name().is_some_and(|(namespace, name)| {
                    implementable_interfaces.contains(&(namespace.to_string(), name.to_string()))
                });
                let features = interface.package_features(layout);
                add(
                    &namespace.name,
                    definition.name()?,
                    1,
                    definition.architectures()?,
                    interface.write_context(
                        layout,
                        projection,
                        members,
                        self.selection.implements(*entity),
                        base_selected,
                    )?,
                    features,
                );
            }
            for entity in &namespace.constants {
                let field = self.database.field(*entity).unwrap();
                let name = field.name()?;
                let constant = Constant::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    field,
                    &namespace.name,
                    name,
                )?;
                if layout.is_package() && projection.is_sys() && !constant.supports_package_sys() {
                    continue;
                }
                add(
                    &namespace.name,
                    name,
                    2,
                    field.architectures()?,
                    constant.write_context(layout, projection),
                    constant.package_features(layout, projection),
                );
            }
            for entity in &namespace.functions {
                let method = self.database.method(*entity).unwrap();
                let name = method.name()?;
                let function = Function::lower(
                    self.database,
                    &self.catalogs.dependencies,
                    method,
                    &namespace.name,
                    name,
                )?;
                if layout.is_package() && projection.is_sys() && !function.supports_package_sys() {
                    continue;
                }
                add(
                    &namespace.name,
                    name,
                    3,
                    method.architectures()?,
                    function.write_context(layout, projection),
                    function.package_features(layout, projection),
                );
            }
        }
        Ok(())
    }

    #[cfg(test)]
    fn type_entity(&self, namespace: &str, name: &str) -> Result<Entity<TypeDef>, Error> {
        let Some(namespace) = self
            .selection
            .namespaces
            .iter()
            .find(|item| item.name == namespace)
        else {
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

    fn enum_variants(&self, entity: Entity<TypeDef>) -> Option<&BTreeSet<String>> {
        match self.selection.enum_variants.get(&entity) {
            Some(EnumVariants::Names(names)) => Some(names),
            Some(EnumVariants::All) | None => None,
        }
    }

    #[cfg(test)]
    fn constant_entity(&self, namespace: &str, name: &str) -> Result<Entity<Field>, Error> {
        let Some(namespace) = self
            .selection
            .namespaces
            .iter()
            .find(|item| item.name == namespace)
        else {
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

    #[cfg(test)]
    fn function_entity(&self, namespace: &str, name: &str) -> Result<Entity<MethodDef>, Error> {
        let Some(namespace) = self
            .selection
            .namespaces
            .iter()
            .find(|item| item.name == namespace)
        else {
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

#[cfg(test)]
fn unique_entity<T: windows_metadata2::Table>(
    mut matches: impl Iterator<Item = Entity<T>>,
    namespace: &str,
    name: &str,
) -> Result<Entity<T>, Error> {
    let Some(result) = matches.next() else {
        return Err(missing(namespace, name));
    };
    if matches.next().is_some() {
        return Err(Error::InvalidType {
            name: format!("{namespace}.{name}"),
            message: "Win32 item is not unique",
        });
    }
    Ok(result)
}

#[cfg(test)]
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
        let catalogs = Arc::new(Win32Catalogs::new(&database).unwrap());
        let selection =
            Win32Selection::new_with_catalogs(&database, catalogs.clone(), None, None, false)
                .unwrap();
        let items = Win32Items {
            database: &database,
            catalogs: &catalogs,
            selection: &selection,
        };
        let mut supported = [0; 5];
        let mut delegate_supported = 0;
        let mut interface_supported = 0;
        let mut defaults = [0; 5];
        let mut scoped_enums = 0;
        let mut gated_scoped_enums = 0;
        let mut unsupported = BTreeMap::<String, usize>::new();

        for namespace in &items.selection.namespaces {
            for entity in &namespace.types {
                let definition = database.definition(*entity).unwrap();
                if definition.category().unwrap() == TypeCategory::Enum
                    && definition.has_attribute("ScopedEnumAttribute").unwrap()
                {
                    scoped_enums += 1;
                    if definition.architectures().unwrap() != 0 {
                        gated_scoped_enums += 1;
                    }
                }
                match NativeType::lower_filtered(
                    &database,
                    &items.catalogs.dependencies,
                    definition,
                    &items.catalogs.nested,
                    None,
                ) {
                    Ok(ty) => {
                        ty.write_sys();
                        if let Some(policy) = ty.default_policy() {
                            defaults[match policy {
                                native_default::Policy::Derive => 0,
                                native_default::Policy::ExplicitLayout => 1,
                                native_default::Policy::FixedArray => 2,
                                native_default::Policy::TypedefArray => 3,
                                native_default::Policy::ScopedEnum => 4,
                            }] += 1;
                        }
                        supported[match ty.kind() {
                            NativeTypeKind::Alias => 0,
                            NativeTypeKind::Enum => 1,
                            NativeTypeKind::Struct => 2,
                        }] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.delegates {
                let definition = database.definition(*entity).unwrap();
                match Delegate::lower(&database, &items.catalogs.dependencies, definition) {
                    Ok(delegate) => {
                        delegate.write_sys();
                        delegate_supported += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for (entity, _) in &namespace.interfaces {
                let definition = database.definition(*entity).unwrap();
                match NativeInterface::lower(
                    &database,
                    &items.catalogs.dependencies,
                    definition,
                    &items.catalogs.interface_bases,
                ) {
                    Ok(interface) => {
                        interface.write_sys();
                        interface_supported += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.constants {
                let field = database.field(*entity).unwrap();
                match Constant::lower(
                    &database,
                    &items.catalogs.dependencies,
                    field,
                    &namespace.name,
                    field.name().unwrap(),
                ) {
                    Ok(constant) => {
                        constant.write_sys();
                        supported[3] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
            for entity in &namespace.functions {
                let method = database.method(*entity).unwrap();
                match Function::lower(
                    &database,
                    &items.catalogs.dependencies,
                    method,
                    &namespace.name,
                    method.name().unwrap(),
                ) {
                    Ok(function) => {
                        function.write_sys();
                        supported[4] += 1;
                    }
                    Err(error) => *unsupported.entry(classify(error)).or_default() += 1,
                }
            }
        }

        assert!(unsupported.is_empty(), "{unsupported:#?}");
        assert_eq!(supported[..3], [12_667, 4_728, 12_714]);
        assert_eq!(supported[3..], [83_641, 14_559]);
        assert_eq!(delegate_supported, 2_159);
        assert_eq!(interface_supported, 4_290);
        assert_eq!(items.native_types().count(), 30_109);
        assert_eq!(items.delegates().count(), 2_159);
        assert_eq!(defaults, [8_584, 2_164, 1_889, 74, 3]);
        assert_eq!((scoped_enums, gated_scoped_enums), (10, 0));
    }

    #[test]
    fn inventory_architecture_variants_and_nested_types() {
        let database = Database::new([Image::new(windows_default::WIN32).unwrap()]).unwrap();
        let catalogs = Arc::new(Win32Catalogs::new(&database).unwrap());
        let selection =
            Win32Selection::new_with_catalogs(&database, catalogs.clone(), None, None, false)
                .unwrap();
        let items = Win32Items {
            database: &database,
            catalogs: &catalogs,
            selection: &selection,
        };
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
        let architecture_type_count = selection
            .namespaces
            .iter()
            .flat_map(|namespace| &namespace.types)
            .filter(|entity| {
                database
                    .definition(**entity)
                    .unwrap()
                    .architectures()
                    .unwrap()
                    != 0
            })
            .count();
        let architecture_constant_count = selection
            .namespaces
            .iter()
            .flat_map(|namespace| &namespace.constants)
            .filter(|entity| database.field(**entity).unwrap().architectures().unwrap() != 0)
            .count();
        let architecture_function_count = selection
            .namespaces
            .iter()
            .flat_map(|namespace| &namespace.functions)
            .filter(|entity| database.method(**entity).unwrap().architectures().unwrap() != 0)
            .count();
        let architecture_interface_count = selection
            .namespaces
            .iter()
            .flat_map(|namespace| namespace.interfaces.iter().map(|(entity, _)| entity))
            .filter(|entity| {
                database
                    .definition(**entity)
                    .unwrap()
                    .architectures()
                    .unwrap()
                    != 0
            })
            .count();
        assert_eq!(
            (
                architecture_rows,
                architecture_groups.len(),
                variant_groups,
                nested_rows,
                architecture_type_count,
                architecture_constant_count,
                architecture_function_count,
                architecture_interface_count,
                items.catalogs.nested_type_count(),
            ),
            (1_054, 671, 374, 2_633, 997, 512, 261, 14, 2_633)
        );
        assert_eq!(
            items.catalogs.nested.values().map(Vec::len).sum::<usize>(),
            nested_rows
        );
        assert_eq!(items.catalogs.nested.len(), 1_925);
    }

    #[test]
    fn inventory_remaining_native_surfaces() {
        let database = Database::new([Image::new(windows_default::WIN32).unwrap()]).unwrap();
        let mut delegates = 0;
        let mut gated_delegates = 0;
        let mut interfaces = 0;
        let mut gated_interfaces = 0;
        let mut interface_methods = 0;
        let mut bitfield_structs = 0;
        let mut bitfield_members = 0;
        let mut direct_handle_shapes = 0;
        let mut direct_handle_shapes_without_typedef = 0;
        let mut void_typedef_shapes = 0;

        for definition in database.definitions() {
            if definition.is_windows_runtime().unwrap() {
                continue;
            }
            match definition.category().unwrap() {
                TypeCategory::Delegate => {
                    delegates += 1;
                    if definition.architectures().unwrap() != 0 {
                        gated_delegates += 1;
                    }
                }
                TypeCategory::Interface => {
                    interfaces += 1;
                    interface_methods += definition.methods().unwrap().count();
                    if definition.architectures().unwrap() != 0 {
                        gated_interfaces += 1;
                    }
                }
                TypeCategory::Struct => {
                    let mut definition_bitfields = 0;
                    let fields = definition
                        .fields()
                        .unwrap()
                        .filter(|field| !field.is_literal().unwrap())
                        .collect::<Vec<_>>();
                    for field in &fields {
                        definition_bitfields += field
                            .attributes()
                            .unwrap()
                            .filter(|attribute| {
                                attribute.name().unwrap() == Some("NativeBitfieldAttribute")
                            })
                            .count();
                    }
                    if definition_bitfields != 0 {
                        bitfield_structs += 1;
                        bitfield_members += definition_bitfields;
                    }
                    if let [field] = fields.as_slice()
                        && field.name().unwrap() == "Value"
                    {
                        match field.signature().unwrap().kind {
                            TypeKind::Void => {
                                void_typedef_shapes += 1;
                            }
                            TypeKind::Boolean
                            | TypeKind::Char
                            | TypeKind::I8
                            | TypeKind::U8
                            | TypeKind::I16
                            | TypeKind::U16
                            | TypeKind::I32
                            | TypeKind::U32
                            | TypeKind::I64
                            | TypeKind::U64
                            | TypeKind::F32
                            | TypeKind::F64
                            | TypeKind::ISize
                            | TypeKind::USize
                            | TypeKind::Pointer(_) => {
                                direct_handle_shapes += 1;
                                if !definition.has_attribute("NativeTypedefAttribute").unwrap() {
                                    direct_handle_shapes_without_typedef += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        assert_eq!(
            (
                delegates,
                gated_delegates,
                interfaces,
                gated_interfaces,
                interface_methods,
                bitfield_structs,
                bitfield_members,
                direct_handle_shapes,
                direct_handle_shapes_without_typedef,
                void_typedef_shapes,
            ),
            (2_159, 43, 4_290, 14, 25_868, 218, 1_228, 11_264, 1, 6)
        );
    }

    fn classify(error: Error) -> String {
        match error {
            Error::UnsupportedType { shape, .. } if shape.starts_with("typed constant ") => {
                "typed constant".to_string()
            }
            Error::UnsupportedType { shape, .. } => {
                shape.split(['(', ' ', '<']).next().unwrap().to_string()
            }
            Error::InvalidType { message, .. } => message.to_string(),
            Error::Metadata(error) => format!("metadata: {error}"),
            other => other.to_string(),
        }
    }
}
