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

        let sig = syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output,
        };

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

        let already_has_guid = item.attrs.iter().any(|attr| {
            self.is_guid_attribute(attr)
                || attr.path().is_ident("guid")
                || attr.path().is_ident("no_guid")
        });

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(delegate),
            &item.attrs,
            &["guid", "no_guid"],
        )?;

        // Handle pseudo-attributes: #[guid(u128)] and #[no_guid].
        for attr in &item.attrs {
            if attr.path().is_ident("guid") {
                let lit: syn::LitInt = attr
                    .parse_args()
                    .map_err(|_| self.error(attr, "`#[guid]` requires a single u128 literal"))?;
                let v = parse_guid_u128(&lit)
                    .map_err(|_| self.error(attr, "invalid u128 literal in `#[guid]`"))?;
                let (d1, d2, d3, d4) = guid::u128_to_guid(v);
                guid::emit_guid_attribute(
                    self.output,
                    metadata::writer::HasAttribute::TypeDef(delegate),
                    d1,
                    d2,
                    d3,
                    d4,
                );
            } else if attr.path().is_ident("no_guid") && !matches!(attr.meta, syn::Meta::Path(_)) {
                return self.err(attr, "`#[no_guid]` attribute does not accept arguments");
            }
        }

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

        if !item.return_attrs.is_empty() {
            let param_id = self
                .output
                .Param("", 0, metadata::ParamAttributes::default());
            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                &item.return_attrs,
                &[],
            )?;
        }

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
