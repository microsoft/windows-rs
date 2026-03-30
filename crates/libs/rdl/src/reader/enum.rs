use super::*;

#[derive(Debug)]
pub struct Enum {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![enum],
    pub name: syn::Ident,
    pub variants: Vec<syn::Variant>,
    pub winrt: bool,
}

impl syn::parse::Parse for Enum {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let variants = content
            .parse_terminated(syn::Variant::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            variants,
            winrt: false,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_enum(&mut self, item: &Enum) -> Result<(), Error> {
        let value_type = self.output.TypeRef("System", "Enum");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if item.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let enum_type = self.output.TypeDef(
            self.namespace,
            self.name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        let Some(attribute) = item
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("repr"))
        else {
            return self.err(item.token, "`repr` attribute not found");
        };

        let Ok(ty) = attribute.parse_args::<syn::Path>() else {
            return self.err(attribute, "`repr` integer type attribute not found");
        };

        let ty = self.encode_path(&ty)?;

        if !matches!(
            ty,
            metadata::Type::I8
                | metadata::Type::U8
                | metadata::Type::I16
                | metadata::Type::U16
                | metadata::Type::I32
                | metadata::Type::U32
                | metadata::Type::I64
                | metadata::Type::U64
        ) {
            return self.err(attribute, "`repr` must be an integer type");
        }

        if let Some(attr) = item.attrs.iter().find(|a| a.path().is_ident("flags")) {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return self.err(attr, "`flags` attribute does not accept arguments");
            }

            let flags_type = self.output.TypeRef("System", "FlagsAttribute");
            let signature = metadata::Signature {
                flags: metadata::MethodCallAttributes::HASTHIS,
                return_type: metadata::Type::Void,
                types: vec![],
            };
            let ctor = self.output.MemberRef(
                ".ctor",
                &signature,
                metadata::writer::MemberRefParent::TypeRef(flags_type),
            );
            self.output.Attribute(
                metadata::writer::HasAttribute::TypeDef(enum_type),
                metadata::writer::AttributeType::MemberRef(ctor),
                &[],
            );
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(enum_type),
            &item.attrs,
            &["repr", "flags"],
        )?;

        self.output.Field(
            "value__",
            &ty,
            metadata::FieldAttributes::Private
                | metadata::FieldAttributes::SpecialName
                | metadata::FieldAttributes::RTSpecialName,
        );

        let type_name = metadata::Type::value_named(self.namespace, self.name);

        for variant in &item.variants {
            let name = variant.ident.to_string();

            let field = self.output.Field(
                &name,
                &type_name,
                metadata::FieldAttributes::Public
                    | metadata::FieldAttributes::Static
                    | metadata::FieldAttributes::Literal,
            );

            let Some((_, value)) = &variant.discriminant else {
                return self.err(variant, "variant value not found");
            };

            let value = self.encode_value(&ty, value)?;

            self.output
                .Constant(metadata::writer::HasConstant::Field(field), &value);
        }

        Ok(())
    }
}
