use super::*;

syn::custom_keyword!(attribute);

#[derive(Debug)]
pub struct Attribute {
    pub attrs: Vec<syn::Attribute>,
    pub token: attribute,
    pub name: syn::Ident,
    pub methods: Vec<syn::TypeBareFn>,
    /// Named instance-field properties, declared as `name: type,` inside the attribute body.
    pub properties: Vec<(syn::Ident, syn::Type)>,
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
        let mut properties = vec![];

        while !content.is_empty() {
            if content.peek(syn::Token![fn]) {
                methods.push(content.parse()?);
                content.parse::<syn::Token![;]>()?;
            } else {
                let prop_name: syn::Ident = content.parse()?;
                content.parse::<syn::Token![:]>()?;
                let prop_ty: syn::Type = content.parse()?;
                content.parse::<syn::Token![,]>()?;
                properties.push((prop_name, prop_ty));
            }
        }

        Ok(Self {
            attrs,
            token,
            name,
            methods,
            properties,
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

        let attr_type = encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        // Emit any Named attributes (defined in metadata or RDL) attached to this attribute definition.
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(attr_type),
            &self.attrs,
            &[],
        )?;

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
                let param_id = encoder.output.Param(
                    &param.name,
                    (sequence + 1).try_into().unwrap(),
                    param.attributes,
                );

                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Param(param_id),
                    &param.attrs,
                    &[],
                )?;
            }
        }

        // Emit public instance fields for named properties (e.g. `version: u32`).
        for (prop_name, prop_ty) in &self.properties {
            let ty = encode_type(encoder, prop_ty)?;
            encoder.output.Field(
                &prop_name.to_string(),
                &ty,
                metadata::FieldAttributes::Public,
            );
        }

        Ok(())
    }
}
