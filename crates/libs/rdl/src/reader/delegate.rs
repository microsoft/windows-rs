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

impl Delegate {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = encoder.output.TypeRef("System", "MulticastDelegate");

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        encoder.generics = self
            .sig
            .generics
            .params
            .iter()
            .map(|generic| {
                if let syn::GenericParam::Type(ty) = generic {
                    Ok(ty.ident.to_string())
                } else {
                    Err(encoder.error(generic, "only type generic parameters are supported"))
                }
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut name = encoder.name.to_string();

        if !encoder.generics.is_empty() {
            name = format!("{name}`{}", encoder.generics.len());
        }

        let delegate = encoder.output.TypeDef(
            encoder.namespace,
            &name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        // Emit any Named attributes (defined in metadata or RDL) attached to this delegate.
        // Skip GUID derivation if an explicit GuidAttribute is already present.
        let already_has_guid = self
            .attrs
            .iter()
            .any(|attr| is_guid_attribute(encoder, attr));

        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(delegate),
            &self.attrs,
            &[],
        )?;

        for (number, name) in encoder.generics.iter().enumerate() {
            encoder.output.GenericParam(
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

        let params = collect_params(encoder, &self.sig)?;

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = encode_return_type(encoder, &self.sig.output)?;

        // For WinRT delegates without an explicit GuidAttribute, derive the GUID from the
        // delegate name and Invoke method signature using the midlrt algorithm.
        if !already_has_guid {
            guid::derive_and_emit_guid(
                encoder.output,
                metadata::writer::HasAttribute::TypeDef(delegate),
                encoder.namespace,
                encoder.name,
                &[("Invoke", types.as_slice(), &return_type)],
            );
        }

        let signature = metadata::Signature {
            flags: Default::default(),
            return_type,
            types,
        };

        encoder
            .output
            .MethodDef("Invoke", &signature, flags, Default::default());

        for (sequence, param) in params.iter().enumerate() {
            let param_id = encoder.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );

            encode_attrs(
                encoder,
                metadata::writer::HasAttribute::Param(param_id),
                &param.attrs,
                &["input", "output", "optional"],
            )?;
        }

        Ok(())
    }
}
