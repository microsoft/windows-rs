use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Class {
    name: String,
    namespace: String,
    default_interface: Option<ClassInterface>,
    interfaces: Vec<ClassInterface>,
    bases: Vec<ClassName>,
    default_constructor: bool,
    agile: bool,
    async_default: bool,
}

struct ClassInterface {
    entity: Entity<TypeDef>,
    namespace: String,
    name: String,
    arguments: Vec<ty::Type>,
    methods: Vec<winrt_interface::NamedMethod>,
    default: bool,
    exclusive: bool,
    factory: bool,
}

struct ClassName {
    namespace: String,
    name: String,
}

impl Class {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        relationships: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
        owner: &str,
    ) -> Result<Self, Error> {
        let name = definition.name()?.to_string();
        let namespace = definition.namespace()?.to_string();
        let mut interfaces = Vec::new();
        let mut seen = BTreeSet::new();
        lower_interfaces(
            database,
            relationships,
            definition.entity(),
            false,
            owner,
            &mut seen,
            &mut interfaces,
        )?;
        lower_factories(database, definition, owner, &mut seen, &mut interfaces)?;
        let bases = lower_bases(database, definition, relationships, owner, &mut interfaces)?;
        interfaces.sort_by(|left, right| {
            (&left.namespace, &left.name, &left.arguments).cmp(&(
                &right.namespace,
                &right.name,
                &right.arguments,
            ))
        });
        let default_interface = interfaces
            .iter()
            .find(|interface| interface.default)
            .map(ClassInterface::clone_model);
        let default_constructor = has_default_constructor(definition)?;
        let async_default = default_interface
            .as_ref()
            .is_some_and(ClassInterface::is_async);
        let agile = async_default || is_agile(definition)?;
        Ok(Self {
            name,
            namespace,
            default_interface,
            interfaces,
            bases,
            default_constructor,
            agile,
            async_default,
        })
    }

    pub(super) fn dependencies(&self, members: &MemberSelection) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        if self.async_default {
            for argument in &self.default_interface.as_ref().unwrap().arguments {
                argument.collect_value_dependencies(&mut dependencies);
            }
            return dependencies;
        }
        for interface in &self.interfaces {
            let selected_methods = interface
                .methods
                .iter()
                .filter(|method| method.selected(members))
                .collect::<Vec<_>>();
            if !interface.default
                && !matches!(members, MemberSelection::All)
                && selected_methods.is_empty()
            {
                continue;
            }
            dependencies.insert((interface.namespace.clone(), interface.name.clone()));
            for argument in &interface.arguments {
                argument.collect_value_dependencies(&mut dependencies);
            }
            for method in selected_methods {
                dependencies.extend(method.method.dependencies());
            }
        }
        dependencies.extend(
            self.bases
                .iter()
                .map(|base| (base.namespace.clone(), base.name.clone())),
        );
        dependencies
    }

    pub(super) fn relationship_members(
        &self,
        members: &MemberSelection,
    ) -> BTreeMap<(String, String), MemberSelection> {
        let mut result = BTreeMap::new();
        for interface in &self.interfaces {
            let selection = match members {
                MemberSelection::All => MemberSelection::All,
                MemberSelection::Names(names)
                    if interface
                        .methods
                        .iter()
                        .any(|method| method.selected(members)) =>
                {
                    MemberSelection::Names(names.clone())
                }
                MemberSelection::Names(_) | MemberSelection::Shell if interface.default => {
                    MemberSelection::Shell
                }
                MemberSelection::Names(_) | MemberSelection::Shell => continue,
            };
            result.insert(
                (interface.namespace.clone(), interface.name.clone()),
                selection,
            );
        }
        for base in &self.bases {
            result
                .entry((base.namespace.clone(), base.name.clone()))
                .or_insert(MemberSelection::Shell);
        }
        result
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let runtime_name = Literal::string(&format!("{}.{}", self.namespace, self.name));
        if self.async_default {
            let default_type = self
                .default_interface
                .as_ref()
                .unwrap()
                .write_async_name(namespace, layout)?;
            return Ok(quote! { pub type #name = #default_type; });
        }
        let agile = self.agile.then(|| {
            quote! {
                unsafe impl Send for #name {}
                unsafe impl Sync for #name {}
            }
        });
        let Some(default_interface) = &self.default_interface else {
            let context = winrt_delegate::MethodContext::new(
                values,
                namespace,
                layout,
                projection,
                &[],
                Some(&self.name),
            );
            let mut names = BTreeMap::new();
            let factories =
                self.write_factories(namespace, layout, &name, &context, &mut names, members)?;
            let impl_block = (!factories.is_empty()).then(|| {
                quote! {
                    impl #name {
                        #(#factories)*
                    }
                }
            });
            return Ok(quote! {
                pub struct #name;
                #impl_block
                impl windows_core::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
            });
        };
        let default_type = default_interface.write_name(namespace, layout)?;
        let default_hierarchy = (!default_interface.exclusive).then(|| quote! { , #default_type });
        let required = self
            .interfaces
            .iter()
            .filter(|interface| !interface.default)
            .filter(|interface| !interface.exclusive && !interface.factory)
            .filter(|interface| {
                matches!(members, MemberSelection::All)
                    || interface
                        .methods
                        .iter()
                        .any(|method| method.selected(members))
            })
            .map(|interface| interface.write_name(namespace, layout))
            .chain(
                self.bases
                    .iter()
                    .map(|base| Ok(base.write_name(namespace, layout))),
            )
            .collect::<Result<Vec<_>, Error>>()?;
        let required_hierarchy = (!required.is_empty()).then(|| {
            quote! {
                windows_core::imp::required_hierarchy!(#name, #(#required),*);
            }
        });
        let constructor = (self.default_constructor
            && !projection.is_minimal()
            && !matches!(members, MemberSelection::Shell))
        .then(|| {
            quote! {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        #name,
                        windows_core::imp::IGenericFactory
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
        });
        let context = winrt_delegate::MethodContext::new(
            values,
            namespace,
            layout,
            projection,
            &[],
            Some(&self.name),
        );
        let mut methods = Vec::new();
        let mut names = BTreeMap::<String, u32>::new();
        for interface in self
            .interfaces
            .iter()
            .filter(|interface| !interface.factory && !projection.is_minimal())
        {
            let interface_type = interface.write_name(namespace, layout)?;
            for method in &interface.methods {
                if !method.is_public() || !method.selected(members) {
                    continue;
                }
                let context_name = &method.context_name;
                let count = names.entry(context_name.clone()).or_default();
                *count += 1;
                let public_name = if *count == 1 {
                    context_name.clone()
                } else {
                    format!("{context_name}{count}")
                };
                methods.push(if interface.default {
                    method.write_public(&context, &public_name, None)?.unwrap()
                } else {
                    method
                        .write_public(&context, &public_name, Some(interface_type.clone()))?
                        .unwrap()
                });
            }
        }
        let factories =
            self.write_factories(namespace, layout, &name, &context, &mut names, members)?;
        let deref = projection.is_minimal().then(|| {
            quote! {
                impl core::ops::Deref for #name {
                    type Target = #default_type;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
            }
        });
        let impl_block = if constructor.is_none() && methods.is_empty() && factories.is_empty() {
            quote! {}
        } else {
            quote! {
                impl #name {
                    #constructor
                    #(#methods)*
                    #(#factories)*
                }
            }
        };
        Ok(quote! {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct #name(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                #name,
                windows_core::IUnknown,
                windows_core::IInspectable
                #default_hierarchy
            );
            #required_hierarchy
            #impl_block
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, #default_type>();
            }
            unsafe impl windows_core::Interface for #name {
                type Vtable = <#default_type as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <#default_type as windows_core::Interface>::IID;
            }
            #deref
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
            #agile
        })
    }

    fn write_factories(
        &self,
        namespace: &str,
        layout: Layout,
        name: &TokenStream,
        context: &winrt_delegate::MethodContext<'_>,
        names: &mut BTreeMap<String, u32>,
        members: &MemberSelection,
    ) -> Result<Vec<TokenStream>, Error> {
        let mut factories = Vec::new();
        for interface in self.interfaces.iter().filter(|interface| {
            interface.factory
                && (matches!(members, MemberSelection::All)
                    || interface
                        .methods
                        .iter()
                        .any(|method| method.selected(members)))
        }) {
            let interface_type = interface.write_name(namespace, layout)?;
            let factory_name = tokens::ident(&interface.name);
            for method in &interface.methods {
                if !method.selected(members) {
                    continue;
                }
                let context_name = &method.context_name;
                let count = names.entry(context_name.clone()).or_default();
                *count += 1;
                let public_name = if *count == 1 {
                    context_name.clone()
                } else {
                    format!("{context_name}{count}")
                };
                factories.push(method.method.write_static_method(
                    context,
                    &public_name,
                    &method.name,
                    &interface.name,
                    &self.namespace,
                    &self.name,
                )?);
            }
            factories.push(quote! {
                fn #factory_name<
                    R,
                    F: FnOnce(&#interface_type) -> windows_core::Result<R>
                >(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<#name, #interface_type> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            });
        }
        Ok(factories)
    }
}

impl ClassInterface {
    fn is_async(&self) -> bool {
        self.namespace == "Windows.Foundation"
            && matches!(
                self.name.as_str(),
                "IAsyncAction"
                    | "IAsyncActionWithProgress"
                    | "IAsyncOperation"
                    | "IAsyncOperationWithProgress"
            )
    }

    fn write_async_name(&self, namespace: &str, layout: Layout) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, &[]))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { windows_future::#name }
        } else {
            quote! { windows_future::#name<#(#arguments),*> }
        })
    }

    fn clone_model(&self) -> Self {
        Self {
            namespace: self.namespace.clone(),
            entity: self.entity,
            name: self.name.clone(),
            arguments: self.arguments.clone(),
            methods: Vec::new(),
            default: self.default,
            exclusive: self.exclusive,
            factory: self.factory,
        }
    }

    fn write_name(&self, namespace: &str, layout: Layout) -> Result<TokenStream, Error> {
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&self.name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, &[]))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }
}

