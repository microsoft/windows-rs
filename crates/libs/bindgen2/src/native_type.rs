use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

/// An owned Win32 native type projection.
pub struct NativeType {
    kind: Kind,
}

enum Kind {
    Alias(Alias),
    Enum(Enum),
    Struct(Struct),
}

/// A projected Win32 native type category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeTypeKind {
    Alias,
    Enum,
    Struct,
}

struct Alias {
    namespace: String,
    name: String,
    ty: native::Type,
}

struct Enum {
    namespace: String,
    name: String,
    ty: native::Type,
    values: Vec<(String, ConstantValue)>,
}

struct Struct {
    namespace: String,
    name: String,
    fields: Vec<(String, native::Type)>,
    union: bool,
    align: Option<u32>,
    packing: Option<u16>,
}

impl NativeType {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        let full_name = format!("{namespace}.{name}");
        match definition.category()? {
            TypeCategory::Enum => {
                let mut ty = None;
                let mut values = Vec::new();
                for field in definition.fields()? {
                    if field.is_literal()? {
                        let value = field
                            .constant()?
                            .ok_or_else(|| Error::InvalidValue {
                                name: full_name.clone(),
                                message: "native enum member has no constant",
                            })?
                            .value()?;
                        values.push((field.name()?.to_string(), value));
                    } else if ty
                        .replace(native::Type::lower(
                            database,
                            field.entity().file(),
                            &full_name,
                            field.signature()?,
                        )?)
                        .is_some()
                    {
                        return Err(Error::InvalidValue {
                            name: full_name,
                            message: "native enum has more than one backing field",
                        });
                    }
                }
                Ok(Self {
                    kind: Kind::Enum(Enum {
                        namespace,
                        name,
                        ty: ty.ok_or(Error::InvalidValue {
                            name: full_name,
                            message: "native enum has no backing field",
                        })?,
                        values,
                    }),
                })
            }
            TypeCategory::Struct => {
                let mut fields = Vec::new();
                for field in definition.fields()? {
                    if !field.is_literal()? {
                        fields.push((
                            field.name()?.to_string(),
                            native::Type::lower(
                                database,
                                field.entity().file(),
                                &full_name,
                                field.signature()?,
                            )?,
                        ));
                    }
                }
                if definition.has_attribute("NativeTypedefAttribute")? {
                    let [(_, ty)] = fields.try_into().map_err(|_| Error::InvalidValue {
                        name: full_name,
                        message: "native typedef does not have one field",
                    })?;
                    return Ok(Self {
                        kind: Kind::Alias(Alias {
                            namespace,
                            name,
                            ty,
                        }),
                    });
                }
                let align = alignment(definition, &full_name)?;
                let packing = definition
                    .layout()?
                    .map(|layout| layout.packing_size())
                    .transpose()?;
                if let Some(packing) = packing
                    && (!packing.is_power_of_two() || packing > 16)
                {
                    return Err(Error::InvalidValue {
                        name: full_name,
                        message: "native packing is not a supported power of two",
                    });
                }
                if align.is_some() && packing.is_some() {
                    return Err(Error::InvalidValue {
                        name: full_name,
                        message: "native type has both alignment and packing",
                    });
                }
                Ok(Self {
                    kind: Kind::Struct(Struct {
                        namespace,
                        name,
                        fields,
                        union: definition.flags()? & 0x10 != 0,
                        align,
                        packing,
                    }),
                })
            }
            category => Err(Error::UnsupportedType {
                name: full_name,
                shape: format!("native definition {category:?}"),
            }),
        }
    }

    /// Returns the projected native type category.
    pub const fn kind(&self) -> NativeTypeKind {
        match self.kind {
            Kind::Alias(_) => NativeTypeKind::Alias,
            Kind::Enum(_) => NativeTypeKind::Enum,
            Kind::Struct(_) => NativeTypeKind::Struct,
        }
    }

    /// Renders a flat Win32 sys type definition.
    pub fn write_sys(&self) -> TokenStream {
        match &self.kind {
            Kind::Alias(value) => value.write_sys(),
            Kind::Enum(value) => value.write_sys(),
            Kind::Struct(value) => value.write_sys(),
        }
    }
}

impl Alias {
    fn write_sys(&self) -> TokenStream {
        let name = tokens::ident(&self.name);
        let ty = self.ty.write(&self.namespace);
        quote! { pub type #name = #ty; }
    }
}

impl Enum {
    fn write_sys(&self) -> TokenStream {
        let name = tokens::ident(&self.name);
        let ty = self.ty.write(&self.namespace);
        let values = self.values.iter().map(|(value_name, value)| {
            let value_name = tokens::ident(value_name);
            let value = native::write_value(&native::Type::from_constant(value), value);
            quote! { pub const #value_name: #name = #value; }
        });
        quote! {
            pub type #name = #ty;
            #(#values)*
        }
    }
}

impl Struct {
    fn write_sys(&self) -> TokenStream {
        let name = tokens::ident(&self.name);
        if self.fields.is_empty() {
            let repr = self.repr();
            if self.union {
                return quote! {
                    #repr
                    #[derive(Clone, Copy)]
                    pub union #name {
                        pub value: u8,
                    }
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                };
            }
            return quote! {
                #repr
                #[derive(Clone, Copy, Default)]
                pub struct #name(pub u8);
            };
        }
        let fields = self.fields.iter().map(|(field_name, ty)| {
            let field_name = tokens::ident(field_name);
            let ty = ty.write(&self.namespace);
            quote! { pub #field_name: #ty, }
        });
        let repr = self.repr();
        if self.union {
            quote! {
                #repr
                #[derive(Clone, Copy)]
                pub union #name {
                    #(#fields)*
                }
                impl Default for #name {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
            }
        } else {
            quote! {
                #repr
                #[derive(Clone, Copy, Default)]
                pub struct #name {
                    #(#fields)*
                }
            }
        }
    }

    fn repr(&self) -> TokenStream {
        if let Some(align) = self.align {
            let align = Literal::u32_unsuffixed(align);
            quote! { #[repr(C, align(#align))] }
        } else if let Some(packing) = self.packing {
            let packing = Literal::u16_unsuffixed(packing);
            quote! { #[repr(C, packed(#packing))] }
        } else {
            quote! { #[repr(C)] }
        }
    }
}

fn alignment(definition: TypeDefinition<'_>, full_name: &str) -> Result<Option<u32>, Error> {
    let Some(attribute) = definition.find_attribute("AlignmentAttribute")? else {
        return Ok(None);
    };
    let arguments = attribute.arguments(&())?;
    let Some(AttributeArgument::Fixed { value, .. }) = arguments.first() else {
        return Err(Error::InvalidValue {
            name: full_name.to_string(),
            message: "alignment attribute has no fixed argument",
        });
    };
    let value = match value {
        AttributeValue::U8(value) => Some(*value as u32),
        AttributeValue::I8(value) => u32::try_from(*value).ok(),
        AttributeValue::U16(value) => Some(*value as u32),
        AttributeValue::I16(value) => u32::try_from(*value).ok(),
        AttributeValue::U32(value) => Some(*value),
        AttributeValue::I32(value) => u32::try_from(*value).ok(),
        _ => None,
    }
    .ok_or_else(|| Error::InvalidValue {
        name: full_name.to_string(),
        message: "alignment attribute is not a positive integer",
    })?;
    if !value.is_power_of_two() {
        return Err(Error::InvalidValue {
            name: full_name.to_string(),
            message: "alignment attribute is not a power of two",
        });
    }
    Ok(Some(value))
}
