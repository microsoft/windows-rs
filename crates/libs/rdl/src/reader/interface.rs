use super::guid;
use super::*;

syn::custom_keyword!(interface);

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub requires: Vec<syn::Path>,
    pub methods: Vec<Method>,
    pub winrt: bool,
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;

        let requires = if input.parse::<syn::Token![:]>().is_ok() {
            let mut requires = vec![input.parse::<syn::Path>()?];
            while input.parse::<syn::Token![+]>().is_ok() {
                requires.push(input.parse::<syn::Path>()?);
            }
            requires
        } else {
            vec![]
        };

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            generics,
            requires,
            methods,
            winrt: false,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_interface(&mut self, item: &Interface) -> Result<(), Error> {
        let mut flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::Interface;

        if item.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let mut generics = Vec::with_capacity(item.generics.params.len());
        for generic in item.generics.params.iter() {
            let syn::GenericParam::Type(generic) = generic else {
                return self.err(generic, "only type generic parameters are supported");
            };
            generics.push(generic.ident.to_string());
        }
        self.generics = generics;

        let mut name = self.name.to_string();

        if !self.generics.is_empty() {
            name = format!("{name}`{}", self.generics.len());
        }

        let interface = self.output.TypeDef(
            self.namespace,
            &name,
            metadata::writer::TypeDefOrRef::default(),
            flags,
        );

        for (number, name) in self.generics.iter().enumerate() {
            self.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(interface),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        let already_has_guid = item.attrs.iter().any(|attr| {
            self.is_guid_attribute(attr)
                || attr.path().is_ident("guid")
                || attr.path().is_ident("no_guid")
        });

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(interface),
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
                    metadata::writer::HasAttribute::TypeDef(interface),
                    d1,
                    d2,
                    d3,
                    d4,
                );
            } else if attr.path().is_ident("no_guid") && !matches!(attr.meta, syn::Meta::Path(_)) {
                return self.err(attr, "`#[no_guid]` attribute does not accept arguments");
            }
        }

        if !item.winrt && item.requires.len() > 1 {
            return self.err(
                &item.requires[1],
                "non-WinRT interface can only inherit from one interface",
            );
        }

        for require in &item.requires {
            let ty = self.encode_path(require)?;
            self.output.InterfaceImpl(interface, &ty);
        }

        let base_flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        let mut method_signatures: Vec<(String, Vec<metadata::Type>, metadata::Type)> = Vec::new();

        for method in &item.methods {
            let mut params = vec![];

            if method.sig.inputs.is_empty() {
                return self.err(&method.sig.ident, "`&self` parameter not found");
            }

            for (sequence, arg) in method.sig.inputs.iter().enumerate() {
                match arg {
                    syn::FnArg::Receiver(receiver) => {
                        if *receiver != syn::parse_quote! { &self } {
                            return self.err(receiver, "`&self` parameter not found");
                        }
                    }
                    syn::FnArg::Typed(pt) => {
                        if sequence == 0 {
                            return self.err(arg, "`&self` parameter not found");
                        }
                        params.push(self.param(pt)?);
                    }
                }
            }

            let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
            let return_type = self.encode_return_type(&method.sig.output)?;

            if !already_has_guid {
                method_signatures.push((
                    method.sig.ident.to_string(),
                    types.clone(),
                    return_type.clone(),
                ));
            }

            let signature = metadata::Signature {
                flags: metadata::MethodCallAttributes::HASTHIS,
                return_type,
                types,
            };

            let mut is_special = false;
            for attr in &method.attrs {
                if attr.path().is_ident("special") {
                    if !matches!(attr.meta, syn::Meta::Path(_)) {
                        return self.err(attr, "`special` attribute does not accept arguments");
                    }
                    is_special = true;
                }
            }

            let mut flags = base_flags;
            if is_special {
                flags |= metadata::MethodAttributes::SpecialName;
            }

            let method_def = self.output.MethodDef(
                &method.sig.ident.to_string(),
                &signature,
                flags,
                Default::default(),
            );

            self.encode_attrs(
                metadata::writer::HasAttribute::MethodDef(method_def),
                &method.attrs,
                &["special"],
            )?;

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
        }

        if !already_has_guid {
            let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = method_signatures
                .iter()
                .map(|(name, types, ret)| (name.as_str(), types.as_slice(), ret))
                .collect();

            guid::derive_and_emit_guid(
                self.output,
                metadata::writer::HasAttribute::TypeDef(interface),
                self.namespace,
                self.name,
                &methods,
            );
        }

        Ok(())
    }
}
