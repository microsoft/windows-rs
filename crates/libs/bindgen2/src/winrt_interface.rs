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
    metadata_name: String,
    pub(super) method: winrt_delegate::Method,
    event: Option<winrt_delegate::EventHandler>,
    public: bool,
}

impl NamedMethod {
    pub(super) fn selected(&self, members: &MemberSelection) -> bool {
        if members.includes(&self.metadata_name, &self.name) {
            return true;
        }
        let MemberSelection::Names(names) = members else {
            return false;
        };
        self.metadata_name
            .strip_prefix("remove_")
            .is_some_and(|name| names.contains(&format!("add_{name}")) || names.contains(name))
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

    pub(super) fn dependencies(&self, members: &MemberSelection) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        for method in self.abi_methods(members) {
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
            for method in required
                .methods
                .iter()
                .filter(|method| method.selected(members))
            {
                dependencies.extend(method.method.dependencies());
            }
        }
        dependencies
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
        let emit_type_name =
            !projection.is_minimal() || (!self.exclusive && !generic_names.is_empty());
        let definition = if generic_names.is_empty() {
            let metadata_name = Literal::byte_string(full_name.as_bytes());
            let metadata_name = emit_type_name.then(|| {
                quote! {
                    const NAME: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(#metadata_name);
                }
            });
            quote! {
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
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
            let names = generic_names
                .iter()
                .map(|name| quote! { .push_other(#name::NAME) });
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
        );
        let methods = self
            .methods
            .iter()
            .filter(|method| method.selected(members))
            .filter_map(|method| {
                method
                    .write_public(&method_context, &method.name, None)
                    .transpose()
            })
            .collect::<Result<Vec<_>, Error>>()?;
        if self.exclusive {
            let vtable =
                self.write_vtable_struct(values, namespace, layout, &vtbl_name, members)?;
            let methods = (projection.is_minimal() && !methods.is_empty()).then(
                || quote! { impl #constrained_generics #name #type_arguments { #(#methods)* } },
            );
            return Ok(quote! {
                #definition
                #methods
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
                    inherited_methods.push(
                        method
                            .write_public(&method_context, &public_name, Some(receiver.clone()))?
                            .unwrap(),
                    );
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
        let runtime_name_impl = (!matches!(members, MemberSelection::Shell)).then(|| {
            quote! {
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
                    projection,
                )
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let implementation = if members.emits_implementation(projection) {
            let vtable = self.write_vtable(values, namespace, layout, members)?;
            quote! {
                pub trait #impl_name #type_arguments: #trait_bases #generic_where {
                    #(#impl_methods)*
                }
                #vtable
            }
        } else {
            self.write_vtable_struct(values, namespace, layout, &vtbl_name, members)?
        };
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
            #runtime_name_impl
            #implementation
        })
    }

    fn write_vtable(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        members: &MemberSelection,
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
        let initializers = self.abi_methods(members).map(|method| {
            let name = tokens::ident(&method.name);
            quote! { #name: #name::<#(#generic_names,)* Identity, OFFSET>, }
        });
        let phantom_values = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData::<#name>, });
        let vtable = self.write_vtable_struct(values, namespace, layout, &vtbl_name, members)?;
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
        members: &MemberSelection,
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
            .abi_methods(members)
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
                metadata_name: metadata_name.to_string(),
                method,
                event,
                public: !metadata_name.starts_with("remove_"),
            })
        })
        .collect()
}
