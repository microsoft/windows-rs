use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

/// An owned projected WinRT enum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    underlying: ty::Type,
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field {
    name: String,
    value: Integer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Integer {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(i64),
    USize(u64),
}

impl Enum {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        full_name: &str,
    ) -> Result<Self, Error> {
        let mut underlying = None;
        let mut fields = Vec::new();
        for field in definition.fields()? {
            if field.is_literal()? {
                let value = field
                    .constant()?
                    .ok_or_else(|| Error::InvalidType {
                        name: full_name.to_string(),
                        message: "literal enum field has no constant",
                    })?
                    .value()?;
                fields.push(Field {
                    name: field.name()?.to_string(),
                    value: Integer::lower(value).ok_or_else(|| Error::InvalidType {
                        name: full_name.to_string(),
                        message: "enum constant is not an integer",
                    })?,
                });
            } else {
                if underlying.is_some() {
                    return Err(Error::InvalidType {
                        name: full_name.to_string(),
                        message: "enum has more than one backing field",
                    });
                }
                underlying = Some(ty::Type::lower(
                    database,
                    definition.entity().file(),
                    full_name,
                    field.signature()?,
                )?);
            }
        }
        let underlying = underlying.ok_or_else(|| Error::InvalidType {
            name: full_name.to_string(),
            message: "enum has no backing field",
        })?;
        if !underlying.is_integer() {
            return Err(Error::InvalidType {
                name: full_name.to_string(),
                message: "enum backing field is not an integer",
            });
        }
        if fields.iter().any(|field| !field.value.matches(&underlying)) {
            return Err(Error::InvalidType {
                name: full_name.to_string(),
                message: "enum constant does not match its backing field",
            });
        }
        Ok(Self { underlying, fields })
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        name: &str,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let ident = tokens::ident(name);
        let underlying = self.underlying.write(namespace, layout)?;
        let fields = self.fields.iter().map(|field| {
            let name = tokens::ident(&field.name);
            let value = field.value.write();
            quote! { pub const #name: Self = Self(#value); }
        });
        let fields = if self.fields.is_empty() {
            quote! {}
        } else {
            quote! { impl #ident { #(#fields)* } }
        };
        let flags = if self.underlying == ty::Type::U32 {
            write_flags(&ident)
        } else {
            quote! {}
        };
        let signature = Literal::byte_string(
            values
                .signature(namespace, name, &mut BTreeSet::new())?
                .as_bytes(),
        );
        let runtime_name = Literal::byte_string(format!("{namespace}.{name}").as_bytes());
        let runtime_name = (!projection.is_minimal()).then(|| {
            quote! {
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#runtime_name);
            }
        });

        Ok(quote! {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct #ident(pub #underlying);
            #fields
            impl windows_core::TypeKind for #ident {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for #ident {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#signature);
                #runtime_name
            }
            #flags
        })
    }

    pub(super) fn signature(
        &self,
        values: &Values,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<String, Error> {
        Ok(format!(
            "enum({namespace}.{name};{})",
            self.underlying
                .runtime_signature(values, stack, &format!("{namespace}.{name}"))?
        ))
    }
}

impl Integer {
    fn lower(value: ConstantValue) -> Option<Self> {
        Some(match value {
            ConstantValue::I8(value) => Self::I8(value),
            ConstantValue::U8(value) => Self::U8(value),
            ConstantValue::I16(value) => Self::I16(value),
            ConstantValue::U16(value) => Self::U16(value),
            ConstantValue::I32(value) => Self::I32(value),
            ConstantValue::U32(value) => Self::U32(value),
            ConstantValue::I64(value) => Self::I64(value),
            ConstantValue::U64(value) => Self::U64(value),
            ConstantValue::ISize(value) => Self::ISize(value),
            ConstantValue::USize(value) => Self::USize(value),
            _ => return None,
        })
    }

    fn write(self) -> Literal {
        match self {
            Self::I8(value) => Literal::i8_unsuffixed(value),
            Self::U8(value) => Literal::u8_unsuffixed(value),
            Self::I16(value) => Literal::i16_unsuffixed(value),
            Self::U16(value) => Literal::u16_unsuffixed(value),
            Self::I32(value) => Literal::i32_unsuffixed(value),
            Self::U32(value) => Literal::u32_unsuffixed(value),
            Self::I64(value) | Self::ISize(value) => Literal::i64_unsuffixed(value),
            Self::U64(value) | Self::USize(value) => Literal::u64_unsuffixed(value),
        }
    }

    fn matches(self, ty: &ty::Type) -> bool {
        matches!(
            (self, ty),
            (Self::I8(_), ty::Type::I8)
                | (Self::U8(_), ty::Type::U8)
                | (Self::I16(_), ty::Type::I16)
                | (Self::U16(_), ty::Type::U16)
                | (Self::I32(_), ty::Type::I32)
                | (Self::U32(_), ty::Type::U32)
                | (Self::I64(_) | Self::ISize(_), ty::Type::I64)
                | (Self::U64(_) | Self::USize(_), ty::Type::U64)
        )
    }
}

fn write_flags(name: &TokenStream) -> TokenStream {
    quote! {
        impl #name {
            pub const fn contains(&self, other: Self) -> bool {
                self.0 & other.0 == other.0
            }
        }
        impl core::ops::BitOr for #name {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl core::ops::BitAnd for #name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl core::ops::BitOrAssign for #name {
            fn bitor_assign(&mut self, other: Self) {
                self.0.bitor_assign(other.0);
            }
        }
        impl core::ops::BitAndAssign for #name {
            fn bitand_assign(&mut self, other: Self) {
                self.0.bitand_assign(other.0);
            }
        }
        impl core::ops::Not for #name {
            type Output = Self;
            fn not(self) -> Self {
                Self(self.0.not())
            }
        }
    }
}
