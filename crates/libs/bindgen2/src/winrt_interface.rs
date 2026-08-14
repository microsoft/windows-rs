use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub(super) struct Interface {
    pub(super) name: String,
    pub(super) namespace: String,
    pub(super) generics: Vec<String>,
    guid: guid::Guid,
    exclusive: bool,
    agile: bool,
    package_dependencies: BTreeSet<(String, String)>,
    pub(super) methods: Vec<NamedMethod>,
    pub(super) required: Vec<RequiredInterface>,
}

#[derive(Clone)]
pub(super) struct NamedMethod {
    pub(super) name: String,
    pub(super) context_name: String,
    metadata_name: String,
    overloaded: bool,
    pub(super) method: winrt_delegate::Method,
    event: Option<winrt_delegate::EventHandler>,
    public: bool,
}

impl NamedMethod {
    pub(super) fn selected(&self, members: &MemberSelection) -> bool {
        if matches!(members, MemberSelection::All) {
            return true;
        }
        let MemberSelection::Names(names) = members else {
            return false;
        };
        if names.contains(&self.name) || (!self.overloaded && names.contains(&self.metadata_name)) {
            return true;
        }
        self.metadata_name
            .strip_prefix("remove_")
            .is_some_and(|name| names.contains(&format!("add_{name}")) || names.contains(name))
    }

    pub(super) fn selected_factory(&self, members: &MemberSelection) -> bool {
        members.includes(&self.metadata_name, &self.name)
    }

    pub(super) fn write_static(
        &self,
        context: &winrt_delegate::MethodContext<'_>,
        public_name: &str,
        factory_name: &str,
    ) -> Result<Option<TokenStream>, Error> {
        if !self.public {
            return Ok(None);
        }
        if let Some(event) = &self.event {
            return Ok(Some(self.method.write_static_event_method(
                context,
                public_name,
                &format!("Remove{}", self.name),
                factory_name,
                event,
            )?));
        }
        Ok(Some(self.method.write_static_method(
            context,
            public_name,
            &self.name,
            factory_name,
        )?))
    }

    pub(super) fn substitute(&mut self, arguments: &[ty::Type]) {
        self.method.substitute(arguments);
        if let Some(event) = &mut self.event {
            event.substitute(arguments);
        }
    }

    pub(super) const fn is_public(&self) -> bool {
        self.public
    }

    pub(super) fn write_public(
        &self,
        context: &winrt_delegate::MethodContext<'_>,
        public_name: &str,
        interface: Option<TokenStream>,
    ) -> Result<Option<TokenStream>, Error> {
        if !self.public {
            return Ok(None);
        }
        let (receiver, prelude) = if let Some(interface) = &interface {
            (
                quote! { this },
                quote! {
                    let this = &windows_core::Interface::cast::<#interface>(self)?;
                },
            )
        } else {
            (quote! { self }, quote! {})
        };
        if let Some(event) = &self.event {
            return Ok(Some(self.method.write_event_method(
                context,
                public_name,
                &format!("Remove{}", self.name),
                event,
                receiver,
                prelude,
            )?));
        }
        Ok(Some(if let Some(interface) = interface {
            self.method.write_forwarded_public_method(
                context,
                public_name,
                &self.name,
                interface,
            )?
        } else {
            self.method
                .write_public_method(context, public_name, receiver)?
        }))
    }
}

#[derive(Clone)]
pub(super) struct RequiredInterface {
    pub(super) namespace: String,
    pub(super) name: String,
    pub(super) arguments: Vec<ty::Type>,
    pub(super) methods: Vec<NamedMethod>,
}

