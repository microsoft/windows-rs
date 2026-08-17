use super::metadata::{named_copyable, named_has_explicit_layout, named_traits};
use super::*;
use std::collections::BTreeSet;

impl Type {
    pub(crate) fn projected_copyable(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        match self {
            Self::Void | Self::Interface { .. } => Ok(false),
            Self::Array { element, .. } => element.projected_copyable(database, stack),
            Self::Named { .. } if self.is_bstr() || self.is_hstring() => Ok(false),
            Self::Named {
                namespace, name, ..
            } => named_copyable(database, namespace, name, stack),
            _ => Ok(true),
        }
    }

    pub(crate) fn projected_has_explicit_layout(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        match self {
            Self::Array { element, .. } => element.projected_has_explicit_layout(database, stack),
            Self::Named {
                namespace, name, ..
            } => named_has_explicit_layout(database, namespace, name, stack),
            _ => Ok(false),
        }
    }

    pub(super) fn qualify_projected_nested(
        mut self,
        namespace: &str,
        projected: &BTreeSet<&str>,
    ) -> Self {
        match &mut self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                **element = element
                    .clone()
                    .qualify_projected_nested(namespace, projected);
            }
            Self::Named {
                namespace: target,
                name,
                ..
            } if target.is_empty() && projected.contains(name.as_str()) => {
                *target = namespace.to_string();
            }
            _ => {}
        }
        self
    }
    pub(crate) fn normalize_alias(self, namespace: &str, name: &str) -> Self {
        match canonical::native_alias_from_name(namespace, name) {
            Some(canonical::Type::BStr | canonical::Type::PcWStr) => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U16),
            },
            Some(canonical::Type::PWStr) => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U16),
            },
            Some(canonical::Type::PcStr) => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U8),
            },
            Some(canonical::Type::PStr) => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U8),
            },
            _ => self,
        }
    }

    pub(crate) fn named_types(&self, mut add: impl FnMut(&str, &str)) {
        self.visit_named(&mut add);
    }

    pub(crate) fn uses_winrt_projection(&self) -> bool {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.uses_winrt_projection()
            }
            Self::Interface { .. } => false,
            Self::Named {
                namespace, name, ..
            } => {
                canonical::type_from_name(namespace, name).is_none()
                    && (namespace == "Windows" || namespace.starts_with("Windows."))
                    && namespace != "Windows.Win32"
                    && !namespace.starts_with("Windows.Win32.")
            }
            _ => false,
        }
    }

    pub(crate) fn package_dependencies(
        &self,
        database: &Database,
        cache: &DependencyCache,
    ) -> Result<BTreeSet<(String, String)>, Error> {
        let mut dependencies = BTreeSet::new();
        self.collect_package_dependencies(
            database,
            cache,
            &mut BTreeSet::new(),
            &mut dependencies,
        )?;
        Ok(dependencies)
    }

    pub(crate) fn is_wrapper(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct {
                continue;
            }
            let fields = definition
                .fields()?
                .filter_map(|field| (!field.is_literal().ok()?).then_some(field))
                .collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            if field.name()? != "Value" {
                continue;
            }
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if ty.is_primitive(database)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn collect_package_dependencies(
        &self,
        database: &Database,
        cache: &DependencyCache,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.collect_package_dependencies(database, cache, stack, dependencies)?;
            }
            Self::Interface {
                namespace,
                name,
                arguments,
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
                cache.expand_interface_bases(namespace, name, stack, dependencies);
                for argument in arguments {
                    argument.collect_value_dependencies(dependencies);
                }
            }
            Self::Named {
                namespace, name, ..
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
                cache.expand(database, namespace, name, stack, dependencies)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub(super) fn collect_definition_direct_dependencies(
        database: &Database,
        definition: TypeDefinition<'_>,
        namespace: &str,
        projected_name: &str,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        match definition.category()? {
            TypeCategory::Delegate => {
                let owner = format!("{namespace}.{projected_name}");
                for method in definition.methods()? {
                    let signature = method.signature()?;
                    Self::lower(
                        database,
                        method.entity().file(),
                        &owner,
                        signature.return_type,
                    )?
                    .collect_direct_dependencies(dependencies);
                    for ty in signature.parameters {
                        Self::lower(database, method.entity().file(), &owner, ty)?
                            .collect_direct_dependencies(dependencies);
                    }
                }
                return Ok(());
            }
            TypeCategory::Enum | TypeCategory::Struct => {}
            _ => return Ok(()),
        }
        let nested = database
            .nested_types_of(definition.entity())
            .enumerate()
            .map(|(index, definition)| {
                Ok((
                    definition.name()?.to_string(),
                    format!("{projected_name}_{index}"),
                    definition,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let substitutions = nested
            .iter()
            .map(|(metadata, projected, _)| (metadata.as_str(), projected.as_str()))
            .collect::<Vec<_>>();
        let typedef = definition.has_attribute("NativeTypedefAttribute")?;
        for field in definition.fields()? {
            if field.is_literal()? {
                continue;
            }
            let ty = Self::lower_with_nested(
                database,
                field.entity().file(),
                projected_name,
                field.signature()?,
                &substitutions,
            )?;
            let ty = if typedef {
                ty.normalize_alias(namespace, projected_name)
            } else {
                ty
            };
            ty.collect_direct_dependencies(dependencies);
        }
        for (_, projected, definition) in nested {
            Self::collect_definition_direct_dependencies(
                database,
                definition,
                namespace,
                &projected,
                dependencies,
            )?;
        }
        Ok(())
    }

    fn collect_direct_dependencies(&self, dependencies: &mut BTreeSet<(String, String)>) {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.collect_direct_dependencies(dependencies);
            }
            Self::Interface {
                namespace, name, ..
            }
            | Self::Named {
                namespace, name, ..
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
            }
            _ => {}
        }
    }

    pub(crate) fn projected_traits(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<TraitSupport, Error> {
        Ok(match self {
            Self::Void => TraitSupport::NONE,
            Self::F32 | Self::F64 => TraitSupport {
                copy: true,
                debug: true,
                partial_eq: true,
                eq: false,
            },
            Self::Array { element, .. } => element.projected_traits(database, stack)?,
            Self::Interface { .. } => TraitSupport {
                copy: false,
                debug: true,
                partial_eq: true,
                eq: true,
            },
            Self::Pointer { .. } | Self::String => TraitSupport::ALL,
            Self::Named {
                namespace, name, ..
            } => {
                if self.is_bstr() || self.is_hstring() {
                    TraitSupport {
                        copy: false,
                        debug: true,
                        partial_eq: true,
                        eq: true,
                    }
                } else if is_core_projection(namespace, name) {
                    TraitSupport::ALL
                } else {
                    let mut traits = named_traits(database, namespace, name, stack)?;
                    if !traits.copy {
                        traits.copy =
                            named_copyable(database, namespace, name, &mut BTreeSet::new())?;
                    }
                    traits
                }
            }
            _ => TraitSupport::ALL,
        })
    }

    fn visit_named(&self, add: &mut impl FnMut(&str, &str)) {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.visit_named(add);
            }
            Self::Interface {
                namespace, name, ..
            }
            | Self::Named {
                namespace, name, ..
            } => {
                add(namespace, name);
            }
            _ => {}
        }
    }

    pub(crate) fn matches(&self, value: &ConstantValue) -> bool {
        matches!(
            (self, value),
            (Self::Boolean, ConstantValue::Boolean(_))
                | (Self::Char, ConstantValue::Char(_))
                | (Self::I8, ConstantValue::I8(_))
                | (Self::U8, ConstantValue::U8(_))
                | (Self::I16, ConstantValue::I16(_))
                | (Self::U16, ConstantValue::U16(_))
                | (Self::I32, ConstantValue::I32(_))
                | (Self::U32, ConstantValue::U32(_))
                | (Self::I64, ConstantValue::I64(_))
                | (Self::U64, ConstantValue::U64(_))
                | (Self::ISize, ConstantValue::ISize(_))
                | (Self::USize, ConstantValue::USize(_))
                | (Self::F32, ConstantValue::F32(_))
                | (Self::F64, ConstantValue::F64(_))
                | (Self::String, ConstantValue::String(_))
        )
    }

    pub(crate) fn constant_underlying(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: &windows_metadata2::Type,
    ) -> Result<Option<(Self, usize)>, Error> {
        let mut stack = BTreeSet::new();
        Self::constant_underlying_inner(database, file, owner, ty, &mut stack, 0)
    }

    fn constant_underlying_inner(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: &windows_metadata2::Type,
        stack: &mut BTreeSet<Entity<TypeDef>>,
        depth: usize,
    ) -> Result<Option<(Self, usize)>, Error> {
        let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
            return Ok(Some((
                Self::lower(database, file, owner, ty.clone())?,
                depth,
            )));
        };
        let Some((namespace, name)) = database.type_name(file, *id)? else {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "constant type has no name",
            });
        };
        let definitions = database.type_definitions(namespace, name);
        if definitions.len() != 1 {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "constant type does not have one definition",
            });
        }
        let entity = definitions[0];
        if !stack.insert(entity) {
            return Err(Error::RecursiveValue(format!("{namespace}.{name}")));
        }
        let definition = database.definition(entity).unwrap();
        let result = match definition.category()? {
            TypeCategory::Enum => {
                let mut underlying = None;
                for field in definition.fields()? {
                    if !field.is_literal()? && underlying.replace(field.signature()?).is_some() {
                        return Err(Error::InvalidType {
                            name: owner.to_string(),
                            message: "native enum has more than one backing field",
                        });
                    }
                }
                let underlying = underlying.ok_or_else(|| Error::InvalidType {
                    name: owner.to_string(),
                    message: "native enum has no backing field",
                })?;
                Self::constant_underlying_inner(
                    database,
                    entity.file(),
                    owner,
                    &underlying,
                    stack,
                    depth + 1,
                )
            }
            TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                let fields = definition.fields()?.collect::<Vec<_>>();
                if fields.len() != 1 {
                    return Err(Error::InvalidType {
                        name: owner.to_string(),
                        message: "native typedef does not have one field",
                    });
                }
                Self::constant_underlying_inner(
                    database,
                    entity.file(),
                    owner,
                    &fields[0].signature()?,
                    stack,
                    depth + 1,
                )
            }
            _ => Ok(None),
        };
        stack.remove(&entity);
        result
    }

    pub(crate) fn accepts_converted(&self, value: &ConstantValue) -> bool {
        if self.matches(value) {
            return true;
        }
        match self {
            Self::Boolean => matches!(value, ConstantValue::U8(0 | 1)),
            Self::Pointer { .. } => integer(value),
            Self::I8
            | Self::U8
            | Self::I16
            | Self::U16
            | Self::I32
            | Self::U32
            | Self::I64
            | Self::U64
            | Self::ISize
            | Self::USize => integer(value),
            _ => false,
        }
    }

    pub(crate) fn signed_i32(&self) -> bool {
        matches!(self, Self::I32)
    }

    pub(crate) fn from_constant(value: &ConstantValue) -> Self {
        match value {
            ConstantValue::Boolean(_) => Self::Boolean,
            ConstantValue::Char(_) => Self::Char,
            ConstantValue::I8(_) => Self::I8,
            ConstantValue::U8(_) => Self::U8,
            ConstantValue::I16(_) => Self::I16,
            ConstantValue::U16(_) => Self::U16,
            ConstantValue::I32(_) => Self::I32,
            ConstantValue::U32(_) => Self::U32,
            ConstantValue::I64(_) => Self::I64,
            ConstantValue::U64(_) => Self::U64,
            ConstantValue::ISize(_) => Self::ISize,
            ConstantValue::USize(_) => Self::USize,
            ConstantValue::F32(_) => Self::F32,
            ConstantValue::F64(_) => Self::F64,
            ConstantValue::String(_) => Self::String,
            ConstantValue::Null => unreachable!(),
        }
    }
}
