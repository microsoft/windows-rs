use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::RwLock,
};

#[derive(Default)]
pub(super) struct DependencyCache {
    values: RwLock<BTreeMap<(String, String), BTreeSet<(String, String)>>>,
    interfaces: RwLock<BTreeMap<(String, String), InterfaceDependencies>>,
    interface_bases: BTreeMap<(String, String), BTreeSet<(String, String)>>,
    sys_namespaces: BTreeSet<String>,
}

#[derive(Clone, Default)]
pub(super) struct InterfaceDependencies {
    pub(super) package: BTreeSet<(String, String)>,
    pub(super) manifest: BTreeSet<(String, String)>,
}

impl DependencyCache {
    pub(super) fn new(
        database: &Database,
        bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
        sys_namespaces: BTreeSet<String>,
    ) -> Result<Self, Error> {
        let mut interface_bases = BTreeMap::<(String, String), BTreeSet<(String, String)>>::new();
        for (entity, bases) in bases {
            let definition = database.definition(*entity).unwrap();
            interface_bases
                .entry((
                    definition.namespace()?.to_string(),
                    definition.name()?.to_string(),
                ))
                .or_default()
                .extend(bases.iter().cloned());
        }
        Ok(Self {
            values: RwLock::default(),
            interfaces: RwLock::default(),
            interface_bases,
            sys_namespaces,
        })
    }

    pub(super) fn package_sys_override(
        &self,
        dependencies: &BTreeSet<(String, String)>,
    ) -> Option<BTreeSet<(String, String)>> {
        if dependencies
            .iter()
            .all(|(namespace, _)| self.supports_package_sys_namespace(namespace))
        {
            return None;
        }
        Some(
            dependencies
                .iter()
                .filter(|(namespace, _)| self.supports_package_sys_namespace(namespace))
                .cloned()
                .collect(),
        )
    }

    fn supports_package_sys_namespace(&self, namespace: &str) -> bool {
        namespace == "Windows.Win32"
            || !namespace.starts_with("Windows.Win32.")
            || self.sys_namespaces.contains(namespace)
    }

