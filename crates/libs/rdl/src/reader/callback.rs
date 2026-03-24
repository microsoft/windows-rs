use super::*;

#[derive(Debug)]
pub struct Callback {
    pub attrs: Vec<syn::Attribute>,
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Callback {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<syn::Token![extern]>()?;
        let abi = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, abi, sig })
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
            types: vec![windows_metadata::Type::named(
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

        for (sequence, param) in params.iter().enumerate() {
            let param_id = self.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );

            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                &param.attrs,
                &["input", "output", "optional"],
            )?;
        }

        Ok(())
    }
}
