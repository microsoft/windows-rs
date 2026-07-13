use super::*;

#[derive(Debug)]
pub struct Fn {
    pub attrs: Vec<syn::Attribute>,
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
    pub return_attrs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for Fn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![extern]>()?;
        let abi: Option<syn::LitStr> = input.parse()?;

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
            abi,
            sig,
            return_attrs,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_fn(&mut self, item: &Fn) -> Result<(), Error> {
        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Static
            | metadata::MethodAttributes::PInvokeImpl;

        let params = self.collect_params(&item.sig)?;

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let mut call_flags = metadata::MethodCallAttributes::default();

        if item.sig.variadic.is_some() {
            call_flags |= metadata::MethodCallAttributes::VARARG;
        }

        let signature = metadata::Signature {
            flags: call_flags,
            return_type: self.encode_return_type(&item.sig.output)?,
            types,
        };

        let name = item.sig.ident.unraw_to_string();

        let method_def = self
            .output
            .MethodDef(&name, &signature, flags, Default::default());

        self.encode_return_attrs(&item.return_attrs)?;
        self.encode_params(&params)?;

        let Some(attribute) = item
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("library"))
        else {
            return self.err(&item.sig, "`library` attribute not found");
        };

        let (library, import) = attribute
            .parse_args_with(
                |input: syn::parse::ParseStream| -> syn::Result<(syn::LitStr, Option<String>)> {
                    let library: syn::LitStr = input.parse()?;
                    let mut import = None;
                    while input.peek(syn::Token![,]) {
                        input.parse::<syn::Token![,]>()?;
                        let ident: syn::Ident = input.parse()?;
                        input.parse::<syn::Token![=]>()?;
                        if ident == "import" {
                            let value: syn::LitStr = input.parse()?;
                            import = Some(value.value());
                        } else {
                            return Err(syn::Error::new(ident.span(), "unknown library option"));
                        }
                    }
                    Ok((library, import))
                },
            )
            .or_else(|_| self.err(attribute.span(), "`library` name missing"))?;

        let mut flags = metadata::PInvokeAttributes::NoMangle;

        if let Some(abi) = &item.abi {
            match abi.value().as_str() {
                "system" => flags |= metadata::PInvokeAttributes::CallConvPlatformapi,
                "C" => flags |= metadata::PInvokeAttributes::CallConvCdecl,
                "fastcall" => flags |= metadata::PInvokeAttributes::CallConvFastcall,
                _ => return self.err(abi, "function abi not supported"),
            }
        } else {
            flags |= metadata::PInvokeAttributes::CallConvPlatformapi;
        }

        let import_name = import.as_deref().unwrap_or(&name);
        self.output
            .ImplMap(method_def, flags, import_name, library.value().as_str());

        if let Some(arch_bits) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(
                metadata::writer::HasAttribute::MethodDef(method_def),
                arch_bits,
            );
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::MethodDef(method_def),
            &item.attrs,
            &["library"],
        )?;

        Ok(())
    }
}
