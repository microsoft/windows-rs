use super::*;

impl Delegate {
    pub(crate) fn lower(
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

    pub(crate) fn model_dependencies(&self) -> &BTreeSet<(String, String)> {
        self.invoke.package_dependencies()
    }

    pub(crate) fn direct_selection_dependencies(&self) -> BTreeSet<(String, String)> {
        self.invoke.selection_dependencies()
    }

    pub(crate) fn expand_package_dependencies(&mut self, graph: &winrt_dependency::ArtifactGraph) {
        self.invoke.expand_package_dependencies(graph);
    }

    pub(crate) fn write(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        explicit: bool,
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
        let vtable_type_arguments = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { <#(#generic_names,)*> }
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
        let vtable_turbofish = if generic_names.is_empty() {
            quote! {}
        } else {
            quote! { ::<#(#generic_names,)*> }
        };
        let doc_hidden = layout.is_package().then(|| quote! { #[doc(hidden)] });
        let cfg = tokens::feature_cfg(
            namespace,
            layout,
            self.model_dependencies()
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let named_phantom_types = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData<#name>, })
            .collect::<Vec<_>>();
        let named_phantom_values = generic_names
            .iter()
            .map(|name| quote! { #name: core::marker::PhantomData::<#name> })
            .collect::<Vec<_>>();
        let named_phantom_initializers = if projection.is_minimal() {
            quote! { #(#named_phantom_values),* }
        } else {
            quote! { #(#named_phantom_values,)* }
        };
        let phantom_types = generic_names
            .iter()
            .map(|name| quote! { core::marker::PhantomData<#name> })
            .collect::<Vec<_>>();
        let guid = self.guid.write_u128();
        let definition = if generic_names.is_empty() {
            quote! {
                #cfg
                windows_core::imp::define_interface!(#name, #vtbl_name, #guid);
                #cfg
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
                #cfg
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #name<#generic_list>(
                    windows_core::IUnknown,
                    #(#phantom_types),*
                ) where #(#constraints),*;
                #cfg
                unsafe impl<#(#constraints),*> windows_core::Interface for #name<#generic_list> {
                    type Vtable = #vtbl_name<#generic_list>;
                    const IID: windows_core::GUID =
                        windows_core::GUID::from_signature(
                            <Self as windows_core::RuntimeType>::SIGNATURE
                        );
                }
                #cfg
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
        let closure_bound = if projection.is_minimal() {
            quote! { Fn #closure_signature + 'static }
        } else {
            quote! { Fn #closure_signature + Send + 'static }
        };
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
            self.invoke.write_upcall_infallible(
                values,
                layout,
                quote! { (this.invoke) },
                false,
                false,
            )?
        } else {
            self.invoke
                .write_upcall(values, layout, quote! { (this.invoke) }, false)?
        };
        let generic_params = public_signature.generic_params;
        let method_generics = if generic_params.is_empty() {
            quote! {}
        } else {
            quote! { <#generic_params> }
        };
        let public_parameters = public_signature.parameters;
        let public_prelude = public_signature.prelude;
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
                    #public_prelude
                    #public_call
                }
            }
        });
        let constructor_visibility = if !projection.has_public_methods() {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        };
        let event_handler = self.name.ends_with("EventHandler") && self.name != "TypedEventHandler";
        let constructor = (!projection.is_minimal() || (explicit && !event_handler)).then(|| {
            quote! {
                #constructor_visibility fn new<F: #closure_bound>(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &#box_name::<#generic_list F>::VTABLE,
                        invoke
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
            }
        });

        let impl_block = (constructor.is_some() || invoke_method.is_some()).then(|| {
            quote! {
                #cfg
                impl #impl_generics #type_name {
                    #constructor
                    #invoke_method
                }
            }
        });
        Ok(quote! {
            #definition
            #impl_block
            #cfg
            #[repr(C)]
            #doc_hidden
            pub struct #vtbl_name #type_arguments #generic_where {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(#abi_signature) -> windows_core::HRESULT,
                #(#named_phantom_types)*
            }
            #cfg
            struct #box_name<
                #generic_list
                F: #closure_bound
            >(
                core::marker::PhantomData<(#generic_list fn() -> F,)>,
            ) #generic_where;
            #cfg
            impl<
                #(#constraints,)*
                F: #closure_bound
            > #box_name<#generic_list F> {
                const VTABLE: #vtbl_name #vtable_type_arguments = #vtbl_name #vtable_turbofish {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface:
                            windows_core::imp::DelegateBox::<#type_name, F>::QueryInterface,
                        AddRef:
                            windows_core::imp::DelegateBox::<#type_name, F>::AddRef,
                        Release:
                            windows_core::imp::DelegateBox::<#type_name, F>::Release,
                    },
                    Invoke: Self::Invoke,
                    #named_phantom_initializers
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

impl EventHandler {
    pub(super) fn bind(
        &self,
        context: &MethodContext<'_>,
        owner: &str,
    ) -> Result<EventBinding, Error> {
        let handler_type = if context.is_package() {
            self.ty.write_name_with_owner(
                context.namespace,
                context.layout,
                context.generics,
                context.owner,
            )?
        } else {
            self.ty
                .write_name(context.namespace, context.layout, context.generics)?
        };
        let signature = self.invoke.write_infallible_signature(
            context.values,
            context.namespace,
            context.layout,
            context.generics,
            context.owner,
        )?;
        let send = if context.is_minimal() {
            TokenStream::new()
        } else {
            quote! { + Send }
        };
        let name = tokens::ident(&self.name);
        let parameters = (0..self.invoke.parameters.len())
            .map(|position| tokens::ident(&format!("a{position}")))
            .collect::<Vec<_>>();
        let statement = if context.is_minimal() {
            let (box_name, arguments) = match &self.ty {
                ty::Type::Named {
                    name, arguments, ..
                } => (
                    tokens::ident(&format!("{name}Box")),
                    arguments
                        .iter()
                        .map(|argument| {
                            argument.write_name(context.namespace, context.layout, context.generics)
                        })
                        .collect::<Result<Vec<_>, Error>>()?,
                ),
                _ => {
                    return Err(Error::InvalidType {
                        name: owner.to_string(),
                        message: "event handler is not a named delegate",
                    });
                }
            };
            quote! {
                let handler: #handler_type = {
                    let com = windows_core::imp::DelegateBox::<#handler_type, F>::new(
                        &#box_name::<#(#arguments,)* F>::VTABLE,
                        #name
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                };
            }
        } else {
            quote! {
                let #name = <#handler_type>::new(move |#(#parameters),*| {
                    #name(#(#parameters),*);
                    Ok(())
                });
            }
        };
        Ok(EventBinding {
            name,
            signature,
            send,
            statement,
        })
    }

    pub(crate) fn substitute(&mut self, arguments: &[ty::Type]) {
        self.ty = self.ty.substitute(arguments);
        self.invoke.substitute(arguments);
    }
}
