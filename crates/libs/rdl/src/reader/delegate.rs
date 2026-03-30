use super::guid;
use super::*;

syn::custom_keyword!(delegate);

#[derive(Debug)]
pub struct Delegate {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Delegate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<delegate>()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, sig })
    }
}

impl Encoder<'_> {
    pub fn encode_delegate(&mut self, item: &Delegate) -> Result<(), Error> {
        let extends = self.output.TypeRef("System", "MulticastDelegate");

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        self.generics = item
            .sig
            .generics
            .params
            .iter()
            .map(|generic| {
                if let syn::GenericParam::Type(ty) = generic {
                    Ok(ty.ident.to_string())
                } else {
                    Err(self.error(generic, "only type generic parameters are supported"))
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut name = self.name.to_string();

        if !self.generics.is_empty() {
            name = format!("{name}`{}", self.generics.len());
        }

        let delegate = self.output.TypeDef(
            self.namespace,
            &name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        let already_has_guid = item.attrs.iter().any(|attr| self.is_guid_attribute(attr));

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(delegate),
            &item.attrs,
            &[],
        )?;

        for (number, name) in self.generics.iter().enumerate() {
            self.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(delegate),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        let params = self.collect_params(&item.sig)?;

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = self.encode_return_type(&item.sig.output)?;

        if !already_has_guid {
            guid::derive_and_emit_guid(
                self.output,
                metadata::writer::HasAttribute::TypeDef(delegate),
                self.namespace,
                self.name,
                &[("Invoke", types.as_slice(), &return_type)],
            );
        }

        let signature = metadata::Signature {
            flags: Default::default(),
            return_type,
            types,
        };

        self.output
            .MethodDef("Invoke", &signature, flags, Default::default());

        for (sequence, param) in params.iter().enumerate() {
            let param_id = self.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );

            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                &param.attrs,
                &["r#in", "out", "opt"],
            )?;
        }

        Ok(())
    }
}
