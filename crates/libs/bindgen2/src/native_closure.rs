use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Closure<'a> {
    database: &'a Database,
    dependencies: &'a native::DependencyCache,
    interface_bases: &'a BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    nested: &'a BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    selected: BTreeSet<Entity<TypeDef>>,
    interface_members: BTreeMap<Entity<TypeDef>, MemberSelection>,
    implementations: BTreeSet<Entity<TypeDef>>,
    namespaces: BTreeMap<Entity<TypeDef>, String>,
    pending: Vec<Entity<TypeDef>>,
}

impl<'a> Closure<'a> {
    pub(super) fn new(
        database: &'a Database,
        dependencies: &'a native::DependencyCache,
        interface_bases: &'a BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
        nested: &'a BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    ) -> Self {
        let parents = database
            .nested_types()
            .map(|(child, parent)| (child.entity(), parent.entity()))
            .collect::<BTreeMap<_, _>>();
        let mut namespaces: BTreeMap<Entity<TypeDef>, String> = database
            .definitions()
            .map(|definition| {
                let entity = definition.entity();
                let mut current = entity;
                let namespace = loop {
                    let definition = database.definition(current).unwrap();
                    let namespace = definition.namespace().unwrap();
                    if !namespace.is_empty() {
                        break namespace.to_string();
                    }
                    let Some(parent) = parents.get(&current) else {
                        break String::new();
                    };
                    current = *parent;
                };
                (entity, namespace)
            })
            .collect();
        for entity in parents.keys() {
            let mut current = *entity;
            let namespace = loop {
                let definition = database.definition(current).unwrap();
                let namespace = definition.namespace().unwrap();
                if !namespace.is_empty() {
                    break namespace.to_string();
                }
                let Some(parent) = parents.get(&current) else {
                    break String::new();
                };
                current = *parent;
            };
            namespaces.insert(*entity, namespace);
        }
        Self {
            database,
            dependencies,
            interface_bases,
            nested,
            selected: BTreeSet::new(),
            interface_members: BTreeMap::new(),
            implementations: BTreeSet::new(),
            namespaces,
            pending: Vec::new(),
        }
    }

    pub(super) fn include_definition(&mut self, entity: Entity<TypeDef>) -> Result<(), Error> {
        let definition = self.database.definition(entity).unwrap();
        if definition.is_windows_runtime()?
            || !matches!(
                definition.category()?,
                TypeCategory::Enum
                    | TypeCategory::Struct
                    | TypeCategory::Delegate
                    | TypeCategory::Interface
            )
        {
            return Ok(());
        }
        if definition.category()? == TypeCategory::Interface {
            self.include_interface(entity, MemberSelection::All);
        } else if self.selected.insert(entity) {
            self.pending.push(entity);
        }
        Ok(())
    }

    pub(super) fn include_interface(&mut self, entity: Entity<TypeDef>, members: MemberSelection) {
        self.selected.insert(entity);
        if let Some(current) = self.interface_members.get_mut(&entity) {
            if current.merge(members) {
                self.pending.push(entity);
            }
        } else {
            self.interface_members.insert(entity, members);
            self.pending.push(entity);
        }
    }

    pub(super) fn include_implementation(&mut self, entity: Entity<TypeDef>) {
        self.implementations.insert(entity);
        self.include_interface(entity, MemberSelection::Shell);
    }

    pub(super) fn include_field(
        &mut self,
        field: windows_metadata2::FieldDefinition<'_>,
        namespace: &str,
    ) -> Result<(), Error> {
        let ty = native::Type::lower(
            self.database,
            field.entity().file(),
            field.name()?,
            field.signature()?,
        )?;
        self.include_native_type(&ty, namespace)
    }

