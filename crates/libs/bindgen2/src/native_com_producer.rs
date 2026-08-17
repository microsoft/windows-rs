use super::*;
use native_com::ReturnKind;
use proc_macro2::TokenStream;
use quote::quote;

impl native_signature::Signature {
    pub(super) fn write_impl_method(
        &self,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        name: &str,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind(layout.is_package());
        if let ReturnKind::Query { guid, object, .. } = return_kind {
            let parameters = self
                .parameters
                .iter()
                .enumerate()
                .map(|(position, parameter)| {
                    self.write_impl_parameter(
                        parameter,
                        namespace,
                        layout,
                        projection,
                        position == guid || position == object,
                    )
                });
            return Ok(quote! {
                fn #method(&self, #(#parameters)*) -> windows_core::Result<()>;
            });
        }
        let retval = match return_kind {
            ReturnKind::Retval { position, ty, .. } => Some((position, ty)),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::VoidInterface { .. }
            | ReturnKind::VoidValue { .. }
            | ReturnKind::Direct { .. }
            | ReturnKind::Indirect(_) => None,
            ReturnKind::Query { .. } => unreachable!(),
        };
        let parameters = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| retval.is_none_or(|(retval, _)| *position != retval))
            .map(|(_, parameter)| {
                self.write_impl_parameter(parameter, namespace, layout, projection, false)
            });
        let result = match return_kind {
            ReturnKind::Retval { ty, .. } => {
                let ty = ty.write_public(namespace, layout);
                quote! { -> windows_core::Result<#ty> }
            }
            ReturnKind::HResult => quote! { -> windows_core::Result<()> },
            ReturnKind::Void | ReturnKind::VoidInterface { .. } | ReturnKind::VoidValue { .. } => {
                quote! {}
            }
            ReturnKind::Direct { ty, interface } => {
                let public = ty.write_public(namespace, layout);
                if interface {
                    quote! { -> Option<#public> }
                } else {
                    quote! { -> #public }
                }
            }
            ReturnKind::Indirect(ty) => {
                let ty = ty.write_public(namespace, layout);
                quote! { -> #ty }
            }
            _ => unreachable!(),
        };
        Ok(quote! {
            fn #method(&self, #(#parameters)*) #result;
        })
    }

    fn write_impl_parameter(
        &self,
        parameter: &native_signature::Parameter,
        namespace: &str,
        layout: Layout,
        projection: Projection,
        abi: bool,
    ) -> TokenStream {
        let name = tokens::ident(&parameter.name);
        if abi {
            let ty = parameter
                .ty
                .write_abi_projection(namespace, layout, projection);
            return quote! { #name: #ty, };
        }
        let ty = parameter.ty.write_public(namespace, layout);
        match parameter.producer_plan() {
            native_signature::ProducerPlan::DirectInterfaceOutput { mutable } => {
                let (_, interface) = parameter.ty.interface_out().unwrap();
                let interface = interface.write_public(namespace, layout);
                if mutable {
                    quote! { #name: windows_core::OutRef<#interface>, }
                } else {
                    quote! { #name: *const Option<#interface>, }
                }
            }
            native_signature::ProducerPlan::InterfacePointer => {
                let ty = parameter
                    .ty
                    .write_interface_pointer(namespace, layout, None)
                    .unwrap();
                quote! { #name: #ty, }
            }
            native_signature::ProducerPlan::InterfaceOutput => {
                quote! { #name: windows_core::OutRef<#ty>, }
            }
            native_signature::ProducerPlan::InterfaceInput => {
                quote! { #name: windows_core::Ref<#ty>, }
            }
            native_signature::ProducerPlan::ByRef => quote! { #name: &#ty, },
            native_signature::ProducerPlan::Array => {
                let ty = parameter.ty.write_public_pointer(namespace, layout);
                quote! { #name: #ty, }
            }
            native_signature::ProducerPlan::Plain => quote! { #name: #ty, },
        }
    }

    pub(super) fn write_impl_upcall(
        &self,
        impl_name: &TokenStream,
        name: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let method = tokens::ident(name);
        let return_kind = self.return_kind(layout.is_package());
        let retval_position = match return_kind {
            ReturnKind::Retval { position, .. } => Some(position),
            ReturnKind::HResult
            | ReturnKind::Void
            | ReturnKind::VoidInterface { .. }
            | ReturnKind::VoidValue { .. }
            | ReturnKind::Direct { .. }
            | ReturnKind::Indirect(_)
            | ReturnKind::Query { .. } => None,
        };
        let arguments = self
            .parameters
            .iter()
            .enumerate()
            .filter(|(position, _)| retval_position != Some(*position))
            .map(|(_, parameter)| {
                let name = tokens::ident(&parameter.name);
                if matches!(
                    parameter.producer_plan(),
                    native_signature::ProducerPlan::ByRef
                        | native_signature::ProducerPlan::InterfaceOutput
                ) {
                    quote! { core::mem::transmute(&#name) }
                } else {
                    quote! { core::mem::transmute_copy(&#name) }
                }
            });
        if matches!(return_kind, ReturnKind::Direct { .. }) {
            return Ok(quote! {
                #impl_name::#method(this, #(#arguments),*)
            });
        }
        if matches!(return_kind, ReturnKind::Indirect(_)) {
            return Ok(quote! {
                *result__ = #impl_name::#method(this, #(#arguments),*);
            });
        }
        if matches!(
            return_kind,
            ReturnKind::Void | ReturnKind::VoidInterface { .. } | ReturnKind::VoidValue { .. }
        ) {
            return Ok(quote! { #impl_name::#method(this, #(#arguments),*); });
        }
        let ReturnKind::Retval {
            position,
            conversion,
            ..
        } = return_kind
        else {
            return Ok(quote! {
                #impl_name::#method(this, #(#arguments),*).into()
            });
        };
        let result = tokens::ident(&self.parameters[position].name);
        let write = if matches!(
            conversion,
            native_signature::ResultConversion::Transmute
                | native_signature::ResultConversion::FromAbi
        ) {
            quote! { #result.write(core::mem::transmute(ok__)); }
        } else {
            quote! { #result.write(ok__); }
        };
        Ok(quote! {
            match #impl_name::#method(this, #(#arguments),*) {
                Ok(ok__) => {
                    #write
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        })
    }
}
