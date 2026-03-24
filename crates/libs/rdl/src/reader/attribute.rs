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

impl Encoder<'_> {
    pub fn encode_attribute(&mut self, item: &Attribute) -> Result<(), Error> {
        let extends = self.output.TypeRef("System", "Attribute");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if item.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let attr_type = self.output.TypeDef(
            self.namespace,
            self.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(attr_type),
            &item.attrs,
            &[],
        )?;

        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::SpecialName
            | metadata::MethodAttributes::RTSpecialName;

        for method in &item.methods {
            let mut params = vec![];

            for arg in method.inputs.iter() {
                params.push(self.bare_param(arg)?);
            }

            let types = params.iter().map(|param| param.ty.clone()).collect();

            let signature = metadata::Signature {
                flags: Default::default(),
                return_type: metadata::Type::Void,
                types,
            };

            self.output
                .MethodDef(".ctor", &signature, flags, Default::default());

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
        }

        for (prop_name, prop_ty) in &item.properties {
            let ty = self.encode_type(prop_ty)?;
            self.output.Field(
                &prop_name.to_string(),
                &ty,
                metadata::FieldAttributes::Public,
            );
        }

        Ok(())
    }
}