    pub(super) fn include_method(
        &mut self,
        method: windows_metadata2::MethodDefinition<'_>,
        namespace: &str,
    ) -> Result<(), Error> {
        let signature = native_signature::Signature::lower(
            self.database,
            self.dependencies,
            method,
            method.name()?,
        )?;
        let mut names = Vec::new();
        signature.named_types(|namespace, name| {
            names.push((namespace.to_string(), name.to_string()));
        });
        for (target, name) in names {
            if target.is_empty() {
                self.include_unqualified_name(namespace, &name)?;
            } else {
                self.include_name(&target, &name)?;
            }
        }
        Ok(())
    }

    pub(super) fn finish(
        mut self,
    ) -> Result<
        (
            BTreeSet<Entity<TypeDef>>,
            BTreeMap<Entity<TypeDef>, MemberSelection>,
        ),
        Error,
    > {
        while let Some(entity) = self.pending.pop() {
            let definition = self.database.definition(entity).unwrap();
            let namespace = self.namespaces[&entity].clone();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Struct => {
                    self.include_value_dependencies(entity, &namespace)?;
                }
                TypeCategory::Delegate => {
                    for method in definition.methods()? {
                        self.include_method(method, &namespace)?;
                    }
                }
                TypeCategory::Interface => {
                    if let Some(bases) = self.interface_bases.get(&entity) {
                        for (namespace, name) in bases {
                            self.include_name(namespace, name)?;
                        }
                    }
                    let members = if self.implementations.contains(&entity) {
                        MemberSelection::All
                    } else {
                        self.interface_members.get(&entity).unwrap().clone()
                    };
                    for method in definition.methods()? {
                        let name = method.name()?;
                        if members.includes(name, name) {
                            self.include_method(method, &namespace)?;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok((self.selected, self.interface_members))
    }

    fn include_value_dependencies(
        &mut self,
        entity: Entity<TypeDef>,
        namespace: &str,
    ) -> Result<(), Error> {
        let definition = self.database.definition(entity).unwrap();
        let typedef = definition.has_attribute("NativeTypedefAttribute")?;
        for field in definition.fields()? {
            if !field.is_literal()? {
                let ty = native::Type::lower(
                    self.database,
                    field.entity().file(),
                    definition.name()?,
                    field.signature()?,
                )?;
                let ty = if typedef {
                    ty.normalize_alias(namespace, definition.name()?)
                } else {
                    ty
                };
                self.include_native_type(&ty, namespace)?;
            }
        }
        let children = self.nested.get(&entity).cloned().unwrap_or_default();
        for child in children {
            self.include_value_dependencies(child, namespace)?;
        }
        Ok(())
    }

    fn include_native_type(&mut self, ty: &native::Type, namespace: &str) -> Result<(), Error> {
        let mut names = Vec::new();
        ty.named_types(|namespace, name| {
            names.push((namespace.to_string(), name.to_string()));
        });
        for (target, name) in names {
            if target.is_empty() {
                self.include_unqualified_name(namespace, &name)?;
            } else {
                self.include_name(&target, &name)?;
            }
        }
        Ok(())
    }

    fn include_unqualified_name(&mut self, namespace: &str, name: &str) -> Result<(), Error> {
        if !self.database.type_definitions(namespace, name).is_empty() {
            return self.include_name(namespace, name);
        }
        let namespaces = self
            .database
            .type_names()
            .filter_map(|(namespace, candidate, _)| {
                (candidate == name).then_some(namespace.to_string())
            })
            .collect::<BTreeSet<_>>();
        for namespace in namespaces {
            self.include_name(&namespace, name)?;
        }
        Ok(())
    }

    fn include_name(&mut self, namespace: &str, name: &str) -> Result<(), Error> {
        for entity in self.database.type_definitions(namespace, name) {
            let definition = self.database.definition(*entity).unwrap();
            if !definition.is_windows_runtime()?
                && definition.category()? == TypeCategory::Interface
            {
                self.include_interface(*entity, MemberSelection::Shell);
            } else {
                self.include_definition(*entity)?;
            }
        }
        Ok(())
    }
}
