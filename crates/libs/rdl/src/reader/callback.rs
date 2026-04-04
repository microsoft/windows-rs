use super::*;

#[derive(Debug)]
pub struct Callback {
    pub attrs: Vec<syn::Attribute>,
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
    pub return_attrs: Vec<syn::Attribute>,
}

impl syn::parse::Parse for Callback {
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
    pub fn encode_callback(&mut self, item: &Callback) -> Result<(), Error> {
        let extends = self.output.TypeRef("System", "MulticastDelegate");

        let flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        let name = self.name.to_string();

        let callback = self.output.TypeDef(
            self.namespace,
            &name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(callback),
            &item.attrs,
            &[],
        )?;

        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        let params = self.collect_params(&item.sig)?;

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = self.encode_return_type(&item.sig.output)?;

        if let Some(variadic) = &item.sig.variadic {
            return self.err(
                variadic,
                "variadic parameters are not supported for callbacks",
            );
        }

        let mut abi = 1; // "system"

        if let Some(value) = &item.abi {
            abi = match value.value().as_str() {
                "system" => 1,
                "C" => 2,
                "fastcall" => 5,
                _ => return self.err(value, "callback abi not supported"),
            };
        }

        let attribute = self.output.TypeRef(
            "System.Runtime.InteropServices",
            "UnmanagedFunctionPointerAttribute",
        );

        let signature = windows_metadata::Signature {
            flags: windows_metadata::MethodCallAttributes::HASTHIS,
            return_type: windows_metadata::Type::Void,
            types: vec![windows_metadata::Type::value_named(
                "System.Runtime.InteropServices",
                "CallingConvention",
            )],
        };

        let ctor = self.output.MemberRef(
            ".ctor",
            &signature,
            windows_metadata::writer::MemberRefParent::TypeRef(attribute),
        );

        self.output.Attribute(
            metadata::writer::HasAttribute::TypeDef(callback),
            windows_metadata::writer::AttributeType::MemberRef(ctor),
            &[(String::new(), windows_metadata::Value::I32(abi))],
        );

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
