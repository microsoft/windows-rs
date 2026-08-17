use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

impl Type {
    pub(crate) fn write_projection(
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
                    if projection.is_sys() {
                        return tokens::ident(name);
                    }
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

    pub(crate) fn write_abi_projection(
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

    pub(crate) fn write_field_projection(
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

    pub(crate) fn write_field_projection_owner(
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

    pub(crate) fn write_constant_projection(
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

    pub(crate) fn mutable_string_pointer(&self) -> bool {
        self.canonical()
            .is_some_and(canonical::Type::is_mutable_string)
    }

    pub(crate) fn write_public(&self, namespace: &str, layout: Layout) -> TokenStream {
        self.write_public_with_owner(namespace, layout, None)
    }

    pub(crate) fn write_param(&self, namespace: &str, layout: Layout, owner: &str) -> TokenStream {
        if layout.is_package() {
            self.write_public_with_owner(namespace, layout, Some(owner))
        } else {
            self.write_public(namespace, layout)
        }
    }

    pub(crate) fn write_public_with_owner(
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

    pub(crate) fn write_public_pointer(&self, namespace: &str, layout: Layout) -> TokenStream {
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

    pub(crate) fn pointee(&self) -> Option<&Self> {
        match self {
            Self::Pointer { element, .. } => Some(element),
            _ => None,
        }
    }

    pub(crate) fn is_interface(&self) -> bool {
        matches!(self, Self::Interface { .. })
    }

    pub(crate) fn interface_out(&self) -> Option<(bool, &Self)> {
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

    pub(crate) fn interface_pointer_depth(&self) -> Option<usize> {
        let mut depth = 0;
        let mut ty = self;
        while let Self::Pointer { element, .. } = ty {
            depth += 1;
            ty = element;
        }
        ty.is_interface().then_some(depth)
    }

    pub(crate) fn write_interface_pointer(
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

    pub(crate) fn is_direct_interface_pointer(&self) -> bool {
        self.pointee().is_some_and(Self::is_interface)
    }
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
