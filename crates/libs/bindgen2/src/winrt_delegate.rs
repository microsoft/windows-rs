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

struct Method {
    parameters: Vec<Parameter>,
    return_type: ty::Type,
}

struct Parameter {
    name: String,
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
        let methods = definition.methods()?.collect::<Vec<_>>();
        let [invoke] = methods.as_slice() else {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "delegate does not have one method",
            });
        };
        if invoke.name()? != "Invoke" {
            return Err(Error::InvalidType {
                name: owner.to_string(),
                message: "delegate method is not Invoke",
            });
        }
        let signature = invoke.signature()?;
        let parameter_rows = invoke.parameters_by_sequence()?;
        let parameters = signature
            .parameters
            .into_iter()
            .zip(parameter_rows.parameters())
            .enumerate()
            .map(|(position, (ty, parameter))| {
                let (name, input_only) = match parameter {
                    Some(parameter) => (
                        parameter.name()?.to_lowercase(),
                        parameter.flags()? & 0x2 == 0,
                    ),
                    None => (format!("p{position}"), true),
                };
                Ok(Parameter {
                    name,
                    input_only,
                    ty: ty::Type::lower(database, definition.entity().file(), owner, ty)?,
                })
            })
            .collect::<Result<_, Error>>()?;
        let guid =
            guid::Guid::from_definition(definition, owner)?.ok_or_else(|| Error::InvalidType {
                name: owner.to_string(),
                message: "delegate has no GUID",
            })?;

        Ok(Self {
            name,
            generics,
            guid,
            invoke: Method {
                parameters,
                return_type: ty::Type::lower(
                    database,
                    definition.entity().file(),
                    owner,
                    signature.return_type,
                )?,
            },
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

        let closure_signature =
            self.invoke
                .write_impl_signature(values, namespace, layout, &self.generics, false)?;
        let public_signature =
            self.invoke
                .write_public_signature(values, namespace, layout, &self.generics)?;
        let public_call =
            self.invoke
                .write_public_call(values, namespace, layout, &self.generics)?;
        let abi_signature =
            self.invoke
                .write_abi_signature(values, namespace, layout, &self.generics)?;
        let upcall = self
            .invoke
            .write_upcall(values, namespace, layout, &self.generics)?;
        let generic_params = public_signature.generic_params;
        let method_generics = if generic_params.is_empty() {
            quote! {}
        } else {
            quote! { <#generic_params> }
        };
        let public_parameters = public_signature.parameters;
        let where_clause = public_signature.where_clause;
        let return_type = public_signature.return_type;

        Ok(quote! {
            #definition
            impl #impl_generics #type_name {
                pub fn new<F: Fn #closure_signature + Send + 'static>(invoke: F) -> Self {
                    let com = windows_core::imp::DelegateBox::<Self, F>::new(
                        &#box_name::<#generic_list F>::VTABLE,
                        invoke
                    );
                    unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
                }
                pub fn Invoke #method_generics(
                    &self,
                    #(#public_parameters)*
                ) #return_type
                #where_clause
                {
                    #public_call
                }
            }
            #[repr(C)]
            pub struct #vtbl_name #type_arguments #generic_where {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(#abi_signature) -> windows_core::HRESULT,
                #(#named_phantom_types)*
            }
            struct #box_name<
                #generic_list
                F: Fn #closure_signature + Send + 'static
            >(
                core::marker::PhantomData<(#generic_list fn() -> F,)>,
            ) #generic_where;
            impl<
                #(#constraints,)*
                F: Fn #closure_signature + Send + 'static
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
                unsafe extern "system" fn Invoke(#abi_signature) -> windows_core::HRESULT {
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

impl Method {
    fn write_impl_signature(
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

    fn write_public_signature(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
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
                    let ty = parameter.ty.write_name(namespace, layout, generics)?;
                    generic_params.push(generic.clone());
                    constraints.push(quote! { #generic: windows_core::Param<#ty>, });
                    Ok(quote! { #name: #generic, })
                } else {
                    let ty = parameter.write_public_type(values, namespace, layout, generics)?;
                    Ok(quote! { #name: #ty, })
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let where_clause = if constraints.is_empty() {
            quote! {}
        } else {
            quote! { where #(#constraints)* }
        };
        let return_name = self.write_return_type(namespace, layout, generics)?;
        Ok(PublicSignature {
            generic_params: quote! { #(#generic_params),* },
            parameters,
            where_clause,
            return_type: quote! { -> windows_core::Result<#return_name> },
        })
    }

    fn write_public_call(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_call_argument(values))
            .collect::<Result<Vec<_>, Error>>()?;
        let return_arguments = match &self.return_type {
            ty::Type::Void => quote! {},
            ty::Type::Vector(element) => {
                let element = element.write_name(namespace, layout, generics)?;
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
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
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
            ty if ty.is_copyable(values, namespace)? => quote! {
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    #call.map(|| result__)
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

    fn write_abi_signature(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
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
                Ok(match (&parameter.ty, parameter.input_only) {
                    (ty::Type::Vector(_), true) => {
                        quote! { #size: u32, #name: *const #abi }
                    }
                    (ty::Type::Vector(_), false) => {
                        quote! { #size: u32, #name: *mut #abi }
                    }
                    (_, true) => quote! { #name: #abi },
                    (_, false) => quote! { #name: *mut #abi },
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let result = match &self.return_type {
            ty::Type::Void => quote! {},
            ty::Type::Vector(element) => {
                let abi = element.write_abi(values, namespace, layout, generics)?;
                quote! { result_size__: *mut u32, result__: *mut *mut #abi }
            }
            ty => {
                let abi = ty.write_abi(values, namespace, layout, generics)?;
                quote! { result__: *mut #abi }
            }
        };
        Ok(quote! {
            this: *mut core::ffi::c_void,
            #(#parameters,)*
            #result
        })
    }

    fn write_upcall(
        &self,
        values: &Values,
        _namespace: &str,
        _layout: Layout,
        _generics: &[String],
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_upcall_argument(values))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(match &self.return_type {
            ty::Type::Void => quote! { (this.invoke)(#(#arguments),*).into() },
            ty::Type::Vector(element) => {
                let write = if element.is_copyable(values, "delegate return")? {
                    quote! { result__.write(ok_data__); }
                } else {
                    quote! { result__.write(core::mem::transmute(ok_data__)); }
                };
                quote! {
                    match (this.invoke)(#(#arguments),*) {
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
                match (this.invoke)(#(#arguments),*) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into()
                }
            },
            _ => quote! {
                match (this.invoke)(#(#arguments),*) {
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
            ty => ty.write_default(namespace, layout, generics)?,
        })
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
        let default = self.ty.write_default(namespace, layout, generics)?;
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element = element.write_default(namespace, layout, generics)?;
                    quote! { &[#element] }
                }
                ty if ty.is_primitive(values) => default,
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

    fn write_public_type(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        let default = self.ty.write_default(namespace, layout, generics)?;
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element = element.write_default(namespace, layout, generics)?;
                    quote! { &[#element] }
                }
                ty if ty.is_copyable(values, namespace)? => default,
                _ => quote! { &#default },
            }
        } else {
            if let ty::Type::Vector(element) = &self.ty {
                let element = element.write_default(namespace, layout, generics)?;
                quote! { &mut [#element] }
            } else {
                quote! { &mut #default }
            }
        })
    }

    fn write_call_argument(&self, values: &Values) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) if element.is_copyable(values, &self.name)? => {
                    quote! { #name.len().try_into().unwrap(), #name.as_ptr() }
                }
                ty::Type::Vector(_) => quote! {
                    #name.len().try_into().unwrap(),
                    core::mem::transmute(#name.as_ptr())
                },
                ty if ty.is_interface() => quote! { #name.param().abi() },
                ty if ty.is_copyable(values, &self.name)? => quote! { #name },
                _ => quote! { core::mem::transmute_copy(#name) },
            }
        } else {
            match &self.ty {
                ty::Type::Vector(element) if element.is_copyable(values, &self.name)? => {
                    quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr() }
                }
                ty::Type::Vector(_) => quote! {
                    #name.len().try_into().unwrap(),
                    core::mem::transmute_copy(&#name)
                },
                ty if ty.is_copyable(values, &self.name)? => quote! { #name },
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

fn trim_generic_arity(name: &str) -> &str {
    name.split_once('`').map_or(name, |(name, _)| name)
}
