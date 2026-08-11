use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

pub(super) struct Delegate {
    name: String,
    generics: Vec<String>,
    guid: guid::Guid,
    invoke: Method,
}

pub(super) struct Method {
    parameters: Vec<Parameter>,
    return_type: ty::Type,
    generic_return_default: bool,
}

pub(super) struct EventHandler {
    ty: ty::Type,
    invoke: Method,
}

pub(super) struct MethodContext<'a> {
    values: &'a Values,
    namespace: &'a str,
    layout: Layout,
    projection: Projection,
    generics: &'a [String],
    owner: Option<&'a str>,
}

struct Parameter {
    name: String,
    metadata_name: String,
    input_only: bool,
    ty: ty::Type,
}

impl Delegate {
    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        owner: &str,
    ) -> Result<Self, Error> {
        let name = trim_generic_arity(definition.name()?).to_string();
        let generics = definition
            .generic_parameters()?
            .map(|parameter| Ok(parameter.name()?.to_string()))
            .collect::<Result<Vec<_>, Error>>()?;
        let mut methods = definition.methods()?;
        let invoke = methods.find(|method| method.name().is_ok_and(|name| name == "Invoke"));
        let Some(invoke) = invoke else {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "delegate has no Invoke method",
            });
        };
        if methods.any(|method| method.name().is_ok_and(|name| name == "Invoke")) {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "delegate has multiple Invoke methods",
            });
        }
        let guid =
            guid::Guid::from_definition(definition, owner)?.ok_or_else(|| Error::InvalidType {
                name: owner.to_string(),
                message: "delegate has no GUID",
            })?;

        Ok(Self {
            name,
            generics,
            guid,
            invoke: Method::lower(database, definition.entity().file(), invoke, owner, true)?,
        })
    }

    pub(super) fn dependencies(
        database: &Database,
        definition: TypeDefinition<'_>,
        owner: &str,
    ) -> Result<BTreeSet<(String, String)>, Error> {
        let model = Self::lower(database, definition, owner)?;
        let mut dependencies = BTreeSet::new();
        model
            .invoke
            .return_type
            .collect_value_dependencies(&mut dependencies);
        for parameter in &model.invoke.parameters {
            parameter.ty.collect_value_dependencies(&mut dependencies);
        }
        dependencies.retain(|(namespace, name)| {
            !(namespace == "System" && name == "Guid")
                && !(namespace == "Windows.Foundation"
                    && (name == "HResult" || name == "EventRegistrationToken"))
        });
        Ok(dependencies)
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let vtbl_name = tokens::ident(&format!("{}_Vtbl", self.name));
        let box_name = tokens::ident(&format!("{}Box", self.name));
        let generic_names = self
            .generics
            .iter()
            .map(|name| tokens::ident(name))
            .collect::<Vec<_>>();
        let type_arguments = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_names),*> }
        };
        let type_name = quote! { #name #type_arguments };
        let constraints = generic_names
            .iter()
            .map(|name| quote! { #name: windows_core::RuntimeType + 'static })
            .collect::<Vec<_>>();
        let impl_generics = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { <#(#constraints),*> }
        };
        let generic_where = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints),* }
        };
        let generic_list = quote! { #(#generic_names,)* };
        let turbofish = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { ::<#(#generic_names),*> }
        };
        let named_phantom_types = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData<#name>, })
            .collect::<Vec<_>>();
        let named_phantom_values = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData::<#name>, })
            .collect::<Vec<_>>();
        let phantom_types = generic_names
            .iter()
            .map(|name| quote! { core::marker::PhantomData<#name> })
            .collect::<Vec<_>>();
        let guid = self.guid.write_u128();
        let definition = if generic_names.is_empty() {
            quote! {
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                impl windows_core::RuntimeType for #name {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
            }
        } else {
            let signature = proc_macro2::Literal::byte_string(
                format!("pinterface({{{}}}", self.guid).as_bytes(),
            );
            let generic_signatures = generic_names.iter().map(|name| {
                quote! { .push_slice(b";").push_other(#name::SIGNATURE) }
            });
            quote! {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name<#generic_list>(
                    windows_core::IUnknown,
                    #(#phantom_types),*
                ) where #(#constraints),*;
                unsafe impl<#(#constraints),*> windows_core::Interface for #name<#generic_list> {
                    type Vtable = #vtbl_name<#generic_list>;
                    const IID: windows_core::GUID =
                        windows_core::GUID::from_signature(
                            <Self as windows_core::RuntimeType>::SIGNATURE
                        );
                }
                impl<#(#constraints),*> windows_core::RuntimeType for #name<#generic_list> {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::new()
                            .push_slice(#signature)
                            #(#generic_signatures)*
                            .push_slice(b")");
                }
            }
        };

        let closure_signature = if projection.is_minimal() {
            self.invoke.write_infallible_signature(
                values,
                namespace,
                layout,
                &self.generics,
                None,
            )?
        } else {
            self.invoke
                .write_impl_signature(values, namespace, layout, &self.generics, false)?
        };
        let closure_bound = quote! { Fn #closure_signature + Send + 'static };
        let method_context =
            MethodContext::new(values, namespace, layout, projection, &self.generics, None);
        let public_signature = self.invoke.write_public_signature(&method_context)?;
        let public_call =
            self.invoke
                .write_public_call(&method_context, &quote! { Invoke }, &quote! { self })?;
        let abi_signature =
            self.invoke
                .write_abi_signature(values, namespace, layout, &self.generics, true)?;
        let abi_signature_named =
            self.invoke
                .write_abi_signature(values, namespace, layout, &self.generics, true)?;
        let upcall = if projection.is_minimal() {
            self.invoke
                .write_upcall_infallible(values, quote! { (this.invoke) }, false)?
        } else {
            self.invoke
                .write_upcall(values, quote! { (this.invoke) }, false)?
        };
        let generic_params = public_signature.generic_params;
        let method_generics = if generic_params.is_empty() {
            quote! {}
        } else {
            quote! { <#generic_params> }
        };
        let public_parameters = public_signature.parameters;
        let where_clause = public_signature.where_clause;
        let return_type = public_signature.return_type;
        let invoke_method = (!projection.is_minimal()).then(|| {
            quote! {
                pub fn Invoke #method_generics(
                    &self,
                    #(#public_parameters)*
                ) #return_type
                #where_clause
                {
                    #public_call
                }
            }
        });

        Ok(quote! {
            #definition
            impl #impl_generics #type_name {
                pub fn new<F: #closure_bound>(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &#box_name::<#generic_list F>::VTABLE,
                        invoke
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                #invoke_method
            }
            #[repr(C)]
            pub struct #vtbl_name #type_arguments #generic_where {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(#abi_signature) -> windows_core::HRESULT,
                #(#named_phantom_types)*
            }
            struct #box_name<
                #generic_list
                F: #closure_bound
            >(
                core::marker::PhantomData<(#generic_list fn() -> F,)>,
            ) #generic_where;
            impl<
                #(#constraints,)*
                F: #closure_bound
            > #box_name<#generic_list F> {
                const VTABLE: #vtbl_name #type_arguments = #vtbl_name #turbofish {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface:
                            windows_core::imp::DelegateBox::<#type_name, F>::QueryInterface,
                        AddRef:
                            windows_core::imp::DelegateBox::<#type_name, F>::AddRef,
                        Release:
                            windows_core::imp::DelegateBox::<#type_name, F>::Release,
                    },
                    Invoke: Self::Invoke,
                    #(#named_phantom_values)*
                };
                unsafe extern "system" fn Invoke(#abi_signature_named) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(
                            this as *mut *mut core::ffi::c_void
                                as *mut windows_core::imp::DelegateBox<#type_name, F>
                        );
                        #upcall
                    }
                }
            }
        })
    }
}