impl ClassName {
    fn write_name(&self, namespace: &str, layout: Layout) -> TokenStream {
        let path = tokens::namespace(namespace, &self.namespace, layout);
        let name = tokens::ident(&self.name);
        quote! { #path #name }
    }
}

fn lower_interfaces(
    database: &Database,
    relationships: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
    owner: Entity<TypeDef>,
    inherited: bool,
    owner_name: &str,
    seen: &mut BTreeSet<(Entity<TypeDef>, Vec<ty::Type>)>,
    result: &mut Vec<ClassInterface>,
) -> Result<(), Error> {
    let Some(interfaces) = relationships.get(&owner) else {
        return Ok(());
    };
    let context = InterfaceLowering {
        database,
        relationships,
        owner_name,
    };
    for relationship in interfaces {
        let interface = relationship.resolve()?;
        lower_interface(&context, interface, &[], inherited, seen, result)?;
    }
    Ok(())
}

fn lower_factories(
    database: &Database,
    definition: TypeDefinition<'_>,
    owner: &str,
    seen: &mut BTreeSet<(Entity<TypeDef>, Vec<ty::Type>)>,
    result: &mut Vec<ClassInterface>,
) -> Result<(), Error> {
    for attribute in definition.attributes()? {
        if !matches!(
            attribute.name()?,
            Some("StaticAttribute" | "ActivatableAttribute" | "ComposableAttribute")
        ) {
            continue;
        }
        for argument in attribute.arguments(&())? {
            let AttributeArgument::Fixed {
                value: AttributeValue::TypeName(type_name),
                ..
            } = argument
            else {
                continue;
            };
            let Some((namespace, name)) = type_name.rsplit_once('.') else {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "factory interface type name has no namespace",
                });
            };
            let entity = database
                .type_definitions(namespace, name)
                .first()
                .copied()
                .ok_or_else(|| Error::InvalidType {
                    name: owner.to_string(),
                    message: "factory interface cannot be resolved",
                })?;
            if !seen.insert((entity, Vec::new())) {
                break;
            }
            let interface = database.definition(entity).unwrap();
            result.push(ClassInterface {
                namespace: namespace.to_string(),
                entity,
                name: trim_generic_arity(name).to_string(),
                arguments: Vec::new(),
                methods: winrt_interface::lower_methods(database, interface, owner)?,
                default: false,
                exclusive: interface.has_attribute("ExclusiveToAttribute")?,
                factory: true,
            });
            break;
        }
    }
    Ok(())
}

