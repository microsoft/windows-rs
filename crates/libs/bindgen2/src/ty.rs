use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum Type {
    Boolean,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    String,
    Object,
    Generic(u32),
    Vector(Box<Self>),
    Named {
        value_type: bool,
        namespace: String,
        name: String,
        arguments: Vec<Self>,
        guid: Option<guid::Guid>,
    },
}

#[derive(Clone, Copy)]
pub(super) struct Properties {
    pub(super) copyable: bool,
    pub(super) eq: bool,
}

impl Type {
    pub(super) fn lower(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
    ) -> Result<Self, Error> {
        if !ty.modifiers.is_empty() {
            return Err(Error::UnsupportedType {
                name: owner.to_string(),
                shape: format!("modified {:?}", ty.kind),
            });
        }
        Self::lower_kind(database, file, owner, ty.kind)
    }

    fn lower_kind(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: TypeKind,
    ) -> Result<Self, Error> {
        Ok(match ty {
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
            TypeKind::Object => Self::Object,
            TypeKind::GenericType(index) => Self::Generic(index),
            TypeKind::Vector(element) => {
                Self::Vector(Box::new(Self::lower(database, file, owner, *element)?))
            }
            TypeKind::Value(id) => Self::named(database, file, owner, id, true, Vec::new())?,
            TypeKind::Class(id) => Self::named(database, file, owner, id, false, Vec::new())?,
            TypeKind::GenericInstance {
                value_type,
                ty,
                arguments,
            } => Self::named(
                database,
                file,
                owner,
                ty,
                value_type,
                arguments
                    .into_iter()
                    .map(|argument| Self::lower(database, file, owner, argument))
                    .collect::<Result<_, _>>()?,
            )?,
            unsupported => {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: format!("{unsupported:?}"),
                });
            }
        })
    }

    fn named(
        database: &Database,
        file: FileId,
        owner: &str,
        id: AnyRowId,
        value_type: bool,
        arguments: Vec<Self>,
    ) -> Result<Self, Error> {
        let (namespace, metadata_name) =
            database
                .type_name(file, id)?
                .ok_or_else(|| Error::InvalidValue {
                    name: owner.to_string(),
                    message: "field type has no name",
                })?;
        let guid = if arguments.is_empty() {
            None
        } else {
            guid::Guid::find(database, namespace, metadata_name, owner)?
        };
        Ok(Self::Named {
            value_type,
            namespace: namespace.to_string(),
            name: trim_generic_arity(metadata_name).to_string(),
            arguments,
            guid,
        })
    }

    pub(super) fn write(&self, namespace: &str) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        Ok(match self {
            Self::Boolean => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::String => quote! { windows_core::HSTRING },
            Self::Named {
                value_type: true,
                namespace: target,
                name,
                arguments,
                ..
            } if target == "System" && name == "Guid" && arguments.is_empty() => {
                quote! { windows_core::GUID }
            }
            Self::Named {
                value_type,
                namespace: target,
                name,
                arguments,
                ..
            } => {
                let path = tokens::namespace(namespace, target);
                let name = tokens::ident(name);
                let arguments = arguments
                    .iter()
                    .map(|argument| argument.write(namespace))
                    .collect::<Result<Vec<_>, _>>()?;
                let name = if arguments.is_empty() {
                    quote! { #path #name }
                } else {
                    quote! { #path #name<#(#arguments),*> }
                };
                if *value_type {
                    name
                } else {
                    quote! { Option<#name> }
                }
            }
            unsupported => {
                return Err(Error::UnsupportedType {
                    name: namespace.to_string(),
                    shape: unsupported.shape(),
                });
            }
        })
    }

    pub(super) fn shape(&self) -> String {
        match self {
            Self::Boolean => "bool".to_string(),
            Self::Char => "char".to_string(),
            Self::I8 => "i8".to_string(),
            Self::U8 => "u8".to_string(),
            Self::I16 => "i16".to_string(),
            Self::U16 => "u16".to_string(),
            Self::I32 => "i32".to_string(),
            Self::U32 => "u32".to_string(),
            Self::I64 => "i64".to_string(),
            Self::U64 => "u64".to_string(),
            Self::F32 => "f32".to_string(),
            Self::F64 => "f64".to_string(),
            Self::String => "string".to_string(),
            Self::Object => "object".to_string(),
            Self::Generic(index) => format!("generic parameter {index}"),
            Self::Vector(element) => format!("vector<{}>", element.shape()),
            Self::Named {
                value_type,
                namespace,
                name,
                arguments,
                ..
            } => {
                let kind = if *value_type { "value" } else { "class" };
                if arguments.is_empty() {
                    format!("{kind} {namespace}.{name}")
                } else {
                    format!("{kind} {namespace}.{name}<...>")
                }
            }
        }
    }

    pub(super) fn primitive_signature(&self) -> Option<&'static str> {
        Some(match self {
            Self::Boolean => "b1",
            Self::Char => "c2",
            Self::I8 => "i1",
            Self::U8 => "u1",
            Self::I16 => "i2",
            Self::U16 => "u2",
            Self::I32 => "i4",
            Self::U32 => "u4",
            Self::I64 => "i8",
            Self::U64 => "u8",
            Self::F32 => "f4",
            Self::F64 => "f8",
            Self::String => "string",
            Self::Object => "cinterface(IInspectable)",
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if namespace == "System" && name == "Guid" && arguments.is_empty() => "g16",
            _ => return None,
        })
    }

    pub(super) fn is_integer(&self) -> bool {
        matches!(
            self,
            Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
        )
    }

    pub(super) fn properties(
        &self,
        values: &Values,
        stack: &mut BTreeSet<(String, String)>,
        owner: &str,
    ) -> Result<Properties, Error> {
        Ok(match self {
            Self::String | Self::Object => Properties {
                copyable: false,
                eq: true,
            },
            Self::F32 | Self::F64 => Properties {
                copyable: true,
                eq: false,
            },
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if namespace == "System" && name == "Guid" && arguments.is_empty() => Properties {
                copyable: true,
                eq: true,
            },
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty() => values.properties(namespace, name, stack)?,
            Self::Named { .. } => Properties {
                copyable: false,
                eq: true,
            },
            Self::Generic(_) | Self::Vector(_) => {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: self.shape(),
                });
            }
            _ => Properties {
                copyable: true,
                eq: true,
            },
        })
    }

    pub(super) fn runtime_signature(
        &self,
        values: &Values,
        stack: &mut BTreeSet<(String, String)>,
        owner: &str,
    ) -> Result<String, Error> {
        if let Some(signature) = self.primitive_signature() {
            return Ok(signature.to_string());
        }
        match self {
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty() => values.signature(namespace, name, stack),
            Self::Named {
                arguments,
                guid: Some(guid),
                ..
            } if !arguments.is_empty() => {
                let mut signature = format!("pinterface({{{guid}}}");
                for argument in arguments {
                    signature.push(';');
                    signature.push_str(&argument.runtime_signature(values, stack, owner)?);
                }
                signature.push(')');
                Ok(signature)
            }
            _ => Err(Error::UnsupportedType {
                name: owner.to_string(),
                shape: self.shape(),
            }),
        }
    }
}

fn trim_generic_arity(name: &str) -> &str {
    name.split_once('`').map_or(name, |(name, _)| name)
}
