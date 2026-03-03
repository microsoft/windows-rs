use super::*;

syn::custom_keyword!(class);

#[derive(Debug)]
pub struct Class {
    pub attrs: Vec<syn::Attribute>,
    pub token: class,
    pub name: syn::Ident,
    pub extends: Option<syn::Path>,
    pub interfaces: Vec<ClassInterface>,
}

impl syn::parse::Parse for Class {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let extends = if input.parse::<syn::Token![:]>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);

        let interfaces = content
            .parse_terminated(ClassInterface::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            extends,
            interfaces,
        })
    }
}

#[derive(Debug)]
pub struct ClassInterface {
    pub attrs: Vec<syn::Attribute>,
    pub ty: syn::Path,
}

impl syn::parse::Parse for ClassInterface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let ty = input.parse()?;

        Ok(Self { attrs, ty })
    }
}

impl Class {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = if let Some(path) = &self.extends {
            let extends = encode_path(encoder, path)?;
            if let metadata::Type::Name(extends) = extends {
                encoder.output.TypeRef(&extends.namespace, &extends.name)
            } else {
                return encoder.err(&self.extends, "invalid base type");
            }
        } else {
            encoder.output.TypeRef("System", "Object")
        };

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        let class = encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        if self
            .attrs
            .iter()
            .any(|attribute| attribute.path().is_ident("activatable"))
        {
            encode_activatable(encoder, class)?;
        }

        for interface in &self.interfaces {
            if interface.attrs.iter().any(|attr| {
                let path = attr.path();
                path.is_ident("statics") || path.is_ident("activatable")
            }) {
                encode_factory(encoder, class, interface)?;
            } else {
                encode_implement(encoder, class, interface)?;
            }
        }

        Ok(())
    }
}

fn encode_implement(
    encoder: &mut Encoder,
    class: metadata::writer::TypeDef,
    interface: &ClassInterface,
) -> Result<(), Error> {
    let ty = encode_path(encoder, &interface.ty)?;

    let interface_impl = encoder.output.InterfaceImpl(class, &ty);

    for attr in &interface.attrs {
        let path = attr.path();

        if path.is_ident("default") {
            let default_attribute = metadata::writer::MemberRefParent::TypeRef(
                encoder
                    .output
                    .TypeRef("Windows.Foundation.Metadata", "DefaultAttribute"),
            );

            let default_ctor = encoder.output.MemberRef(
                ".ctor",
                &metadata::Signature {
                    flags: metadata::MethodCallAttributes::HASTHIS,
                    ..Default::default()
                },
                default_attribute,
            );

            encoder.output.Attribute(
                metadata::writer::HasAttribute::InterfaceImpl(interface_impl),
                metadata::writer::AttributeType::MemberRef(default_ctor),
                &[],
            );
        }
    }

    Ok(())
}

fn encode_factory(
    encoder: &mut Encoder,
    class: metadata::writer::TypeDef,
    interface: &ClassInterface,
) -> Result<(), Error> {
    for attr in &interface.attrs {
        let path = attr.path();
        let interface = encode_path(encoder, &interface.ty)?;

        let interface = if let metadata::Type::Name(interface) = interface {
            format!("{}.{}", interface.namespace, interface.name) // TODO: impl Display for TypeName
        } else {
            return encoder.err(attr, "invalid type name");
        };

        let activatable = if path.is_ident("activatable") {
            true
        } else if path.is_ident("statics") {
            false
        } else {
            return encoder.err(attr, "invalid class factory attribute");
        };

        let attribute = if activatable {
            encoder
                .output
                .TypeRef("Windows.Foundation.Metadata", "ActivatableAttribute")
        } else {
            encoder
                .output
                .TypeRef("Windows.Foundation.Metadata", "StaticAttribute")
        };

        let signature = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: metadata::Type::Void,
            types: vec![metadata::Type::named("System", "Type"), metadata::Type::U32],
        };

        let ctor = encoder.output.MemberRef(
            ".ctor",
            &signature,
            metadata::writer::MemberRefParent::TypeRef(attribute),
        );

        encoder.output.Attribute(
            metadata::writer::HasAttribute::TypeDef(class),
            metadata::writer::AttributeType::MemberRef(ctor),
            &[
                (String::new(), metadata::Value::Utf8(interface)),
                (String::new(), metadata::Value::U32(1)),
            ],
        );
    }

    Ok(())
}

fn encode_activatable(
    encoder: &mut Encoder,
    class: metadata::writer::TypeDef,
) -> Result<(), Error> {
    let attribute = encoder
        .output
        .TypeRef("Windows.Foundation.Metadata", "ActivatableAttribute");

    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::HASTHIS,
        return_type: metadata::Type::Void,
        types: vec![metadata::Type::U32],
    };

    let ctor = encoder.output.MemberRef(
        ".ctor",
        &signature,
        metadata::writer::MemberRefParent::TypeRef(attribute),
    );

    encoder.output.Attribute(
        metadata::writer::HasAttribute::TypeDef(class),
        metadata::writer::AttributeType::MemberRef(ctor),
        &[(String::new(), metadata::Value::U32(1))],
    );

    Ok(())
}
