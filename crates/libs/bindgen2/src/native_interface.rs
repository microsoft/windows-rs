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
                return Err(Error::InvalidValue {
                    name: full_name,
                    message: "native interface has more than one base",
                });
            }
        };
        let mut names = BTreeMap::<String, u32>::new();
        let methods = definition
            .methods()?
            .map(|method| {
                let name = method_name(method)?;
                let count = names.entry(name.clone()).or_default();
                *count += 1;
                let name = if *count == 1 {
                    name
                } else {
                    format!("{name}{count}")
                };
                let signature = native_signature::Signature::lower(database, method, &full_name)?;
                if signature.flags & 0x20 == 0 {
                    return Err(Error::InvalidValue {
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
    pub fn write_sys(&self) -> TokenStream {
        self.write_sys_context(Layout::Flat)
    }

    pub(super) fn write_sys_context(&self, layout: Layout) -> TokenStream {
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
        quote! {
            #iid
            #architectures
            #[repr(C)]
            pub struct #name {
                #base
                #(#methods)*
            }
        }
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
