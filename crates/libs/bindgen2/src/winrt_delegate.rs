use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;

mod delegate;
mod parameter;
mod upcall;

#[derive(Clone)]
pub(super) struct Delegate {
    name: String,
    generics: Vec<String>,
    guid: guid::Guid,
    invoke: Method,
}

#[derive(Clone)]
pub(super) struct Method {
    parameters: Vec<Parameter>,
    return_type: ty::Type,
    package_dependencies: BTreeSet<(String, String)>,
    generic_return_default: bool,
    noexcept: bool,
}

#[derive(Clone)]
pub(super) struct EventHandler {
    name: String,
    ty: ty::Type,
    invoke: Method,
}

struct EventBinding {
    name: TokenStream,
    signature: TokenStream,
    send: TokenStream,
    statement: TokenStream,
}

pub(super) struct MethodContext<'a> {
    values: &'a Values,
    namespace: &'a str,
    layout: Layout,
    projection: Projection,
    generics: &'a [String],
    owner: Option<&'a str>,
}

#[derive(Clone)]
struct Parameter {
    name: String,
    metadata_name: String,
    input_only: bool,
    by_ref: bool,
    array_ref: bool,
    ty: ty::Type,
}

struct PublicSignature {
    generic_params: TokenStream,
    parameters: Vec<TokenStream>,
    prelude: TokenStream,
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

    const fn is_package(&self) -> bool {
        self.layout.is_package()
    }

    const fn is_minimal(&self) -> bool {
        self.projection.is_minimal()
    }

