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
        let base = match bases.get(&definition.entity()).map(Vec::as_slice) {
            None | Some([]) => None,
            Some([base]) => Some(base.clone()),
            Some(_) => {
                return Err(Error::InvalidType {
                    name: full_name,
                    message: "native interface has more than one base",
                });
            }
        };
        let mut names = BTreeMap::<String, u32>::new();
        let methods = definition
            .methods()?
            .map(|method| {
                let metadata_name = method_name(method)?;
                let count = names.entry(metadata_name.clone()).or_default();
                *count += 1;
                let name = if *count == 1 {
                    metadata_name.clone()
                } else {
                    format!("{metadata_name}{count}")
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
            guid: if has_iunknown_base(database, definition.entity(), bases, &mut BTreeSet::new())?
            {
                guid::Guid::from_definition(definition, &full_name)?
            } else {
                None
            },
            methods,
        })
    }

    /// Renders a flat Win32 sys vtable and optional IID.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(Layout::Flat, Projection::Sys, &MemberSelection::All)
            .unwrap()
    }

    pub(super) fn write_context(
        &self,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
    ) -> Result<TokenStream, Error> {
        if !projection.is_sys() {
            return self.write_rich(layout, projection, members);
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
            let parameters = method
                .signature
                .write_vtable_parameters(&self.namespace, layout);
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
        let (base, base_vtbl) = self.base.as_ref().map_or_else(
            || {
                (
                    quote! { windows_core::IUnknown },
                    quote! { windows_core::IUnknown_Vtbl },
                )
            },
            |(namespace, base)| {
                if base == "IUnknown" {
                    (
                        quote! { windows_core::IUnknown },
                        quote! { windows_core::IUnknown_Vtbl },
                    )
                } else {
                    let path = tokens::namespace(&self.namespace, namespace, layout);
                    let base = tokens::ident(base);
                    let base_vtbl = tokens::ident(&format!("{base}_Vtbl"));
                    (quote! { #path #base }, quote! { #path #base_vtbl })
                }
            },
        );
        let methods = self.methods.iter().map(|method| {
            let name = tokens::ident(&method.name);
            if !method.selected(members) {
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
        let wrappers = if wrappers.is_empty() {
            quote! {}
        } else {
            quote! { impl #name { #(#wrappers)* } }
        };
        let implementation = if self.can_implement(members) {
            self.write_implementation(layout, projection)?
        } else {
            quote! {}
        };
        Ok(quote! {
            #identity
            #architectures
            windows_core::imp::interface_hierarchy!(#name, #base);
            #wrappers
            #architectures
            #[repr(C)]
            pub struct #vtbl_name {
                pub base__: #base_vtbl,
                #(#methods)*
            }

            #implementation
            impl windows_core::RuntimeName for #name {}
        })
    }

    fn can_implement(&self, members: &MemberSelection) -> bool {
        self.guid.is_some()
            && self
                .base
                .as_ref()
                .is_some_and(|(_, name)| name == "IUnknown")
            && self.methods.iter().all(|method| method.selected(members))
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

impl Method {
    fn selected(&self, members: &MemberSelection) -> bool {
        members.includes(&self.metadata_name, &self.name)
    }
}

fn has_iunknown_base(
    database: &Database,
    entity: Entity<TypeDef>,
    bases: &BTreeMap<Entity<TypeDef>, Vec<(String, String)>>,
    stack: &mut BTreeSet<Entity<TypeDef>>,
) -> Result<bool, Error> {
    let definition = database.definition(entity).unwrap();
    if definition.name()? == "IUnknown"
        && guid::Guid::from_definition(definition, definition.name()?)?
            .is_some_and(guid::Guid::is_iunknown)
    {
        return Ok(true);
    }
    if !stack.insert(entity) {
        return Err(Error::RecursiveInterface(
            database.definition(entity).unwrap().name()?.to_string(),
        ));
    }
    let mut result = false;
    if let Some(names) = bases.get(&entity) {
        for (namespace, name) in names {
            for base in database.type_definitions(namespace, name) {
                if has_iunknown_base(database, *base, bases, stack)? {
                    result = true;
                    break;
                }
            }
            if result {
                break;
            }
        }
    }
    stack.remove(&entity);
    Ok(result)
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
