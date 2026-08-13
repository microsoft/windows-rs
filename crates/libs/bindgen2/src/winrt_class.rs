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
    composable: bool,
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

    pub(super) fn dependencies(
        &self,
        members: &MemberSelection,
        implementations: Option<&Filter>,
        projection: Projection,
    ) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        if self.async_default {
            for argument in &self.default_interface.as_ref().unwrap().arguments {
                argument.collect_value_dependencies(&mut dependencies);
            }
            return dependencies;
        }
        let implemented = self.has_implemented_interface(implementations);
        for interface in &self.interfaces {
            let implementation_factory =
                projection.is_minimal() && interface.factory && interface.composable && implemented;
            let default_factory = projection.is_minimal()
                && interface.factory
                && interface.composable
                && !implemented
                && matches!(
                    members,
                    MemberSelection::Names(names)
                        if !names.is_empty()
                            && !names.iter().any(|name| name.starts_with("CreateInstance"))
                );
            let primary_constructor = interface
                .methods
                .iter()
                .find(|method| method.name == "CreateInstance")
                .or_else(|| {
                    interface
                        .methods
                        .iter()
                        .find(|method| method.name.starts_with("CreateInstance"))
                })
                .map(|method| method.name.as_str());
            let selected_methods = interface
                .methods
                .iter()
                .filter(|method| {
                    method.selected_factory(members)
                        || (implementation_factory && method.name == "CreateInstance")
                        || (default_factory && primary_constructor == Some(method.name.as_str()))
                })
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

    fn artifact_dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        for interface in self
            .interfaces
            .iter()
            .filter(|interface| !interface.factory)
        {
            dependencies.insert((interface.namespace.clone(), interface.name.clone()));
            for argument in &interface.arguments {
                argument.collect_value_dependencies(&mut dependencies);
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
        implementations: Option<&Filter>,
        projection: Projection,
    ) -> BTreeMap<(String, String), MemberSelection> {
        let mut result = BTreeMap::new();
        let implemented = self.has_implemented_interface(implementations);
        for interface in &self.interfaces {
            if projection.is_minimal()
                && interface.factory
                && interface.composable
                && self.has_implemented_interface(implementations)
            {
                result.insert(
                    (interface.namespace.clone(), interface.name.clone()),
                    MemberSelection::All,
                );
                continue;
            }
            if projection.is_minimal()
                && interface.factory
                && interface.composable
                && !implemented
                && matches!(
                    members,
                    MemberSelection::Names(names)
                        if !names.is_empty()
                            && !names.iter().any(|name| name.starts_with("CreateInstance"))
                )
                && let Some(method) = interface
                    .methods
                    .iter()
                    .find(|method| method.name == "CreateInstance")
                    .or_else(|| {
                        interface
                            .methods
                            .iter()
                            .find(|method| method.name.starts_with("CreateInstance"))
                    })
            {
                result.insert(
                    (interface.namespace.clone(), interface.name.clone()),
                    MemberSelection::Names(BTreeSet::from([method.name.clone()])),
                );
                continue;
            }
            let selection = match members {
                MemberSelection::All => MemberSelection::All,
                MemberSelection::Names(names)
                    if interface
                        .methods
                        .iter()
                        .any(|method| method.selected_factory(members)) =>
                {
                    let mut names = names.clone();
                    if interface.composable && names.contains("CreateInstance") {
                        names.extend(
                            interface
                                .methods
                                .iter()
                                .filter(|method| {
                                    method.name.starts_with("CreateInstance")
                                        || method.context_name.starts_with("CreateInstance")
                                })
                                .map(|method| method.name.clone()),
                        );
                    }
                    MemberSelection::Names(names)
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

    fn has_implemented_interface(&self, implementations: Option<&Filter>) -> bool {
        implementations.is_some_and(|implementations| {
            self.interfaces.iter().any(|interface| {
                !interface.factory
                    && implementations.includes(&interface.namespace, &interface.name)
            })
        })
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
        implementations: Option<&BTreeSet<Entity<TypeDef>>>,
        member_selections: &BTreeMap<Entity<TypeDef>, MemberSelection>,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let runtime_name = Literal::string(&format!("{}.{}", self.namespace, self.name));
        let class_features = tokens::feature_names(
            namespace,
            layout,
            self.artifact_dependencies()
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let class_cfg = tokens::feature_cfg_set(&class_features, false);
        if self.async_default {
            let default_type = self
                .default_interface
                .as_ref()
                .unwrap()
                .write_async_name(namespace, layout)?;
            return Ok(quote! {
                #class_cfg
                pub type #name = #default_type;
            });
        }
        let agile = self.agile.then(|| {
            quote! {
                #class_cfg
                unsafe impl Send for #name {}
                #class_cfg
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
            let factories = self.write_factories(
                namespace,
                layout,
                projection,
                &name,
                &context,
                &mut names,
                members,
                implementations,
                member_selections,
            )?;
            let impl_block = (!factories.is_empty()).then(|| {
                quote! {
                    #class_cfg
                    impl #name {
                        #(#factories)*
                    }
                }
            });
            return Ok(quote! {
                #class_cfg
                pub struct #name;
                #impl_block
                #class_cfg
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
            .filter(|interface| !interface.exclusive)
            .filter(|interface| {
                member_selections
                    .get(&interface.entity)
                    .is_some_and(|members| !matches!(members, MemberSelection::Shell))
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
                #class_cfg
                windows_core::imp::required_hierarchy!(#name, #(#required),*);
            }
        });
        let constructor = (self.default_constructor
            && match members {
                MemberSelection::All => true,
                MemberSelection::Names(names) => names.contains("CreateInstance"),
                MemberSelection::Shell => false,
            })
        .then(|| {
            let visibility = if projection.is_minimal() {
                quote! { pub(crate) }
            } else {
                quote! { pub }
            };
            quote! {
                #visibility fn new() -> windows_core::Result<Self> {
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
                let method_tokens = if interface.default {
                    method.write_public(&context, &public_name, None)?.unwrap()
                } else {
                    method
                        .write_public(&context, &public_name, Some(interface_type.clone()))?
                        .unwrap()
                };
                let mut features = tokens::feature_names(
                    namespace,
                    layout,
                    method
                        .method
                        .dependencies()
                        .iter()
                        .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
                );
                features.retain(|feature| !class_features.contains(feature));
                let cfg = tokens::feature_cfg_set(&features, false);
                methods.push(quote! { #cfg #method_tokens });
            }
        }
        let factories = self.write_factories(
            namespace,
            layout,
            projection,
            &name,
            &context,
            &mut names,
            members,
            implementations,
            member_selections,
        )?;
        let deref = projection.is_minimal().then(|| {
            quote! {
                #class_cfg
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
                #class_cfg
                impl #name {
                    #constructor
                    #(#methods)*
                    #(#factories)*
                }
            }
        };
        let iterable = if projection.is_minimal() {
            TokenStream::new()
        } else {
            self.interfaces
                .iter()
                .find(|interface| {
                    interface.namespace == "Windows.Foundation.Collections"
                        && interface.name == "IIterable"
                        && interface.arguments.len() == 1
                        && interface
                            .methods
                            .iter()
                            .any(|method| method.name == "First" && method.selected(members))
                })
                .map(|interface| {
                    let item = interface.arguments[0].write_name(namespace, layout, &[])?;
                    Ok::<_, Error>(winrt_collection::iterable(
                        &name,
                        &TokenStream::new(),
                        &TokenStream::new(),
                        &item,
                        &class_cfg,
                    ))
                })
                .transpose()?
                .unwrap_or_default()
        };
        Ok(quote! {
            #class_cfg
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct #name(windows_core::IUnknown);
            #class_cfg
            windows_core::imp::interface_hierarchy!(
                #name,
                windows_core::IUnknown,
                windows_core::IInspectable
                #default_hierarchy
            );
            #required_hierarchy
            #impl_block
            #class_cfg
            impl windows_core::RuntimeType for #name {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, #default_type>();
            }
            #class_cfg
            unsafe impl windows_core::Interface for #name {
                type Vtable = <#default_type as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <#default_type as windows_core::Interface>::IID;
            }
            #deref
            #class_cfg
            impl windows_core::RuntimeName for #name {
                const NAME: &'static str = #runtime_name;
            }
            #agile
            #iterable
        })
    }

    fn write_factories(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        name: &TokenStream,
        context: &winrt_delegate::MethodContext<'_>,
        names: &mut BTreeMap<String, u32>,
        members: &MemberSelection,
        implementations: Option<&BTreeSet<Entity<TypeDef>>>,
        member_selections: &BTreeMap<Entity<TypeDef>, MemberSelection>,
    ) -> Result<Vec<TokenStream>, Error> {
        let class_features = tokens::feature_names(
            namespace,
            layout,
            self.artifact_dependencies()
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let implemented = implementations.is_some_and(|implementations| {
            self.interfaces
                .iter()
                .any(|interface| implementations.contains(&interface.entity))
        });
        let mut factories = Vec::new();
        let mut helpers = Vec::new();
        for interface in self.interfaces.iter().filter(|interface| {
            let interface_members = member_selections
                .get(&interface.entity)
                .unwrap_or(&MemberSelection::Shell);
            let default_composable_constructor = projection.is_minimal()
                && !implemented
                && interface.composable
                && matches!(
                    members,
                    MemberSelection::Names(names)
                        if !names.is_empty()
                            && !names.iter().any(|name| name.starts_with("CreateInstance"))
                )
                && interface
                    .methods
                    .iter()
                    .any(|method| method.name.starts_with("CreateInstance"));
            interface.factory
                && (matches!(members, MemberSelection::All)
                    || interface.methods.iter().any(|method| {
                        method.selected_factory(members) || method.selected(interface_members)
                    })
                    || default_composable_constructor
                    || (projection.is_minimal() && implemented && interface.composable))
        }) {
            let interface_members = member_selections
                .get(&interface.entity)
                .unwrap_or(&MemberSelection::Shell);
            let interface_type = interface.write_name(namespace, layout)?;
            let factory_name = tokens::ident(&interface.name);
            let primary_constructor = interface
                .methods
                .iter()
                .find(|method| method.name == "CreateInstance")
                .or_else(|| {
                    interface
                        .methods
                        .iter()
                        .find(|method| method.name.starts_with("CreateInstance"))
                })
                .map(|method| method.name.as_str());
            let mut emitted = false;
            for method in &interface.methods {
                if !method.is_public() {
                    continue;
                }
                let selected =
                    method.selected_factory(members) || method.selected(interface_members);
                let implementation_constructor = projection.is_minimal()
                    && implemented
                    && interface.composable
                    && method.name == "CreateInstance";
                let default_constructor = projection.is_minimal()
                    && !implemented
                    && interface.composable
                    && primary_constructor == Some(method.name.as_str());
                if !selected && !implementation_constructor && !default_constructor {
                    continue;
                }
                emitted = true;
                let context_name = &method.context_name;
                let count = names.entry(context_name.clone()).or_default();
                *count += 1;
                let public_name = if *count == 1 {
                    context_name.clone()
                } else {
                    format!("{context_name}{count}")
                };
                let mut features = tokens::feature_names(
                    namespace,
                    layout,
                    method
                        .method
                        .dependencies()
                        .iter()
                        .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
                );
                features.retain(|feature| !class_features.contains(feature));
                let cfg = tokens::feature_cfg_set(&features, false);
                if interface.composable {
                    factories.extend(
                        method
                            .method
                            .write_composable_methods(
                                context,
                                &public_name,
                                &method.name,
                                &interface.name,
                                (selected || default_constructor)
                                    && (!projection.is_minimal() || !implemented),
                                !projection.is_minimal() || implementation_constructor,
                            )?
                            .into_iter()
                            .map(|tokens| quote! { #cfg #tokens }),
                    );
                } else {
                    let tokens = method
                        .write_static(
                            context,
                            &public_name,
                            &interface.name,
                            &self.namespace,
                            &self.name,
                        )?
                        .unwrap();
                    factories.push(quote! { #cfg #tokens });
                }
            }
            if !emitted {
                continue;
            }
            let mut features = tokens::feature_names(
                namespace,
                layout,
                [(&interface.namespace, &interface.name)]
                    .into_iter()
                    .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
            );
            features.retain(|feature| !class_features.contains(feature));
            let cfg = tokens::feature_cfg_set(&features, false);
            helpers.push(quote! {
                #cfg
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
        factories.extend(helpers);
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
            composable: self.composable,
        }
    }

    fn write_name(&self, namespace: &str, layout: Layout) -> Result<TokenStream, Error> {
        if namespace != self.namespace
            && let Some(crate_name) = external::winrt_crate(&self.namespace, &self.name)
        {
            let crate_name = tokens::ident(crate_name);
            let name = tokens::ident(&self.name);
            let arguments = self
                .arguments
                .iter()
                .map(|argument| argument.write_name(namespace, layout, &[]))
                .collect::<Result<Vec<_>, Error>>()?;
            return Ok(if arguments.is_empty() {
                quote! { #crate_name::#name }
            } else {
                quote! { #crate_name::#name<#(#arguments),*> }
            });
        }
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
        let attribute_name = attribute.name()?;
        if !matches!(
            attribute_name,
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
                composable: attribute_name == Some("ComposableAttribute"),
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
        composable: false,
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