    pub(super) fn interface_dependencies(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
    ) -> Result<InterfaceDependencies, Error> {
        let key = (namespace.to_string(), name.to_string());
        if let Some(dependencies) = self.interfaces.read().unwrap().get(&key) {
            return Ok(dependencies.clone());
        }
        let mut dependencies = InterfaceDependencies::default();
        let owner = format!("{namespace}.{name}");
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            for method in definition.methods()? {
                let signature = native_signature::Signature::lower(database, self, method, &owner)?;
                dependencies
                    .package
                    .extend(signature.package_dependencies().iter().cloned());
                dependencies
                    .manifest
                    .extend(signature.manifest_dependencies());
            }
        }
        self.interfaces
            .write()
            .unwrap()
            .insert(key, dependencies.clone());
        Ok(dependencies)
    }

    fn direct(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
    ) -> Result<BTreeSet<(String, String)>, Error> {
        let key = (namespace.to_string(), name.to_string());
        if let Some(dependencies) = self.values.read().unwrap().get(&key) {
            return Ok(dependencies.clone());
        }
        let mut dependencies = BTreeSet::new();
        for entity in database.type_definitions(namespace, name) {
            Type::collect_definition_direct_dependencies(
                database,
                database.definition(*entity).unwrap(),
                namespace,
                name,
                &mut dependencies,
            )?;
        }
        if let Some(bases) = self.interface_bases.get(&key) {
            dependencies.extend(bases.iter().cloned());
        }
        self.values
            .write()
            .unwrap()
            .insert(key, dependencies.clone());
        Ok(dependencies)
    }

    fn expand(
        &self,
        database: &Database,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        let key = (namespace.to_string(), name.to_string());
        if is_core_projection(namespace, name) || !stack.insert(key.clone()) {
            return Ok(());
        }
        for (namespace, name) in self.direct(database, namespace, name)? {
            dependencies.insert((namespace.clone(), name.clone()));
            self.expand(database, &namespace, &name, stack, dependencies)?;
        }
        stack.remove(&key);
        Ok(())
    }

    fn expand_interface_bases(
        &self,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) {
        let key = (namespace.to_string(), name.to_string());
        if !stack.insert(key.clone()) {
            return;
        }
        if let Some(bases) = self.interface_bases.get(&key) {
            for (namespace, name) in bases {
                dependencies.insert((namespace.clone(), name.clone()));
                self.expand_interface_bases(namespace, name, stack, dependencies);
            }
        }
        stack.remove(&key);
    }
}

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
    Array {
        element: Box<Self>,
        len: usize,
    },
    Pointer {
        mutable: bool,
        element: Box<Self>,
    },
    Interface {
        namespace: String,
        name: String,
        arguments: Vec<ty::Type>,
    },
    Named {
        namespace: String,
        name: String,
        canonical: Option<canonical::Type>,
    },
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
    pub(super) fn named(namespace: impl Into<String>, name: impl Into<String>) -> Self {
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

    fn canonical(&self) -> Option<canonical::Type> {
        match self {
            Self::Named { canonical, .. } => *canonical,
            _ => None,
        }
    }

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
            Self::String if layout.is_package() => quote! { windows_sys::core::PCWSTR },
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
                canonical,
            } => {
                if projection.is_sys()
                    && layout.is_package()
                    && let Some(core) = sys_core_projection(target, name)
                {
                    return core;
                }
                if let Some(canonical) = canonical {
                    return canonical.write();
                }
                if target.is_empty() && name == "PCWSTR" {
                    return quote! { windows_core::PCWSTR };
                }
                if !projection.is_sys()
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
            Self::Named { .. } if !projection.is_sys() && (self.is_bstr() || self.is_hstring()) => {
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
            Self::Named { .. } if !projection.is_sys() && (self.is_bstr() || self.is_hstring()) => {
                let value = self.write_public(namespace, layout);
                quote! { core::mem::ManuallyDrop<#value> }
            }
            _ => self.write_projection(namespace, layout, projection),
        }
    }

    pub(super) fn write_field_projection_owner(
        &self,
        namespace: &str,
        owner: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        if !layout.is_package() {
            return self.write_field_projection(namespace, layout, projection);
        }
        match self {
            Self::Array { element, len } => {
                let element =
                    element.write_field_projection_owner(namespace, owner, layout, projection);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#element; #len] }
            }
            Self::Pointer { mutable, element } => {
                let element = if element.is_interface() && !projection.is_sys() {
                    let interface = element.write_public(namespace, layout);
                    quote! { Option<#interface> }
                } else if !projection.is_sys() && (element.is_bstr() || element.is_hstring()) {
                    element.write_public(namespace, layout)
                } else {
                    element.write_field_projection_owner(namespace, owner, layout, projection)
                };
                if *mutable {
                    quote! { *mut #element }
                } else {
                    quote! { *const #element }
                }
            }
            Self::Named {
                namespace: target,
                name,
                ..
            } if target == namespace && name == owner => quote! { Self },
            _ => self.write_field_projection(namespace, layout, projection),
        }
    }

    pub(super) fn write_constant_projection(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> TokenStream {
        match (self.canonical(), projection.is_sys(), layout.is_package()) {
            (Some(canonical::Type::PStr), true, true) => quote! { windows_sys::core::PCSTR },
            (Some(canonical::Type::PWStr), true, true) => quote! { windows_sys::core::PCWSTR },
            (Some(canonical::Type::PStr), true, false) => quote! { PCSTR },
            (Some(canonical::Type::PWStr), true, false) => quote! { PCWSTR },
            (Some(canonical::Type::PStr), false, _) => quote! { windows_core::PCSTR },
            (Some(canonical::Type::PWStr), false, _) => quote! { windows_core::PCWSTR },
            _ => self.write_projection(namespace, layout, projection),
        }
    }

    pub(super) fn mutable_string_pointer(&self) -> bool {
        self.canonical()
            .is_some_and(canonical::Type::is_mutable_string)
    }

    pub(super) fn write_public(&self, namespace: &str, layout: Layout) -> TokenStream {
        self.write_public_with_owner(namespace, layout, None)
    }

    pub(super) fn write_public_with_owner(
        &self,
        namespace: &str,
        layout: Layout,
        owner: Option<&str>,
    ) -> TokenStream {
        match self {
            Self::Interface {
                namespace: target,
                name,
                arguments,
            } => {
                if owner.is_some_and(|owner| target == namespace && name == owner) {
                    return quote! { Self };
                }
                if arguments.is_empty()
                    && let Some(core) = core_projection(target, name)
                {
                    core
                } else {
                    ty::Type::Named {
                        value_type: false,
                        namespace: target.clone(),
                        name: name.clone(),
                        arguments: arguments.clone(),
                        guid: None,
                        canonical: canonical::winrt_type_from_name(target, name),
                    }
                    .write_name(namespace, layout, &[])
                    .unwrap()
                }
            }
            _ => self.write_projection(namespace, layout, Projection::Minimal),
        }
    }

    pub(super) fn write_public_pointer(&self, namespace: &str, layout: Layout) -> TokenStream {
        if let Self::Pointer { mutable, element } = self {
            let element = if element.is_interface() {
                let element = element.write_public(namespace, layout);
                quote! { Option<#element> }
            } else {
                element.write_public(namespace, layout)
            };
            if *mutable {
                quote! { *mut #element }
            } else {
                quote! { *const #element }
            }
        } else {
            self.write_public(namespace, layout)
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

    pub(super) fn interface_out(&self) -> Option<(bool, &Self)> {
        let Self::Pointer { mutable, element } = self else {
            return None;
        };
        if element.is_interface() {
            return Some((*mutable, element));
        }
        let Self::Pointer { element, .. } = element.as_ref() else {
            return None;
        };
        element.is_interface().then_some((*mutable, element))
    }

    pub(super) fn interface_pointer_depth(&self) -> Option<usize> {
        let mut depth = 0;
        let mut ty = self;
        while let Self::Pointer { element, .. } = ty {
            depth += 1;
            ty = element;
        }
        ty.is_interface().then_some(depth)
    }

    pub(super) fn write_interface_pointer(
        &self,
        namespace: &str,
        layout: Layout,
        owner: Option<&str>,
    ) -> Option<TokenStream> {
        fn write(
            ty: &Type,
            namespace: &str,
            layout: Layout,
            owner: Option<&str>,
        ) -> Option<TokenStream> {
            match ty {
                Type::Interface { .. } => {
                    let interface = ty.write_public_with_owner(namespace, layout, owner);
                    Some(quote! { Option<#interface> })
                }
                Type::Pointer { mutable, element } => {
                    let element = write(element, namespace, layout, owner)?;
                    Some(if *mutable {
                        quote! { *mut #element }
                    } else {
                        quote! { *const #element }
                    })
                }
                _ => None,
            }
        }
        write(self, namespace, layout, owner)
    }

    pub(super) fn is_direct_interface_pointer(&self) -> bool {
        self.pointee().is_some_and(Self::is_interface)
    }

    pub(super) fn producer_by_ref(&self, database: &Database) -> Result<bool, Error> {
        if self.is_bstr()
            || self.is_hstring()
            || self.is_pcstr()
            || self.is_pcwstr()
            || self.is_guid()
        {
            return Ok(true);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct {
                continue;
            }
            if !definition.has_attribute("NativeTypedefAttribute")? {
                return Ok(true);
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                return Ok(true);
            };
            let field = Self::lower(
                database,
                field.entity().file(),
                &format!("{namespace}.{name}"),
                field.signature()?,
            )?;
            if field.is_const_string()
                || field.mutable_string_pointer()
                || field.is_bstr()
                || field.is_hstring()
            {
                return Ok(true);
            }
            return Ok(!field.producer_primitive(database, &mut BTreeSet::new())?);
        }
        Ok(false)
    }

    pub(super) fn producer_primitive(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return self.is_primitive(database);
        };
        if self
            .canonical()
            .is_some_and(canonical::Type::is_native_primitive)
        {
            return Ok(true);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    if field.name()? != "Value" {
                        continue;
                    }
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.producer_primitive(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(super) fn resolves_to_delegate(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.resolves_to_delegate(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(super) fn is_delegate(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            if database.definition(*entity).unwrap().category()? == TypeCategory::Delegate {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn needs_output_pointer_cast(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        if let Self::Pointer { mutable, element } = self {
            return Ok(*mutable && **element == Self::Void);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        if is_core_projection(namespace, name) {
            return Ok(false);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            if field.name()? != "Value" {
                continue;
            }
            let field = Self::lower(database, field.entity().file(), name, field.signature()?)?;
            if matches!(field, Self::Pointer { .. })
                || field.needs_output_pointer_cast(database, stack)?
            {
                result = true;
                break;
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(super) fn is_mutable_void_double_pointer(&self) -> bool {
        matches!(
            self,
            Self::Pointer {
                mutable: true,
                element,
            } if matches!(
                element.as_ref(),
                Self::Pointer {
                    mutable: true,
                    element,
                } if element.as_ref() == &Self::Void
            )
        )
    }

    pub(super) fn is_wrapper_underlying(&self, database: &Database) -> Result<bool, Error> {
        Ok(self.is_mutable_void_double_pointer()
            || (self.is_primitive(database)?
                && !self.resolves_to_delegate(database, &mut BTreeSet::new())?
                && !matches!(
                    self,
                    Self::Pointer { element, .. } if element.as_ref() != &Self::Void
                )))
    }

    pub(super) fn is_noncanonical_pointer_alias(&self, database: &Database) -> Result<bool, Error> {
        Ok(self.pointer_alias(database)?.is_some())
    }

    pub(super) fn pointer_alias(&self, database: &Database) -> Result<Option<Self>, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(None);
        };
        if is_core_projection(namespace, name) {
            return Ok(None);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition
                .fields()?
                .filter_map(|field| (!field.is_literal().ok()?).then_some(field))
                .collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if matches!(
                ty,
                Self::Pointer { ref element, .. } if element.as_ref() != &Self::Void
            ) && !ty.is_mutable_void_double_pointer()
            {
                return Ok(Some(ty));
            }
        }
        Ok(None)
    }

    pub(super) fn resolved_pointer_alias(
        &self,
        database: &Database,
    ) -> Result<Option<Self>, Error> {
        self.resolved_pointer_alias_inner(database, &mut BTreeSet::new())
    }

    fn resolved_pointer_alias_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<Option<Self>, Error> {
        if let Some(ty) = self.pointer_alias(database)? {
            return Ok(Some(ty));
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(None);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(None);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if let Some(ty) = ty.resolved_pointer_alias_inner(database, stack)? {
                stack.remove(&key);
                return Ok(Some(ty));
            }
        }
        stack.remove(&key);
        Ok(None)
    }

    pub(super) fn is_primitive(&self, database: &Database) -> Result<bool, Error> {
        self.is_primitive_inner(database, &mut BTreeSet::new())
    }

    pub(super) fn is_integer(&self, database: &Database) -> Result<bool, Error> {
        self.is_integer_inner(database, &mut BTreeSet::new())
    }

    pub(super) fn is_newtype(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum => return Ok(true),
                TypeCategory::Struct => {
                    let mut fields = Vec::new();
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            fields.push(field);
                        }
                    }
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    if field.name()? != "Value" {
                        continue;
                    }
                    let ty =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if ty.is_primitive(database)?
                        && !matches!(
                            ty,
                            Self::Pointer { ref element, .. }
                                if element.as_ref() != &Self::Void
                        )
                    {
                        return Ok(true);
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn is_integer_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(matches!(
                self,
                Self::Char
                    | Self::I8
                    | Self::U8
                    | Self::I16
                    | Self::U16
                    | Self::I32
                    | Self::U32
                    | Self::I64
                    | Self::U64
                    | Self::ISize
                    | Self::USize
            ));
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum => {
                    result = true;
                    break;
                }
                TypeCategory::Struct if definition.has_attribute("NativeTypedefAttribute")? => {
                    let fields = definition.fields()?.collect::<Vec<_>>();
                    let [field] = fields.as_slice() else {
                        continue;
                    };
                    let field =
                        Self::lower(database, field.entity().file(), name, field.signature()?)?;
                    if field.is_integer_inner(database, stack)? {
                        result = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    fn is_primitive_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(matches!(
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
            ));
        };
        if self
            .canonical()
            .is_some_and(canonical::Type::is_native_primitive)
        {
            return Ok(true);
        }
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        let mut result = false;
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            match definition.category()? {
                TypeCategory::Enum | TypeCategory::Delegate => {
                    result = true;
                    break;
                }
                TypeCategory::Struct => {}
                _ => {}
            }
        }
        stack.remove(&key);
        Ok(result)
    }

    pub(super) fn is_hresult(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_hresult)
    }

    pub(super) fn is_void_alias(&self, database: &Database) -> Result<bool, Error> {
        self.is_void_alias_inner(database, &mut BTreeSet::new())
    }

    fn is_void_alias_inner(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        if self == &Self::Void {
            return Ok(true);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        let key = (namespace.clone(), name.clone());
        if !stack.insert(key.clone()) {
            return Ok(false);
        }
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct
                || !definition.has_attribute("NativeTypedefAttribute")?
            {
                continue;
            }
            let fields = definition.fields()?.collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if ty.is_void_alias_inner(database, stack)? {
                stack.remove(&key);
                return Ok(true);
            }
        }
        stack.remove(&key);
        Ok(false)
    }

    pub(super) fn is_guid(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_guid)
    }

    pub(super) fn is_hresult_package(&self) -> bool {
        self.is_hresult()
    }

    pub(super) fn is_ntstatus(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_ntstatus)
    }

    pub(super) fn is_bool(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_bool)
    }

    pub(super) fn is_bstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_bstr)
    }

    pub(super) fn is_hstring(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_hstring)
    }

    pub(super) fn is_pcwstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pcwstr)
    }

    pub(super) fn is_pstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pstr)
    }

    pub(super) fn is_pcstr(&self) -> bool {
        self.canonical().is_some_and(canonical::Type::is_pcstr)
    }

    pub(super) fn is_const_string(&self) -> bool {
        self.canonical()
            .is_some_and(canonical::Type::is_const_string)
    }

    pub(super) fn is_indirect_return(&self, database: &Database) -> Result<bool, Error> {
        if self.uses_winrt_projection() {
            return Ok(false);
        }
        if self.is_hresult() {
            return Ok(false);
        }
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? == TypeCategory::Struct {
                if !definition.has_attribute("NativeTypedefAttribute")? {
                    return Ok(true);
                }
                let fields = definition.fields()?.collect::<Vec<_>>();
                if !matches!(fields.as_slice(), [field] if field.name()? == "Value") {
                    return Ok(true);
                }
                let field = Self::lower(
                    database,
                    fields[0].entity().file(),
                    &format!("{namespace}.{name}"),
                    fields[0].signature()?,
                )?;
                return field.is_indirect_return(database);
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
            Self::I8 | Self::U8 => (1, 1),
            Self::I16 | Self::U16 => (2, 2),
            Self::I64 | Self::U64 | Self::F64 => (8, 8),
            Self::Array { element, len } => {
                let (size, align) = element.abi_layout(database, stack)?;
                (size.saturating_mul(*len), align.saturating_mul(*len))
            }
            Self::Named {
                namespace, name, ..
            } => {
                let key = (namespace.clone(), name.clone());
                if !stack.insert(key.clone()) {
                    return Ok((0, 1));
                }
                let mut result = None::<(usize, usize)>;
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
                    result = Some(result.map_or(definition_layout, |result| {
                        (
                            result.0.max(definition_layout.0),
                            result.1.max(definition_layout.1),
                        )
                    }));
                }
                stack.remove(&key);
                result.unwrap_or((4, 4))
            }
            _ => (4, 4),
        })
    }
}

pub(super) fn metadata_has_oversized_member(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    metadata_type_has_oversized_member(database, file, ty, None, &mut BTreeSet::new())
}

pub(super) fn metadata_exceeds_retval_limit(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
) -> Result<bool, Error> {
    let layout = metadata_type_layout(database, file, ty, None, &mut BTreeSet::new())?;
    Ok(layout.0 > 16)
}

fn metadata_type_layout(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: Option<Entity<TypeDef>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<(usize, usize), Error> {
    Ok(match &ty.kind {
        TypeKind::I8 | TypeKind::U8 => (1, 1),
        TypeKind::I16 | TypeKind::U16 => (2, 2),
        TypeKind::I64 | TypeKind::U64 | TypeKind::F64 => (8, 8),
        TypeKind::Array {
            element,
            rank,
            sizes,
            lower_bounds,
        } if *rank == 1 && sizes.len() == 1 && lower_bounds.iter().all(|bound| *bound == 0) => {
            let (size, align) = metadata_type_layout(database, file, element, owner, stack)?;
            (
                size.saturating_mul(sizes[0] as usize),
                align.saturating_mul(sizes[0] as usize).max(1),
            )
        }
        TypeKind::Value(id) => {
            let (namespace, name) =
                database
                    .type_name(file, *id)?
                    .ok_or_else(|| Error::InvalidType {
                        name: "retval".to_string(),
                        message: "native retval type has no name",
                    })?;
            let mut definitions = match database.resolve_type(file, *id)? {
                TypeResolution::Definition(definition) => vec![definition],
                TypeResolution::Candidates(candidates) => candidates.iter().collect(),
                TypeResolution::Specification(_) => Vec::new(),
            };
            if definitions.is_empty()
                && namespace.is_empty()
                && let Some(owner) = owner
            {
                definitions.extend(
                    database
                        .nested_types_of(owner)
                        .filter(|definition| {
                            definition.name().is_ok_and(|candidate| candidate == name)
                        })
                        .map(|definition| definition.entity()),
                );
            }
            let mut result = None::<(usize, usize)>;
            for entity in definitions {
                if !stack.insert(entity) {
                    continue;
                }
                let definition = database.definition(entity).unwrap();
                if definition.category()? == TypeCategory::Struct {
                    let explicit = definition
                        .type_attributes()?
                        .contains(TypeAttributes::EXPLICIT_LAYOUT);
                    let packing = definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .filter(|packing| *packing != 0)
                        .map(usize::from);
                    let mut layout = (0usize, 1usize);
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        let (field_size, mut field_align) = metadata_type_layout(
                            database,
                            field.entity().file(),
                            &field.signature()?,
                            Some(entity),
                            stack,
                        )?;
                        if let Some(packing) = packing {
                            field_align = field_align.min(packing);
                        }
                        if explicit {
                            layout.0 = layout.0.max(field_size);
                        } else {
                            layout.0 = align_up(layout.0, field_align);
                            layout.0 = layout.0.saturating_add(field_size);
                        }
                        layout.1 = layout.1.max(field_align);
                    }
                    result = Some(result.map_or(layout, |result| {
                        (result.0.max(layout.0), result.1.max(layout.1))
                    }));
                }
                stack.remove(&entity);
            }
            result.unwrap_or((4, 4))
        }
        _ => (4, 4),
    })
}

fn metadata_type_has_oversized_member(
    database: &Database,
    file: FileId,
    ty: &windows_metadata2::Type,
    owner: Option<Entity<TypeDef>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<bool, Error> {
    match &ty.kind {
        TypeKind::Array {
            element,
            rank,
            sizes,
            lower_bounds,
        } if *rank == 1 && sizes.len() == 1 && lower_bounds.iter().all(|bound| *bound == 0) => {
            let element = Type::lower(database, file, "retval", (**element).clone())?;
            Ok(element
                .abi_layout(database, &mut BTreeSet::new())?
                .0
                .saturating_mul(sizes[0] as usize)
                > 16)
        }
        TypeKind::Value(id) => {
            let (namespace, name) =
                database
                    .type_name(file, *id)?
                    .ok_or_else(|| Error::InvalidType {
                        name: "retval".to_string(),
                        message: "native retval type has no name",
                    })?;
            let mut definitions = match database.resolve_type(file, *id)? {
                TypeResolution::Definition(definition) => vec![definition],
                TypeResolution::Candidates(candidates) => candidates.iter().collect(),
                TypeResolution::Specification(_) => Vec::new(),
            };
            if definitions.is_empty()
                && namespace.is_empty()
                && let Some(owner) = owner
            {
                definitions.extend(
                    database
                        .nested_types_of(owner)
                        .filter(|definition| {
                            definition.name().is_ok_and(|candidate| candidate == name)
                        })
                        .map(|definition| definition.entity()),
                );
            }
            for entity in definitions {
                if !stack.insert(entity) {
                    continue;
                }
                let definition = database.definition(entity).unwrap();
                if definition.category()? == TypeCategory::Struct {
                    for field in definition.fields()? {
                        if field.is_literal()? {
                            continue;
                        }
                        if metadata_type_has_oversized_member(
                            database,
                            field.entity().file(),
                            &field.signature()?,
                            Some(entity),
                            stack,
                        )? {
                            stack.remove(&entity);
                            return Ok(true);
                        }
                    }
                }
                stack.remove(&entity);
            }
            Ok(false)
        }
        _ => Ok(false),
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
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    if definitions.is_empty() {
        result = TraitSupport::NONE;
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
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
                    || definition
                        .layout()?
                        .map(|layout| layout.packing_size())
                        .transpose()?
                        .is_some()
                {
                    TraitSupport {
                        copy: false,
                        ..TraitSupport::NONE
                    }
                } else {
                    let nested = database
                        .nested_types_of(entity)
                        .enumerate()
                        .map(|(index, definition)| {
                            Ok((definition.name()?.to_string(), format!("{name}_{index}")))
                        })
                        .collect::<Result<Vec<_>, Error>>()?;
                    let substitutions = nested
                        .iter()
                        .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
                        .collect::<Vec<_>>();
                    let projected = nested
                        .iter()
                        .map(|(_, projected)| projected.as_str())
                        .collect::<BTreeSet<_>>();
                    let mut fields = TraitSupport::ALL;
                    for field in definition.fields()? {
                        if !field.is_literal()? {
                            let ty = Type::lower_with_nested(
                                database,
                                field.entity().file(),
                                name,
                                field.signature()?,
                                &substitutions,
                            )?
                            .qualify_projected_nested(namespace, &projected);
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

fn named_copyable(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<bool, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(false);
    }
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    if definitions.is_empty() {
        stack.remove(&key);
        return Ok(false);
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
        let copyable = match definition.category()? {
            TypeCategory::Enum | TypeCategory::Delegate => true,
            TypeCategory::Struct => {
                let nested = database
                    .nested_types_of(entity)
                    .enumerate()
                    .map(|(index, definition)| {
                        Ok((definition.name()?.to_string(), format!("{name}_{index}")))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                let substitutions = nested
                    .iter()
                    .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
                    .collect::<Vec<_>>();
                let projected = nested
                    .iter()
                    .map(|(_, projected)| projected.as_str())
                    .collect::<BTreeSet<_>>();
                let mut copyable = true;
                for field in definition.fields()? {
                    if !field.is_literal()? {
                        let ty = Type::lower_with_nested(
                            database,
                            field.entity().file(),
                            name,
                            field.signature()?,
                            &substitutions,
                        )?
                        .qualify_projected_nested(namespace, &projected);
                        if !ty.projected_copyable(database, stack)? {
                            copyable = false;
                            break;
                        }
                    }
                }
                copyable
            }
            _ => false,
        };
        if !copyable {
            stack.remove(&key);
            return Ok(false);
        }
    }
    stack.remove(&key);
    Ok(true)
}

fn named_has_explicit_layout(
    database: &Database,
    namespace: &str,
    name: &str,
    stack: &mut BTreeSet<(String, String)>,
) -> Result<bool, Error> {
    let key = (namespace.to_string(), name.to_string());
    if !stack.insert(key.clone()) {
        return Ok(false);
    }
    let mut definitions = database.type_definitions(namespace, name).to_vec();
    if definitions.is_empty() {
        definitions = projected_nested_definitions(database, namespace, name);
    }
    for entity in definitions {
        let definition = database.definition(entity).unwrap();
        if definition
            .type_attributes()?
            .contains(TypeAttributes::EXPLICIT_LAYOUT)
        {
            stack.remove(&key);
            return Ok(true);
        }
        if definition.category()? != TypeCategory::Struct {
            continue;
        }
        let nested = database
            .nested_types_of(entity)
            .enumerate()
            .map(|(index, definition)| {
                Ok((definition.name()?.to_string(), format!("{name}_{index}")))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let substitutions = nested
            .iter()
            .map(|(metadata, projected)| (metadata.as_str(), projected.as_str()))
            .collect::<Vec<_>>();
        let projected = nested
            .iter()
            .map(|(_, projected)| projected.as_str())
            .collect::<BTreeSet<_>>();
        for field in definition.fields()? {
            if field.is_literal()? {
                continue;
            }
            let ty = Type::lower_with_nested(
                database,
                field.entity().file(),
                name,
                field.signature()?,
                &substitutions,
            )?
            .qualify_projected_nested(namespace, &projected);
            if ty.projected_has_explicit_layout(database, stack)? {
                stack.remove(&key);
                return Ok(true);
            }
        }
    }
    stack.remove(&key);
    Ok(false)
}

fn projected_nested_definitions(
    database: &Database,
    namespace: &str,
    name: &str,
) -> Vec<Entity<TypeDef>> {
    let mut parent = name;
    let mut indices = Vec::new();
    while let Some((candidate, index)) = parent.rsplit_once('_') {
        let Ok(index) = index.parse::<usize>() else {
            break;
        };
        indices.push(index);
        parent = candidate;
        let roots = database.type_definitions(namespace, parent);
        if roots.is_empty() {
            continue;
        }
        let mut definitions = Vec::new();
        for root in roots {
            let mut current = *root;
            let mut found = true;
            for index in indices.iter().rev() {
                let Some(nested) = database.nested_types_of(current).nth(*index) else {
                    found = false;
                    break;
                };
                current = nested.entity();
            }
            if found {
                definitions.push(current);
            }
        }
        if !definitions.is_empty() {
            return definitions;
        }
    }
    Vec::new()
}

pub(super) fn core_projection(namespace: &str, name: &str) -> Option<TokenStream> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if !win32 {
        return None;
    }
    if let Some(canonical) = canonical::type_from_name(namespace, name)
        .or_else(|| canonical::native_core_from_name(namespace, name))
    {
        return Some(canonical.write());
    }
    None
}

fn sys_core_projection(namespace: &str, name: &str) -> Option<TokenStream> {
    let win32 = namespace == "Windows.Win32" || namespace.starts_with("Windows.Win32.");
    if !win32 {
        return None;
    }
    if let Some(canonical) = canonical::type_from_name(namespace, name)
        .or_else(|| canonical::native_core_from_name(namespace, name))
    {
        return Some(canonical.write_sys());
    }
    None
}

impl Type {
    pub(super) fn projected_copyable(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        match self {
            Self::Void | Self::Interface { .. } => Ok(false),
            Self::Array { element, .. } => element.projected_copyable(database, stack),
            Self::Named { .. } if self.is_bstr() || self.is_hstring() => Ok(false),
            Self::Named {
                namespace, name, ..
            } => named_copyable(database, namespace, name, stack),
            _ => Ok(true),
        }
    }

    pub(super) fn projected_has_explicit_layout(
        &self,
        database: &Database,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<bool, Error> {
        match self {
            Self::Array { element, .. } => element.projected_has_explicit_layout(database, stack),
            Self::Named {
                namespace, name, ..
            } => named_has_explicit_layout(database, namespace, name, stack),
            _ => Ok(false),
        }
    }

    fn qualify_projected_nested(mut self, namespace: &str, projected: &BTreeSet<&str>) -> Self {
        match &mut self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                **element = element
                    .clone()
                    .qualify_projected_nested(namespace, projected);
            }
            Self::Named {
                namespace: target,
                name,
                ..
            } if target.is_empty() && projected.contains(name.as_str()) => {
                *target = namespace.to_string();
            }
            _ => {}
        }
        self
    }
    pub(super) fn normalize_alias(self, namespace: &str, name: &str) -> Self {
        match canonical::native_alias_from_name(namespace, name) {
            Some(canonical::Type::BStr | canonical::Type::PcWStr) => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U16),
            },
            Some(canonical::Type::PWStr) => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U16),
            },
            Some(canonical::Type::PcStr) => Self::Pointer {
                mutable: false,
                element: Box::new(Self::U8),
            },
            Some(canonical::Type::PStr) => Self::Pointer {
                mutable: true,
                element: Box::new(Self::U8),
            },
            _ => self,
        }
    }

    pub(super) fn named_types(&self, mut add: impl FnMut(&str, &str)) {
        self.visit_named(&mut add);
    }

    pub(super) fn uses_winrt_projection(&self) -> bool {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.uses_winrt_projection()
            }
            Self::Interface { .. } => false,
            Self::Named {
                namespace, name, ..
            } => {
                canonical::type_from_name(namespace, name).is_none()
                    && (namespace == "Windows" || namespace.starts_with("Windows."))
                    && namespace != "Windows.Win32"
                    && !namespace.starts_with("Windows.Win32.")
            }
            _ => false,
        }
    }

    pub(super) fn package_dependencies(
        &self,
        database: &Database,
        cache: &DependencyCache,
    ) -> Result<BTreeSet<(String, String)>, Error> {
        let mut dependencies = BTreeSet::new();
        self.collect_package_dependencies(
            database,
            cache,
            &mut BTreeSet::new(),
            &mut dependencies,
        )?;
        Ok(dependencies)
    }

    pub(super) fn is_wrapper(&self, database: &Database) -> Result<bool, Error> {
        let Self::Named {
            namespace, name, ..
        } = self
        else {
            return Ok(false);
        };
        for entity in database.type_definitions(namespace, name) {
            let definition = database.definition(*entity).unwrap();
            if definition.category()? != TypeCategory::Struct {
                continue;
            }
            let fields = definition
                .fields()?
                .filter_map(|field| (!field.is_literal().ok()?).then_some(field))
                .collect::<Vec<_>>();
            let [field] = fields.as_slice() else {
                continue;
            };
            if field.name()? != "Value" {
                continue;
            }
            let ty = Self::lower(
                database,
                field.entity().file(),
                definition.name()?,
                field.signature()?,
            )?;
            if ty.is_primitive(database)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn collect_package_dependencies(
        &self,
        database: &Database,
        cache: &DependencyCache,
        stack: &mut BTreeSet<(String, String)>,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.collect_package_dependencies(database, cache, stack, dependencies)?;
            }
            Self::Interface {
                namespace,
                name,
                arguments,
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
                cache.expand_interface_bases(namespace, name, stack, dependencies);
                for argument in arguments {
                    argument.collect_value_dependencies(dependencies);
                }
            }
            Self::Named {
                namespace, name, ..
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
                cache.expand(database, namespace, name, stack, dependencies)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn collect_definition_direct_dependencies(
        database: &Database,
        definition: TypeDefinition<'_>,
        namespace: &str,
        projected_name: &str,
        dependencies: &mut BTreeSet<(String, String)>,
    ) -> Result<(), Error> {
        match definition.category()? {
            TypeCategory::Delegate => {
                let owner = format!("{namespace}.{projected_name}");
                for method in definition.methods()? {
                    let signature = method.signature()?;
                    Self::lower(
                        database,
                        method.entity().file(),
                        &owner,
                        signature.return_type,
                    )?
                    .collect_direct_dependencies(dependencies);
                    for ty in signature.parameters {
                        Self::lower(database, method.entity().file(), &owner, ty)?
                            .collect_direct_dependencies(dependencies);
                    }
                }
                return Ok(());
            }
            TypeCategory::Enum | TypeCategory::Struct => {}
            _ => return Ok(()),
        }
        let nested = database
            .nested_types_of(definition.entity())
            .enumerate()
            .map(|(index, definition)| {
                Ok((
                    definition.name()?.to_string(),
                    format!("{projected_name}_{index}"),
                    definition,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let substitutions = nested
            .iter()
            .map(|(metadata, projected, _)| (metadata.as_str(), projected.as_str()))
            .collect::<Vec<_>>();
        let typedef = definition.has_attribute("NativeTypedefAttribute")?;
        for field in definition.fields()? {
            if field.is_literal()? {
                continue;
            }
            let ty = Self::lower_with_nested(
                database,
                field.entity().file(),
                projected_name,
                field.signature()?,
                &substitutions,
            )?;
            let ty = if typedef {
                ty.normalize_alias(namespace, projected_name)
            } else {
                ty
            };
            ty.collect_direct_dependencies(dependencies);
        }
        for (_, projected, definition) in nested {
            Self::collect_definition_direct_dependencies(
                database,
                definition,
                namespace,
                &projected,
                dependencies,
            )?;
        }
        Ok(())
    }

    fn collect_direct_dependencies(&self, dependencies: &mut BTreeSet<(String, String)>) {
        match self {
            Self::Array { element, .. } | Self::Pointer { element, .. } => {
                element.collect_direct_dependencies(dependencies);
            }
            Self::Interface {
                namespace, name, ..
            }
            | Self::Named {
                namespace, name, ..
            } => {
                dependencies.insert((namespace.clone(), name.clone()));
            }
            _ => {}
        }
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
            Self::Named {
                namespace, name, ..
            } => {
                if self.is_bstr() || self.is_hstring() {
                    TraitSupport {
                        copy: false,
                        debug: true,
                        partial_eq: true,
                        eq: true,
                    }
                } else if is_core_projection(namespace, name) {
                    TraitSupport::ALL
                } else {
                    let mut traits = named_traits(database, namespace, name, stack)?;
                    if !traits.copy {
                        traits.copy =
                            named_copyable(database, namespace, name, &mut BTreeSet::new())?;
                    }
                    traits
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
            Self::Interface {
                namespace, name, ..
            }
            | Self::Named {
                namespace, name, ..
            } => {
                add(namespace, name);
            }
            _ => {}
        }
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
    ) -> Result<Option<(Self, usize)>, Error> {
        let mut stack = BTreeSet::new();
        Self::constant_underlying_inner(database, file, owner, ty, &mut stack, 0)
    }

    fn constant_underlying_inner(
        database: &Database,
        file: FileId,
        owner: &str,
        ty: &windows_metadata2::Type,
        stack: &mut BTreeSet<Entity<TypeDef>>,
        depth: usize,
    ) -> Result<Option<(Self, usize)>, Error> {
        let (TypeKind::Value(id) | TypeKind::Class(id)) = &ty.kind else {
            return Ok(Some((
                Self::lower(database, file, owner, ty.clone())?,
                depth,
            )));
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
                Self::constant_underlying_inner(
                    database,
                    entity.file(),
                    owner,
                    &underlying,
                    stack,
                    depth + 1,
                )
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
                    depth + 1,
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
        (Type::USize, ConstantValue::USize(value) | ConstantValue::U64(value))
            if *value > u32::MAX as u64 =>
        {
            let value = Literal::u64_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::USize, ConstantValue::I32(value)) => {
            let value = Literal::i32_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::USize, ConstantValue::I64(value)) => {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as usize };
        }
        (Type::ISize, ConstantValue::ISize(value) | ConstantValue::I64(value))
            if !(i32::MIN as i64..=i32::MAX as i64).contains(value) =>
        {
            let value = Literal::i64_suffixed(*value);
            return quote! { #value as isize };
        }
        (Type::ISize, ConstantValue::U32(value)) => {
            let value = Literal::u32_suffixed(*value);
            return quote! { #value as isize };
        }
        (Type::ISize, ConstantValue::U64(value)) => {
            let value = Literal::u64_suffixed(*value);
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
