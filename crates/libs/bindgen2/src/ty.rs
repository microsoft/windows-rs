use super::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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
    pub(super) fn substitute(&self, arguments: &[Self]) -> Self {
        match self {
            Self::Generic(index) => arguments
                .get(*index as usize)
                .cloned()
                .unwrap_or_else(|| self.clone()),
            Self::Vector(element) => Self::Vector(Box::new(element.substitute(arguments))),
            Self::Named {
                value_type,
                namespace,
                name,
                arguments: nested,
                guid,
            } => Self::Named {
                value_type: *value_type,
                namespace: namespace.clone(),
                name: name.clone(),
                arguments: nested
                    .iter()
                    .map(|argument| argument.substitute(arguments))
                    .collect(),
                guid: *guid,
            },
            _ => self.clone(),
        }
    }

    pub(super) fn collect_value_dependencies(&self, dependencies: &mut BTreeSet<(String, String)>) {
        match self {
            Self::Vector(element) => element.collect_value_dependencies(dependencies),
            Self::Named {
                namespace,
                name,
                arguments,
                ..
            } => {
                if canonical::winrt_type_from_name(namespace, name).is_none() {
                    dependencies.insert((namespace.clone(), name.clone()));
                }
                for argument in arguments {
                    argument.collect_value_dependencies(dependencies);
                }
            }
            _ => {}
        }
    }

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
            TypeKind::Void => Self::Void,
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
                .ok_or_else(|| Error::InvalidType {
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

    pub(super) fn write(
        &self,
        namespace: &str,
        layout: Layout,
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        if let Some(canonical) = self.canonical() {
            return Ok(canonical.write());
        }
        Ok(match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::Object => quote! { Option<windows_core::IInspectable> },
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
                value_type,
                namespace: target,
                name,
                arguments,
                ..
            } => {
                if layout == Layout::Package
                    && external::package_crate(target, name)
                    && let Some(crate_name) = external::package_crate_name(target, name)
                {
                    let crate_name = tokens::ident(crate_name);
                    let name = tokens::ident(name);
                    let arguments = arguments
                        .iter()
                        .map(|argument| argument.write(namespace, layout))
                        .collect::<Result<Vec<_>, _>>()?;
                    let name = if arguments.is_empty() {
                        quote! { #crate_name::#name }
                    } else {
                        quote! { #crate_name::#name<#(#arguments),*> }
                    };
                    return Ok(if *value_type {
                        name
                    } else {
                        quote! { Option<#name> }
                    });
                }
                let path = tokens::namespace(namespace, target, layout);
                let name = tokens::ident(name);
                let arguments = arguments
                    .iter()
                    .map(|argument| argument.write(namespace, layout))
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
            Self::Void => "void".to_string(),
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
            Self::Void => return None,
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
            _ if self.canonical().is_some_and(canonical::Type::is_guid) => "g16",
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

    pub(super) fn is_event_token(&self) -> bool {
        matches!(self, Self::I64)
            || self
                .canonical()
                .is_some_and(canonical::Type::is_event_token)
    }

    fn canonical(&self) -> Option<canonical::Type> {
        match self {
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty() => canonical::winrt_type_from_name(namespace, name),
            _ => None,
        }
    }

    pub(super) fn properties(
        &self,
        values: &Values,
        stack: &mut BTreeSet<(String, String)>,
        owner: &str,
    ) -> Result<Properties, Error> {
        Ok(match self {
            Self::Void => {
                return Err(Error::UnsupportedType {
                    name: owner.to_string(),
                    shape: self.shape(),
                });
            }
            Self::String | Self::Object => Properties {
                copyable: false,
                eq: true,
            },
            Self::F32 | Self::F64 => Properties {
                copyable: true,
                eq: false,
            },
            _ if self.canonical().is_some_and(canonical::Type::is_guid) => Properties {
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

    pub(super) fn write_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        if let Some(canonical) = self.canonical() {
            return Ok(canonical.write());
        }
        Ok(match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::Object => quote! { windows_core::IInspectable },
            Self::Generic(index) => {
                let Some(name) = generics.get(*index as usize) else {
                    return Err(Error::UnsupportedType {
                        name: namespace.to_string(),
                        shape: format!("generic parameter {index}"),
                    });
                };
                let name = tokens::ident(name);
                quote! { #name }
            }

            Self::Vector(element) => element.write_name(namespace, layout, generics)?,
            Self::Named {
                namespace: target,
                name,
                arguments,
                ..
            } => {
                if (namespace != target
                    || (layout == Layout::Package && external::package_crate(target, name)))
                    && let Some(crate_name) = if layout == Layout::Package {
                        external::package_crate_name(target, name)
                    } else {
                        external::winrt_crate(target, name)
                    }
                {
                    let crate_name = tokens::ident(crate_name);
                    let name = tokens::ident(name);
                    let arguments = arguments
                        .iter()
                        .map(|argument| argument.write_name(namespace, layout, generics))
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(if arguments.is_empty() {
                        quote! { #crate_name::#name }
                    } else {
                        quote! { #crate_name::#name<#(#arguments),*> }
                    });
                }
                let path = tokens::namespace(namespace, target, layout);
                let name = tokens::ident(name);
                let arguments = arguments
                    .iter()
                    .map(|argument| argument.write_name(namespace, layout, generics))
                    .collect::<Result<Vec<_>, _>>()?;
                if arguments.is_empty() {
                    quote! { #path #name }
                } else {
                    quote! { #path #name<#(#arguments),*> }
                }
            }
            _ => self.write(namespace, layout)?,
        })
    }

    pub(super) fn write_minimal_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        if let Self::Named {
            namespace: target,
            name,
            arguments,
            ..
        } = self
            && let Some(crate_name) = external::minimal_crate(target, name)
        {
            let crate_name = tokens::ident(crate_name);
            let name = tokens::ident(name);
            let arguments = arguments
                .iter()
                .map(|argument| argument.write_name(namespace, layout, generics))
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(if arguments.is_empty() {
                quote! { #crate_name::#name }
            } else {
                quote! { #crate_name::#name<#(#arguments),*> }
            });
        }
        self.write_name(namespace, layout, generics)
    }

    pub(super) fn write_name_with_owner(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        owner: Option<&str>,
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        let Self::Named {
            namespace: target,
            name,
            arguments,
            ..
        } = self
        else {
            return self.write_name(namespace, layout, generics);
        };
        if owner.is_some_and(|owner| target == namespace && name == owner) {
            return Ok(quote! { Self });
        }
        if arguments.is_empty() {
            return self.write_name(namespace, layout, generics);
        }
        let path = tokens::namespace(namespace, target, layout);
        let name = tokens::ident(name);
        let arguments = arguments
            .iter()
            .map(|argument| argument.write_name_with_owner(namespace, layout, generics, owner))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }

    pub(super) fn is_external_minimal(&self) -> bool {
        matches!(
            self,
            Self::Named {
                namespace,
                name,
                ..
            } if external::minimal_crate(namespace, name).is_some()
        )
    }

    pub(super) fn write_default(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        let name = self.write_name(namespace, layout, generics)?;
        Ok(match self {
            Self::Generic(_) => quote! { <#name as windows_core::Type<#name>>::Default },
            Self::Object
            | Self::Named {
                value_type: false, ..
            } => quote! { Option<#name> },
            _ => name,
        })
    }

    pub(super) fn write_array_element(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<proc_macro2::TokenStream, Error> {
        if matches!(
            self,
            Self::String
                | Self::Object
                | Self::Named {
                    value_type: false,
                    ..
                }
        ) {
            self.write_name(namespace, layout, generics)
        } else {
            self.write_default(namespace, layout, generics)
        }
    }

    pub(super) fn write_abi(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<proc_macro2::TokenStream, Error> {
        use quote::quote;

        if let Some(canonical) = self.canonical() {
            return Ok(canonical.write());
        }
        Ok(match self {
            Self::Void => quote! { core::ffi::c_void },
            Self::String
            | Self::Object
            | Self::Named {
                value_type: false, ..
            } => {
                quote! { *mut core::ffi::c_void }
            }
            Self::Generic(index) => {
                let Some(name) = generics.get(*index as usize) else {
                    return Err(Error::UnsupportedType {
                        name: namespace.to_string(),
                        shape: format!("generic parameter {index}"),
                    });
                };
                let name = tokens::ident(name);
                quote! { windows_core::AbiType<#name> }
            }
            Self::Vector(element)
                if matches!(
                    element.as_ref(),
                    Self::String
                        | Self::Object
                        | Self::Named {
                            value_type: false,
                            ..
                        }
                ) =>
            {
                element.write_name(namespace, layout, generics)?
            }
            Self::Vector(element) => element.write_abi(values, namespace, layout, generics)?,
            Self::Named {
                value_type: true,
                namespace: target,
                name,
                arguments,
                ..
            } if arguments.is_empty() => {
                let written = self.write_name(namespace, layout, generics)?;
                match values.get(target, name) {
                    Some(Value::Enum(_)) => written,
                    Some(Value::Struct(_))
                        if values
                            .properties(target, name, &mut BTreeSet::new())?
                            .copyable =>
                    {
                        written
                    }
                    Some(Value::Struct(_)) => quote! { core::mem::MaybeUninit<#written> },
                    None => {
                        return Err(Error::InvalidType {
                            name: format!("{target}.{name}"),
                            message: "referenced value was not selected",
                        });
                    }
                }
            }
            Self::Named { .. } => {
                return Err(Error::UnsupportedType {
                    name: namespace.to_string(),
                    shape: self.shape(),
                });
            }
            _ => self.write_name(namespace, layout, generics)?,
        })
    }

    pub(super) fn is_interface(&self) -> bool {
        matches!(
            self,
            Self::Object
                | Self::Generic(_)
                | Self::Named {
                    value_type: false,
                    ..
                }
        )
    }

    pub(super) fn is_primitive(&self, values: &Values) -> bool {
        match self {
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
            | Self::F64 => true,
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty() => {
                canonical::winrt_type_from_name(namespace, name).is_some()
                    || matches!(values.get(namespace, name), Some(Value::Enum(_)))
            }

            _ => false,
        }
    }

    pub(super) fn package_input_by_ref(&self, _values: &Values, layout: Layout) -> bool {
        if layout != Layout::Package {
            return false;
        }
        matches!(
            self,
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty()
                && canonical::winrt_type_from_name(namespace, name)
                    .is_some_and(canonical::Type::is_guid)
        )
    }

    pub(super) fn is_copyable(&self, values: &Values, _owner: &str) -> Result<bool, Error> {
        Ok(match self {
            Self::Void | Self::Generic(_) | Self::Vector(_) => false,
            Self::String
            | Self::Object
            | Self::Named {
                value_type: false, ..
            } => false,
            Self::Named {
                value_type: true,
                namespace,
                name,
                arguments,
                ..
            } if arguments.is_empty()
                && canonical::winrt_type_from_name(namespace, name).is_none() =>
            {
                values
                    .properties(namespace, name, &mut BTreeSet::new())?
                    .copyable
            }
            _ => true,
        })
    }
}

fn trim_generic_arity(name: &str) -> &str {
    name.split_once('`').map_or(name, |(name, _)| name)
}
