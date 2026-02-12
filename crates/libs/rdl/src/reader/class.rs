use super::*;

pub fn encode_class(encoder: &mut Encoder, item: &syntax::Class) -> Result<(), Error> {
    let mut implements = vec![];
    let mut factories = vec![];

    for interface in &item.interfaces {
        if interface.attrs.iter().any(|attr| {
            let path = attr.path();
            path.is_ident("statics") || path.is_ident("activatable")
        }) {
            factories.push(interface);
        } else {
            implements.push(interface);
        }
    }

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

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        flags,
    );

    // TODO: write interfaces

    Ok(())
}