    const fn has_public_methods(&self) -> bool {
        self.projection.has_public_methods()
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
            name: parameter.name.clone(),
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
                let array_ref = matches!(&ty.kind, TypeKind::ByRef(inner) if matches!(inner.kind, TypeKind::Vector(_)));
                let by_ref =
                    matches!(&ty.kind, TypeKind::ByRef(inner) if !matches!(inner.kind, TypeKind::Vector(_)));
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
                    by_ref,
                    array_ref,
                    ty: ty::Type::lower(database, file, owner, ty)?,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            parameters,
            return_type: ty::Type::lower(database, file, owner, signature.return_type)?,
            package_dependencies: BTreeSet::new(),
            generic_return_default,
            noexcept: method.find_attribute("NoExceptionAttribute")?.is_some(),
        })
    }

    pub(super) fn selection_dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        self.return_type
            .collect_value_dependencies(&mut dependencies);
        for parameter in &self.parameters {
            parameter.ty.collect_value_dependencies(&mut dependencies);
        }
        dependencies
    }

    pub(super) fn package_dependencies(&self) -> &BTreeSet<(String, String)> {
        &self.package_dependencies
    }

    pub(super) fn expand_package_dependencies(&mut self, graph: &winrt_dependency::ArtifactGraph) {
        self.package_dependencies = graph.expand(&self.selection_dependencies());
    }

    pub(super) fn write_public_method(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        receiver: TokenStream,
    ) -> Result<TokenStream, Error> {
        self.write_public_method_with(context, public_name, abi_name, receiver, quote! {})
    }

    pub(super) fn write_forwarded_public_method(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        interface: TokenStream,
    ) -> Result<TokenStream, Error> {
        let receiver = quote! { this };
        let cast = if self.noexcept {
            quote! {
                let this = &windows_core::Interface::cast::<#interface>(self).unwrap();
            }
        } else {
            quote! {
                let this = &windows_core::Interface::cast::<#interface>(self)?;
            }
        };
        self.write_public_method_with(context, public_name, abi_name, receiver, cast)
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
        let EventBinding {
            name: binding,
            signature,
            send,
            statement,
        } = handler.bind(context, &name.to_string())?;
        let visibility = if !context.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        Ok(quote! {
            #visibility fn #name<F>(&self, #binding: F) -> windows_core::Result<windows_core::EventRevoker>
            where
                F: Fn #signature #send + 'static,
            {
                #prelude
                #statement
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    let token__ = (windows_core::Interface::vtable(#receiver).#name)(
                        windows_core::Interface::as_raw(#receiver),
                        windows_core::Interface::as_raw(&#binding),
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
        let signature_prelude = signature.prelude;
        let return_type = signature.return_type;
        let where_clause = signature.where_clause;
        let visibility = if !context.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        Ok(quote! {
            #visibility fn #public_name #method_generics(&self, #(#parameters)*) #return_type
            #where_clause
            {
                #prelude
                #signature_prelude
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
        Ok(if self.noexcept {
            if matches!(self.return_type, ty::Type::Void) {
                quote! { fn #name(&self, #(#parameters),*); }
            } else {
                let return_type = self
                    .return_type
                    .write_default(namespace, layout, generics)?;
                quote! { fn #name(&self, #(#parameters),*) -> #return_type; }
            }
        } else {
            quote! {
                fn #name(&self, #(#parameters),*) -> windows_core::Result<#return_type>;
            }
        })
    }

    pub(super) fn write_static_method(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        factory_name: &str,
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
        let signature_prelude = signature.prelude;
        let where_clause = signature.where_clause;
        let return_type = signature.return_type;
        let visibility = if !context.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        Ok(quote! {
            #visibility fn #public_name #method_generics(#(#parameters)*) #return_type
            #where_clause
            {
                #signature_prelude
                Self::#factory_name(|this| #call)
            }
        })
    }

    pub(super) fn write_static_event_method(
        &self,
        context: &MethodContext<'_>,
        name: &str,
        remove_name: &str,
        factory_name: &str,
        handler: &EventHandler,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(name);
        let remove_name = tokens::ident(remove_name);
        let factory_name = tokens::ident(factory_name);
        let EventBinding {
            name: binding,
            signature,
            send,
            statement,
        } = handler.bind(context, &name.to_string())?;
        let visibility = if !context.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        Ok(quote! {
            #visibility fn #name<F>(#binding: F) -> windows_core::Result<windows_core::EventRevoker>
            where
                F: Fn #signature #send + 'static,
            {
                #statement
                Self::#factory_name(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    let token__ = (windows_core::Interface::vtable(this).#name)(
                        windows_core::Interface::as_raw(this),
                        windows_core::Interface::as_raw(&#binding),
                        &mut result__
                    ).map(|| result__)?;
                    Ok(windows_core::EventRevoker::new(
                        this.clone(),
                        token__,
                        windows_core::Interface::vtable(this).#remove_name
                    ))
                })
            }
        })
    }

    pub(super) fn write_composable_methods(
        &self,
        context: &MethodContext<'_>,
        public_name: &str,
        abi_name: &str,
        factory_name: &str,
        regular: bool,
        compose: bool,
    ) -> Result<Vec<TokenStream>, Error> {
        if self.parameters.len() < 2 {
            return Err(Error::InvalidType {
                name: abi_name.to_string(),
                message: "composable factory method has too few parameters",
            });
        }
        let ordinary = &self.parameters[..self.parameters.len() - 2];
        let mut generic_parameters = Vec::new();
        let mut constraints = Vec::new();
        let parameters = ordinary
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
                    generic_parameters.push(generic.clone());
                    constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                    return Ok(quote! { #name: #generic, });
                }
                let ty = parameter.write_public_type(context)?;
                Ok(quote! { #name: #ty, })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let arguments = ordinary
            .iter()
            .map(|parameter| parameter.write_call_argument(context))
            .collect::<Result<Vec<_>, Error>>()?;
        let regular_generics = if generic_parameters.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_parameters),*> }
        };
        let regular_where = (!constraints.is_empty()).then(|| quote! { where #(#constraints)* });
        let compose_generics = if generic_parameters.is_empty() {
            quote! { <T> }
        } else {
            quote! { <T, #(#generic_parameters),*> }
        };
        let abi_name = tokens::ident(abi_name);
        let factory_name = tokens::ident(factory_name);
        let regular_name = if public_name == "CreateInstance" {
            tokens::ident("new")
        } else {
            tokens::ident(public_name)
        };
        let compose_name = if public_name == "CreateInstance" {
            tokens::ident("compose")
        } else {
            tokens::ident(&format!("{public_name}_compose"))
        };
        let visibility = if !context.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        let mut result = Vec::new();
        if regular {
            result.push(quote! {
                #visibility fn #regular_name #regular_generics(
                    #(#parameters)*
                ) -> windows_core::Result<Self>
                #regular_where
                {
                    Self::#factory_name(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).#abi_name)(
                            windows_core::Interface::as_raw(this),
                            #(#arguments,)*
                            core::ptr::null_mut(),
                            core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
            });
        }
        if compose {
            result.push(quote! {
                #visibility fn #compose_name #compose_generics(
                    #(#parameters)* compose: T
                ) -> windows_core::Result<Self>
                where
                    T: windows_core::Compose,
                    #(#constraints)*
                {
                    Self::#factory_name(|this| unsafe {
                        let (derived__, base__) = windows_core::Compose::compose(compose);
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).#abi_name)(
                            windows_core::Interface::as_raw(this),
                            #(#arguments,)*
                            core::mem::transmute_copy(&derived__),
                            base__ as *mut _ as _,
                            &mut result__,
                        )
                        .ok()?;
                        let _ = &derived__;
                        windows_core::Type::from_abi(result__)
                    })
                }
            });
        }
        Ok(result)
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
        let mut preludes = Vec::new();
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .map(|(position, parameter)| {
                let name = tokens::ident(&parameter.name);
                if parameter.input_only
                    && let ty::Type::Named {
                        namespace,
                        name: type_name,
                        arguments,
                        ..
                    } = &parameter.ty
                    && namespace == "Windows.Foundation"
                    && type_name == "IReference"
                    && let [argument] = arguments.as_slice()
                {
                    let argument =
                        argument.write_name(context.namespace, context.layout, context.generics)?;
                    let value = tokens::ident(&format!("{}__", parameter.name));
                    preludes.push(quote! {
                        let #value = #name.map(
                            <windows_reference::IReference<#argument> as From<_>>::from
                        );
                    });
                    Ok(quote! { #name: Option<#argument>, })
                } else if parameter.input_only && parameter.ty.is_interface() {
                    let generic = tokens::ident(&format!("P{position}"));
                    let ty = parameter.ty.write_name_with_owner(
                        context.namespace,
                        context.layout,
                        context.generics,
                        context.owner,
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
        let return_name = if let (
            Some(owner),
            ty::Type::Named {
                namespace,
                name,
                value_type: false,
                ..
            },
        ) = (context.owner, &self.return_type)
            && namespace == context.namespace
            && name == owner
        {
            quote! { Self }
        } else if let ty::Type::Named {
            namespace,
            name,
            arguments,
            ..
        } = &self.return_type
            && namespace == "Windows.Foundation"
            && name == "IReference"
            && let [argument] = arguments.as_slice()
        {
            argument.write_name(context.namespace, context.layout, context.generics)?
        } else if context.is_minimal() {
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
            self.write_return_type_with_owner(
                context.namespace,
                context.layout,
                context.generics,
                context.owner,
            )?
        };
        let return_type = if self.noexcept {
            if matches!(self.return_type, ty::Type::Void) {
                quote! {}
            } else {
                let return_name = if let (
                    Some(owner),
                    ty::Type::Named {
                        namespace,
                        name,
                        value_type: false,
                        ..
                    },
                ) = (context.owner, &self.return_type)
                    && namespace == context.namespace
                    && name == owner
                {
                    quote! { Option<Self> }
                } else {
                    self.return_type.write_default(
                        context.namespace,
                        context.layout,
                        context.generics,
                    )?
                };
                quote! { -> #return_name }
            }
        } else {
            quote! { -> windows_core::Result<#return_name> }
        };
        Ok(PublicSignature {
            generic_params: quote! { #(#generic_params),* },
            parameters,
            prelude: quote! { #(#preludes)* },
            where_clause,
            return_type,
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
                let element = element.write_name_with_owner(
                    context.namespace,
                    context.layout,
                    context.generics,
                    context.owner,
                )?;
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
        if self.noexcept {
            return Ok(match &self.return_type {
                ty::Type::Void => quote! {
                    unsafe {
                        let hresult__ = #call;
                        debug_assert!(hresult__.0 == 0);
                    }
                },
                ty if ty.is_copyable(context.values, context.namespace)? => quote! {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let hresult__ = #call;
                        debug_assert!(hresult__.0 == 0);
                        result__
                    }
                },
                _ => quote! {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let hresult__ = #call;
                        debug_assert!(hresult__.0 == 0);
                        core::mem::transmute(result__)
                    }
                },
            });
        }
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
            ty::Type::String if context.is_minimal() => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.map(|| {
                        let hstring: windows_core::HSTRING = core::mem::transmute(result__);
                        hstring.to_string_lossy()
                    })
                }
            },
            ty::Type::Named {
                namespace,
                name,
                arguments,
                ..
            } if namespace == "Windows.Foundation"
                && name == "IReference"
                && let [argument] = arguments.as_slice() =>
            {
                let argument =
                    argument.write_name(context.namespace, context.layout, context.generics)?;
                quote! {
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        #call
                            .and_then(|| windows_core::Type::from_abi(result__))
                            .and_then(
                                |r__: windows_reference::IReference<#argument>| r__.Value()
                            )
                    }
                }
            }
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

    fn write_return_type(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_return_type_with_owner(namespace, layout, generics, None)
    }

    fn write_return_type_with_owner(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        owner: Option<&str>,
    ) -> Result<TokenStream, Error> {
        Ok(match &self.return_type {
            ty::Type::Void => quote! { () },
            ty::Type::Object => self
                .return_type
                .write_name_with_owner(namespace, layout, generics, owner)?,
            ty::Type::Vector(element) => {
                let element = element.write_name_with_owner(namespace, layout, generics, owner)?;
                quote! { windows_core::Array<#element> }
            }
            ty::Type::Generic(_) if !self.generic_return_default => self
                .return_type
                .write_name_with_owner(namespace, layout, generics, owner)?,
            ty::Type::Named {
                value_type: false, ..
            } => self
                .return_type
                .write_name_with_owner(namespace, layout, generics, owner)?,
            ty => ty.write_default(namespace, layout, generics)?,
        })
    }
}