struct InterfaceLowering<'a> {
    database: &'a Database,
    relationships: &'a BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
    owner_name: &'a str,
}

fn lower_interface(
    context: &InterfaceLowering<'_>,
    interface: &InterfaceBase,
    owner_arguments: &[ty::Type],
    inherited: bool,
    seen: &mut BTreeSet<(Entity<TypeDef>, Vec<ty::Type>)>,
    result: &mut Vec<ClassInterface>,
) -> Result<(), Error> {
    let arguments = interface
        .arguments
        .iter()
        .cloned()
        .map(|argument| {
            ty::Type::lower(
                context.database,
                interface.file,
                context.owner_name,
                argument,
            )
        })
        .collect::<Result<Vec<_>, Error>>()?
        .into_iter()
        .map(|argument| argument.substitute(owner_arguments))
        .collect::<Vec<_>>();
    if let Some(required) = context.relationships.get(&interface.entity) {
        for relationship in required {
            let base = relationship.resolve()?;
            lower_interface(context, base, &arguments, inherited, seen, result)?;
        }
    }
    if !seen.insert((interface.entity, arguments.clone())) {
        if interface.default
            && let Some(existing) = result.iter_mut().find(|existing| {
                existing.entity == interface.entity && existing.arguments == arguments
            })
        {
            existing.default = true;
        }
        return Ok(());
    }
    let definition = context.database.definition(interface.entity).unwrap();
    let metadata_name = definition.name()?;
    let mut methods =
        winrt_interface::lower_methods(context.database, definition, context.owner_name)?;
    for method in &mut methods {
        method.substitute(&arguments);
    }
    result.push(ClassInterface {
        entity: interface.entity,
        namespace: definition.namespace()?.to_string(),
        name: trim_generic_arity(metadata_name).to_string(),
        arguments,
        methods,
        default: interface.default && !inherited,
        exclusive: definition.has_attribute("ExclusiveToAttribute")?,
        factory: false,
    });
    Ok(())
}

