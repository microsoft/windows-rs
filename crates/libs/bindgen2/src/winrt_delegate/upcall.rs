use super::*;

impl Method {
    pub(crate) fn write_abi_signature(
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
                Ok(
                    match (
                        &parameter.ty,
                        parameter.input_only,
                        parameter.array_ref,
                        named,
                    ) {
                        (ty::Type::Vector(_), _, true, true) => {
                            quote! { #size: *mut u32, #name: *mut *mut #abi }
                        }
                        (ty::Type::Vector(_), _, true, false) => {
                            quote! { *mut u32, *mut *mut #abi }
                        }
                        (ty::Type::Vector(_), true, false, true) => {
                            quote! { #size: u32, #name: *const #abi }
                        }
                        (ty::Type::Vector(_), false, false, true) => {
                            quote! { #size: u32, #name: *mut #abi }
                        }
                        (ty::Type::Vector(_), true, false, false) => quote! { u32, *const #abi },
                        (ty::Type::Vector(_), false, false, false) => quote! { u32, *mut #abi },
                        (_, true, _, true) if parameter.by_ref => {
                            quote! { #name: &#abi }
                        }
                        (_, true, _, false) if parameter.by_ref => {
                            quote! { &#abi }
                        }
                        (_, true, _, true) => quote! { #name: #abi },
                        (_, false, _, true) => quote! { #name: *mut #abi },
                        (_, true, _, false) => quote! { #abi },
                        (_, false, _, false) => quote! { *mut #abi },
                    },
                )
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
        layout: Layout,
        inner: TokenStream,
        has_this: bool,
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_upcall_argument(values, layout))
            .collect::<Result<Vec<_>, Error>>()?;
        let this = has_this.then(|| quote! { this, });
        Ok(match &self.return_type {
            ty::Type::Void => quote! { #inner(#this #(#arguments),*).into() },
            ty::Type::Vector(_) => {
                let write = quote! { result__.write(ok_data__); };
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

    pub(crate) fn write_method_upcall(
        &self,
        values: &Values,
        layout: Layout,
        inner: TokenStream,
        has_this: bool,
    ) -> Result<TokenStream, Error> {
        if self.noexcept {
            self.write_upcall_infallible(values, layout, inner, has_this, true)
        } else {
            self.write_upcall(values, layout, inner, has_this)
        }
    }

    pub(super) fn write_upcall_infallible(
        &self,
        values: &Values,
        layout: Layout,
        inner: TokenStream,
        has_this: bool,
        bind_copy: bool,
    ) -> Result<TokenStream, Error> {
        let arguments = self
            .parameters
            .iter()
            .map(|parameter| parameter.write_upcall_argument(values, layout))
            .collect::<Result<Vec<_>, Error>>()?;
        let this = has_this.then(|| quote! { this, });
        Ok(match &self.return_type {
            ty::Type::Void => quote! {
                #inner(#this #(#arguments),*);
                windows_core::HRESULT(0)
            },
            ty::Type::Vector(_) => {
                let write = quote! { result__.write(ok_data__); };
                quote! {
                    let (ok_data__, ok_data_len__) =
                        #inner(#this #(#arguments),*).into_abi();
                    #write
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
            }
            ty if ty.is_copyable(values, "delegate return")? => {
                if bind_copy {
                    quote! {
                        let ok__ = #inner(#this #(#arguments),*);
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                } else {
                    quote! {
                        result__.write(#inner(#this #(#arguments),*));
                        windows_core::HRESULT(0)
                    }
                }
            }
            _ => quote! {
                let ok__ = #inner(#this #(#arguments),*);
                result__.write(core::mem::transmute_copy(&ok__));
                core::mem::forget(ok__);
                windows_core::HRESULT(0)
            },
        })
    }
}
