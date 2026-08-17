use super::*;
use std::collections::{BTreeMap, BTreeSet};

type NamedEntity<T> = (String, i32, Entity<T>);

pub(crate) struct Win32Catalogs {
    pub(super) definitions: Vec<NativeDefinition>,
    pub(super) apis: Vec<NativeApis>,
    pub(super) nested: BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
    pub(super) interface_bases: BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    pub(super) dependencies: native::DependencyCache,
}

pub(super) struct NativeDefinition {
    pub(super) namespace: String,
    pub(super) name: String,
    pub(super) architectures: i32,
    pub(super) entity: Entity<TypeDef>,
    pub(super) kind: NativeKind,
}

pub(super) enum NativeKind {
    Enum(Vec<String>),
    Struct,
    Delegate,
    Interface,
}

pub(super) struct NativeApis {
    pub(super) namespace: String,
    pub(super) constants: Vec<NamedEntity<Field>>,
    pub(super) functions: Vec<NamedEntity<MethodDef>>,
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
