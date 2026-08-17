use super::*;

impl Type {
    pub(crate) fn named(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        let namespace = namespace.into();
        let name = name.into();
        let canonical = canonical::type_from_name(&namespace, &name)
            .or_else(|| canonical::native_alias_from_name(&namespace, &name));
        Self::Named {
            namespace,
            name,
            canonical,
        }
    }

    pub(crate) fn lower_parameter(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
        input_only: bool,
    ) -> Result<Self, Error> {
        let ty = Self::lower(database, file, owner, ty)?;
        Ok(if input_only { ty.into_input() } else { ty })
    }

    pub(crate) fn lower(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
    ) -> Result<Self, Error> {
        Self::lower_with_nested(database, file, owner, ty, &[])
    }

    pub(crate) fn lower_with_nested(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
        nested: &[(&str, &str)],
    ) -> Result<Self, Error> {
        let mut is_const = false;
        for modifier in ty.modifiers {
            let Some((namespace, name)) = database.type_name(file, modifier.ty)? else {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "native type modifier has no name",
                });
            };
            if namespace == "System.Runtime.CompilerServices" && name == "IsConst" {
                is_const = true;
            } else {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: format!("modifier {namespace}.{name}"),
                });
            }
        }
        if is_const && !matches!(ty.kind, TypeKind::Pointer(_)) {
            return Err(Error::UnsupportedType {
                name: owner.to_string(),
                shape: format!("const {:?}", ty.kind),
            });
        }
        Ok(match ty.kind {
            TypeKind::Void => Self::Void,
            TypeKind::Boolean => Self::Boolean,
            TypeKind::Char => Self::Char,
            TypeKind::I8 => Self::I8,
            TypeKind::U8 => Self::U8,
            TypeKind::I16 => Self::I16,
            TypeKind::U16 => Self::U16,
            TypeKind::I32 => Self::I32,
            TypeKind::U32 => Self::U32,
            TypeKind::I64 => Self::I64,
            TypeKind::U64 => Self::U64,
            TypeKind::F32 => Self::F32,
            TypeKind::F64 => Self::F64,
            TypeKind::String => Self::String,
            TypeKind::ISize => Self::ISize,
            TypeKind::USize => Self::USize,
            TypeKind::Array {
                element,
                rank,
                sizes,
                lower_bounds,
            } => {
                if rank != 1 || sizes.len() != 1 || lower_bounds.iter().any(|bound| *bound != 0) {
                    return Err(Error::UnsupportedType {
                        name: owner.to_string(),
                        shape: format!(
                            "array rank {rank}, sizes {sizes:?}, lower bounds {lower_bounds:?}"
                        ),
                    });
                }
                Self::Array {
                    element: Box::new(Self::lower_with_nested(
                        database, file, owner, *element, nested,
                    )?),
                    len: sizes[0] as usize,
                }
            }
            TypeKind::Pointer(element) => {
                let element = Self::lower_with_nested(database, file, owner, *element, nested)?;
                Self::Pointer {
                    mutable: !is_const,
                    element: Box::new(if is_const {
                        element.into_const_pointer_chain()
                    } else {
                        element
                    }),
                }
            }
            TypeKind::Value(id) => {
                let (namespace, name) =
                    database
                        .type_name(file, id)?
                        .ok_or_else(|| Error::InvalidType {
                            name: owner.to_string(),
                            message: "native type has no name",
                        })?;
                let name = if namespace.is_empty() {
                    nested
                        .iter()
                        .find_map(|(metadata, projected)| (*metadata == name).then_some(*projected))
                        .unwrap_or(name)
                } else {
                    name
                };
                Self::named(namespace, name)
            }
            TypeKind::Class(id) => {
                let Some((namespace, name)) = database.type_name(file, id)? else {
                    return Err(Error::InvalidType {
                        name: owner.to_string(),
                        message: "native class type has no name",
                    });
                };
                let mut delegate = false;
                for entity in database.type_definitions(namespace, name) {
                    if database.definition(*entity).unwrap().category()? == TypeCategory::Delegate {
                        delegate = true;
                        break;
                    }
                }
                if delegate {
                    Self::named(namespace, name)
                } else {
                    Self::Interface {
                        namespace: namespace.to_string(),
                        name: name.to_string(),
                        arguments: Vec::new(),
                    }
                }
            }
            TypeKind::GenericInstance {
                value_type: false,
                ty: generic,
                arguments,
            } => {
                let (namespace, name) =
                    database
                        .type_name(file, generic)?
                        .ok_or_else(|| Error::InvalidType {
                            name: owner.to_string(),
                            message: "native generic interface has no name",
                        })?;
                Self::Interface {
                    namespace: namespace.to_string(),
                    name: name.split('`').next().unwrap_or(name).to_string(),
                    arguments: arguments
                        .into_iter()
                        .map(|argument| ty::Type::lower(database, file, owner, argument))
                        .collect::<Result<_, _>>()?,
                }
            }
            unsupported => {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: format!("{unsupported:?}"),
                });
            }
        })
    }

    fn into_input(self) -> Self {
        match self {
            Self::Array { element, len } => Self::Array {
                element: Box::new(element.into_input()),
                len,
            },
            Self::Pointer { element, .. } => Self::Pointer {
                mutable: false,
                element: Box::new(element.into_input()),
            },
            Self::Named {
                namespace,
                canonical: Some(canonical::Type::PWStr),
                ..
            } => Self::Named {
                namespace,
                name: "PCWSTR".to_string(),
                canonical: Some(canonical::Type::PcWStr),
            },
            Self::Named {
                namespace,
                canonical: Some(canonical::Type::PStr),
                ..
            } => Self::Named {
                namespace,
                name: "PCSTR".to_string(),
                canonical: Some(canonical::Type::PcStr),
            },
            _ => self,
        }
    }

    fn into_const_pointer_chain(self) -> Self {
        match self {
            Self::Array { element, len } => Self::Array {
                element: Box::new(element.into_const_pointer_chain()),
                len,
            },
            Self::Pointer { element, .. } => Self::Pointer {
                mutable: false,
                element: Box::new(element.into_const_pointer_chain()),
            },
            _ => self,
        }
    }
}
