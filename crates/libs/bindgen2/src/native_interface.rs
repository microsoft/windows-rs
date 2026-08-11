use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

/// An owned Win32 native interface projection.
pub struct NativeInterface {
    architectures: i32,
    namespace: String,
    name: String,
    base: Option<(String, String)>,
    hierarchy: Vec<(String, String)>,
    guid: Option<guid::Guid>,
    methods: Vec<Method>,
}

struct Method {
    architectures: i32,
    metadata_name: String,
    name: String,
    signature: native_signature::Signature,
}

impl NativeInterface {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        let full_name = format!("{namespace}.{name}");
        let hierarchy =
            collect_interface_bases(database, definition.entity(), bases, &mut BTreeSet::new())?;
        let base = hierarchy.last().cloned();
        let own_guid = if name == "IUnknown" {
            guid::Guid::from_definition(definition, &full_name)?
        } else {
            None
        };
        let com_identity = if own_guid.is_some_and(guid::Guid::is_iunknown) {
            true
        } else if let Some((namespace, name)) = hierarchy.first() {
            is_iunknown(database, namespace, name)?
        } else {
            false
        };
        let mut names = BTreeMap::<String, u32>::new();
        let methods = definition
            .methods()?
            .map(|method| {
                let metadata_name = method.name()?.to_string();
                let projected_name = method_name(method)?;
                let count = names.entry(projected_name.clone()).or_default();
                *count += 1;
                let name = if *count == 1 {
                    projected_name
                } else {
                    format!("{projected_name}{count}")
                };
                let signature = native_signature::Signature::lower(database, method, &full_name)?;
                if signature.flags & 0x20 == 0 {
                    return Err(Error::InvalidType {
                        name: full_name.clone(),
                        message: "native interface method has no instance receiver",
                    });
                }
                if signature.flags & 0x0f == 0x05 {
                    return Err(Error::UnsupportedType {
                        name: full_name.clone(),
                        shape: "variadic native interface method".to_string(),
                    });
                }
                Ok(Method {
                    architectures: method.architectures()?,
                    metadata_name,
                    name,
                    signature,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            architectures: definition.architectures()?,
            namespace,
            name,
            base,
            guid: if com_identity {
                if own_guid.is_some() {
                    own_guid
                } else {
                    guid::Guid::from_definition(definition, &full_name)?
                }
            } else {
                None
            },
            hierarchy,
            methods,
        })
    }

    /// Renders a flat Win32 sys vtable and optional IID.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(Layout::Flat, Projection::Sys, &MemberSelection::All, None)
            .unwrap()
    }

    pub(super) fn write_context(
        &self,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
        implementation: Option<bool>,
    ) -> Result<TokenStream, Error> {
        if !projection.is_sys() {
            return self.write_rich(layout, projection, members, implementation);
        }
        let architectures = tokens::architectures(self.architectures);
        let name = tokens::ident(&format!("{}_Vtbl", self.name));
        let iid = self.guid.map(|guid| {
            let name = tokens::ident(&format!("IID_{}", self.name));
            let guid = guid.write_value();
            quote! {
                #architectures
                pub const #name: GUID = #guid;
            }
        });
        let base = self.base.as_ref().map(|(namespace, name)| {
            let path = tokens::namespace(&self.namespace, namespace, layout);
            let name = tokens::ident(&format!("{name}_Vtbl"));
            quote! { pub base__: #path #name, }
        });
        let methods = self.methods.iter().map(|method| {
            let architectures = tokens::architectures(method.architectures);
            let name = tokens::ident(&method.name);
            let parameters =
                if self.namespace == "Windows.Win32.System.Com" && self.name == "IUnknown" {
                    match method.name.as_str() {
                        "QueryInterface" => quote! {
                            this: *mut core::ffi::c_void,
                            iid: *const GUID,
                            interface: *mut *mut core::ffi::c_void
                        },
                        "AddRef" | "Release" => quote! { this: *mut core::ffi::c_void },
                        _ => unreachable!(),
                    }
                } else {
                    method.signature.write_vtable_parameters_projection(
                        &self.namespace,
                        layout,
                        Projection::Sys,
                    )
                };
            let result = method.signature.write_result(&self.namespace, layout);
            quote! {
                #architectures
                pub #name: unsafe extern "system" fn(#parameters) #result,
            }
        });
        Ok(quote! {
            #iid
            #architectures
            #[repr(C)]
            pub struct #name {
                #base
                #(#methods)*
            }
        })
    }

