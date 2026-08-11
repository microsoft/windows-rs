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
    Interface { namespace: String, name: String },
    Named { namespace: String, name: String },
}

#[derive(Clone, Copy)]
pub(super) struct TraitSupport {
    pub(super) copy: bool,
    pub(super) debug: bool,
    pub(super) partial_eq: bool,
    pub(super) eq: bool,
}

impl TraitSupport {
    pub(super) const NONE: Self = Self {
        copy: false,
        debug: false,
        partial_eq: false,
        eq: false,
    };

    pub(super) const ALL: Self = Self {
        copy: true,
        debug: true,
        partial_eq: true,
        eq: true,
    };

    pub(super) fn combine(&mut self, other: Self) {
        self.copy &= other.copy;
        self.debug &= other.debug;
        self.partial_eq &= other.partial_eq;
        self.eq &= other.eq;
    }
}

impl Type {
    pub(super) fn lower_parameter(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: windows_metadata2::Type,
        input_only: bool,
    ) -> Result<Self, Error> {
        let ty = Self::lower(database, file, owner, ty)?;
        Ok(if input_only { ty.into_input() } else { ty })
    }

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
            TypeKind::Pointer(element) => Self::Pointer {
                mutable: !is_const,
                element: Box::new(Self::lower_with_nested(
                    database, file, owner, *element, nested,
                )?),
            },
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
                Self::Named {
                    namespace: namespace.to_string(),
                    name: name.to_string(),
                }
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
                    Self::Named {
                        namespace: namespace.to_string(),
                        name: name.to_string(),
                    }
                } else {
                    Self::Interface {
                        namespace: namespace.to_string(),
                        name: name.to_string(),
                    }
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

    pub(super) fn write_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
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
            Self::String if !projection.is_sys() => quote! { windows_core::PCWSTR },
            Self::String => quote! { PCWSTR },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::Array { element, len } => {
                let element = element.write_projection(namespace, layout, projection);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#element; #len] }
            }

            Self::Pointer { mutable, element } => {
                let element = element.write_projection(namespace, layout, projection);
                if *mutable {
                    quote! { *mut #element }
                } else {
                    quote! { *const #element }
                }
            }
            Self::Interface { .. } => quote! { *mut core::ffi::c_void },
            Self::Named {
                namespace: target,
                name,
            } => {
                if projection.is_minimal()
                    && let Some(crate_name) = external::minimal_crate(target, name)
                {
                    let crate_name = tokens::ident(crate_name);
                    let name = tokens::ident(name);
                    return quote! { #crate_name::#name };
                }
                if !projection.is_sys()
                    && let Some(core) = core_projection(target, name)
                {
                    return core;
                }
                let path = tokens::namespace(namespace, target, layout);
                let name = tokens::ident(name);
                quote! { #path #name }
            }
        }
    }

    pub(super) fn write_abi_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        match self {
            Self::Array { element, len } => {
                let element = element.write_abi_projection(namespace, layout, projection);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#element; #len] }
            }
            Self::Pointer { mutable, element } => {
                let element = element.write_abi_projection(namespace, layout, projection);
                if *mutable {
                    quote! { *mut #element }
                } else {
                    quote! { *const #element }
                }
            }
            Self::Interface { .. } => quote! { *mut core::ffi::c_void },
            Self::Named { namespace, name }
                if !projection.is_sys()
                    && name == "HSTRING"
                    && (namespace == "Windows.Win32"
                        || namespace.starts_with("Windows.Win32.")) =>
            {
                quote! { *mut core::ffi::c_void }
            }
            _ => self.write_projection(namespace, layout, projection),
        }
    }

    pub(super) fn write_field_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        match self {
            Self::Array { element, len } if !projection.is_sys() => {
                let element = element.write_field_projection(namespace, layout, projection);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#element; #len] }
            }
            Self::Interface { .. } if !projection.is_sys() => {
                let interface = self.write_public(namespace, layout);
                quote! { core::mem::ManuallyDrop<Option<#interface>> }
            }
            _ => self.write_projection(namespace, layout, projection),
        }
    }

    pub(super) fn write_constant_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if !projection.is_sys()
            && let Self::Named {
                namespace: target,
                name,
            } = self
            && (target == "Windows.Win32" || target.starts_with("Windows.Win32."))
        {
            return match name.as_str() {
                "PSTR" => quote! { windows_core::PCSTR },
                "PWSTR" => quote! { windows_core::PCWSTR },
                _ => self.write_projection(namespace, layout, projection),
            };
        }
        self.write_projection(namespace, layout, projection)
    }

    pub(super) fn mutable_string_pointer(&self) -> bool {
        matches!(
            self,
            Self::Named {
                namespace,
                name,
            } if (namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32."))
                && (name == "PSTR" || name == "PWSTR")
        )
    }

    pub(super) fn write_public(&self, namespace: &str, layout: Layout) -> TokenStream {
        match self {
            Self::Interface {
                namespace: target,
                name,
            } => {
                if let Some(core) = core_projection(target, name) {
                    core
                } else {
                    let path = tokens::namespace(namespace, target, layout);
                    let name = tokens::ident(name);
                    quote! { #path #name }
                }
            }
            _ => self.write_projection(namespace, layout, Projection::Minimal),
        }
    }

    pub(super) fn pointee(&self) -> Option<&Self> {
        match self {
            Self::Pointer { element, .. } => Some(element),
            _ => None,
        }
    }

    pub(super) fn is_interface(&self) -> bool {
        matches!(self, Self::Interface { .. })
    }

    pub(super) fn is_hresult(&self) -> bool {
        matches!(
            self,
            Self::Named { namespace, name }
                if name == "HRESULT"
                    && (namespace == "Windows.Win32"
                        || namespace == "Windows.Win32.Foundation")
        )
    }

    pub(super) fn is_bool(&self) -> bool {
        matches!(
            self,
            Self::Named { namespace, name }
                if name == "BOOL"
                    && (namespace == "Windows.Win32"
                        || namespace.starts_with("Windows.Win32."))
        )
    }

    pub(super) fn is_pcwstr(&self) -> bool {
        matches!(
            self,
            Self::Named { namespace, name }
                if name == "PCWSTR"
                    && (namespace == "Windows.Win32"
                        || namespace.starts_with("Windows.Win32."))
        )
    }

    pub(super) fn is_indirect_return(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named { namespace, name } = self else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? == TypeCategory::Struct
                && !definition.has_attribute("NativeTypedefAttribute")?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn exceeds_retval_limit(&self, database: &Database) -> Result<bool, Error> {
        Ok(self.abi_layout(database, &mut BTreeSet::new())?.0 > 16)
    }

    fn abi_layout(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<(usize, usize), Error> {
        Ok(match self {
            Self::Void => (0, 1),
            Self::Boolean | Self::I8 | Self::U8 => (1, 1),
            Self::Char | Self::I16 | Self::U16 => (2, 2),
            Self::I64 | Self::U64 | Self::F64 => (8, 8),
            Self::Array { element, len } => {
                let (size, align) = element.abi_layout(database, stack)?;
                (size.saturating_mul(*len), align)
            }
            Self::Named { namespace, name } => {
                let key = (namespace.clone(), name.clone());
                if !stack.insert(key.clone()) {
                    return Ok((0, 1));
                }
                let mut result = (4usize, 4usize);
                for entity in database.type_definitions(namespace, name) {
                    let definition = database.definition(*entity).unwrap();
                    if definition.category()? != TypeCategory::Struct {
                        continue;
                    }
                    let explicit = definition
                        .type_attributes()?
                        .contains(TypeAttributes::EXPLICIT_LAYOUT);
                    let packing = definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .filter(|packing| *packing != 0)
                        .map(usize::from);
                    let mut definition_layout = (0usize, 1usize);
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        let (field_size, mut field_align) =
                            Self::lower(database, field.entity().file(), name, field.signature()?)?
                                .abi_layout(database, stack)?;
                        if let Some(packing) = packing {
                            field_align = field_align.min(packing);
                        }
                        if explicit {
                            definition_layout.0 = definition_layout.0.max(field_size);
                        } else {
                            definition_layout.0 = align_up(definition_layout.0, field_align);
                            definition_layout.0 = definition_layout.0.saturating_add(field_size);
                        }
                        definition_layout.1 = definition_layout.1.max(field_align);
                    }
                    if definition_layout.0 > result.0 {
                        result = definition_layout;
                    }
                }
                stack.remove(&key);
                result
            }
            Self::I32 | Self::U32 | Self::F32 => (4, 4),
            Self::String
            | Self::ISize
            | Self::USize
            | Self::Pointer { .. }
            | Self::Interface { .. } => (8, 8),
        })
    }
}

fn align_up(value: usize, align: usize) -> usize {
    value.saturating_add(align - 1) & !(align - 1)
}

pub(super) fn is_core_projection(namespace: &str, name: &str) -> bool {
    core_projection(namespace, name).is_some()
}

fn named_traits(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<TraitSupport, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(TraitSupport::NONE);
    }
    let mut result = TraitSupport::ALL;
    let definitions = database.type_definitions(namespace, name);
    if definitions.is_empty() {
        result = TraitSupport::NONE;
    }
    for entity in definitions {
        let definition = database.definition(*entity).unwrap();
        let traits = match definition.category()? {
            TypeCategory::Enum => TraitSupport::ALL,
            TypeCategory::Delegate => TraitSupport {
                copy: true,
                debug: true,
                partial_eq: false,
                eq: false,
            },
            TypeCategory::Struct => {
                if definition
                    .type_attributes()?
                    .contains(TypeAttributes::EXPLICIT_LAYOUT)
                    || definition.has_attribute("AlignmentAttribute")?
                    || definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .is_some()
                {
                    TraitSupport::NONE
                } else {
                    let mut fields = TraitSupport::ALL;
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            let ty = Type::lower(
                                database,
                                field.entity().file(),
                                name,
                                field.signature()?,
                            )?;
                            fields.combine(ty.projected_traits(database, stack)?);
                        }
                    }
                    fields
                }
            }
            _ => TraitSupport::NONE,
        };
        result.combine(traits);
    }
    stack.remove(&key);
    Ok(result)
}

pub(super) fn core_projection(namespace: &str, name: &str) -> Option<TokenStream> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if !win32 {
        return None;
    }
    Some(match name {
        "GUID" => quote! { windows_core::GUID },
        "HRESULT" => quote! { windows_core::HRESULT },
        "BOOL" => quote! { windows_core::BOOL },
        "PSTR" => quote! { windows_core::PSTR },
        "PWSTR" => quote! { windows_core::PWSTR },
        "PCSTR" => quote! { windows_core::PCSTR },
        "PCWSTR" => quote! { windows_core::PCWSTR },
        "BSTR" => quote! { windows_core::BSTR },
        "HSTRING" => quote! { windows_core::HSTRING },
        "IUnknown" => quote! { windows_core::IUnknown },
        "IInspectable" => quote! { windows_core::IInspectable },
        "NTSTATUS" => quote! { windows_core::NTSTATUS },
        "RPC_STATUS" => quote! { windows_core::RPC_STATUS },
        "EventRegistrationToken" => quote! { i64 },
        _ => return None,
    })
}

impl Type {
    pub(super) fn normalize_alias(self, namespace: &str, name: &str) -> Self {
        match (namespace, name) {
            ("Windows.Win32", "BSTR" | "PCWSTR") => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U16),
            },
            ("Windows.Win32", "PWSTR") => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U16),
            },
            ("Windows.Win32", "PCSTR") => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U8),
            },
            ("Windows.Win32", "PSTR") => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U8),
            },
            _ => self,
        }
    }

    pub(super) fn named_types(&self, mut add: impl FnMut(&str, &str)) {
        self.visit_named(&mut add);
    }

    pub(super) fn projected_traits(
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
            Self::Named { namespace, name } => {
                if is_core_projection(namespace, name) {
                    TraitSupport::ALL
                } else {
                    named_traits(database, namespace, name, stack)?
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
            Self::Interface { namespace, name } | Self::Named { namespace, name } => {
                add(namespace, name);
            }
            _ => {}
        }
    }

    fn into_input(self) -> Self {
        match self {
            Self::Named { namespace, name } if namespace == "Windows.Win32" && name == "PWSTR" => {
                Self::Named {
                    namespace,
                    name: "PCWSTR".to_string(),
                }
            }
            Self::Named { namespace, name } if namespace == "Windows.Win32" && name == "PSTR" => {
                Self::Named {
                    namespace,
                    name: "PCSTR".to_string(),
                }
            }
            _ => self,
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
                Self::constant_underlying_inner(database, entity.file(), owner, &underlying, stack)
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
