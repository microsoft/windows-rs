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

impl Enum {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let value_type = encoder.output.TypeRef("System", "Enum");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if self.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        let enum_type = encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        let Some(attribute) = self
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("repr"))
        else {
            return encoder.err(self.token, "`repr` attribute not found");
        };

        let Ok(ty) = attribute.parse_args::<syn::Path>() else {
            return encoder.err(attribute, "`repr` integer type attribute not found");
        };

        let ty = encode_path(encoder, &ty)?;

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
            return encoder.err(attribute, "`repr` must be an integer type");
        }

        // Handle the special `#[flags]` attribute by encoding it as `System.FlagsAttribute`.
        if let Some(attr) = self.attrs.iter().find(|a| a.path().is_ident("flags")) {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return encoder.err(attr, "`flags` attribute does not accept arguments");
            }

            let flags_type = encoder.output.TypeRef("System", "FlagsAttribute");
            let signature = metadata::Signature {
                flags: metadata::MethodCallAttributes::HASTHIS,
                return_type: metadata::Type::Void,
                types: vec![],
            };
            let ctor = encoder.output.MemberRef(
                ".ctor",
                &signature,
                metadata::writer::MemberRefParent::TypeRef(flags_type),
            );
            encoder.output.Attribute(
                metadata::writer::HasAttribute::TypeDef(enum_type),
                metadata::writer::AttributeType::MemberRef(ctor),
                &[],
            );
        }

        // Emit any Named attributes (defined in metadata or RDL) attached to this enum.
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(enum_type),
            &self.attrs,
            &["repr", "flags"],
        )?;

        encoder.output.Field(
            "value__",
            &ty,
            metadata::FieldAttributes::Private
                | metadata::FieldAttributes::SpecialName
                | metadata::FieldAttributes::RTSpecialName,
        );

        let type_name = metadata::Type::named(encoder.namespace, encoder.name);

        for variant in &self.variants {
            let name = variant.ident.to_string();

            let field = encoder.output.Field(
                &name,
                &type_name,
                metadata::FieldAttributes::Public
                    | metadata::FieldAttributes::Static
                    | metadata::FieldAttributes::Literal,
            );

            let Some((_, value)) = &variant.discriminant else {
                return encoder.err(variant, "variant value not found");
            };

            let value = encode_value(encoder, &ty, value)?;

            encoder
                .output
                .Constant(metadata::writer::HasConstant::Field(field), &value);
        }

        Ok(())
    }
}

#[test]
#[should_panic(
    expected = r#"{ message: "`repr` must be an integer type", file_name: ".rdl", line: 4, column: 4 }"#
)]
fn repr_must_be_integer() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    #[repr(bool)]
    enum AsyncStatus {
        Started = 0,
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "`flags` attribute does not accept arguments", file_name: ".rdl", line: 5, column: 4 }"#
)]
fn flags_with_args_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    #[repr(u32)]
    #[flags(something)]
    enum VirtualKeyModifiers {
        None = 0,
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
