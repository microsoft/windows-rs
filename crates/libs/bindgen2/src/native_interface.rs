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
    hierarchy_method_dependencies: BTreeSet<(String, String)>,
    com_identity: bool,
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
        dependencies: &native::DependencyCache,
        definition: TypeDefinition<'_>,
        bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        let full_name = format!("{namespace}.{name}");
        let hierarchy =
            collect_interface_bases(database, definition.entity(), bases, &mut BTreeSet::new())?;
        let mut hierarchy_method_dependencies = BTreeSet::new();
        for (namespace, name) in &hierarchy {
            hierarchy_method_dependencies
                .extend(dependencies.interface_method_dependencies(database, namespace, name)?);
        }
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
                let signature =
                    native_signature::Signature::lower(database, dependencies, method, &full_name)?;
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
            com_identity,
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
            hierarchy_method_dependencies,
            methods,
        })
    }

    /// Renders a flat Win32 sys vtable and optional IID.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(
            Layout::Flat,
            Projection::Sys,
            &MemberSelection::All,
            None,
            false,
        )
        .unwrap()
    }

    #[cfg(test)]
    pub fn write_package(&self) -> TokenStream {
        self.write_context(
            Layout::Package,
            Projection::Default,
            &MemberSelection::All,
            None,
            true,
        )
        .unwrap()
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn write_context(
        &self,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
        implementation: Option<bool>,
        base_selected: bool,
    ) -> Result<TokenStream, Error> {
        if !projection.is_sys() {
            return self.write_rich(layout, projection, members, implementation, base_selected);
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
        base_selected: bool,
    ) -> Result<TokenStream, Error> {
        if !layout.is_package()
            && self.guid.is_none()
            && self.methods.iter().any(|method| method.selected(members))
        {
            return Err(Error::UnsupportedType {
                name: format!("{}.{}", self.namespace, self.name),
                shape: "selected rich native interface without COM identity".to_string(),
            });
        }
        let architectures = tokens::architectures(self.architectures);
        let class_features = self.class_features(layout);
        let class_cfg = tokens::feature_cfg_set(&class_features, false);
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let identity = if let Some(guid) = self.guid {
            let guid = guid.write_u128();
            quote! {
                #class_cfg
                #architectures
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
            }
        } else if layout.is_package() {
            quote! {
                #class_cfg
                #architectures
                windows_core::imp::define_interface!(#name, #vtbl_name, 0);
            }
        } else {
            quote! {
                #class_cfg
                #architectures
                windows_core::imp::define_interface!(#name, #vtbl_name);
            }
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
        let base_vtbl = self.base.as_ref().map(|(namespace, base)| {
            if base == "IUnknown" {
                quote! { windows_core::IUnknown_Vtbl }
            } else {
                let path = tokens::namespace(&self.namespace, namespace, layout);
                let base_vtbl = tokens::ident(&format!("{base}_Vtbl"));
                quote! { #path #base_vtbl }
            }
        });
        let base_field = base_vtbl.map(|base_vtbl| quote! { pub base__: #base_vtbl, });
        let hierarchy = self
            .hierarchy
            .iter()
            .map(|(namespace, name)| write_base(namespace, name));
        let hierarchy = (!self.hierarchy.is_empty()).then(|| {
            quote! {
                #class_cfg
                #architectures
                windows_core::imp::interface_hierarchy!(#name, #(#hierarchy),*);
            }
        });
        let deref = self.base.as_ref().and_then(|(namespace, base)| {
            (base != "IUnknown").then(|| {
                let base = write_base(namespace, base);
                quote! {
                    #class_cfg
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
            let features = self.method_features(method, layout, &class_features);
            let yes = tokens::feature_cfg_set(&features, false);
            if features.is_empty() {
                quote! {
                    #architectures
                    pub #name: unsafe extern "system" fn(#parameters) #result,
                }
            } else {
                let no = tokens::feature_cfg_set(&features, true);
                quote! {
                    #yes
                    #architectures
                    pub #name: unsafe extern "system" fn(#parameters) #result,
                    #no
                    #name: usize,
                }
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
                    let method_tokens = method.signature.write_com_method(
                        &self.namespace,
                        layout,
                        projection,
                        &method.name,
                        &self.name,
                    )?;
                    let features = self.method_features(method, layout, &class_features);
                    let cfg = tokens::feature_cfg_set(&features, false);
                    Ok(quote! { #cfg #method_tokens })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            if wrappers.is_empty() {
                quote! {}
            } else {
                quote! {
                    #class_cfg
                    impl #name { #(#wrappers)* }
                }
            }
        };
        let implement = match implementation {
            None if layout.is_package() => self.can_implement_package(members, base_selected),
            None => self.can_implement(members, base_selected),
            Some(false) => false,
            Some(true) if self.supports_implementation(base_selected) => true,
            Some(true) => {
                return Err(Error::InvalidType {
                    name: format!("{}.{}", self.namespace, self.name),
                    message: "requested native interface cannot be implemented",
                });
            }
        };
        let mut runtime_features = class_features.clone();
        runtime_features.extend(self.hierarchy_method_features(layout));
        for method in &self.methods {
            runtime_features.extend(self.method_features(method, layout, &BTreeSet::new()));
        }
        let runtime_cfg = tokens::feature_cfg_set(&runtime_features, false);
        let runtime_name = ((self.guid.is_some() || layout.is_package())
            && implementation != Some(false))
        .then(|| {
            quote! {
                #runtime_cfg
                impl windows_core::RuntimeName for #name {}
            }
        });
        let doc_hidden = layout.is_package().then(|| quote! { #[doc(hidden)] });
        let implementation = if implement {
            self.write_implementation(layout, projection)?
        } else {
            quote! {}
        };
        Ok(quote! {
            #identity
            #deref
            #hierarchy
            #wrappers
            #class_cfg
            #architectures
            #[repr(C)]
            #doc_hidden
            pub struct #vtbl_name {
                #base_field
                #(#methods)*
            }

            #implementation
            #runtime_name
        })
    }

    pub(super) fn can_implement(&self, members: &MemberSelection, base_selected: bool) -> bool {
        self.supports_implementation(base_selected)
            && self.methods.iter().all(|method| method.selected(members))
    }

    pub(super) fn can_implement_package(
        &self,
        members: &MemberSelection,
        base_selected: bool,
    ) -> bool {
        self.methods.iter().all(|method| method.selected(members))
            && self
                .methods
                .iter()
                .all(|method| method.signature.supports_implementation())
            && self
                .base
                .as_ref()
                .is_none_or(|(_, name)| name == "IUnknown" || base_selected)
    }

    fn supports_implementation(&self, base_selected: bool) -> bool {
        self.guid.is_some()
            && self
                .methods
                .iter()
                .all(|method| method.signature.supports_implementation())
            && self
                .base
                .as_ref()
                .is_some_and(|(_, name)| name == "IUnknown" || base_selected)
    }

    pub(super) fn base_name(&self) -> Option<(&str, &str)> {
        self.base
            .as_ref()
            .map(|(namespace, name)| (namespace.as_str(), name.as_str()))
    }

    fn write_implementation(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let mut implementation_features = self.class_features(layout);
        implementation_features.extend(self.hierarchy_method_features(layout));
        for method in &self.methods {
            implementation_features.extend(self.method_features(method, layout, &BTreeSet::new()));
        }
        let implementation_cfg = tokens::feature_cfg_set(&implementation_features, false);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let impl_name = tokens::ident(&format!("{}_Impl", self.name));
        let scoped = !self.com_identity;
        let (base_impl, base_new) = self
            .base
            .as_ref()
            .map(|(namespace, base)| {
                if base == "IUnknown" {
                    (
                        quote! { windows_core::IUnknownImpl },
                        if scoped {
                            quote! { windows_core::IUnknown_Vtbl::new::<Identity, 0>() }
                        } else {
                            quote! { windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
                        },
                    )
                } else {
                    let path = tokens::namespace(&self.namespace, namespace, layout);
                    let base_impl = tokens::ident(&format!("{base}_Impl"));
                    let base_vtbl = tokens::ident(&format!("{base}_Vtbl"));
                    (
                        quote! { #path #base_impl },
                        if scoped {
                            quote! { #path #base_vtbl::new::<Identity>() }
                        } else {
                            quote! { #path #base_vtbl::new::<Identity, OFFSET>() }
                        },
                    )
                }
            })
            .map_or_else(
                || (None, None),
                |(base_impl, base_new)| (Some(base_impl), Some(base_new)),
            );
        let hierarchy_matches = self
            .hierarchy
            .iter()
            .filter(|(_, base)| base != "IUnknown")
            .map(|(namespace, base)| {
                let path = tokens::namespace(&self.namespace, namespace, layout);
                let base = tokens::ident(base);
                quote! { || iid == &<#path #base as windows_core::Interface>::IID }
            });
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
                let result = method.signature.write_vtable_result_projection(
                    &self.namespace,
                    layout,
                    projection,
                );
                let upcall =
                    method
                        .signature
                        .write_impl_upcall(&impl_name, &method.name, layout)?;
                if scoped {
                    Ok(quote! {
                        #architectures
                        unsafe extern "system" fn #method_name<Identity: #impl_name>(
                            #signature
                        ) #result {
                            unsafe {
                                let this =
                                    (this as *mut *mut core::ffi::c_void)
                                        as *const windows_core::ScopedHeap;
                                let this = &*((*this).this as *const Identity);
                                #upcall
                            }
                        }
                    })
                } else {
                    Ok(quote! {
                        #architectures
                        unsafe extern "system" fn #method_name<
                            Identity: #impl_name,
                            const OFFSET: isize
                        >(#signature) #result {
                            unsafe {
                                let this: &Identity =
                                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                #upcall
                            }
                        }
                    })
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let initializers = self.methods.iter().map(|method| {
            let architectures = tokens::architectures(method.architectures);
            let method_name = tokens::ident(&method.name);
            if scoped {
                quote! {
                    #architectures
                    #method_name: #method_name::<Identity>,
                }
            } else {
                quote! {
                    #architectures
                    #method_name: #method_name::<Identity, OFFSET>,
                }
            }
        });
        let trait_definition = base_impl.map_or_else(
            || quote! { pub trait #impl_name { #(#trait_methods)* } },
            |base_impl| quote! { pub trait #impl_name: #base_impl { #(#trait_methods)* } },
        );
        if scoped {
            let impl_vtbl = tokens::ident(&format!("{}_ImplVtbl", self.name));
            let base_initializer = base_new.map(|base_new| quote! { base__: #base_new, });
            return Ok(quote! {
                #implementation_cfg
                #trait_definition
                #implementation_cfg
                impl #vtbl_name {
                    pub const fn new<Identity: #impl_name>() -> Self {
                        #(#functions)*
                        Self {
                            #base_initializer
                            #(#initializers)*
                        }
                    }
                }
                #implementation_cfg
                struct #impl_vtbl<T: #impl_name>(core::marker::PhantomData<T>);
                #implementation_cfg
                impl<T: #impl_name> #impl_vtbl<T> {
                    const VTABLE: #vtbl_name = #vtbl_name::new::<T>();
                }
                #implementation_cfg
                impl #name {
                    pub fn new<'a, T: #impl_name>(
                        this: &'a T
                    ) -> windows_core::ScopedInterface<'a, Self> {
                        let this = windows_core::ScopedHeap {
                            vtable: &#impl_vtbl::<T>::VTABLE as *const _ as *const _,
                            this: this as *const _ as *const _,
                        };
                        let this =
                            core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
                        unsafe {
                            windows_core::ScopedInterface::new(
                                core::mem::transmute(&this.vtable)
                            )
                        }
                    }
                }
            });
        }
        let base_new = base_new.unwrap();
        Ok(quote! {
            #implementation_cfg
            #trait_definition
            #implementation_cfg
            impl #vtbl_name {
                pub const fn new<Identity: #impl_name, const OFFSET: isize>() -> Self {
                    #(#functions)*
                    Self {
                        base__: #base_new,
                        #(#initializers)*
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<#name as windows_core::Interface>::IID #(#hierarchy_matches)*
                }
            }
        })
    }

    fn class_features(&self, layout: Layout) -> BTreeSet<String> {
        tokens::feature_names(
            &self.namespace,
            layout,
            self.hierarchy
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    fn hierarchy_method_features(&self, layout: Layout) -> BTreeSet<String> {
        tokens::feature_names(
            &self.namespace,
            layout,
            self.hierarchy_method_dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    pub(super) fn package_features(&self, layout: Layout) -> BTreeSet<String> {
        let mut features = self.class_features(layout);
        for method in &self.methods {
            features.extend(tokens::feature_names(
                &self.namespace,
                layout,
                method
                    .signature
                    .manifest_dependencies()
                    .iter()
                    .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
            ));
        }
        features
    }

    fn method_features(
        &self,
        method: &Method,
        layout: Layout,
        parent: &BTreeSet<String>,
    ) -> BTreeSet<String> {
        let mut features = tokens::feature_names(
            &self.namespace,
            layout,
            method
                .signature
                .package_dependencies()
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        features.retain(|feature| !parent.contains(feature));
        features
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
