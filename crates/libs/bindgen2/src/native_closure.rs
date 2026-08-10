use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Closure<'a> {
    database: &'a Database,
    interface_bases: &'a BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    selected: BTreeSet<Entity<TypeDef>>,
    pending: Vec<Entity<TypeDef>>,
}

impl<'a> Closure<'a> {
    pub(super) fn new(
        database: &'a Database,
        interface_bases: &'a BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    ) -> Self {
        Self {
            database,
            interface_bases,
            selected: BTreeSet::new(),
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
        if self.selected.insert(entity) {
            self.pending.push(entity);
        }
        Ok(())
    }

    pub(super) fn include_field(
        &mut self,
        field: windows_metadata2::FieldDefinition<'_>,
    ) -> Result<(), Error> {
        let ty = native::Type::lower(
            self.database,
            field.entity().file(),
            field.name()?,
            field.signature()?,
        )?;
        self.include_native_type(&ty)
    }

    pub(super) fn include_method(
        &mut self,
        method: windows_metadata2::MethodDefinition<'_>,
    ) -> Result<(), Error> {
        let signature = native_signature::Signature::lower(self.database, method, method.name()?)?;
        let mut names = Vec::new();
        signature.named_types(|namespace, name| {
            if !namespace.is_empty() {
                names.push((namespace.to_string(), name.to_string()));
            }
        });
        for (namespace, name) in names {
            self.include_name(&namespace, &name)?;
        }
        Ok(())
    }

    pub(super) fn finish(mut self) -> Result<BTreeSet<Entity<TypeDef>>, Error> {
        while let Some(entity) = self.pending.pop() {
            let definition = self.database.definition(entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Struct => {
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
                                ty.normalize_alias(definition.namespace()?, definition.name()?)
                            } else {
                                ty
                            };
                            self.include_native_type(&ty)?;
                        }
                    }
                }
                TypeCategory::Delegate => {
                    for method in definition.methods()? {
                        self.include_method(method)?;
                    }
                }
                TypeCategory::Interface => {
                    if let Some(bases) = self.interface_bases.get(&entity) {
                        for (namespace, name) in bases {
                            self.include_name(namespace, name)?;
                        }
                    }
                    for method in definition.methods()? {
                        self.include_method(method)?;
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(self.selected)
    }

    fn include_native_type(&mut self, ty: &native::Type) -> Result<(), Error> {
        let mut names = Vec::new();
        ty.named_types(|namespace, name| {
            if !namespace.is_empty() {
                names.push((namespace.to_string(), name.to_string()));
            }
        });
        for (namespace, name) in names {
            self.include_name(&namespace, &name)?;
        }
        Ok(())
    }

    fn include_name(&mut self, namespace: &str, name: &str) -> Result<(), Error> {
        for entity in self.database.type_definitions(namespace, name) {
            self.include_definition(*entity)?;
        }
        Ok(())
    }
}