fn lower_bases(
    database: &Database,
    definition: TypeDefinition<'_>,
    relationships: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
    owner: &str,
    interfaces: &mut Vec<ClassInterface>,
) -> Result<Vec<ClassName>, Error> {
    let mut bases = Vec::new();
    let mut current = definition;
    let mut seen_interfaces = interfaces
        .iter()
        .map(|interface| (interface.entity, interface.arguments.clone()))
        .collect::<BTreeSet<_>>();
    while let Some(identity) = current.base_type()? {
        let Some((namespace, name)) = database.type_name(identity.file, identity.ty)? else {
            break;
        };
        if namespace == "System" && name == "Object" {
            break;
        }
        let entity = resolve_definition(database, identity, owner)?;
        let base = database.definition(entity).unwrap();
        lower_interfaces(
            database,
            relationships,
            entity,
            true,
            owner,
            &mut seen_interfaces,
            interfaces,
        )?;
        bases.push(ClassName {
            namespace: base.namespace()?.to_string(),
            name: base.name()?.to_string(),
        });
        current = base;
    }
    Ok(bases)
}

fn resolve_definition(
    database: &Database,
    identity: TypeIdentity,
    owner: &str,
) -> Result<Entity<TypeDef>, Error> {
    match database.resolve_type(identity.file, identity.ty)? {
        TypeResolution::Definition(entity) => Ok(entity),
        TypeResolution::Candidates(candidates) => {
            candidates.first().ok_or_else(|| Error::InvalidType {
                name: owner.to_string(),
                message: "class relationship cannot be resolved",
            })
        }
        TypeResolution::Specification(_) => Err(Error::InvalidType {
            name: owner.to_string(),
            message: "class relationship is a type specification",
        }),
    }
}

fn has_default_constructor(definition: TypeDefinition<'_>) -> Result<bool, Error> {
    for attribute in definition.attributes()? {
        if attribute.name()? != Some("ActivatableAttribute") {
            continue;
        }

        let arguments = attribute.arguments(&())?;
        if !arguments.iter().any(|argument| {
            matches!(
                argument,
                AttributeArgument::Fixed {
                    value: AttributeValue::TypeName(_),
                    ..
                }
            )
        }) {
            return Ok(true);
        }
    }
    Ok(false)
}
