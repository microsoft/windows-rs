use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

/// An owned Win32 native type projection.
pub struct NativeType {
    architectures: i32,
    kind: Kind,
    artifact_dependencies: Option<BTreeSet<(String, String)>>,
    artifact_sys_dependencies: Option<BTreeSet<(String, String)>>,
    sys_dependencies: BTreeSet<(String, String)>,
    sys_manifest_dependencies: BTreeSet<(String, String)>,
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
    wrapper: bool,
    dependencies: BTreeSet<(String, String)>,
    manifest_dependencies: BTreeSet<(String, String)>,
}

struct Enum {
    namespace: String,
    name: String,
    ty: native::Type,
    values: Vec<(String, ConstantValue)>,
    scoped: bool,
    flags: bool,
    cast_values: bool,
    dependencies: BTreeSet<(String, String)>,
    manifest_dependencies: BTreeSet<(String, String)>,
}

struct Struct {
    namespace: String,
    name: String,
    fields: Vec<(String, native::Type)>,
    field_copy: Vec<bool>,
    nested: Vec<NativeType>,
    union: bool,
    copyable: bool,
    explicit_layout: bool,
    manual_clone: bool,
    align: Option<u32>,
    packing: Option<u16>,
    default: native_default::Policy,
    traits: native::TraitSupport,
    dependencies: BTreeSet<(String, String)>,
    manifest_dependencies: BTreeSet<(String, String)>,
    bitfields: Vec<Bitfield>,
}

struct Bitfield {
    field: String,
    name: String,
    offset: u32,
    width: u32,
    ty: native::Type,
}

impl NativeType {
    pub(super) fn lower_filtered(
        database: &Database,
        cache: &native::DependencyCache,
        definition: TypeDefinition<'_>,
        relationships: &BTreeMap<Entity<TypeDef>, Vec<Entity<TypeDef>>>,
        enum_variants: Option<&BTreeSet<String>>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        Self::lower_named(
            database,
            cache,
            definition,
            relationships,
            &namespace,
            name,
            enum_variants,
        )
    }

