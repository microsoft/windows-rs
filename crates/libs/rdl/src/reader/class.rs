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
    let interface = encode_path(encoder, &interface.ty)?;

    let implement = encoder.output.InterfaceImpl(class, &interface);

    // TODO: add attributes to implement

    Ok(())
}

fn encode_factory(
    encoder: &mut Encoder,
    class: metadata::writer::TypeDef,
    interface: &syntax::ClassInterface,
) -> Result<(), Error> {
    todo!()
}
