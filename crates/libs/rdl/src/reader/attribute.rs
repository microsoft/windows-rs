use super::*;

pub fn encode_attribute(encoder: &mut Encoder, item: &syntax::Attribute) -> Result<(), Error> {
    let extends = encoder.output.TypeRef("System", "Attribute");

    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

    if item.winrt {
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

    for method in &item.methods {
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
