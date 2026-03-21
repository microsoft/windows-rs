use super::*;

#[derive(Debug)]
pub struct Fn {
    pub attrs: Vec<syn::Attribute>,
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Fn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![extern]>()?;
        let abi = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, abi, sig })
    }
}

impl Fn {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Static
            | metadata::MethodAttributes::PInvokeImpl;

        let params = collect_params(encoder, &self.sig)?;

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let mut call_flags = metadata::MethodCallAttributes::default();

        if self.sig.variadic.is_some() {
            call_flags |= metadata::MethodCallAttributes::VARARG;
        }

        let signature = metadata::Signature {
            flags: call_flags,
            return_type: encode_return_type(encoder, &self.sig.output)?,
            types,
        };

        let name = self.sig.ident.to_string();

        let method_def = encoder
            .output
            .MethodDef(&name, &signature, flags, Default::default());

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
                &["input", "out", "opt"],
            )?;
        }

        let Some(attribute) = self
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("library"))
        else {
            return encoder.err(&self.sig, "`library` attribute not found");
        };

        let (library, last_error) = attribute
            .parse_args_with(
                |input: syn::parse::ParseStream| -> syn::Result<(syn::LitStr, bool)> {
                    let library: syn::LitStr = input.parse()?;
                    let mut last_error = false;
                    if input.peek(syn::Token![,]) {
                        input.parse::<syn::Token![,]>()?;
                        let ident: syn::Ident = input.parse()?;
                        if ident != "last_error" {
                            return Err(syn::Error::new(ident.span(), "unknown library option"));
                        }
                        input.parse::<syn::Token![=]>()?;
                        let value: syn::LitBool = input.parse()?;
                        last_error = value.value();
                    }
                    Ok((library, last_error))
                },
            )
            .or_else(|_| encoder.err(attribute.span(), "`library` name missing"))?;

        let mut flags = metadata::PInvokeAttributes::NoMangle;

        if last_error {
            flags |= metadata::PInvokeAttributes::SupportsLastError;
        }

        if let Some(abi) = &self.abi {
            match abi.value().as_str() {
                "system" => flags |= metadata::PInvokeAttributes::CallConvPlatformapi,
                "C" => flags |= metadata::PInvokeAttributes::CallConvCdecl,
                "fastcall" => flags |= metadata::PInvokeAttributes::CallConvFastcall,
                _ => return encoder.err(abi, "function abi not supported"),
            }
        } else {
            flags |= metadata::PInvokeAttributes::CallConvPlatformapi;
        }

        encoder
            .output
            .ImplMap(method_def, flags, &name, library.value().as_str());

        // Emit any Named attributes (defined in metadata or RDL) attached to this function.
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::MethodDef(method_def),
            &self.attrs,
            &["library"],
        )?;

        Ok(())
    }
}
