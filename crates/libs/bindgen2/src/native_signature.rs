use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) struct Signature {
    pub(super) flags: u8,
    parameters: Vec<Parameter>,
    return_type: native::Type,
}

struct Parameter {
    name: String,
    ty: native::Type,
}

impl Signature {
    pub(super) fn lower(
        database: &Database,
        method: windows_metadata2::MethodDefinition<'_>,
        owner: &str,
    ) -> Result<Self, Error> {
        let MethodSignature {
            flags,
            return_type,
            parameters,
            ..
        } = method.signature()?;
        let parameter_rows = method.parameters_by_sequence()?;
        let parameters = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                let parameter = parameter_rows.parameters()[position];
                let flags = parameter
                    .map(|parameter| parameter.flags())
                    .transpose()?
                    .unwrap_or(0);
                Ok(Parameter {
                    name: parameter
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    ty: native::Type::lower_parameter(
                        database,
                        method.entity().file(),
                        owner,
                        ty,
                        flags & 0x0001 != 0 && flags & 0x0002 == 0,
                    )?,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self {
            flags,
            parameters,
            return_type: native::Type::lower(database, method.entity().file(), owner, return_type)?,
        })
    }

    pub(super) fn write_parameters(&self, namespace: &str) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter.ty.write(namespace);
            quote! { #name: #ty }
        });
        quote! { #(#parameters),* }
    }

    pub(super) fn write_vtable_parameters(&self, namespace: &str) -> TokenStream {
        let parameters = self.parameters.iter().map(|parameter| {
            let ty = parameter.ty.write(namespace);
            quote! { #ty }
        });
        quote! { *mut core::ffi::c_void #(, #parameters)* }
    }

    pub(super) fn write_result(&self, namespace: &str) -> TokenStream {
        if self.return_type == native::Type::Void {
            quote! {}
        } else {
            let ty = self.return_type.write(namespace);
            quote! { -> #ty }
        }
    }
}
