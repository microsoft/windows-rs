use super::guid;
use super::*;

syn::custom_keyword!(delegate);

#[derive(Debug)]
pub struct Delegate {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
    pub return_attrs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for Delegate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<delegate>()?;

        let fn_token: syn::Token![fn] = input.parse()?;
        let ident: syn::Ident = input.parse()?;
        let generics: syn::Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);
        let (inputs, variadic) = parse_fn_inputs(&content)?;

        let (output, return_attrs) = parse_return_type_with_attrs(input)?;

        input.parse::<syn::Token![;]>()?;

        let sig = make_sig(
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        );

        Ok(Self {
            attrs,
            sig,
            return_attrs,
        })
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

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(delegate),
            &item.attrs,
            &["guid", "no_guid"],
        )?;

        let already_has_guid = self.encode_guid_pseudo_attrs(
            metadata::writer::HasAttribute::TypeDef(delegate),
            &item.attrs,
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

        // Delegates are always WinRT – validate that all parameter and return types are WinRT.
        for arg in &item.sig.inputs {
            if let syn::FnArg::Typed(pt) = arg {
                let ty = self.encode_type(&pt.ty)?;
                self.validate_type_is_winrt(&pt.ty, &ty)?;
            }
        }

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = self.encode_return_type(&item.sig.output)?;

        if let syn::ReturnType::Type(_, return_syn_ty) = &item.sig.output {
            self.validate_type_is_winrt(return_syn_ty.as_ref(), &return_type)?;
        }

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

        self.encode_return_attrs(&item.return_attrs)?;
        self.encode_params(&params)?;

        Ok(())
    }
}