    fn lower_named(
        database: &Database,
        cache: &native::DependencyCache,
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
                let ty = ty.ok_or(Error::InvalidType {
                    name: full_name,
                    message: "native enum has no backing field",
                })?;
                let dependencies = ty.package_dependencies(database, cache)?;
                let manifest_dependencies = ty.manifest_dependencies(database)?;
                let cast_values =
                    database
                        .type_definitions(namespace, &name)
                        .iter()
                        .any(|entity| {
                            database.definition(*entity).unwrap().category().unwrap()
                                != TypeCategory::Enum
                        });
                let sys_dependencies = cache.package_sys_dependencies(&dependencies);
                let sys_manifest_dependencies =
                    cache.package_sys_dependencies(&manifest_dependencies);
                Ok(Self {
                    architectures,
                    artifact_dependencies: None,
                    artifact_sys_dependencies: None,
                    sys_dependencies,
                    sys_manifest_dependencies,
                    kind: Kind::Enum(Enum {
                        namespace: namespace.to_string(),
                        name,
                        ty,
                        values,
                        scoped: definition.has_attribute("ScopedEnumAttribute")?,
                        flags: definition.has_attribute("FlagsAttribute")?,
                        cast_values,
                        dependencies,
                        manifest_dependencies,
                    }),
                })
            }
            TypeCategory::Struct => {
                let mut nested_names = relationships
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
                let mut bitfields = Vec::new();
                for field in definition.fields()? {
                    if !field.is_literal()? {
                        let field_name = field.name()?.to_string();
                        let field_ty = native::Type::lower_with_nested(
                            database,
                            field.entity().file(),
                            &full_name,
                            field.signature()?,
                            &substitutions,
                        )?;
                        for attribute in field.attributes()? {
                            if attribute.name()? != Some("NativeBitfieldAttribute") {
                                continue;
                            }
                            let arguments = attribute.arguments(&())?;
                            let [
                                AttributeArgument::Fixed {
                                    value: AttributeValue::String(name),
                                    ..
                                },
                                AttributeArgument::Fixed { value: offset, .. },
                                AttributeArgument::Fixed { value: width, .. },
                            ] = arguments.as_slice()
                            else {
                                continue;
                            };
                            let Some(offset) = attribute_u32(offset) else {
                                continue;
                            };
                            let Some(width) = attribute_u32(width) else {
                                continue;
                            };
                            bitfields.push(Bitfield {
                                field: field_name.clone(),
                                name: name.clone(),
                                offset,
                                width,
                                ty: field_ty.clone(),
                            });
                        }
                        fields.push((field_name, field_ty));
                    }
                }
                nested_names.sort_by(|left, right| left.1.cmp(&right.1));
                let mut nested = nested_names
                    .into_iter()
                    .map(|(_, projected, entity)| {
                        Self::lower_named(
                            database,
                            cache,
                            database.definition(entity).unwrap(),
                            relationships,
                            namespace,
                            projected,
                            None,
                        )
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let native_typedef = definition.has_attribute("NativeTypedefAttribute")?;
                if let [(field, ty)] = fields.as_slice()
                    && field == "Value"
                    && !native_typedef
                    && ty.is_primitive(database)?
                {
                    let ty = ty.clone().normalize_alias(namespace, &name);
                    let dependencies = ty.package_dependencies(database, cache)?;
                    let manifest_dependencies = ty.manifest_dependencies(database)?;
                    let sys_dependencies = cache.package_sys_dependencies(&dependencies);
                    let sys_manifest_dependencies =
                        cache.package_sys_dependencies(&manifest_dependencies);
                    return Ok(Self {
                        architectures,
                        artifact_dependencies: None,
                        artifact_sys_dependencies: None,
                        sys_dependencies,
                        sys_manifest_dependencies,
                        kind: Kind::Alias(Alias {
                            namespace: namespace.to_string(),
                            name,
                            ty,
                            wrapper: true,
                            dependencies,
                            manifest_dependencies,
                        }),
                    });
                }
                if native_typedef {
                    if !nested.is_empty() {
                        return Err(Error::InvalidType {
                            name: full_name,
                            message: "native typedef has nested definitions",
                        });
                    }
                    let [(field, ty)] = fields.try_into().map_err(|_| Error::InvalidType {
                        name: full_name,
                        message: "native typedef does not have one field",
                    })?;
                    let wrapper = field == "Value" && ty.is_wrapper_underlying(database)?;
                    let ty = ty.normalize_alias(namespace, &name);
                    let dependencies = ty.package_dependencies(database, cache)?;
                    let manifest_dependencies = ty.manifest_dependencies(database)?;
                    let sys_dependencies = cache.package_sys_dependencies(&dependencies);
                    let sys_manifest_dependencies =
                        cache.package_sys_dependencies(&manifest_dependencies);
                    return Ok(Self {
                        architectures,
                        artifact_dependencies: None,
                        artifact_sys_dependencies: None,
                        sys_dependencies,
                        sys_manifest_dependencies,
                        kind: Kind::Alias(Alias {
                            namespace: namespace.to_string(),
                            name,
                            ty,
                            wrapper,
                            dependencies,
                            manifest_dependencies,
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
                let union = definition
                    .type_attributes()?
                    .contains(TypeAttributes::EXPLICIT_LAYOUT);
                let mut field_copy = Vec::with_capacity(fields.len());
                for (_, ty) in &fields {
                    let nested_copy = match ty {
                        native::Type::Named {
                            namespace: target_namespace,
                            name: target_name,
                        } if target_namespace.is_empty() || target_namespace == namespace => {
                            nested.iter().find_map(|value| {
                                let Kind::Struct(value) = &value.kind else {
                                    return None;
                                };
                                (value.name == *target_name).then_some(value.copyable)
                            })
                        }
                        _ => None,
                    };
                    field_copy.push(if let Some(copyable) = nested_copy {
                        copyable
                    } else {
                        ty.projected_copyable(database, &mut BTreeSet::new())?
                    });
                }
                let copyable = field_copy.iter().all(|copyable| *copyable);
                let mut explicit_layout = union;
                if !explicit_layout {
                    for (_, ty) in &fields {
                        let nested_layout = match ty {
                            native::Type::Named {
                                namespace: target_namespace,
                                name: target_name,
                            } if target_namespace.is_empty() || target_namespace == namespace => {
                                nested.iter().find_map(|value| {
                                    let Kind::Struct(value) = &value.kind else {
                                        return None;
                                    };
                                    (value.name == *target_name).then_some(value.explicit_layout)
                                })
                            }
                            _ => None,
                        };
                        if if let Some(explicit_layout) = nested_layout {
                            explicit_layout
                        } else {
                            ty.projected_has_explicit_layout(database, &mut BTreeSet::new())?
                        } {
                            explicit_layout = true;
                            break;
                        }
                    }
                }
                let manual_clone = !union && !copyable && explicit_layout;
                let traits = if !union
                    && (definition
                        .type_attributes()?
                        .contains(TypeAttributes::EXPLICIT_LAYOUT)
                        || packing.is_some())
                {
                    native::TraitSupport::NONE
                } else {
                    let mut traits = native::TraitSupport::ALL;
                    let mut stack = BTreeSet::new();
                    for (_, ty) in &fields {
                        let nested_traits = match ty {
                            native::Type::Named {
                                namespace: target_namespace,
                                name: target_name,
                            } if target_namespace.is_empty() || target_namespace == namespace => {
                                nested.iter().find_map(|value| {
                                    let Kind::Struct(value) = &value.kind else {
                                        return None;
                                    };
                                    (value.name == *target_name).then_some(
                                        if value.union || value.packing.is_some() {
                                            native::TraitSupport {
                                                copy: true,
                                                ..native::TraitSupport::NONE
                                            }
                                        } else {
                                            value.traits
                                        },
                                    )
                                })
                            }
                            _ => None,
                        };
                        if let Some(nested_traits) = nested_traits {
                            traits.combine(nested_traits);
                        } else if let native::Type::Named {
                            namespace: target_namespace,
                            name: target_name,
                        } = ty
                            && target_namespace.is_empty()
                        {
                            traits.combine(
                                native::Type::Named {
                                    namespace: namespace.to_string(),
                                    name: target_name.clone(),
                                }
                                .projected_traits(database, &mut stack)?,
                            );
                        } else {
                            traits.combine(ty.projected_traits(database, &mut stack)?);
                        }
                    }
                    traits
                };
                let mut dependencies = BTreeSet::new();
                let mut manifest_dependencies = BTreeSet::new();
                for (_, ty) in &fields {
                    dependencies.extend(ty.package_dependencies(database, cache)?);
                    manifest_dependencies.extend(ty.manifest_dependencies(database)?);
                }
                for nested in &nested {
                    let (_, nested_dependencies) = nested.dependencies();
                    dependencies.extend(nested_dependencies);
                    let (_, nested_dependencies) = nested.manifest_dependencies();
                    manifest_dependencies.extend(nested_dependencies);
                }
                let sys_dependencies = cache.package_sys_dependencies(&dependencies);
                let sys_manifest_dependencies =
                    cache.package_sys_dependencies(&manifest_dependencies);
                for nested in &mut nested {
                    nested.inherit_artifact_dependencies(&dependencies, &sys_dependencies);
                }
                Ok(Self {
                    architectures,
                    artifact_dependencies: None,
                    artifact_sys_dependencies: None,
                    sys_dependencies,
                    sys_manifest_dependencies,
                    kind: Kind::Struct(Struct {
                        namespace: namespace.to_string(),
                        name,
                        fields,
                        field_copy,
                        nested,
                        union,
                        copyable,
                        explicit_layout,
                        manual_clone,
                        align,
                        packing,
                        default,
                        traits,
                        dependencies,
                        manifest_dependencies,
                        bitfields,
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
        self.write_context(Layout::Flat, Projection::Sys)
    }

    #[cfg(test)]
    pub fn write_package(&self) -> TokenStream {
        self.write_context(Layout::Package, Projection::Default)
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        match &self.kind {
            Kind::Alias(value) => &value.name,
            Kind::Enum(value) => &value.name,
            Kind::Struct(value) => &value.name,
        }
    }

    fn write_context(&self, layout: Layout, projection: Projection) -> TokenStream {
        let items = self
            .write_items_context(layout, projection, &[])
            .into_iter()
            .map(|(_, _, tokens)| tokens);
        quote! { #(#items)* }
    }

    pub(super) fn write_sys_items_context(
        &self,
        layout: Layout,
        custom_derives: &[String],
    ) -> Vec<(&str, u8, TokenStream)> {
        let architectures = tokens::architectures(self.architectures);
        let cfg = self.cfg(layout, Projection::Sys);
        match &self.kind {
            Kind::Alias(value) => {
                let tokens = value.write(layout, Projection::Sys);
                vec![(&value.name, 1, quote! { #architectures #cfg #tokens })]
            }
            Kind::Enum(value) => value.write_items(&architectures, &cfg, layout, Projection::Sys),
            Kind::Struct(value) => {
                vec![(
                    &value.name,
                    1,
                    value.write(
                        &architectures,
                        &cfg,
                        layout,
                        Projection::Sys,
                        custom_derives,
                    ),
                )]
            }
        }
    }

    pub(super) fn write_items_context(
        &self,
        layout: Layout,
        projection: Projection,
        custom_derives: &[String],
    ) -> Vec<(&str, u8, TokenStream)> {
        let result: Vec<(&str, u8, TokenStream)> = if !projection.is_sys()
            || (layout.is_package()
                && match &self.kind {
                    Kind::Alias(value) => native::is_core_projection(&value.namespace, &value.name),
                    Kind::Enum(value) => native::is_core_projection(&value.namespace, &value.name),
                    Kind::Struct(value) => {
                        native::is_core_projection(&value.namespace, &value.name)
                    }
                }) {
            let (namespace, name) = match &self.kind {
                Kind::Alias(value) => (&value.namespace, &value.name),
                Kind::Enum(value) => (&value.namespace, &value.name),
                Kind::Struct(value) => (&value.namespace, &value.name),
            };
            if native::is_core_projection(namespace, name) {
                return Vec::new();
            }
            let architectures = tokens::architectures(self.architectures);
            let cfg = self.cfg(layout, projection);
            if let Kind::Struct(value) = &self.kind {
                vec![(
                    value.name.as_str(),
                    1,
                    value.write(&architectures, &cfg, layout, projection, custom_derives),
                )]
            } else if let Kind::Alias(value) = &self.kind {
                let tokens = value.write(layout, projection);
                vec![(
                    value.name.as_str(),
                    1,
                    quote! { #architectures #cfg #tokens },
                )]
            } else if let Kind::Enum(value) = &self.kind {
                value.write_items(&architectures, &cfg, layout, projection)
            } else {
                unreachable!()
            }
        } else {
            self.write_sys_items_context(layout, custom_derives)
        };
        result
    }

    pub(super) fn package_features(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> BTreeSet<String> {
        let (namespace, dependencies) = self.manifest_dependencies();
        let dependencies = if projection.is_sys() {
            &self.sys_manifest_dependencies
        } else {
            &dependencies
        };
        tokens::feature_names(
            namespace,
            layout,
            dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    pub(super) fn supports_package_sys(&self) -> bool {
        match &self.kind {
            Kind::Alias(value) => !value.ty.uses_winrt_projection(),
            Kind::Enum(value) => !value.ty.uses_winrt_projection(),
            Kind::Struct(value) => {
                !value
                    .fields
                    .iter()
                    .any(|(_, ty)| ty.uses_winrt_projection())
                    && value.nested.iter().all(Self::supports_package_sys)
            }
        }
    }

    fn cfg(&self, layout: Layout, projection: Projection) -> TokenStream {
        let (namespace, dependencies) = self.dependencies();
        let dependencies = if projection.is_sys() {
            self.artifact_sys_dependencies
                .as_ref()
                .unwrap_or(&self.sys_dependencies)
        } else {
            self.artifact_dependencies.as_ref().unwrap_or(&dependencies)
        };
        tokens::feature_cfg(
            namespace,
            layout,
            dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    fn inherit_artifact_dependencies(
        &mut self,
        dependencies: &BTreeSet<(String, String)>,
        sys_dependencies: &BTreeSet<(String, String)>,
    ) {
        self.artifact_dependencies = Some(dependencies.clone());
        self.artifact_sys_dependencies = Some(sys_dependencies.clone());
        if let Kind::Struct(value) = &mut self.kind {
            for nested in &mut value.nested {
                nested.inherit_artifact_dependencies(dependencies, sys_dependencies);
            }
        }
    }

    fn dependencies(&self) -> (&str, BTreeSet<(String, String)>) {
        let mut dependencies = BTreeSet::new();
        let namespace = match &self.kind {
            Kind::Alias(value) => {
                dependencies.extend(value.dependencies.iter().cloned());
                value.namespace.as_str()
            }
            Kind::Enum(value) => {
                dependencies.extend(value.dependencies.iter().cloned());
                value.namespace.as_str()
            }
            Kind::Struct(value) => {
                dependencies.extend(value.dependencies.iter().cloned());
                value.namespace.as_str()
            }
        };
        (namespace, dependencies)
    }

    fn manifest_dependencies(&self) -> (&str, BTreeSet<(String, String)>) {
        match &self.kind {
            Kind::Alias(value) => (
                value.namespace.as_str(),
                value.manifest_dependencies.clone(),
            ),
            Kind::Enum(value) => (
                value.namespace.as_str(),
                value.manifest_dependencies.clone(),
            ),
            Kind::Struct(value) => (
                value.namespace.as_str(),
                value.manifest_dependencies.clone(),
            ),
        }
    }
}

impl Alias {
    fn write(&self, layout: Layout, projection: Projection) -> TokenStream {
        let name = tokens::ident(&self.name);
        let ty = if !projection.is_sys()
            && let Some((mutable, interface)) = self.ty.interface_out()
        {
            let interface = interface.write_public(&self.namespace, layout);
            if mutable {
                quote! { *mut Option<#interface> }
            } else {
                quote! { *const Option<#interface> }
            }
        } else {
            self.ty
                .write_projection(&self.namespace, layout, projection)
        };
        if self.wrapper && matches!(projection, Projection::Default) {
            quote! {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
                pub struct #name(pub #ty);
            }
        } else {
            quote! { pub type #name = #ty; }
        }
    }
}

impl Enum {
    fn write_items(
        &self,
        architectures: &TokenStream,
        cfg: &TokenStream,
        layout: Layout,
        projection: Projection,
    ) -> Vec<(&str, u8, TokenStream)> {
        let name = tokens::ident(&self.name);
        let ty = self
            .ty
            .write_projection(&self.namespace, layout, projection);
        if self.scoped {
            let values = self.values.iter().map(|(value_name, value)| {
                let value_name = tokens::ident(value_name);
                let value = native::write_value(&native::Type::from_constant(value), value);
                quote! { pub const #value_name: Self = Self(#value); }
            });
            let derive = if projection.is_sys() {
                quote! { #[derive(Clone, Copy)] }
            } else {
                quote! { #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)] }
            };
            let flags = (!projection.is_sys() && self.flags)
                .then(|| enum_model::write_flags(&name))
                .unwrap_or_default();
            return vec![(
                self.name.as_str(),
                1,
                quote! {
                    #architectures
                    #[repr(transparent)]
                    #cfg
                    #derive
                    pub struct #name(pub #ty);
                    #architectures
                    #cfg
                    impl #name {
                        #(#values)*
                    }
                    #flags
                },
            )];
        }
        let mut result = vec![(
            self.name.as_str(),
            1,
            quote! { #architectures #cfg pub type #name = #ty; },
        )];
        result.extend(self.values.iter().map(|(value_name, value)| {
            let ident = tokens::ident(value_name);
            let value = native::write_value(&native::Type::from_constant(value), value);
            let value = if self.cast_values {
                quote! { #value as _ }
            } else {
                value
            };
            (
                value_name.as_str(),
                2,
                quote! { #architectures #cfg pub const #ident: #name = #value; },
            )
        }));
        result
    }
}

impl Struct {
    fn write(
        &self,
        architectures: &TokenStream,
        cfg: &TokenStream,
        layout: Layout,
        projection: Projection,
        custom_derives: &[String],
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
                    #cfg
                    #[derive(Clone, Copy)]
                    pub union #name {
                        pub value: u8,
                    }
                    #architectures
                    #cfg
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
            let (derive, default) =
                self.default_tokens(&name, architectures, cfg, projection, custom_derives);
            return quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub struct #name(pub u8);
                #default
                #(#nested)*
            };
        }
        let fields =
            self.fields
                .iter()
                .zip(&self.field_copy)
                .map(|((field_name, ty), copyable)| {
                    let field_name = tokens::ident(field_name);
                    let projected = ty.write_field_projection_owner(
                        &self.namespace,
                        &self.name,
                        layout,
                        projection,
                    );
                    let projected = if self.union
                        && !projection.is_sys()
                        && !copyable
                        && !ty.is_interface()
                        && !ty.is_bstr()
                        && !ty.is_hstring()
                    {
                        quote! { core::mem::ManuallyDrop<#projected> }
                    } else {
                        projected
                    };
                    quote! { pub #field_name: #projected, }
                });
        let repr = self.repr();
        let nested = self
            .nested
            .iter()
            .map(|nested| nested.write_context(layout, projection));
        if self.union {
            let derive = if projection.is_sys() || self.copyable {
                quote! { #[derive(Clone, Copy)] }
            } else {
                quote! {}
            };
            let manual_clone = (!projection.is_sys() && !self.copyable).then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Clone for #name {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                }
            });
            quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub union #name {
                    #(#fields)*
                }
                #manual_clone
                #architectures
                #cfg
                impl Default for #name {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #(#nested)*
            }
        } else {
            let (derive, default) =
                self.default_tokens(&name, architectures, cfg, projection, custom_derives);
            let bitfields = self.write_bitfields(&name, architectures, cfg, projection);
            quote! {
                #repr
                #architectures
                #cfg
                #derive
                pub struct #name {
                    #(#fields)*
                }
                #bitfields
                #default
                #(#nested)*
            }
        }
    }

    fn write_bitfields(
        &self,
        name: &TokenStream,
        architectures: &TokenStream,
        cfg: &TokenStream,
        projection: Projection,
    ) -> TokenStream {
        if projection.is_sys() || self.bitfields.is_empty() {
            return quote! {};
        }
        let accessors = self
            .bitfields
            .iter()
            .filter_map(Bitfield::write)
            .collect::<Vec<_>>();
        if accessors.is_empty() {
            return quote! {};
        }
        quote! {
            #architectures
            #cfg
            impl #name {
                #(#accessors)*
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
        cfg: &TokenStream,
        projection: Projection,
        custom_derives: &[String],
    ) -> (TokenStream, TokenStream) {
        let custom_derives = custom_derives
            .iter()
            .map(|derive| tokens::ident(derive))
            .collect::<Vec<_>>();
        if !projection.is_sys() && self.manual_clone {
            let default = (self.default != native_default::Policy::Derive).then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            });
            return (
                quote! {},
                quote! {
                    #architectures
                    #cfg
                    impl Clone for #name {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                    #default
                },
            );
        }
        if !projection.is_sys() && self.packing.is_none() {
            let copy = self.traits.copy.then(|| quote! { , Copy });
            let debug = self.traits.debug.then(|| quote! { , Debug });
            let partial_eq = self.traits.partial_eq.then(|| quote! { , PartialEq });
            let eq = self.traits.eq.then(|| quote! { , Eq });
            let derive_default = matches!(
                self.default,
                native_default::Policy::Derive | native_default::Policy::ScopedEnum
            )
            .then(|| quote! { , Default });
            let default = (!matches!(
                self.default,
                native_default::Policy::Derive | native_default::Policy::ScopedEnum
            ))
            .then(|| {
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                }
            });
            return (
                quote! {
                    #[derive(Clone #copy #debug #(, #custom_derives)* #derive_default #eq #partial_eq)]
                },
                default.unwrap_or_default(),
            );
        }
        if !projection.is_sys() && !self.copyable {
            if self.default != native_default::Policy::Derive {
                let derive = (!custom_derives.is_empty())
                    .then(|| quote! { #[derive(#(#custom_derives),*)] })
                    .unwrap_or_default();
                return (
                    derive,
                    quote! {
                        #architectures
                        #cfg
                        impl Default for #name {
                            fn default() -> Self {
                                unsafe { core::mem::zeroed() }
                            }
                        }
                    },
                );
            }
            return (
                quote! { #[derive(#(#custom_derives,)* Default)] },
                quote! {},
            );
        }
        if self.default != native_default::Policy::Derive {
            (
                quote! { #[derive(Clone, Copy #(, #custom_derives)*)] },
                quote! {
                    #architectures
                    #cfg
                    impl Default for #name {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                },
            )
        } else {
            (
                quote! { #[derive(Clone, Copy #(, #custom_derives)*, Default)] },
                quote! {},
            )
        }
    }
}

impl Bitfield {
    fn write(&self) -> Option<TokenStream> {
        let (ty, unsigned, bits, signed) = match self.ty {
            native::Type::U8 => (quote! { u8 }, quote! { u8 }, 8, false),
            native::Type::I8 => (quote! { i8 }, quote! { u8 }, 8, true),
            native::Type::U16 => (quote! { u16 }, quote! { u16 }, 16, false),
            native::Type::I16 => (quote! { i16 }, quote! { u16 }, 16, true),
            native::Type::U32 => (quote! { u32 }, quote! { u32 }, 32, false),
            native::Type::I32 => (quote! { i32 }, quote! { u32 }, 32, true),
            native::Type::U64 => (quote! { u64 }, quote! { u64 }, 64, false),
            native::Type::I64 => (quote! { i64 }, quote! { u64 }, 64, true),
            _ => return None,
        };
        let field = tokens::ident(&self.field);
        let getter = tokens::ident(&self.name);
        let setter = tokens::ident(&format!("set_{}", self.name));
        let offset = self.offset;
        let width = self.width;
        if width == 1 {
            let get = if offset == 0 {
                quote! { self.#field & 1 != 0 }
            } else {
                let offset = Literal::u32_unsuffixed(offset);
                quote! { (self.#field >> #offset) & 1 != 0 }
            };
            let (clear, place) = if offset == 0 {
                (quote! { !1 }, quote! { value as #ty })
            } else {
                let offset = Literal::u32_unsuffixed(offset);
                (
                    quote! { !(1 << #offset) },
                    quote! { (value as #ty) << #offset },
                )
            };
            return Some(quote! {
                pub fn #getter(&self) -> bool {
                    #get
                }
                pub fn #setter(&mut self, value: bool) {
                    self.#field = (self.#field & #clear) | (#place);
                }
            });
        }
        let high = bits - offset - width;
        let low = bits - width;
        let get = match (high, low) {
            (0, 0) => quote! { self.#field },
            (0, low) => {
                let low = Literal::u32_unsuffixed(low);
                quote! { self.#field >> #low }
            }
            (high, low) => {
                let high = Literal::u32_unsuffixed(high);
                let low = Literal::u32_unsuffixed(low);
                quote! { (self.#field << #high) >> #low }
            }
        };
        let mask_value = if width >= 64 {
            u64::MAX
        } else {
            (1u64 << width) - 1
        };
        let mask = Literal::u64_unsuffixed(mask_value);
        let clear = if offset == 0 {
            quote! { !#mask }
        } else {
            let offset = Literal::u32_unsuffixed(offset);
            quote! { !(#mask << #offset) }
        };
        let set = if signed {
            let place = if offset == 0 {
                quote! { value as #unsigned & #mask }
            } else {
                let offset = Literal::u32_unsuffixed(offset);
                quote! { (value as #unsigned & #mask) << #offset }
            };
            quote! {
                self.#field = ((self.#field as #unsigned & #clear) | (#place)) as #ty;
            }
        } else {
            let place = if offset == 0 {
                quote! { value & #mask }
            } else {
                let offset = Literal::u32_unsuffixed(offset);
                quote! { (value & #mask) << #offset }
            };
            quote! {
                self.#field = (self.#field & #clear) | (#place);
            }
        };
        Some(quote! {
            pub fn #getter(&self) -> #ty {
                #get
            }
            pub fn #setter(&mut self, value: #ty) {
                #set
            }
        })
    }
}

fn attribute_u32(value: &AttributeValue) -> Option<u32> {
    match value {
        AttributeValue::I8(value) => u32::try_from(*value).ok(),
        AttributeValue::U8(value) => Some((*value).into()),
        AttributeValue::I16(value) => u32::try_from(*value).ok(),
        AttributeValue::U16(value) => Some((*value).into()),
        AttributeValue::I32(value) => u32::try_from(*value).ok(),
        AttributeValue::U32(value) => Some(*value),
        AttributeValue::I64(value) => u32::try_from(*value).ok(),
        AttributeValue::U64(value) => u32::try_from(*value).ok(),
        _ => None,
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
