use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

/// An owned Win32 constant projection.
pub struct Constant {
    architectures: i32,
    namespace: String,
    name: String,
    ty: native::Type,
    value: Value,
    dependencies: BTreeSet<(String, String)>,
    sys_dependencies: BTreeSet<(String, String)>,
    wrapper: bool,
    converted_wrapper: Option<native::Type>,
}

enum Value {
    Fixed {
        underlying: native::Type,
        alias_depth: usize,
        value: ConstantValue,
        ansi: bool,
    },
    Guid(guid::Guid),
    PropertyKey {
        guid: guid::Guid,
        guid_field: String,
        pid_field: String,
        pid_type: native::Type,
        pid: ConstantValue,
    },
}

impl Constant {
    pub(super) fn lower(
        database: &Database,
        cache: &native::DependencyCache,
        field: windows_metadata2::FieldDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let architectures = field.architectures()?;
        let signature = field.signature()?;
        let ty = native::Type::lower(
            database,
            field.entity().file(),
            &full_name,
            signature.clone(),
        )?;
        let guid = guid::Guid::from_field(field, &full_name)?;
        let constant = field.constant()?;

        if let Some(guid) = guid {
            let value = if let Some(constant) = constant {
                let (guid_field, pid_field, pid_type) =
                    property_fields(database, field.entity().file(), &signature, &full_name)?;
                Value::PropertyKey {
                    guid,
                    guid_field,
                    pid_field,
                    pid_type,
                    pid: constant.value()?,
                }
            } else {
                Value::Guid(guid)
            };
            let dependencies = ty.package_dependencies(database, cache)?;
            let sys_dependencies = cache.package_sys_dependencies(&dependencies);
            let wrapper = ty.is_wrapper(database)?;
            return Ok(Self {
                architectures,
                namespace: namespace.to_string(),
                name: name.to_string(),
                ty,
                value,
                dependencies,
                sys_dependencies,
                wrapper,
                converted_wrapper: None,
            });
        }

        let value = constant
            .ok_or_else(|| Error::InvalidType {
                name: full_name.clone(),
                message: "constant field has no Constant row",
            })?
            .value()?;
        let (underlying, alias_depth) = if ty.matches(&value) {
            (ty.clone(), 0)
        } else {
            native::Type::constant_underlying(
                database,
                field.entity().file(),
                &full_name,
                &signature,
            )?
            .ok_or_else(|| Error::UnsupportedType {
                name: full_name.clone(),
                shape: format!("typed constant {ty:?} <- {value:?}"),
            })?
        };
        if !underlying.accepts_converted(&value) {
            return Err(Error::UnsupportedType {
                name: full_name,
                shape: format!("typed constant {ty:?} <- {value:?}"),
            });
        };
        let dependencies = ty.package_dependencies(database, cache)?;
        let sys_dependencies = cache.package_sys_dependencies(&dependencies);
        let wrapper = ty.is_wrapper(database)?;
        let converted_wrapper = if alias_depth > 1 {
            match constant_immediate_type(database, field.entity().file(), &signature, &full_name)?
            {
                Some(ty) if ty.is_wrapper(database)? => Some(ty),
                _ => None,
            }
        } else {
            None
        };
        Ok(Self {
            architectures,
            namespace: namespace.to_string(),
            name: name.to_string(),
            ty,
            value: Value::Fixed {
                underlying,
                alias_depth,
                value,
                ansi: is_ansi(field)?,
            },
            dependencies,
            sys_dependencies,
            wrapper,
            converted_wrapper,
        })
    }

