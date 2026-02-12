use super::*;

pub fn encode_class(encoder: &mut Encoder, item: &syntax::Class) -> Result<(), Error> {
    let extends = if let Some(path) = &item.extends {
        let extends = encode_path(encoder, path)?;
        if let metadata::Type::Name(extends) = extends {
            encoder.output.TypeRef(&extends.namespace, &extends.name)
        } else {
            return encoder.err(&item.extends, "invalid base type");
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

    for interface in &item.interfaces {
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

fn encode_implement(
    encoder: &mut Encoder,
    class: metadata::writer::TypeDef,
    interface: &syntax::ClassInterface,
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
                &metadata::Signature::default(),
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
    interface: &syntax::ClassInterface,
) -> Result<(), Error> {
    todo!()
}
