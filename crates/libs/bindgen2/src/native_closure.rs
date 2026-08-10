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
        self.include_type(field.entity().file(), &field.signature()?)
    }

    pub(super) fn include_method(
        &mut self,
        method: windows_metadata2::MethodDefinition<'_>,
    ) -> Result<(), Error> {
        let signature = method.signature()?;
        self.include_type(method.entity().file(), &signature.return_type)?;
        for ty in &signature.parameters {
            self.include_type(method.entity().file(), ty)?;
        }
        Ok(())
    }

    pub(super) fn finish(mut self) -> Result<BTreeSet<Entity<TypeDef>>, Error> {
        while let Some(entity) = self.pending.pop() {
            let definition = self.database.definition(entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Struct => {
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            self.include_field(field)?;
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

    fn include_type(&mut self, file: FileId, ty: &windows_metadata2::Type) -> Result<(), Error> {
        match &ty.kind {
            TypeKind::Pointer(element)
            | TypeKind::ByRef(element)
            | TypeKind::Vector(element)
            | TypeKind::Pinned(element) => self.include_type(file, element)?,
            TypeKind::Array { element, .. } => self.include_type(file, element)?,
            TypeKind::Value(id) | TypeKind::Class(id) => self.include_named(file, *id)?,
            TypeKind::GenericInstance { ty, arguments, .. } => {
                self.include_named(file, *ty)?;
                for argument in arguments {
                    self.include_type(file, argument)?;
                }
            }
            TypeKind::FunctionPointer(signature) => {
                self.include_type(file, &signature.return_type)?;
                for parameter in &signature.parameters {
                    self.include_type(file, parameter)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn include_named(&mut self, file: FileId, id: AnyRowId) -> Result<(), Error> {
        let Some((namespace, name)) = self.database.type_name(file, id)? else {
            return Ok(());
        };
        if namespace.is_empty() {
            return Ok(());
        }
        self.include_name(namespace, name)
    }

    fn include_name(&mut self, namespace: &str, name: &str) -> Result<(), Error> {
        for entity in self.database.type_definitions(namespace, name) {
            self.include_definition(*entity)?;
        }
        Ok(())
    }
}
