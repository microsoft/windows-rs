use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Interface {
    name: String,
    namespace: String,
    generics: Vec<String>,
    guid: guid::Guid,
    exclusive: bool,
    methods: Vec<NamedMethod>,
    required: Vec<RequiredInterface>,
}

pub(super) struct NamedMethod {
    pub(super) name: String,
    pub(super) method: winrt_delegate::Method,
}

struct RequiredInterface {
    namespace: String,
    name: String,
    arguments: Vec<ty::Type>,
    methods: Vec<NamedMethod>,
}

impl Interface {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        interface_bases: &BTreeMap<Entity<TypeDef>, Vec<InterfaceBase>>,
        owner: &str,
    ) -> Result<Self, Error> {
        let name = trim_generic_arity(definition.name()?).to_string();
        let namespace = definition.namespace()?.to_string();
        let generics = definition
            .generic_parameters()?
            .map(|parameter| Ok(parameter.name()?.to_string()))
            .collect::<Result<Vec<_>, Error>>()?;
        let methods = lower_methods(database, definition, owner)?;
        let root_arguments = (0..generics.len())
            .map(|index| ty::Type::Generic(index as u32))
            .collect::<Vec<_>>();
        let mut required = Vec::new();
        let mut seen = BTreeSet::new();
        Self::lower_required(
            database,
            interface_bases,
            definition.entity(),
            &root_arguments,
            owner,
            &mut seen,
            &mut required,
        )?;
        required.sort_by(|left, right| {
            (&left.namespace, &left.name, &left.arguments).cmp(&(
                &right.namespace,
                &right.name,
                &right.arguments,
            ))
        });
        let guid =
            guid::Guid::from_definition(definition, owner)?.ok_or_else(|| Error::InvalidType {
                name: owner.to_string(),
                message: "interface has no GUID",
            })?;
        let exclusive = definition.has_attribute("ExclusiveToAttribute")?;
        Ok(Self {
            name,
            namespace,
            generics,
            guid,
            exclusive,
            methods,
            required,
        })
    }

    fn lower_required(
        database: &Database,
        interface_bases: &BTreeMap<Entity<TypeDef>, Vec<InterfaceBase>>,
        entity: Entity<TypeDef>,
        owner_arguments: &[ty::Type],
        owner: &str,
        seen: &mut BTreeSet<Entity<TypeDef>>,
        result: &mut Vec<RequiredInterface>,
    ) -> Result<(), Error> {
        let Some(bases) = interface_bases.get(&entity) else {
            return Ok(());
        };
        for base in bases {
            let arguments = base
                .arguments
                .iter()
                .cloned()
                .map(|argument| ty::Type::lower(database, base.file, owner, argument))
                .collect::<Result<Vec<_>, Error>>()?
                .into_iter()
                .map(|argument| argument.substitute(owner_arguments))
                .collect::<Vec<_>>();
            Self::lower_required(
                database,
                interface_bases,
                base.entity,
                &arguments,
                owner,
                seen,
                result,
            )?;
            if !seen.insert(base.entity) {
                continue;
            }
            let definition = database.definition(base.entity).unwrap();
            let namespace = definition.namespace()?.to_string();
            let metadata_name = definition.name()?;
            let mut methods = lower_methods(
                database,
                definition,
                &format!("{}.{}", definition.namespace()?, metadata_name),
            )?;
            for method in &mut methods {
                method.method.substitute(&arguments);
            }
            result.push(RequiredInterface {
                namespace,
                name: trim_generic_arity(metadata_name).to_string(),
                arguments,
                methods,
            });
        }
        Ok(())
    }

    pub(super) fn dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        for method in &self.methods {
            dependencies.extend(method.method.dependencies());
        }
        dependencies.extend(
            self.required
                .iter()
                .map(|required| (required.namespace.clone(), required.name.clone())),
        );
        for required in &self.required {
            for argument in &required.arguments {
                argument.collect_value_dependencies(&mut dependencies);
            }
        }
        dependencies
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let impl_name = tokens::ident(&format!("{}_Impl", self.name));
        let generic_names = self
            .generics
            .iter()
            .map(|name| tokens::ident(name))
            .collect::<Vec<_>>();
        let constraints = generic_names
            .iter()
            .map(|name| quote! { #name: windows_core::RuntimeType + 'static })
            .collect::<Vec<_>>();
        let type_arguments = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_names),*> }
        };
        let constrained_generics = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { <#(#constraints),*> }
        };
        let generic_where = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints),* }
        };
        let guid = self.guid.write_u128();
        let full_name = format!("{}.{}", self.namespace, self.name);
        let definition = if generic_names.is_empty() {
            let metadata_name = Literal::byte_string(full_name.as_bytes());
            quote! {
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(#metadata_name);
                }
            }
        } else {
            let signature =
                Literal::byte_string(format!("pinterface({{{}}}", self.guid).as_bytes());
            let metadata_name =
                Literal::byte_string(format!("{}`{}<", full_name, generic_names.len()).as_bytes());
            let signatures = generic_names
                .iter()
                .map(|name| quote! { .push_slice(b";").push_other(#name::SIGNATURE) });
            let names = generic_names
                .iter()
                .map(|name| quote! { .push_other(#name::NAME) });
            quote! {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name #type_arguments(
                    windows_core::IUnknown,
                    core::marker::PhantomData<#(#generic_names),*>
                ) #generic_where;
                impl #constrained_generics windows_core::imp::CanInto<windows_core::IUnknown>
                    for #name #type_arguments {}
                impl #constrained_generics windows_core::imp::CanInto<windows_core::IInspectable>
                    for #name #type_arguments {}
                unsafe impl #constrained_generics windows_core::Interface
                    for #name #type_arguments
                {
                    type Vtable = #vtbl_name #type_arguments;
                    const IID: windows_core::GUID =
                        windows_core::GUID::from_signature(
                            <Self as windows_core::RuntimeType>::SIGNATURE
                        );
                }
                impl #constrained_generics windows_core::RuntimeType for #name #type_arguments {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::new()
                            .push_slice(#signature)
                            #(#signatures)*
                            .push_slice(b")");
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::new()
                            .push_slice(#metadata_name)
                            #(#names)*
                            .push_slice(b">");
                }
            }
        };
        if self.exclusive {
            let vtable = self.write_vtable_struct(values, namespace, layout, &vtbl_name)?;
            return Ok(quote! {
                #definition
                #vtable
            });
        }
        let hierarchy = generic_names.is_empty().then(|| {
            quote! {
                windows_core::imp::interface_hierarchy!(
                    #name,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
            }
        });
        let required_types = self
            .required
            .iter()
            .map(|required| required.write_name(namespace, layout, &self.generics))
            .collect::<Result<Vec<_>, Error>>()?;
        let required_hierarchy = (!required_types.is_empty()).then(|| {
            quote! { windows_core::imp::required_hierarchy!(#name #type_arguments, #(#required_types),*); }
        });
        let method_context =
            winrt_delegate::MethodContext::new(values, namespace, layout, &self.generics);
        let methods = self
            .methods
            .iter()
            .map(|method| {
                method
                    .method
                    .write_public_method(&method_context, &method.name, quote! { self })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let mut inherited_methods = Vec::new();
        let mut public_names =
            self.methods
                .iter()
                .fold(BTreeMap::<String, u32>::new(), |mut names, method| {
                    *names.entry(method.name.clone()).or_default() += 1;
                    names
                });
        for required in &self.required {
            let receiver = required.write_name(namespace, layout, &self.generics)?;
            for method in &required.methods {
                let count = public_names.entry(method.name.clone()).or_default();
                *count += 1;
                let public_name = if *count == 1 {
                    method.name.clone()
                } else {
                    format!("{}{}", method.name, count)
                };
                inherited_methods.push(method.method.write_forwarded_public_method(
                    &method_context,
                    &public_name,
                    &method.name,
                    receiver.clone(),
                )?);
            }
        }
        let runtime_name = Literal::string(&full_name);
        let runtime_class_name = (!generic_names.is_empty()).then(|| {
            quote! {
                const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                    <Self as windows_core::RuntimeType>::NAME;
            }
        });
        let trait_bases = if required_types.is_empty() {
            quote! { windows_core::IUnknownImpl }
        } else {
            let bases = self
                .required
                .iter()
                .map(|required| required.write_impl_name(namespace, layout, &self.generics))
                .collect::<Result<Vec<_>, Error>>()?;
            quote! { #(#bases)+* }
        };
        let impl_methods = self
            .methods
            .iter()
            .map(|method| {
                method.method.write_impl_method(
                    values,
                    namespace,
                    layout,
                    &self.generics,
                    &method.name,
                )
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let vtable = self.write_vtable(values, namespace, layout, &name, &vtbl_name, &impl_name)?;
        let methods_impl = (!methods.is_empty() || !inherited_methods.is_empty()).then(|| {
            quote! {
                impl #constrained_generics #name #type_arguments {
                    #(#methods)*
                    #(#inherited_methods)*
                }
            }
        });

        Ok(quote! {
            #definition
            #hierarchy
            #required_hierarchy
            #methods_impl
            impl #constrained_generics windows_core::RuntimeName for #name #type_arguments {
                const NAME: &'static str = #runtime_name;
                #runtime_class_name
            }
            pub trait #impl_name #type_arguments: #trait_bases #generic_where {
                #(#impl_methods)*
            }
            #vtable
        })
    }

    fn write_vtable(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        name: &TokenStream,
        vtbl_name: &TokenStream,
        impl_name: &TokenStream,
    ) -> Result<TokenStream, Error> {
        let generic_names = self
            .generics
            .iter()
            .map(|name| tokens::ident(name))
            .collect::<Vec<_>>();
        let constraints = generic_names
            .iter()
            .map(|name| quote! { #name: windows_core::RuntimeType + 'static })
            .collect::<Vec<_>>();
        let type_arguments = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_names),*> }
        };
        let constrained_generics = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { <#(#constraints),*> }
        };
        let functions = self
            .methods
            .iter()
            .map(|method| {
                let method_name = tokens::ident(&method.name);
                let signature =
                    method
                        .method
                        .write_abi_signature(values, namespace, layout, &self.generics)?;
                let upcall = method.method.write_upcall(
                    values,
                    quote! { #impl_name::#method_name },
                    true,
                )?;
                Ok(quote! {
                    unsafe extern "system" fn #method_name<
                        #(#constraints,)*
                        Identity: #impl_name #type_arguments,
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
            let name = tokens::ident(&method.name);
            quote! { #name: #name::<#(#generic_names,)* Identity, OFFSET>, }
        });
        let phantom_values = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData::<#name>, });
        let vtable = self.write_vtable_struct(values, namespace, layout, vtbl_name)?;
        Ok(quote! {
            impl #constrained_generics #vtbl_name #type_arguments {
                pub const fn new<
                    Identity: #impl_name #type_arguments,
                    const OFFSET: isize
                >() -> Self {
                    #(#functions)*
                    Self {
                        base__: windows_core::IInspectable_Vtbl::new::<
                            Identity,
                            #name #type_arguments,
                            OFFSET
                        >(),
                        #(#initializers)*
                        #(#phantom_values)*
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<#name #type_arguments as windows_core::Interface>::IID
                }
            }
            #vtable
        })
    }

    fn write_vtable_struct(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        vtbl_name: &TokenStream,
    ) -> Result<TokenStream, Error> {
        let generic_names = self
            .generics
            .iter()
            .map(|name| tokens::ident(name))
            .collect::<Vec<_>>();
        let constraints = generic_names
            .iter()
            .map(|name| quote! { #name: windows_core::RuntimeType + 'static })
            .collect::<Vec<_>>();
        let type_arguments = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_names),*> }
        };
        let generic_where = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints),* }
        };
        let fields = self
            .methods
            .iter()
            .map(|method| {
                let name = tokens::ident(&method.name);
                let signature =
                    method
                        .method
                        .write_abi_signature(values, namespace, layout, &self.generics)?;
                Ok(quote! {
                    pub #name: unsafe extern "system" fn(#signature) -> windows_core::HRESULT,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let phantom_fields = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData<#name>, });
        Ok(quote! {
            #[repr(C)]
            pub struct #vtbl_name #type_arguments #generic_where {
                pub base__: windows_core::IInspectable_Vtbl,
                #(#fields)*
                #(#phantom_fields)*
            }
        })
    }
}

impl RequiredInterface {
    fn write_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&self.name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, generics))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }

    fn write_impl_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&format!("{}_Impl", self.name));
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, generics))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }
}

pub(super) fn lower_methods(
    database: &Database,
    definition: TypeDefinition<'_>,
    owner: &str,
) -> Result<Vec<NamedMethod>, Error> {
    let mut names = BTreeMap::<String, u32>::new();
    definition
        .methods()?
        .map(|method| {
            let metadata_name = method.name()?;
            let mut name = if method.flags()? & 0x800 != 0 {
                if let Some(name) = metadata_name.strip_prefix("get_") {
                    name.to_string()
                } else if let Some(name) = metadata_name.strip_prefix("put_") {
                    format!("Set{name}")
                } else if let Some(name) = metadata_name.strip_prefix("add_") {
                    name.to_string()
                } else if let Some(name) = metadata_name.strip_prefix("remove_") {
                    format!("Remove{name}")
                } else {
                    metadata_name.to_string()
                }
            } else if let Some(attribute) = method.find_attribute("OverloadAttribute")? {
                let arguments = attribute.arguments(&())?;
                let overload = arguments.iter().find_map(|argument| match argument {
                    AttributeArgument::Fixed {
                        value: AttributeValue::String(value),
                        ..
                    } => Some(value.as_str()),
                    _ => None,
                });
                match overload {
                    Some(overload)
                        if overload
                            .strip_prefix(metadata_name)
                            .is_some_and(|suffix| suffix.parse::<u32>().is_ok()) =>
                    {
                        metadata_name.to_string()
                    }
                    Some(overload) => overload.to_string(),
                    None => metadata_name.to_string(),
                }
            } else {
                metadata_name.to_string()
            };
            let count = names.entry(name.clone()).or_default();
            *count += 1;
            if *count > 1 {
                name.push_str(&count.to_string());
            }
            Ok(NamedMethod {
                name,
                method: winrt_delegate::Method::lower(
                    database,
                    definition.entity().file(),
                    method,
                    owner,
                    false,
                )?,
            })
        })
        .collect()
}