impl Interface {
    pub(super) fn implicitly_implements(
        &self,
        members: &MemberSelection,
        projection: Projection,
    ) -> bool {
        !self.exclusive
            && (members.emits_implementation(projection)
                || (projection.is_minimal()
                    && matches!(members, MemberSelection::Shell)
                    && self.methods.is_empty()))
    }

    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        interface_bases: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
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
        let agile = is_agile(definition)?;
        let mut result = Self {
            name,
            namespace,
            generics,
            guid,
            exclusive,
            agile,
            package_dependencies: BTreeSet::new(),
            methods,
            required,
        };
        result.package_dependencies = result.direct_artifact_dependencies();
        Ok(result)
    }

    fn lower_required(
        database: &Database,
        interface_bases: &BTreeMap<Entity<TypeDef>, Vec<InterfaceRelationship>>,
        entity: Entity<TypeDef>,
        owner_arguments: &[ty::Type],
        owner: &str,
        seen: &mut BTreeSet<Entity<TypeDef>>,
        result: &mut Vec<RequiredInterface>,
    ) -> Result<(), Error> {
        let Some(bases) = interface_bases.get(&entity) else {
            return Ok(());
        };
        for relationship in bases {
            let base = relationship.resolve()?;
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
                method.substitute(&arguments);
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

    pub(super) fn selection_dependencies(
        &self,
        members: &MemberSelection,
        retain_abi_prefix: bool,
    ) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        let methods = if retain_abi_prefix {
            self.abi_methods(members).collect::<Vec<_>>()
        } else {
            self.methods
                .iter()
                .filter(|method| method.selected(members))
                .collect()
        };
        for method in methods {
            dependencies.extend(method.method.selection_dependencies());
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
            for method in required
                .methods
                .iter()
                .filter(|method| method.selected(members))
            {
                dependencies.extend(method.method.selection_dependencies());
            }
        }
        dependencies
    }

    pub(super) fn direct_artifact_dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        for required in &self.required {
            dependencies.insert((required.namespace.clone(), required.name.clone()));
            for argument in &required.arguments {
                argument.collect_value_dependencies(&mut dependencies);
            }
        }
        dependencies
    }

    pub(super) fn expand_package_dependencies(&mut self, graph: &winrt_dependency::ArtifactGraph) {
        self.package_dependencies = graph.expand(&self.direct_artifact_dependencies());
        for method in &mut self.methods {
            method.method.expand_package_dependencies(graph);
        }
        for required in &mut self.required {
            for method in &mut required.methods {
                method.method.expand_package_dependencies(graph);
            }
        }
    }

    pub(super) fn relationship_members(
        &self,
        members: &MemberSelection,
    ) -> BTreeMap<(String, String), MemberSelection> {
        let mut result = BTreeMap::new();
        for required in &self.required {
            let selection = match members {
                MemberSelection::All => MemberSelection::All,
                MemberSelection::Names(names)
                    if required
                        .methods
                        .iter()
                        .any(|method| method.selected(members)) =>
                {
                    MemberSelection::Names(names.clone())
                }
                MemberSelection::Names(_) | MemberSelection::Shell => MemberSelection::Shell,
            };
            result.insert(
                (required.namespace.clone(), required.name.clone()),
                selection,
            );
        }
        result
    }

    fn abi_methods<'a>(
        &'a self,
        members: &'a MemberSelection,
    ) -> impl Iterator<Item = &'a NamedMethod> {
        let count = match members {
            MemberSelection::All => self.methods.len(),
            MemberSelection::Names(_) => self
                .methods
                .iter()
                .rposition(|method| method.selected(members))
                .map_or(0, |index| index + 1),
            MemberSelection::Shell => 0,
        };
        self.methods[..count].iter()
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        members: &MemberSelection,
        implementation: Option<bool>,
        explicit: bool,
    ) -> Result<TokenStream, Error> {
        let implementation = implementation.or_else(|| self.exclusive.then_some(false));
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let impl_name = tokens::ident(&format!("{}_Impl", self.name));
        let artifact_features = tokens::feature_names(
            namespace,
            layout,
            self.package_dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let artifact_cfg = tokens::feature_cfg_set(&artifact_features, false);
        let mut implementation_dependencies = self.package_dependencies.clone();
        for method in self.abi_methods(members) {
            implementation_dependencies
                .extend(method.method.package_dependencies().iter().cloned());
        }
        for required in &self.required {
            for method in required
                .methods
                .iter()
                .filter(|method| method.selected(members))
            {
                implementation_dependencies
                    .extend(method.method.package_dependencies().iter().cloned());
            }
        }
        let implementation_features = tokens::feature_names(
            namespace,
            layout,
            implementation_dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let implementation_cfg = tokens::feature_cfg_set(&implementation_features, false);
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
        let emit_type_name = !projection.is_minimal()
            || implementation == Some(true)
            || (!self.exclusive && !generic_names.is_empty());
        let definition = if generic_names.is_empty() {
            let metadata_name = Literal::byte_string(full_name.as_bytes());
            let metadata_name = emit_type_name.then(|| {
                quote! {
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(#metadata_name);
                }
            });
            quote! {
                #artifact_cfg
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #artifact_cfg
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                    #metadata_name
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
            let names = generic_names.iter().enumerate().map(|(index, name)| {
                let separator = (index != 0).then(|| quote! { .push_slice(b", ") });
                quote! { #separator .push_other(#name::NAME) }
            });
            let metadata_name = emit_type_name.then(|| {
                quote! {
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::new()
                            .push_slice(#metadata_name)
                            #(#names)*
                            .push_slice(b">");
                }
            });
            quote! {
                #artifact_cfg
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name #type_arguments(
                    windows_core::IUnknown
                    #(, core::marker::PhantomData<#generic_names>)*
                ) #generic_where;
                #artifact_cfg
                impl #constrained_generics windows_core::imp::CanInto<windows_core::IUnknown>
                    for #name #type_arguments {}
                #artifact_cfg
                impl #constrained_generics windows_core::imp::CanInto<windows_core::IInspectable>
                    for #name #type_arguments {}
                #artifact_cfg
                unsafe impl #constrained_generics windows_core::Interface
                    for #name #type_arguments
                {
                    type Vtable = #vtbl_name #type_arguments;
                    const IID: windows_core::GUID =
                        windows_core::GUID::from_signature(
                            <Self as windows_core::RuntimeType>::SIGNATURE
                        );
                }
                #artifact_cfg
                impl #constrained_generics windows_core::RuntimeType for #name #type_arguments {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::new()
                            .push_slice(#signature)
                            #(#signatures)*
                            .push_slice(b")");
                    #metadata_name
                }
            }
        };
        let method_context = winrt_delegate::MethodContext::new(
            values,
            namespace,
            layout,
            projection,
            &self.generics,
            Some(&self.name),
        );
        let methods =
            self.methods
                .iter()
                .filter(|method| method.selected(members))
                .filter(|_| {
                    !projection.is_minimal()
                        || if implementation == Some(true) {
                            explicit && !self.name.ends_with("Overrides")
                        } else {
                            !self.name.ends_with("Factory") && !self.name.ends_with("Statics")
                        }
                })
                .filter_map(|method| {
                    let tokens = method
                        .write_public(&method_context, &method.name, None)
                        .transpose();
                    tokens.map(|tokens| {
                        tokens.map(|tokens| {
                            let mut features =
                                tokens::feature_names(
                                    namespace,
                                    layout,
                                    method.method.package_dependencies().iter().map(
                                        |(namespace, name)| (namespace.as_str(), name.as_str()),
                                    ),
                                );
                            features.retain(|feature| !artifact_features.contains(feature));
                            let cfg = tokens::feature_cfg_set(&features, false);
                            quote! { #cfg #tokens }
                        })
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
        let agile = self.agile.then(|| {
            quote! {
                #artifact_cfg
                unsafe impl #constrained_generics Send for #name #type_arguments {}
                #artifact_cfg
                unsafe impl #constrained_generics Sync for #name #type_arguments {}
            }
        });
        if self.exclusive {
            let vtable = if implementation == Some(true) {
                let impl_methods = self
                    .abi_methods(members)
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
                let vtable = self.write_vtable(
                    values,
                    namespace,
                    layout,
                    members,
                    &implementation_cfg,
                    &artifact_cfg,
                )?;
                let runtime_name = Literal::string(&full_name);
                let runtime_class_name = (!generic_names.is_empty()).then(|| {
                    quote! {
                        const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                            <Self as windows_core::RuntimeType>::NAME;
                    }
                });
                let implementation_enabled =
                    implementation.unwrap_or_else(|| members.emits_implementation(projection));
                let runtime_name_impl = (!projection.is_minimal()
                    || implementation == Some(true)
                    || (implementation.is_none()
                        && (!matches!(members, MemberSelection::Shell) || implementation_enabled)))
                    .then(|| {
                        quote! {
                            #implementation_cfg
                            impl #constrained_generics windows_core::RuntimeName
                                for #name #type_arguments
                            {
                                const NAME: &'static str = #runtime_name;
                                #runtime_class_name
                            }
                        }
                    });
                quote! {
                    #runtime_name_impl
                    #implementation_cfg
                    pub trait #impl_name #type_arguments: windows_core::IUnknownImpl #generic_where {
                        #(#impl_methods)*
                    }
                    #vtable
                }
            } else {
                self.write_vtable_struct(
                    values,
                    namespace,
                    layout,
                    &vtbl_name,
                    members,
                    projection.is_minimal() && implementation != Some(true),
                    &artifact_cfg,
                )?
            };
            let methods = (projection.is_minimal() && !methods.is_empty()).then(|| {
                quote! {
                    #artifact_cfg
                    impl #constrained_generics #name #type_arguments { #(#methods)* }
                }
            });
            return Ok(quote! {
                #definition
                #methods
                #agile
                #vtable
            });
        }
        let hierarchy = generic_names.is_empty().then(|| {
            quote! {
                #artifact_cfg
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
        let required_hierarchy = if required_types.is_empty() {
            quote! {}
        } else if generic_names.is_empty() {
            quote! {
                #artifact_cfg
                windows_core::imp::required_hierarchy!(
                    #name #type_arguments,
                    #(#required_types),*
                );
            }
        } else {
            quote! {
                #(
                    #artifact_cfg
                    impl #constrained_generics windows_core::imp::CanInto<#required_types>
                        for #name #type_arguments
                    {
                        const QUERY: bool = true;
                    }
                )*
            }
        };
        let mut inherited_methods = Vec::new();
        let mut public_names = self
            .methods
            .iter()
            .filter(|method| method.public && method.selected(members))
            .fold(BTreeMap::<String, u32>::new(), |mut names, method| {
                *names.entry(method.name.clone()).or_default() += 1;
                names
            });
        if !projection.is_minimal() {
            for required in &self.required {
                let receiver = required.write_name(namespace, layout, &self.generics)?;
                for method in &required.methods {
                    if !method.public || !method.selected(members) {
                        continue;
                    }
                    let count = public_names.entry(method.name.clone()).or_default();
                    *count += 1;
                    let public_name = if *count == 1 {
                        method.name.clone()
                    } else {
                        format!("{}{}", method.name, count)
                    };
                    let tokens = method
                        .write_public(&method_context, &public_name, Some(receiver.clone()))?
                        .unwrap();
                    let mut features = tokens::feature_names(
                        namespace,
                        layout,
                        method
                            .method
                            .package_dependencies()
                            .iter()
                            .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
                    );
                    features.retain(|feature| !artifact_features.contains(feature));
                    let cfg = tokens::feature_cfg_set(&features, false);
                    inherited_methods.push(quote! { #cfg #tokens });
                }
            }
        }
        let runtime_name = Literal::string(&full_name);
        let runtime_class_name = (!generic_names.is_empty()).then(|| {
            quote! {
                const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
                    <Self as windows_core::RuntimeType>::NAME;
            }
        });
        let implementation_enabled = implementation.unwrap_or_else(|| {
            members.emits_implementation(projection)
                || (projection.is_minimal()
                    && matches!(members, MemberSelection::Shell)
                    && self.methods.is_empty())
        });
        let runtime_name_impl = (!projection.is_minimal()
            || implementation == Some(true)
            || (implementation.is_none()
                && (!matches!(members, MemberSelection::Shell) || implementation_enabled)))
            .then(|| {
                quote! {
                    #implementation_cfg
                    impl #constrained_generics windows_core::RuntimeName for #name #type_arguments {
                        const NAME: &'static str = #runtime_name;
                        #runtime_class_name
                    }
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
            .abi_methods(members)
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
        let implementation = if implementation_enabled {
            let vtable = self.write_vtable(
                values,
                namespace,
                layout,
                members,
                &implementation_cfg,
                &artifact_cfg,
            )?;
            quote! {
                #implementation_cfg
                pub trait #impl_name #type_arguments: #trait_bases #generic_where {
                    #(#impl_methods)*
                }
                #vtable
            }
        } else {
            self.write_vtable_struct(
                values,
                namespace,
                layout,
                &vtbl_name,
                members,
                projection.is_minimal() && !implementation_enabled,
                &artifact_cfg,
            )?
        };
        let methods_impl = (!methods.is_empty() || !inherited_methods.is_empty()).then(|| {
            quote! {
                #artifact_cfg
                impl #constrained_generics #name #type_arguments {
                    #(#methods)*
                    #(#inherited_methods)*
                }
            }
        });
        let winrt_collection::Conveniences {
            before_runtime_name,
            after_implementation,
        } = winrt_collection::write(self, namespace, layout, projection, members, &artifact_cfg)?;
        Ok(quote! {
            #definition
            #hierarchy
            #required_hierarchy
            #methods_impl
            #agile
            #before_runtime_name
            #runtime_name_impl
            #implementation
            #after_implementation
        })
    }

    fn write_vtable(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        members: &MemberSelection,
        implementation_cfg: &TokenStream,
        artifact_cfg: &TokenStream,
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
        let functions = self
            .abi_methods(members)
            .map(|method| {
                let method_name = tokens::ident(&method.name);
                let signature = method.method.write_abi_signature(
                    values,
                    namespace,
                    layout,
                    &self.generics,
                    true,
                )?;
                let upcall = method.method.write_method_upcall(
                    values,
                    layout,
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
        let initializers = self.abi_methods(members).map(|method| {
            let name = tokens::ident(&method.name);
            quote! { #name: #name::<#(#generic_names,)* Identity, OFFSET>, }
        });
        let phantom_values = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData::<#name>, });
        let vtable = self.write_vtable_struct(
            values,
            namespace,
            layout,
            &vtbl_name,
            members,
            false,
            artifact_cfg,
        )?;
        Ok(quote! {
            #implementation_cfg
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
        members: &MemberSelection,
        placeholder_prefix: bool,
        artifact_cfg: &TokenStream,
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
        let artifact_features = tokens::feature_names(
            namespace,
            layout,
            self.package_dependencies
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let fields = self
            .abi_methods(members)
            .map(|method| {
                let name = tokens::ident(&method.name);
                if placeholder_prefix && !method.selected(members) {
                    return Ok(quote! { #name: usize, });
                }
                let signature = method.method.write_abi_signature(
                    values,
                    namespace,
                    layout,
                    &self.generics,
                    false,
                )?;
                let mut features = tokens::feature_names(
                    namespace,
                    layout,
                    method
                        .method
                        .package_dependencies()
                        .iter()
                        .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
                );
                features.retain(|feature| !artifact_features.contains(feature));
                let yes = tokens::feature_cfg_set(&features, false);
                let no = tokens::feature_cfg_set(&features, true);
                let fallback = (!features.is_empty()).then(|| {
                    quote! {
                        #no
                        #name: usize,
                    }
                });
                Ok(quote! {
                    #yes
                    pub #name: unsafe extern "system" fn(#signature) -> windows_core::HRESULT,
                    #fallback
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let phantom_fields = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData<#name>, });
        let doc_hidden = layout.is_package().then(|| quote! { #[doc(hidden)] });
        Ok(quote! {
            #artifact_cfg
            #[repr(C)]
            #doc_hidden
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
        self.write_named(namespace, layout, generics, &self.name)
    }

    fn write_impl_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_named(namespace, layout, generics, &format!("{}_Impl", self.name))
    }

    fn write_named(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        emitted_name: &str,
    ) -> Result<TokenStream, Error> {
        let path =
            if let Some(crate_name) = layout.winrt_crate(namespace, &self.namespace, &self.name) {
                let crate_name = tokens::ident(crate_name);
                quote! { #crate_name:: }
            } else {
                tokens::namespace(namespace, &self.namespace, layout)
            };
        let name = tokens::ident(emitted_name);
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
    let metadata_names = definition
        .methods()?
        .map(|method| Ok(method.name()?.to_string()))
        .collect::<Result<BTreeSet<_>, Error>>()?;
    definition
        .methods()?
        .map(|method| {
            let metadata_name = method.name()?;
            let event_name = metadata_name.strip_prefix("add_");
            if let Some(event_name) = event_name
                && !metadata_names.contains(&format!("remove_{event_name}"))
            {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "event add method has no matching remove method",
                });
            }
            let overload_attribute = method.find_attribute("OverloadAttribute")?;
            let (mut name, context_name) = if method.flags()? & 0x800 != 0 {
                if let Some(name) = metadata_name.strip_prefix("get_") {
                    (name.to_string(), name.to_string())
                } else if let Some(name) = metadata_name.strip_prefix("put_") {
                    (format!("Set{name}"), format!("Set{name}"))
                } else if let Some(name) = metadata_name.strip_prefix("add_") {
                    (name.to_string(), name.to_string())
                } else if let Some(name) = metadata_name.strip_prefix("remove_") {
                    (format!("Remove{name}"), format!("Remove{name}"))
                } else {
                    (metadata_name.to_string(), metadata_name.to_string())
                }
            } else if let Some(attribute) = overload_attribute {
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
                        (metadata_name.to_string(), metadata_name.to_string())
                    }
                    Some(overload) => (overload.to_string(), overload.to_string()),
                    None => (metadata_name.to_string(), metadata_name.to_string()),
                }
            } else {
                (metadata_name.to_string(), metadata_name.to_string())
            };
            let count = names.entry(name.clone()).or_default();
            *count += 1;
            if *count > 1 {
                name.push_str(&count.to_string());
            }
            let method = winrt_delegate::Method::lower(
                database,
                definition.entity().file(),
                method,
                owner,
                false,
            )?;
            let event = if event_name.is_some() {
                Some(method.lower_event_handler(database, owner)?)
            } else {
                None
            };
            Ok(NamedMethod {
                name,
                context_name,
                metadata_name: metadata_name.to_string(),
                overloaded: overload_attribute.is_some(),
                method,
                event,
                public: !metadata_name.starts_with("remove_"),
            })
        })
        .collect()
}
