use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

/// An owned Win32 native type projection.
pub struct NativeType {
    architectures: i32,
    kind: Kind,
}

enum Kind {
    Alias(Alias),
    Enum(Enum),
    Struct(Struct),
}

/// A projected Win32 native type category.
#[cfg(test)]
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
    scoped: bool,
}

struct Struct {
    namespace: String,
    name: String,
    fields: Vec<(String, native::Type)>,
    nested: Vec<NativeType>,
    union: bool,
    align: Option<u32>,
    packing: Option<u16>,
    default: native_default::Policy,
}

impl NativeType {
    pub(super) fn lower_filtered(
        database: &Database,
        definition: TypeDefinition<'_>,
        relationships: &BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
        enum_variants: Option<&BTreeSet<String>>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        Self::lower_named(
            database,
            definition,
            relationships,
            &namespace,
            name,
            enum_variants,
        )
    }

    fn lower_named(
        database: &Database,
        definition: TypeDefinition<'_>,
        relationships: &BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
        namespace: &str,
        name: String,
        enum_variants: Option<&BTreeSet<String>>,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let architectures = definition.architectures()?;
        match definition.category()? {
            TypeCategory::Enum => {
                let mut ty = None;
                let mut values = Vec::new();
                for field in definition.fields()? {
                    if field.is_literal()? {
                        let field_name = field.name()?;
                        if enum_variants.is_some_and(|variants| !variants.contains(field_name)) {
                            continue;
                        }
                        let value = field
                            .constant()?
                            .ok_or_else(|| Error::InvalidType {
                                name: full_name.clone(),
                                message: "native enum member has no constant",
                            })?
                            .value()?;
                        values.push((field_name.to_string(), value));
                    } else if ty
                        .replace(native::Type::lower(
                            database,
                            field.entity().file(),
                            &full_name,
                            field.signature()?,
                        )?)
                        .is_some()
                    {
                        return Err(Error::InvalidType {
                            name: full_name,
                            message: "native enum has more than one backing field",
                        });
                    }
                }
                Ok(Self {
                    architectures,
                    kind: Kind::Enum(Enum {
                        namespace: namespace.to_string(),
                        name,
                        ty: ty.ok_or(Error::InvalidType {
                            name: full_name,
                            message: "native enum has no backing field",
                        })?,
                        values,
                        scoped: definition.has_attribute("ScopedEnumAttribute")?,
                    }),
                })
            }
            TypeCategory::Struct => {
                let nested_names = relationships
                    .get(&definition.entity())
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(index, entity)| {
                        Ok((
                            database.definition(*entity).unwrap().name()?.to_string(),
                            format!("{name}_{index}"),
                            *entity,
                        ))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let substitutions = nested_names
                    .iter()
                    .map(|(metadata, projected, _)| (metadata.as_str(), projected.as_str()))
                    .collect::<Vec<_>>();
                let mut fields = Vec::new();
                for field in definition.fields()? {
                    if !field.is_literal()? {
                        fields.push((
                            field.name()?.to_string(),
                            native::Type::lower_with_nested(
                                database,
                                field.entity().file(),
                                &full_name,
                                field.signature()?,
                                &substitutions,
                            )?,
                        ));
                    }
                }
                let nested = nested_names
                    .into_iter()
                    .map(|(_, projected, entity)| {
                        Self::lower_named(
                            database,
                            database.definition(entity).unwrap(),
                            relationships,
                            namespace,
                            projected,
                            None,
                        )
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                if let [(field, ty)] = fields.as_slice()
                    && field == "Value"
                    && ty.is_handle_primitive()
                {
                    let ty = ty.clone().normalize_alias(namespace, &name);
                    return Ok(Self {
                        architectures,
                        kind: Kind::Alias(Alias {
                            namespace: namespace.to_string(),
                            name,
                            ty,
                        }),
                    });
                }
                if definition.has_attribute("NativeTypedefAttribute")? {
                    if !nested.is_empty() {
                        return Err(Error::InvalidType {
                            name: full_name,
                            message: "native typedef has nested definitions",
                        });
                    }
                    let [(_, ty)] = fields.try_into().map_err(|_| Error::InvalidType {
                        name: full_name,
                        message: "native typedef does not have one field",
                    })?;
                    let ty = ty.normalize_alias(namespace, &name);
                    return Ok(Self {
                        architectures,
                        kind: Kind::Alias(Alias {
                            namespace: namespace.to_string(),
                            name,
                            ty,
                        }),
                    });
                }
                let guid = normalize_guid(&mut fields);
                let align = alignment(definition, &full_name)?;
                let default = if guid {
                    native_default::Policy::Derive
                } else {
                    native_default::classify(database, definition, relationships)?
                };
                let packing = definition
                    .layout()?
                    .map(|layout| layout.packing_size())
                    .transpose()?;
                if let Some(packing) = packing
                    && (!packing.is_power_of_two() || packing > 16)
                {
                    return Err(Error::InvalidType {
                        name: full_name,
                        message: "native packing is not a supported power of two",
                    });
                }
                if align.is_some() && packing.is_some() {
                    return Err(Error::InvalidType {
                        name: full_name,
                        message: "native type has both alignment and packing",
                    });
                }
                Ok(Self {
                    architectures,
                    kind: Kind::Struct(Struct {
                        namespace: namespace.to_string(),
                        name,
                        fields,
                        nested,
                        union: definition
                            .type_attributes()?
                            .contains(TypeAttributes::EXPLICIT_LAYOUT),
                        align,
                        packing,
                        default,
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
    #[cfg(test)]
    pub const fn kind(&self) -> NativeTypeKind {
        match self.kind {
            Kind::Alias(_) => NativeTypeKind::Alias,
            Kind::Enum(_) => NativeTypeKind::Enum,
            Kind::Struct(_) => NativeTypeKind::Struct,
        }
    }

    #[cfg(test)]
    pub(super) const fn default_policy(&self) -> Option<native_default::Policy> {
        match &self.kind {
            Kind::Struct(value) => Some(value.default),
            Kind::Alias(_) | Kind::Enum(_) => None,
        }
    }

    /// Renders a flat Win32 sys type definition.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(Layout::Flat, Projection::Default)
    }

    fn write_context(&self, layout: Layout, projection: Projection) -> TokenStream {
        let items = self
            .write_items_context(layout, projection)
            .into_iter()
            .map(|(_, _, tokens)| tokens);
        quote! { #(#items)* }
    }

    pub(super) fn write_sys_items_context(&self, layout: Layout) -> Vec<(&str, u8, TokenStream)> {
        let architectures = tokens::architectures(self.architectures);
        match &self.kind {
            Kind::Alias(value) => {
                let tokens = value.write_sys(layout);
                vec![(&value.name, 1, quote! { #architectures #tokens })]
            }
            Kind::Enum(value) => value.write_sys_items(&architectures, layout),
            Kind::Struct(value) => {
                vec![(
                    &value.name,
                    1,
                    value.write(&architectures, layout, Projection::Default),
                )]
            }
        }
    }

    pub(super) fn write_items_context(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Vec<(&str, u8, TokenStream)> {
        if projection.is_minimal() {
            let (namespace, name) = match &self.kind {
                Kind::Alias(value) => (&value.namespace, &value.name),
                Kind::Enum(value) => (&value.namespace, &value.name),
                Kind::Struct(value) => (&value.namespace, &value.name),
            };
            if native::is_core_projection(namespace, name) {
                return Vec::new();
            }
            let architectures = tokens::architectures(self.architectures);
            if let Kind::Struct(value) = &self.kind {
                return vec![(
                    &value.name,
                    1,
                    value.write(&architectures, layout, projection),
                )];
            }
        }
        self.write_sys_items_context(layout)
    }
}

impl Alias {
    fn write_sys(&self, layout: Layout) -> TokenStream {
        let name = tokens::ident(&self.name);
        let ty = self.ty.write(&self.namespace, layout);
        quote! { pub type #name = #ty; }
    }
}

impl Enum {
    fn write_sys_items(
        &self,
        architectures: &TokenStream,
        layout: Layout,
    ) -> Vec<(&str, u8, TokenStream)> {
        let name = tokens::ident(&self.name);
        let ty = self.ty.write(&self.namespace, layout);
        if self.scoped {
            let values = self.values.iter().map(|(value_name, value)| {
                let value_name = tokens::ident(value_name);
                let value = native::write_value(&native::Type::from_constant(value), value);
                quote! { pub const #value_name: Self = Self(#value); }
            });
            return vec![(
                self.name.as_str(),
                1,
                quote! {
                    #architectures
                    #[repr(transparent)]
                    #[derive(Clone, Copy)]
                    pub struct #name(pub #ty);
                    #architectures
                    impl #name {
                        #(#values)*
                    }
                },
            )];
        }
        let mut result = vec![(
            self.name.as_str(),
            1,
            quote! { #architectures pub type #name = #ty; },
        )];
        result.extend(self.values.iter().map(|(value_name, value)| {
            let ident = tokens::ident(value_name);
            let value = native::write_value(&native::Type::from_constant(value), value);
            (
                value_name.as_str(),
                2,
                quote! { #architectures pub const #ident: #name = #value; },
            )
        }));
        result
    }
}

impl Struct {
    fn write(
        &self,
        architectures: &TokenStream,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        let name = tokens::ident(&self.name);
        if self.fields.is_empty() {
            let repr = self.repr();
            if self.union {
                let nested = self
                    .nested
                    .iter()
                    .map(|nested| nested.write_context(layout, projection));
                return quote! {
                    #repr
                    #architectures
                    #[derive(Clone, Copy)]
                    pub union #name {
                        pub value: u8,
                    }
                    #architectures
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #(#nested)*
                };
            }
            let nested = self
                .nested
                .iter()
                .map(|nested| nested.write_context(layout, projection));
            let (derive, default) = self.default_tokens(&name, architectures, projection);
            return quote! {
                #repr
                #architectures
                #derive
                pub struct #name(pub u8);
                #default
                #(#nested)*
            };
        }
        let fields = self.fields.iter().map(|(field_name, ty)| {
            let field_name = tokens::ident(field_name);
            let ty = ty.write(&self.namespace, layout);
            quote! { pub #field_name: #ty, }
        });
        let repr = self.repr();
        let nested = self
            .nested
            .iter()
            .map(|nested| nested.write_context(layout, projection));
        if self.union {
            quote! {
                #repr
                #architectures
                #[derive(Clone, Copy)]
                pub union #name {
                    #(#fields)*
                }
                #architectures
                impl Default for #name {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #(#nested)*
            }
        } else {
            let (derive, default) = self.default_tokens(&name, architectures, projection);
            quote! {
                #repr
                #architectures
                #derive
                pub struct #name {
                    #(#fields)*
                }
                #default
                #(#nested)*
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

    fn default_tokens(
        &self,
        name: &TokenStream,
        architectures: &TokenStream,
        projection: Projection,
    ) -> (TokenStream, TokenStream) {
        if projection.is_minimal() && self.align.is_none() && self.packing.is_none() {
            let debug = self
                .fields
                .iter()
                .all(|(_, ty)| ty.supports_debug())
                .then(|| quote! { , Debug });
            let partial_eq = self
                .fields
                .iter()
                .all(|(_, ty)| ty.supports_partial_eq())
                .then(|| quote! { , PartialEq });
            let eq = self
                .fields
                .iter()
                .all(|(_, ty)| ty.supports_eq())
                .then(|| quote! { , Eq });
            let derive_default =
                (self.default == native_default::Policy::Derive).then(|| quote! { , Default });
            let default = (self.default != native_default::Policy::Derive).then(|| {
                quote! {
                    #architectures
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            });
            return (
                quote! { #[derive(Clone, Copy #debug #derive_default #eq #partial_eq)] },
                default.unwrap_or_default(),
            );
        }
        if self.default != native_default::Policy::Derive {
            (
                quote! { #[derive(Clone, Copy)] },
                quote! {
                    #architectures
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                },
            )
        } else {
            (quote! { #[derive(Clone, Copy, Default)] }, quote! {})
        }
    }
}

fn normalize_guid(fields: &mut [(String, native::Type)]) -> bool {
    let [
        (data1, native::Type::U32),
        (data2, native::Type::U16),
        (data3, native::Type::U16),
        (data4, native::Type::Array { element, len: 8 }),
    ] = fields
    else {
        return false;
    };
    if **element != native::Type::U8
        || data1 != "Data1"
        || data2 != "Data2"
        || data3 != "Data3"
        || data4 != "Data4"
    {
        return false;
    }
    data1.make_ascii_lowercase();
    data2.make_ascii_lowercase();
    data3.make_ascii_lowercase();
    data4.make_ascii_lowercase();
    true
}

fn alignment(definition: TypeDefinition<'_>, full_name: &str) -> Result<Option<u32>, Error> {
    let Some(attribute) = definition.find_attribute("AlignmentAttribute")? else {
        return Ok(None);
    };
    let arguments = attribute.arguments(&())?;
    let Some(AttributeArgument::Fixed { value, .. }) = arguments.first() else {
        return Err(Error::InvalidType {
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
    .ok_or_else(|| Error::InvalidType {
        name: full_name.to_string(),
        message: "alignment attribute is not a positive integer",
    })?;
    if !value.is_power_of_two() {
        return Err(Error::InvalidType {
            name: full_name.to_string(),
            message: "alignment attribute is not a power of two",
        });
    }
    Ok(Some(value))
}
