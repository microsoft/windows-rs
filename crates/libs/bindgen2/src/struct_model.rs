use super::*;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::collections::BTreeSet;

/// An owned projected WinRT struct.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Field {
    name: String,
    ty: ty::Type,
}

impl Struct {
    pub(super) fn dependencies(&self) -> BTreeSet<(String, String)> {
        let mut dependencies = BTreeSet::new();
        for field in &self.fields {
            field.ty.collect_value_dependencies(&mut dependencies);
        }
        dependencies
    }

    pub(super) fn lower(
        database: &Database,
        definition: TypeDefinition<'_>,
        full_name: &str,
    ) -> Result<Self, Error> {
        let fields = definition
            .fields()?
            .map(|field| {
                Ok(Field {
                    name: field.name()?.to_string(),
                    ty: ty::Type::lower(
                        database,
                        definition.entity().file(),
                        full_name,
                        field.signature()?,
                    )?,
                })
            })
            .collect::<Result<_, Error>>()?;
        Ok(Self { fields })
    }

    pub(super) fn write(
        &self,
        values: &Values,
        namespace: &str,
        name: &str,
        layout: Layout,
    ) -> Result<TokenStream, Error> {
        let ident = tokens::ident(name);
        let fields = self
            .fields
            .iter()
            .map(|field| {
                let name = tokens::ident(&field.name);
                let ty = field.ty.write(namespace, layout)?;
                Ok(quote! { pub #name: #ty, })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let full_name = format!("{namespace}.{name}");
        let properties = self.properties(values, &mut BTreeSet::new(), &full_name)?;
        let derive = match (properties.copyable, properties.eq) {
            (true, true) => quote! { #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)] },
            (true, false) => quote! { #[derive(Clone, Copy, Debug, Default, PartialEq)] },
            (false, true) => quote! { #[derive(Clone, Debug, Default, Eq, PartialEq)] },
            (false, false) => quote! { #[derive(Clone, Debug, Default, PartialEq)] },
        };
        let type_kind = if properties.copyable {
            quote! { CopyType }
        } else {
            quote! { CloneType }
        };
        let signature = Literal::byte_string(
            values
                .signature(namespace, name, &mut BTreeSet::new())?
                .as_bytes(),
        );
        let runtime_name = Literal::byte_string(full_name.as_bytes());

        Ok(quote! {
            #[repr(C)]
            #derive
            pub struct #ident {
                #(#fields)*
            }
            impl windows_core::TypeKind for #ident {
                type TypeKind = windows_core::#type_kind;
            }
            impl windows_core::RuntimeType for #ident {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#signature);
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(#runtime_name);
            }
        })
    }

    pub(super) fn properties(
        &self,
        values: &Values,
        stack: &mut BTreeSet<(String, String)>,
        owner: &str,
    ) -> Result<ty::Properties, Error> {
        let mut result = ty::Properties {
            copyable: true,
            eq: true,
        };
        for field in &self.fields {
            let field = field.ty.properties(values, stack, owner)?;
            result.copyable &= field.copyable;
            result.eq &= field.eq;
        }
        Ok(result)
    }

    pub(super) fn runtime_signature(
        &self,
        values: &Values,
        namespace: &str,
        name: &str,
        stack: &mut BTreeSet<(String, String)>,
    ) -> Result<String, Error> {
        let owner = format!("{namespace}.{name}");
        let mut signature = format!("struct({owner}");
        for field in &self.fields {
            signature.push(';');
            signature.push_str(&field.ty.runtime_signature(values, stack, &owner)?);
        }
        signature.push(')');
        Ok(signature)
    }
}