    /// Renders a flat Win32 sys constant.
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
        &self.name
    }

    pub(super) fn write_context(&self, layout: Layout, projection: Projection) -> TokenStream {
        let architectures = tokens::architectures(self.architectures);
        let dependencies = if projection.is_sys() {
            &self.sys_dependencies
        } else {
            &self.dependencies
        };
        let cfg = tokens::feature_cfg(
            &self.namespace,
            layout,
            dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let name = tokens::ident(&self.name);
        let value = match &self.value {
            Value::Guid(guid) => {
                if layout.is_package() {
                    let guid = guid.write_u128();
                    if projection.is_sys() {
                        quote! {
                            pub const #name: windows_sys::core::GUID =
                                windows_sys::core::GUID::from_u128(#guid);
                        }
                    } else {
                        quote! {
                            pub const #name: windows_core::GUID =
                                windows_core::GUID::from_u128(#guid);
                        }
                    }
                } else if projection.is_sys() {
                    let guid = guid.write_value();
                    quote! {
                        pub const #name: GUID = #guid;
                    }
                } else {
                    let guid = guid.write_u128();
                    quote! {
                        pub const #name: windows_core::GUID =
                            windows_core::GUID::from_u128(#guid);
                    }
                }
            }
            Value::PropertyKey {
                guid,
                guid_field,
                pid_field,
                pid_type,
                pid,
            } => {
                let ty = self
                    .ty
                    .write_constant_projection(&self.namespace, layout, projection);
                let guid = if layout.is_package() {
                    let guid = guid.write_u128();
                    if projection.is_sys() {
                        quote! { windows_sys::core::GUID::from_u128(#guid) }
                    } else {
                        quote! { windows_core::GUID::from_u128(#guid) }
                    }
                } else if projection.is_sys() {
                    guid.write_value()
                } else {
                    let guid = guid.write_u128();
                    quote! { windows_core::GUID::from_u128(#guid) }
                };
                let guid_field = tokens::ident(guid_field);
                let pid_field = tokens::ident(pid_field);
                let pid = native::write_value(&native::Type::from_constant(pid), pid);
                let pid = if !projection.is_sys() && matches!(pid_type, native::Type::Named { .. })
                {
                    let ty =
                        pid_type.write_constant_projection(&self.namespace, layout, projection);
                    quote! { #ty(#pid) }
                } else {
                    pid
                };
                quote! {
                    pub const #name: #ty = #ty {
                        #guid_field: #guid,
                        #pid_field: #pid,
                    };
                }
            }
            Value::Fixed {
                value: ConstantValue::String(value),
                ansi: true,
                ..
            } => {
                if !projection.is_sys() {
                    let value = string_literal(value);
                    return quote! {
                        #architectures
                        pub const #name: windows_core::PCSTR = windows_core::s!(#value);
                    };
                }
                if layout.is_package() {
                    let value = string_literal(value);
                    quote! {
                        pub const #name: windows_sys::core::PCSTR =
                            windows_sys::core::s!(#value);
                    }
                } else {
                    let bytes = value
                        .bytes()
                        .chain(std::iter::once(0))
                        .map(Literal::u8_unsuffixed);
                    quote! {
                        pub const #name: PCSTR = [#(#bytes),*].as_ptr();
                    }
                }
            }
            Value::Fixed {
                value: ConstantValue::String(value),
                ..
            } => {
                if !projection.is_sys() {
                    let value = string_literal(value);
                    return quote! {
                        #architectures
                        pub const #name: windows_core::PCWSTR = windows_core::w!(#value);
                    };
                }
                if layout.is_package() {
                    let value = string_literal(value);
                    quote! {
                        pub const #name: windows_sys::core::PCWSTR =
                            windows_sys::core::w!(#value);
                    }
                } else {
                    let units = value
                        .encode_utf16()
                        .chain(std::iter::once(0))
                        .map(Literal::u16_unsuffixed);
                    quote! {
                        pub const #name: PCWSTR = [#(#units),*].as_ptr();
                    }
                }
            }
            Value::Fixed {
                underlying,
                alias_depth,
                value,
                ..
            } => {
                let ty = self
                    .ty
                    .write_constant_projection(&self.namespace, layout, projection);
                let value = if self.ty == *underlying {
                    native::write_value(underlying, value)
                } else {
                    Self::write_converted(
                        underlying,
                        *alias_depth,
                        value,
                        self.wrapper && underlying.signed_i32(),
                    )
                };
                let value = if !projection.is_sys()
                    && let Some(wrapper) = &self.converted_wrapper
                {
                    let wrapper =
                        wrapper.write_constant_projection(&self.namespace, layout, projection);
                    quote! { #wrapper(#value) }
                } else {
                    value
                };
                if !projection.is_sys()
                    && (self.ty.is_hresult()
                        || self.ty.is_ntstatus()
                        || (layout.is_package() && self.wrapper)
                        || self.ty.mutable_string_pointer())
                {
                    quote! { pub const #name: #ty = #ty(#value); }
                } else {
                    quote! { pub const #name: #ty = #value; }
                }
            }
        };
        quote! { #architectures #cfg #value }
    }

    pub(super) fn package_features(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> BTreeSet<String> {
        let dependencies = if projection.is_sys() {
            &self.sys_dependencies
        } else {
            &self.dependencies
        };
        tokens::feature_names(
            &self.namespace,
            layout,
            dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    pub(super) fn supports_package_sys(&self) -> bool {
        !self.ty.uses_winrt_projection()
    }

    fn write_converted(
        underlying: &native::Type,
        alias_depth: usize,
        value: &ConstantValue,
        signed_error: bool,
    ) -> TokenStream {
        if matches!(underlying, native::Type::Boolean) {
            return match value {
                ConstantValue::U8(0) => quote! { false },
                ConstantValue::U8(1) => quote! { true },
                _ => unreachable!(),
            };
        }
        if signed_error && let ConstantValue::I32(value) = value {
            return format!("0x{:X}_u32 as _", *value as u32).parse().unwrap();
        }
        if matches!(underlying, native::Type::ISize | native::Type::USize) {
            return native::write_value(underlying, value);
        }
        if alias_depth <= 1 && underlying.matches(value) {
            return native::write_value(underlying, value);
        }
        Self::write_wide_cast(value)
    }

    fn write_wide_cast(value: &ConstantValue) -> TokenStream {
        let fits_i32 = |value: i128| (i32::MIN as i128..=i32::MAX as i128).contains(&value);
        let value = match value {
            ConstantValue::U32(value) if !fits_i32(*value as i128) => Literal::u32_suffixed(*value),
            ConstantValue::I64(value) if !fits_i32(*value as i128) => Literal::i64_suffixed(*value),
            ConstantValue::U64(value) if !fits_i32(*value as i128) => Literal::u64_suffixed(*value),
            ConstantValue::ISize(value) if !fits_i32(*value as i128) => {
                Literal::i64_suffixed(*value)
            }
            ConstantValue::USize(value) if !fits_i32(*value as i128) => {
                Literal::u64_suffixed(*value)
            }
            _ => {
                let value = native::write_value(&native::Type::from_constant(value), value);
                return quote! { #value as _ };
            }
        };
        quote! { #value as _ }
    }
}

fn constant_immediate_type(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: &str,
) -> Result<Option<native::Type>, Error> {
    let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
        return Ok(None);
    };
    let Some((namespace, name)) = database.type_name(file, *id)? else {
        return Ok(None);
    };
    let definitions = database.type_definitions(namespace, name);
    let [entity] = definitions else {
        return Ok(None);
    };
    let definition = database.definition(*entity).unwrap();
    if !matches!(
        definition.category()?,
        TypeCategory::Enum | TypeCategory::Struct
    ) {
        return Ok(None);
    }
    let fields = definition
        .fields()?
        .filter_map(|field| (!field.is_literal().ok()?).then_some(field))
        .collect::<Vec<_>>();
    let [field] = fields.as_slice() else {
        return Ok(None);
    };
    Ok(Some(native::Type::lower(
        database,
        field.entity().file(),
        owner,
        field.signature()?,
    )?))
}

pub(super) fn string_literal(value: &str) -> TokenStream {
    if !value.contains('\0') && !value.contains('\'') {
        let value = Literal::string(value);
        return quote! { #value };
    }
    let mut literal = String::from("\"");
    for value in value.chars() {
        literal.extend(value.escape_default());
    }
    literal.push('"');
    literal.parse().unwrap()
}

fn property_fields(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: &str,
) -> Result<(String, String, native::Type), Error> {
    let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
        return Err(Error::InvalidType {
            name: owner.to_string(),
            message: "GUID-backed constant is not a named struct",
        });
    };
    let Some((namespace, name)) = database.type_name(file, *id)? else {
        return Err(Error::InvalidType {
            name: owner.to_string(),
            message: "GUID-backed constant type has no name",
        });
    };
    let definitions = database.type_definitions(namespace, name);
    if definitions.len() != 1 {
        return Err(Error::InvalidType {
            name: owner.to_string(),
            message: "GUID-backed constant type does not have one definition",
        });
    }
    let fields = database
        .definition(definitions[0])
        .unwrap()
        .fields()?
        .map(|field| {
            Ok::<_, Error>((
                field.name()?.to_string(),
                native::Type::lower(database, field.entity().file(), owner, field.signature()?)?,
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [first, second] = fields.try_into().map_err(|_| Error::InvalidType {
        name: owner.to_string(),
        message: "GUID-backed constant struct does not have two fields",
    })?;
    Ok((first.0, second.0, second.1))
}

fn is_ansi(field: windows_metadata2::FieldDefinition<'_>) -> Result<bool, Error> {
    let Some(attribute) = field.find_attribute("NativeEncodingAttribute")? else {
        return Ok(false);
    };
    Ok(matches!(
        attribute.arguments(&())?.first(),
        Some(AttributeArgument::Fixed {
            value: AttributeValue::String(value),
            ..
        }) if value == "ansi"
    ))
}
