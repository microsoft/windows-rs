use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Type {
    Void,
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
    ISize,
    USize,
    Array { element: Box<Self>, len: usize },
    Pointer { mutable: bool, element: Box<Self> },
    Named { namespace: String, name: String },
}

impl Type {
    pub(super) fn lower(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
    ) -> Result<Self, Error> {
        Self::lower_with_nested(database, file, owner, ty, &[])
    }

    pub(super) fn lower_with_nested(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
        nested: &[(&str, &str)],
    ) -> Result<Self, Error> {
        let mut is_const = false;
        for modifier in ty.modifiers {
            let Some((namespace, name)) = database.type_name(file, modifier.ty)? else {
                return Err(Error::InvalidValue {
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
            TypeKind::Pointer(element) => Self::Pointer {
                mutable: !is_const,
                element: Box::new(Self::lower_with_nested(
                    database, file, owner, *element, nested,
                )?),
            },
            TypeKind::Value(id) | TypeKind::Class(id) => {
                let (namespace, name) =
                    database
                        .type_name(file, id)?
                        .ok_or_else(|| Error::InvalidValue {
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
                Self::Named {
                    namespace: namespace.to_string(),
                    name: name.to_string(),
                }
            }
            TypeKind::GenericInstance {
                value_type: false, ..
            } => Self::Pointer {
                mutable: true,
                element: Box::new(Self::Void),
            },
            unsupported => {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: format!("{unsupported:?}"),
                });
            }
        })
    }

    pub(super) fn write(&self, namespace: &str) -> TokenStream {
        match self {
            Self::Void => quote! { core::ffi::c_void },
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
            Self::String => quote! { PCWSTR },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::Array { element, len } => {
                let element = element.write(namespace);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#element; #len] }
            }
            Self::Pointer { mutable, element } => {
                let element = element.write(namespace);
                if *mutable {
                    quote! { *mut #element }
                } else {
                    quote! { *const #element }
                }
            }
            Self::Named {
                namespace: target,
                name,
            } => {
                let path = tokens::namespace(namespace, target);
                let name = tokens::ident(name);
                quote! { #path #name }
            }
        }
    }

    pub(super) fn matches(&self, value: &ConstantValue) -> bool {
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

    pub(super) fn constant_underlying(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: &windows_metadata2::Type,
    ) -> Result<Option<Self>, Error> {
        let mut stack = BTreeSet::new();
        Self::constant_underlying_inner(database, file, owner, ty, &mut stack)
    }

    fn constant_underlying_inner(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: &windows_metadata2::Type,
        stack: &mut BTreeSet<Entity<TypeDef>>,
    ) -> Result<Option<Self>, Error> {
        let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
            return Ok(Some(Self::lower(database, file, owner, ty.clone())?));
        };
        let Some((namespace, name)) = database.type_name(file, *id)? else {
            return Err(Error::InvalidValue {
                name: owner.to_string(),
                message: "constant type has no name",
            });
        };
        let definitions = database.type_definitions(namespace, name);
        if definitions.len() != 1 {
            return Err(Error::InvalidValue {
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
                        return Err(Error::InvalidValue {
                            name: owner.to_string(),
                            message: "native enum has more than one backing field",
                        });
                    }
                }
                let underlying = underlying.ok_or_else(|| Error::InvalidValue {
                    name: owner.to_string(),
                    message: "native enum has no backing field",
                })?;
                Self::constant_underlying_inner(database, entity.file(), owner, &underlying, stack)
            }
            TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                let fields = definition.fields()?.collect::<Vec<_>>();
                if fields.len() != 1 {
                    return Err(Error::InvalidValue {
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
                )
            }
            _ => Ok(None),
        };
        stack.remove(&entity);
        result
    }

    pub(super) fn accepts_converted(&self, value: &ConstantValue) -> bool {
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

    pub(super) fn signed_i32(&self) -> bool {
        matches!(self, Self::I32)
    }

    pub(super) fn is_handle_primitive(&self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::Char
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::F32
                | Self::F64
                | Self::ISize
                | Self::USize
                | Self::Pointer { .. }
        )
    }

    pub(super) fn from_constant(value: &ConstantValue) -> Self {
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

pub(super) fn write_value(ty: &Type, value: &ConstantValue) -> TokenStream {
    match (ty, value) {
        (Type::USize, ConstantValue::USize(value)) if *value > u32::MAX as u64 => {
            let value = Literal::u64_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::ISize, ConstantValue::ISize(value))
            if !(i32::MIN as i64..=i32::MAX as i64).contains(value) =>
        {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as isize };
        }
        _ => {}
    }
    let literal = match value {
        ConstantValue::Boolean(value) => return quote! { #value },
        ConstantValue::Char(value) | ConstantValue::U16(value) => Literal::u16_unsuffixed(*value),
        ConstantValue::I8(value) => Literal::i8_unsuffixed(*value),
        ConstantValue::U8(value) => Literal::u8_unsuffixed(*value),
        ConstantValue::I16(value) => Literal::i16_unsuffixed(*value),
        ConstantValue::I32(value) => Literal::i32_unsuffixed(*value),
        ConstantValue::U32(value) => Literal::u32_unsuffixed(*value),
        ConstantValue::I64(value) | ConstantValue::ISize(value) => Literal::i64_unsuffixed(*value),
        ConstantValue::U64(value) | ConstantValue::USize(value) => Literal::u64_unsuffixed(*value),
        ConstantValue::F32(value) => Literal::f32_unsuffixed(*value),
        ConstantValue::F64(value) => Literal::f64_unsuffixed(*value),
        ConstantValue::String(_) | ConstantValue::Null => unreachable!(),
    };
    quote! { #literal }
}

fn integer(value: &ConstantValue) -> bool {
    matches!(
        value,
        ConstantValue::I8(_)
            | ConstantValue::U8(_)
            | ConstantValue::I16(_)
            | ConstantValue::U16(_)
            | ConstantValue::I32(_)
            | ConstantValue::U32(_)
            | ConstantValue::I64(_)
            | ConstantValue::U64(_)
            | ConstantValue::ISize(_)
            | ConstantValue::USize(_)
    )
}