struct PublicSignature {
    generic_params: TokenStream,
    parameters: Vec<TokenStream>,
    where_clause: TokenStream,
    return_type: TokenStream,
}

impl<'a> MethodContext<'a> {
    pub(super) const fn new(
        values: &'a Values,
        namespace: &'a str,
        layout: Layout,
        projection: Projection,
        generics: &'a [String],
        owner: Option<&'a str>,
    ) -> Self {
        Self {
            values,
            namespace,
            layout,
            projection,
            generics,
            owner,
        }
    }
}

impl Method {
    pub(super) fn lower_event_handler(
        &self,
        database: &Database,
        owner: &str,
    ) -> Result<EventHandler, Error> {
        let [parameter] = self.parameters.as_slice() else {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "event add method does not have one parameter",
            });
        };
        if !parameter.input_only || !self.return_type.is_event_token() {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "event add method has an invalid signature",
            });
        }
        let ty::Type::Named {
            namespace,
            name,
            arguments,
            ..
        } = &parameter.ty
        else {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "event handler is not a named delegate",
            });
        };
        let metadata_name = if arguments.is_empty() {
            name.clone()
        } else {
            format!("{name}`{}", arguments.len())
        };
        let entity = database
            .type_definitions(namespace, &metadata_name)
            .first()
            .copied()
            .ok_or_else(|| Error::InvalidType {
                name: owner.to_string(),
                message: "event handler delegate cannot be resolved",
            })?;
        let definition = database.definition(entity).unwrap();
        if definition.category()? != TypeCategory::Delegate {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "event handler is not a delegate",
            });
        }
        let mut delegate = Delegate::lower(database, definition, owner)?;
        delegate.invoke.substitute(arguments);
        Ok(EventHandler {
            ty: parameter.ty.clone(),
            invoke: delegate.invoke,
        })
    }

    pub(super) fn substitute(&mut self, arguments: &[ty::Type]) {
        self.return_type = self.return_type.substitute(arguments);
        for parameter in &mut self.parameters {
            parameter.ty = parameter.ty.substitute(arguments);
        }
    }

    pub(super) fn lower(
        database: &Database,
        file: FileId,
        method: windows_metadata2::MethodDefinition<'_>,
        owner: &str,
        generic_return_default: bool,
    ) -> Result<Self, Error> {
        let signature = method.signature()?;
        let parameter_rows = method.parameters_by_sequence()?;
        let parameters = signature
            .parameters
            .into_iter()
            .zip(parameter_rows.parameters())
            .enumerate()
            .map(|(position, (ty, parameter))| {
                let (metadata_name, input_only) = match parameter {
                    Some(parameter) => {
                        (parameter.name()?.to_string(), parameter.flags()? & 0x2 == 0)
                    }
                    None => (format!("p{position}"), true),
                };
                let name = metadata_name.to_lowercase();
                if !ty.modifiers.is_empty() && !matches!(&ty.kind, TypeKind::ByRef(_)) {
                    return Err(Error::UnsupportedType {
                        name: owner.to_string(),
                        shape: format!("modified callable parameter {:?}", ty.kind),
                    });
                }
                let ty = match ty.kind {
                    TypeKind::ByRef(inner) => windows_metadata2::Type {
                        modifiers: Vec::new(),
                        kind: inner.kind,
                    },
                    kind => windows_metadata2::Type {
                        modifiers: Vec::new(),
                        kind,
                    },
                };
                Ok(Parameter {
                    name,
                    metadata_name,
                    input_only,
                    ty: ty::Type::lower(database, file, owner, ty)?,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            parameters,
            return_type: ty::Type::lower(database, file, owner, signature.return_type)?,
            generic_return_default,
        })
    }

    pub(super) fn dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        self.return_type
            .collect_value_dependencies(&mut dependencies);
        for parameter in &self.parameters {
            parameter.ty.collect_value_dependencies(&mut dependencies);
        }
        dependencies
    }

    pub(super) fn write_public_method(
        &self,
        context: &MethodContext<'_>,
        name: &str,
        receiver: TokenStream,
    ) -> Result<TokenStream, Error> {
        self.write_public_method_with(context, name, name, receiver, quote! {})
    }

    pub(super) fn write_forwarded_public_method(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        interface: TokenStream,
    ) -> Result<TokenStream, Error> {
        let receiver = quote! { this };
        self.write_public_method_with(
            context,
            public_name,
            abi_name,
            receiver,
            quote! {
                let this = &windows_core::Interface::cast::<#interface>(self)?;
            },
        )
    }

    pub(super) fn write_event_method(
        &self,
        context: &MethodContext<'_>,
        name: &str,
        remove_name: &str,
        handler: &EventHandler,
        receiver: TokenStream,
        prelude: TokenStream,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(name);
        let remove_name = tokens::ident(remove_name);
        let handler_type =
            handler
                .ty
                .write_name(context.namespace, context.layout, context.generics)?;
        let signature = handler.invoke.write_infallible_signature(
            context.values,
            context.namespace,
            context.layout,
            context.generics,
            context.owner,
        )?;
        let send = quote! { + Send };
        let arguments = (0..handler.invoke.parameters.len())
            .map(|position| tokens::ident(&format!("a{position}")))
            .collect::<Vec<_>>();
        let handler = if context.projection.is_minimal() {
            quote! {
                let handler = <#handler_type>::new(handler);
            }
        } else {
            quote! {
                let handler = <#handler_type>::new(move |#(#arguments),*| {
                    handler(#(#arguments),*);
                    Ok(())
                });
            }
        };
        Ok(quote! {
            pub fn #name<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
            where
                F: Fn #signature #send + 'static,
            {
                #prelude
                #handler
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    let token__ = (windows_core::Interface::vtable(#receiver).#name)(
                        windows_core::Interface::as_raw(#receiver),
                        windows_core::Interface::as_raw(&handler),
                        &mut result__
                    ).map(|| result__)?;
                    Ok(windows_core::EventRevoker::new(
                        #receiver.clone(),
                        token__,
                        windows_core::Interface::vtable(#receiver).#remove_name
                    ))
                }
            }
        })
    }

    fn write_public_method_with(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        receiver: TokenStream,
        prelude: TokenStream,
    ) -> Result<TokenStream, Error> {
        let public_name = tokens::ident(public_name);
        let abi_name = tokens::ident(abi_name);
        let signature = self.write_public_signature(context)?;
        let call = self.write_public_call(context, &abi_name, &receiver)?;
        let method_generics = if signature.generic_params.is_empty() {
            quote! {}
        } else {
            let generics = signature.generic_params;
            quote! { <#generics> }
        };
        let parameters = signature.parameters;
        let return_type = signature.return_type;
        let where_clause = signature.where_clause;
        Ok(quote! {
            pub fn #public_name #method_generics(&self, #(#parameters)*) #return_type
            #where_clause
            {
                #prelude
                #call
            }
        })
    }

    pub(super) fn write_impl_method(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        name: &str,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(name);
        let parameters = self
            .parameters
            .iter()
            .map(|parameter| {
                let name = tokens::ident(&parameter.metadata_name);
                let ty = parameter.write_impl_type(values, namespace, layout, generics)?;
                Ok(quote! { #name: #ty })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let return_type = self.write_return_type(namespace, layout, generics)?;
        Ok(quote! {
            fn #name(&self, #(#parameters),*) -> windows_core::Result<#return_type>;
        })
    }

    pub(super) fn write_static_method(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        factory_name: &str,
        class_namespace: &str,
        class_name: &str,
    ) -> Result<TokenStream, Error> {
        let public_name = tokens::ident(public_name);
        let abi_name = tokens::ident(abi_name);
        let factory_name = tokens::ident(factory_name);
        let signature = self.write_public_signature(context)?;
        let call = self.write_public_call(context, &abi_name, &quote! { this })?;
        let method_generics = if signature.generic_params.is_empty() {
            quote! {}
        } else {
            let generics = signature.generic_params;
            quote! { <#generics> }
        };
        let parameters = signature.parameters;
        let where_clause = signature.where_clause;
        let return_type = match &self.return_type {
            ty::Type::Named {
                namespace, name, ..
            } if namespace == class_namespace && name == class_name => {
                quote! { -> windows_core::Result<Self> }
            }
            _ => signature.return_type,
        };
        Ok(quote! {
            pub fn #public_name #method_generics(#(#parameters)*) #return_type
            #where_clause
            {
                Self::#factory_name(|this| #call)
            }
        })
    }

    pub(super) fn write_impl_signature(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        named: bool,
    ) -> Result<TokenStream, Error> {
        let parameters = self
            .parameters
            .iter()
            .map(|parameter| {
                let name = tokens::ident(&parameter.name);
                let ty = parameter.write_impl_type(values, namespace, layout, generics)?;
                Ok(if named {
                    quote! { #name: #ty }
                } else {
                    ty
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let return_type = self.write_return_type(namespace, layout, generics)?;
        Ok(quote! { (#(#parameters),*) -> windows_core::Result<#return_type> })
    }

    fn write_infallible_signature(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        owner: Option<&str>,
    ) -> Result<TokenStream, Error> {
        let parameters = self
            .parameters
            .iter()
            .map(|parameter| {
                parameter.write_impl_type_owner(values, namespace, layout, generics, owner)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let return_type = self.write_return_type(namespace, layout, generics)?;
        Ok(if matches!(self.return_type, ty::Type::Void) {
            quote! { (#(#parameters),*) }
        } else {
            quote! { (#(#parameters),*) -> #return_type }
        })
    }

    fn write_public_signature(
        &self,
        context: &MethodContext<'_>,
    ) -> Result<PublicSignature, Error> {
        let mut generic_params = Vec::new();
        let mut constraints = Vec::new();
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .map(|(position, parameter)| {
                let name = tokens::ident(&parameter.name);
                if parameter.input_only && parameter.ty.is_interface() {
                    let generic = tokens::ident(&format!("P{position}"));
                    let ty = parameter.ty.write_name(
                        context.namespace,
                        context.layout,
                        context.generics,
                    )?;
                    generic_params.push(generic.clone());
                    constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                    Ok(quote! { #name: #generic, })
                } else {
                    let ty = parameter.write_public_type(context)?;
                    Ok(quote! { #name: #ty, })
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let where_clause = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints)* }
        };
        let return_name = if context.projection.is_minimal() {
            if matches!(self.return_type, ty::Type::String) {
                quote! { String }
            } else if self.return_type.is_external_minimal() {
                self.return_type.write_minimal_name(
                    context.namespace,
                    context.layout,
                    context.generics,
                )?
            } else {
                self.write_return_type(context.namespace, context.layout, context.generics)?
            }
        } else {
            self.write_return_type(context.namespace, context.layout, context.generics)?
        };
        Ok(PublicSignature {
            generic_params: quote! { #(#generic_params),* },
            parameters,
            where_clause,
            return_type: quote! { -> windows_core::Result<#return_name> },
        })
    }

    pub(super) fn write_public_call(
        &self,
        context: &MethodContext<'_>,
        method: &TokenStream,
        receiver: &TokenStream,
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_call_argument(context))
            .collect::<Result<Vec<_>, Error>>()?;
        let return_arguments = match &self.return_type {
            ty::Type::Void => quote! {},
            ty::Type::Vector(element) => {
                let element =
                    element.write_name(context.namespace, context.layout, context.generics)?;
                quote! {
                    windows_core::Array::<#element>::set_abi_len(
                        core::mem::transmute(&mut result__)
                    ),
                    result__.as_mut_ptr() as *mut _ as _
                }
            }
            _ => quote! { &mut result__ },
        };
        let call = quote! {
            (windows_core::Interface::vtable(#receiver).#method)(
                windows_core::Interface::as_raw(#receiver),
                #(#arguments,)*
                #return_arguments
            )
        };
        Ok(match &self.return_type {
            ty::Type::Void => quote! { unsafe { #call.ok() } },
            ty::Type::Vector(_) => quote! {
                unsafe {
                    let mut result__ = core::mem::MaybeUninit::zeroed();
                    #call.map(|| result__.assume_init())
                }
            },
            ty if ty.is_copyable(context.values, context.namespace)? => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.map(|| result__)
                }
            },
            ty::Type::String if context.projection.is_minimal() => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.map(|| {
                        let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                        hstring.to_string_lossy()
                    })
                }
            },
            ty if ty.is_interface() => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.and_then(|| windows_core::Type::from_abi(result__))
                }
            },
            _ => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.map(|| core::mem::transmute(result__))
                }
            },
        })
    }

    pub(super) fn write_abi_signature(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        named: bool,
    ) -> Result<TokenStream, Error> {
        let parameters = self
            .parameters
            .iter()
            .map(|parameter| {
                let name = tokens::ident(&parameter.name);
                let size = tokens::ident(&format!("{}_array_size", parameter.name));
                let abi = parameter
                    .ty
                    .write_abi(values, namespace, layout, generics)?;
                Ok(match (&parameter.ty, parameter.input_only, named) {
                    (ty::Type::Vector(_), true, true) => {
                        quote! { #size: u32, #name: *const #abi }
                    }
                    (ty::Type::Vector(_), false, true) => {
                        quote! { #size: u32, #name: *mut #abi }
                    }
                    (ty::Type::Vector(_), true, false) => quote! { u32, *const #abi },
                    (ty::Type::Vector(_), false, false) => quote! { u32, *mut #abi },
                    (_, true, true) => quote! { #name: #abi },
                    (_, false, true) => quote! { #name: *mut #abi },
                    (_, true, false) => quote! { #abi },
                    (_, false, false) => quote! { *mut #abi },
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let result = match (&self.return_type, named) {
            (ty::Type::Void, _) => quote! {},
            (ty::Type::Vector(element), true) => {
                let abi = element.write_abi(values, namespace, layout, generics)?;
                quote! { result_size__: *mut u32, result__: *mut *mut #abi }
            }
            (ty::Type::Vector(element), false) => {
                let abi = element.write_abi(values, namespace, layout, generics)?;
                quote! { *mut u32, *mut *mut #abi }
            }
            (ty, true) => {
                let abi = ty.write_abi(values, namespace, layout, generics)?;
                quote! { result__: *mut #abi }
            }
            (ty, false) => {
                let abi = ty.write_abi(values, namespace, layout, generics)?;
                quote! { *mut #abi }
            }
        };
        let this = if named {
            quote! { this: *mut core::ffi::c_void, }
        } else {
            quote! { *mut core::ffi::c_void, }
        };
        Ok(quote! { #this #(#parameters,)* #result })
    }

    pub(super) fn write_upcall(
        &self,
        values: &Values,
        inner: TokenStream,
        has_this: bool,
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_upcall_argument(values))
            .collect::<Result<Vec<_>, Error>>()?;
        let this = has_this.then(|| quote! { this, });
        Ok(match &self.return_type {
            ty::Type::Void => quote! { #inner(#this #(#arguments),*).into() },
            ty::Type::Vector(element) => {
                let write = if element.is_copyable(values, "delegate return")? {
                    quote! { result__.write(ok_data__); }
                } else {
                    quote! { result__.write(core::mem::transmute(ok_data__)); }
                };
                quote! {
                    match #inner(#this #(#arguments),*) {
                        Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            #write
                            result_size__.write(ok_data_len__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into()
                    }
                }
            }
            ty if ty.is_copyable(values, "delegate return")? => quote! {
                match #inner(#this #(#arguments),*) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into()
                }
            },
            _ => quote! {
                match #inner(#this #(#arguments),*) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into()
                }
            },
        })
    }

    fn write_upcall_infallible(
        &self,
        values: &Values,
        inner: TokenStream,
        has_this: bool,
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_upcall_argument(values))
            .collect::<Result<Vec<_>, Error>>()?;
        let this = has_this.then(|| quote! { this, });
        Ok(match &self.return_type {
            ty::Type::Void => quote! {
                #inner(#this #(#arguments),*);
                windows_core::HRESULT(0)
            },
            ty::Type::Vector(element) => {
                let write = if element.is_copyable(values, "delegate return")? {
                    quote! { result__.write(ok_data__); }
                } else {
                    quote! { result__.write(core::mem::transmute(ok_data__)); }
                };
                quote! {
                    let (ok_data__, ok_data_len__) =
                        #inner(#this #(#arguments),*).into_abi();
                    #write
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
            }
            ty if ty.is_copyable(values, "delegate return")? => quote! {
                result__.write(#inner(#this #(#arguments),*));
                windows_core::HRESULT(0)
            },
            _ => quote! {
                let ok__ = #inner(#this #(#arguments),*);
                result__.write(core::mem::transmute_copy(&ok__));
                core::mem::forget(ok__);
                windows_core::HRESULT(0)
            },
        })
    }

    fn write_return_type(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        Ok(match &self.return_type {
            ty::Type::Void => quote! { () },
            ty::Type::Vector(element) => {
                let element = element.write_name(namespace, layout, generics)?;
                quote! { windows_core::Array<#element> }
            }
            ty::Type::Generic(_) if !self.generic_return_default => {
                self.return_type.write_name(namespace, layout, generics)?
            }
            ty::Type::Named {
                value_type: false, ..
            } => self.return_type.write_name(namespace, layout, generics)?,
            ty => ty.write_default(namespace, layout, generics)?,
        })
    }
}

impl EventHandler {
    pub(super) fn substitute(&mut self, arguments: &[ty::Type]) {
        self.ty = self.ty.substitute(arguments);
        self.invoke.substitute(arguments);
    }
}

impl Parameter {
    fn write_impl_type(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_impl_type_owner(values, namespace, layout, generics, None)
    }

    fn write_impl_type_owner(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        owner: Option<&str>,
    ) -> Result<TokenStream, Error> {
        let default = self.ty.write_default(namespace, layout, generics)?;
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element = element.write_default(namespace, layout, generics)?;
                    quote! { &[#element] }
                }
                ty if ty.is_primitive(values) => default,
                ty::Type::Named {
                    namespace: target,
                    name,
                    value_type: false,
                    ..
                } if owner.is_some_and(|owner| target == namespace && name == owner) => {
                    quote! { windows_core::Ref<Self> }
                }
                ty if ty.is_interface() => {
                    let name = ty.write_name(namespace, layout, generics)?;
                    quote! { windows_core::Ref<#name> }
                }
                _ => quote! { &#default },
            }
        } else {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element = element.write_default(namespace, layout, generics)?;
                    quote! { &mut [#element] }
                }
                ty if ty.is_interface() => {
                    let name = ty.write_name(namespace, layout, generics)?;
                    quote! { windows_core::OutRef<#name> }
                }
                _ => quote! { &mut #default },
            }
        })
    }

    fn write_public_type(&self, context: &MethodContext<'_>) -> Result<TokenStream, Error> {
        if self.input_only && context.projection.is_minimal() && matches!(self.ty, ty::Type::String)
        {
            return Ok(quote! { &str });
        }
        let default = self.write_public_default(context)?;
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element = element.write_default(
                        context.namespace,
                        context.layout,
                        context.generics,
                    )?;
                    quote! { &[#element] }
                }
                ty if ty.is_copyable(context.values, context.namespace)? => default,
                _ => quote! { &#default },
            }
        } else {
            if let ty::Type::Vector(element) = &self.ty {
                let element =
                    element.write_default(context.namespace, context.layout, context.generics)?;
                quote! { &mut [#element] }
            } else {
                quote! { &mut #default }
            }
        })
    }

    fn write_public_default(&self, context: &MethodContext<'_>) -> Result<TokenStream, Error> {
        if let (
            Some(owner),
            ty::Type::Named {
                namespace,
                name,
                value_type,
                ..
            },
        ) = (context.owner, &self.ty)
            && namespace == context.namespace
            && name == owner
        {
            return Ok(if *value_type {
                quote! { Self }
            } else {
                quote! { Option<Self> }
            });
        }
        self.ty
            .write_default(context.namespace, context.layout, context.generics)
    }

    fn write_call_argument(&self, context: &MethodContext<'_>) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::String if context.projection.is_minimal() => {
                    quote! {
                        core::mem::transmute_copy(&windows_core::HSTRING::from(#name))
                    }
                }
                ty::Type::Vector(element) if element.is_copyable(context.values, &self.name)? => {
                    quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                }
                ty::Type::Vector(_) => quote! {
                    #name.len().try_into().unwrap(),
                    core::mem::transmute(#name.as_ptr())
                },
                ty if ty.is_interface() => quote! { #name.param().abi() },
                ty if ty.is_copyable(context.values, &self.name)? => quote! { #name },
                _ => quote! { core::mem::transmute_copy(#name) },
            }
        } else {
            match &self.ty {
                ty::Type::Vector(element) if element.is_copyable(context.values, &self.name)? => {
                    quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr() }
                }
                ty::Type::Vector(_) => quote! {
                    #name.len().try_into().unwrap(),
                    core::mem::transmute_copy(&#name)
                },
                ty if ty.is_copyable(context.values, &self.name)? => quote! { #name },
                _ => quote! { #name as *mut _ as _ },
            }
        })
    }

    fn write_upcall_argument(&self, values: &Values) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        let size = tokens::ident(&format!("{}_array_size", self.name));
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(_) => quote! {
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&#name),
                        #size as usize
                    )
                },
                ty if ty.is_primitive(values) => quote! { #name },
                ty if ty.is_interface() => quote! { core::mem::transmute_copy(&#name) },
                _ => quote! { core::mem::transmute(&#name) },
            }
        } else {
            if let ty::Type::Vector(_) = &self.ty {
                quote! {
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&#name),
                        #size as usize
                    )
                }
            } else {
                quote! { core::mem::transmute_copy(&#name) }
            }
        })
    }
}
