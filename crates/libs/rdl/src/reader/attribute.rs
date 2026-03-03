use super::*;

syn::custom_keyword!(attribute);

#[derive(Debug)]
pub struct Attribute {
    pub attrs: Vec<syn::Attribute>,
    pub token: attribute,
    pub name: syn::Ident,
    pub methods: Vec<syn::TypeBareFn>,
    pub winrt: bool,
}

impl syn::parse::Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
            content.parse::<syn::Token![;]>()?;
        }

        Ok(Self {
            attrs,
            token,
            name,
            methods,
            winrt: false,
        })
    }
}

impl Attribute {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = encoder.output.TypeRef("System", "Attribute");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if self.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::SpecialName
            | metadata::MethodAttributes::RTSpecialName;

        for method in &self.methods {
            let mut params = vec![];

            for arg in method.inputs.iter() {
                params.push(bare_param(encoder, arg)?);
            }

            let types = params.iter().map(|param| param.ty.clone()).collect();

            let signature = metadata::Signature {
                flags: Default::default(),
                return_type: metadata::Type::Void,
                types,
            };

            encoder
                .output
                .MethodDef(".ctor", &signature, flags, Default::default());

            for (sequence, param) in params.iter().enumerate() {
                encoder.output.Param(
                    &param.name,
                    (sequence + 1).try_into().unwrap(),
                    param.attributes,
                );
            }
        }

        Ok(())
    }
}