    fn write_rich(
        &self,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
        implementation: Option<bool>,
    ) -> Result<TokenStream, Error> {
        let Some(guid) = self.guid else {
            return Err(Error::UnsupportedType {
                name: format!("{}.{}", self.namespace, self.name),
                shape: "rich native interface without COM identity".to_string(),
            });
        };
        let architectures = tokens::architectures(self.architectures);
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let guid = guid.write_u128();
        let identity = quote! {
            #architectures
            windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
        };
        let write_base = |namespace: &str, name: &str| {
            if let Some(core) = native::core_projection(namespace, name) {
                core
            } else {
                let path = tokens::namespace(&self.namespace, namespace, layout);
                let name = tokens::ident(name);
                quote! { #path #name }
            }
        };
        let base_vtbl = self.base.as_ref().map_or_else(
            || quote! { windows_core::IUnknown_Vtbl },
            |(namespace, base)| {
                if base == "IUnknown" {
                    quote! { windows_core::IUnknown_Vtbl }
                } else {
                    let path = tokens::namespace(&self.namespace, namespace, layout);
                    let base_vtbl = tokens::ident(&format!("{base}_Vtbl"));
                    quote! { #path #base_vtbl }
                }
            },
        );
        let hierarchy = self
            .hierarchy
            .iter()
            .map(|(namespace, name)| write_base(namespace, name));
        let deref = self.base.as_ref().and_then(|(namespace, base)| {
            (base != "IUnknown").then(|| {
                let base = write_base(namespace, base);
                quote! {
                    #architectures
                    impl core::ops::Deref for #name {
                        type Target = #base;
                        fn deref(&self) -> &Self::Target {
                            unsafe { core::mem::transmute(self) }
                        }
                    }
                }
            })
        });
        let methods = self.methods.iter().map(|method| {
            let name = tokens::ident(&method.name);
            if !method.selected(members) && implementation != Some(true) {
                return quote! { #name: usize, };
            }
            let architectures = tokens::architectures(method.architectures);
            let parameters = method.signature.write_vtable_parameters_projection(
                &self.namespace,
                layout,
                projection,
            );
            let result = method.signature.write_vtable_result_projection(
                &self.namespace,
                layout,
                projection,
            );
            quote! {
                #architectures
                pub #name: unsafe extern "system" fn(#parameters) #result,
            }
        });
        let wrappers = if projection.is_minimal() && implementation == Some(true) {
            quote! {}
        } else {
            let wrappers = self
                .methods
                .iter()
                .filter(|method| method.selected(members))
                .map(|method| {
                    method
                        .signature
                        .write_com_method(&self.namespace, layout, &method.name)
                })
                .collect::<Result<Vec<_>, Error>>()?;
            if wrappers.is_empty() {
                quote! {}
            } else {
                quote! { impl #name { #(#wrappers)* } }
            }
        };
        let implement = match implementation {
            None => self.can_implement(members),
            Some(false) => false,
            Some(true) if self.supports_implementation() => true,
            Some(true) => {
                return Err(Error::InvalidType {
                    name: format!("{}.{}", self.namespace, self.name),
                    message: "requested native interface cannot be implemented",
                });
            }
        };
        let runtime_name = (implementation != Some(false)).then(|| {
            quote! { impl windows_core::RuntimeName for #name {} }
        });
        let implementation = if implement {
            self.write_implementation(layout, projection)?
        } else {
            quote! {}
        };
        Ok(quote! {
            #identity
            #deref
            #architectures
            windows_core::imp::interface_hierarchy!(#name, #(#hierarchy),*);
            #wrappers
            #architectures
            #[repr(C)]
            pub struct #vtbl_name {
                pub base__: #base_vtbl,
                #(#methods)*
            }

            #implementation
            #runtime_name
        })
    }

    fn can_implement(&self, members: &MemberSelection) -> bool {
        self.supports_implementation() && self.methods.iter().all(|method| method.selected(members))
    }

    fn supports_implementation(&self) -> bool {
        self.guid.is_some()
            && self
                .base
                .as_ref()
                .is_some_and(|(_, name)| name == "IUnknown")
    }

    fn write_implementation(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let impl_name = tokens::ident(&format!("{}_Impl", self.name));
        let trait_methods = self
            .methods
            .iter()
            .map(|method| {
                let method_tokens = method.signature.write_impl_method(
                    &self.namespace,
                    layout,
                    projection,
                    &method.name,
                )?;
                let architectures = tokens::architectures(method.architectures);
                Ok(quote! { #architectures #method_tokens })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let functions = self
            .methods
            .iter()
            .map(|method| {
                let architectures = tokens::architectures(method.architectures);
                let method_name = tokens::ident(&method.name);
                let signature = method.signature.write_vtable_parameters_named(
                    &self.namespace,
                    layout,
                    projection,
                );
                let upcall = method
                    .signature
                    .write_impl_upcall(&impl_name, &method.name)?;
                Ok(quote! {
                    #architectures
                    unsafe extern "system" fn #method_name<
                        Identity: #impl_name,
                        const OFFSET: isize
                    >(#signature) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            #upcall
                        }
                    }
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let initializers = self.methods.iter().map(|method| {
            let architectures = tokens::architectures(method.architectures);
            let method_name = tokens::ident(&method.name);
            quote! {
                #architectures
                #method_name: #method_name::<Identity, OFFSET>,
            }
        });
        Ok(quote! {
            pub trait #impl_name: windows_core::IUnknownImpl {
                #(#trait_methods)*
            }
            impl #vtbl_name {
                pub const fn new<Identity: #impl_name, const OFFSET: isize>() -> Self {
                    #(#functions)*
                    Self {
                        base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                        #(#initializers)*
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<#name as windows_core::Interface>::IID
                }
            }
        })
    }
}

fn collect_interface_bases(
    database: &Database,
    entity: Entity<TypeDef>,
    bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<Vec<(String, String)>, Error> {
    if !stack.insert(entity) {
        return Err(Error::RecursiveInterface(
            database.definition(entity).unwrap().name()?.to_string(),
        ));
    }
    let result = match bases.get(&entity).map(Vec::as_slice) {
        None | Some([]) => Vec::new(),
        Some([(namespace, name)]) => {
            let mut hierarchy = Vec::new();
            let mut resolved = None;
            for base in database.type_definitions(namespace, name) {
                let candidate = collect_interface_bases(database, *base, bases, stack)?;
                if let Some(previous) = &resolved {
                    if previous != &candidate {
                        return Err(Error::InvalidType {
                            name: format!("{namespace}.{name}"),
                            message: "native interface definitions have conflicting bases",
                        });
                    }
                } else {
                    resolved = Some(candidate);
                }
            }
            if let Some(ancestors) = resolved {
                hierarchy = ancestors;
            }
            hierarchy.push((namespace.clone(), name.clone()));
            hierarchy
        }
        Some(_) => {
            return Err(Error::InvalidType {
                name: database.definition(entity).unwrap().name()?.to_string(),
                message: "native interface has more than one base",
            });
        }
    };
    stack.remove(&entity);
    Ok(result)
}

impl Method {
    fn selected(&self, members: &MemberSelection) -> bool {
        members.includes(&self.metadata_name, &self.name)
    }
}

fn is_iunknown(database: &Database, namespace: &str, name: &str) -> Result<bool, Error> {
    if name != "IUnknown" {
        return Ok(false);
    }
    for entity in database.type_definitions(namespace, name) {
        let definition = database.definition(*entity).unwrap();
        if guid::Guid::from_definition(definition, name)?.is_some_and(guid::Guid::is_iunknown) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn method_name(method: windows_metadata2::MethodDefinition<'_>) -> Result<String, Error> {
    let name = method.name()?;
    if method.flags()? & 0x0800 != 0 {
        return Ok(if let Some(name) = name.strip_prefix("get_") {
            name.to_string()
        } else if let Some(name) = name.strip_prefix("put_") {
            format!("Set{name}")
        } else if let Some(name) = name.strip_prefix("add_") {
            name.to_string()
        } else if let Some(name) = name.strip_prefix("remove_") {
            format!("Remove{name}")
        } else {
            name.to_string()
        });
    }
    if let Some(attribute) = method.find_attribute("OverloadAttribute")? {
        let arguments = attribute.arguments(&())?;
        if let Some(AttributeArgument::Fixed {
            value: AttributeValue::String(overload),
            ..
        }) = arguments.first()
        {
            if let Some(suffix) = overload.strip_prefix(name)
                && suffix.parse::<u32>().is_ok()
            {
                return Ok(name.to_string());
            }
            return Ok(overload.clone());
        }
    }
    Ok(name.to_string())
}
