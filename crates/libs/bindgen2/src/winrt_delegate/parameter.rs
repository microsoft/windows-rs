use super::*;

impl Parameter {
    pub(super) fn write_impl_type(
        &self,
        values: &Values,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_impl_type_owner(values, namespace, layout, generics, None)
    }

    pub(super) fn write_impl_type_owner(
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
                    let element = if self.array_ref {
                        element.write_array_element(namespace, layout, generics)?
                    } else {
                        element.write_default(namespace, layout, generics)?
                    };
                    quote! { &[#element] }
                }
                ty if ty.package_impl_input_by_ref(layout) => quote! { &#default },
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
                    let element = element.write_array_element(namespace, layout, generics)?;
                    if self.array_ref {
                        quote! { &mut windows_core::Array<#element> }
                    } else {
                        quote! { &mut [#element] }
                    }
                }
                ty if ty.is_interface() => {
                    let name = ty.write_name(namespace, layout, generics)?;
                    quote! { windows_core::OutRef<#name> }
                }
                _ => quote! { &mut #default },
            }
        })
    }

    pub(super) fn write_public_type(
        &self,
        context: &MethodContext<'_>,
    ) -> Result<TokenStream, Error> {
        if self.input_only && context.is_minimal() && matches!(self.ty, ty::Type::String) {
            return Ok(quote! { &str });
        }
        let default = self.write_public_default(context)?;
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::Vector(element) => {
                    let element_name = element.write_array_element(
                        context.namespace,
                        context.layout,
                        context.generics,
                    )?;
                    let element = if context.is_package() && element.is_interface() {
                        quote! { Option<#element_name> }
                    } else {
                        element_name
                    };
                    quote! { &[#element] }
                }
                ty if ty.is_copyable(context.values, context.namespace)? => default,
                _ => quote! { &#default },
            }
        } else if let ty::Type::Vector(element) = &self.ty {
            let element = if self.array_ref {
                element.write_array_element(context.namespace, context.layout, context.generics)?
            } else {
                element.write_default(context.namespace, context.layout, context.generics)?
            };
            if self.array_ref {
                quote! { &mut windows_core::Array<#element> }
            } else {
                quote! { &mut [#element] }
            }
        } else {
            quote! { &mut #default }
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

    pub(super) fn write_call_argument(
        &self,
        context: &MethodContext<'_>,
    ) -> Result<TokenStream, Error> {
        let name = tokens::ident(&self.name);
        Ok(if self.input_only {
            match &self.ty {
                ty::Type::String if context.is_minimal() => {
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
                ty::Type::Named {
                    namespace,
                    name,
                    arguments,
                    ..
                } if namespace == "Windows.Foundation"
                    && name == "IReference"
                    && arguments.len() == 1 =>
                {
                    let value = tokens::ident(&format!("{}__", self.name));
                    quote! { windows_core::Param::param(#value.as_ref()).abi() }
                }
                _ if self.by_ref => quote! { &#name },
                ty if ty.is_interface() => quote! { #name.param().abi() },
                ty if ty.is_copyable(context.values, &self.name)? => quote! { #name },
                _ => quote! { core::mem::transmute_copy(#name) },
            }
        } else {
            match &self.ty {
                ty::Type::Vector(_) if self.array_ref => {
                    quote! { #name.set_abi_len(), #name as *mut _ as _ }
                }
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

    pub(super) fn write_upcall_argument(
        &self,
        values: &Values,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
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
                ty if ty.package_impl_input_by_ref(layout) => {
                    quote! { core::mem::transmute(&#name) }
                }
                ty if ty.is_primitive(values) => quote! { #name },
                ty if ty.is_interface() => quote! { core::mem::transmute_copy(&#name) },
                _ => quote! { core::mem::transmute(&#name) },
            }
        } else if matches!(self.ty, ty::Type::Vector(_)) && self.array_ref {
            quote! {
                &mut windows_core::imp::array_proxy(
                    core::mem::transmute_copy(&#name),
                    #size
                )
            }
        } else if let ty::Type::Vector(_) = &self.ty {
            quote! {
                core::slice::from_raw_parts_mut(
                    core::mem::transmute_copy(&#name),
                    #size as usize
                )
            }
        } else {
            quote! { core::mem::transmute_copy(&#name) }
        })
    }
}
