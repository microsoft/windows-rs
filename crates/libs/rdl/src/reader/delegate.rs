use super::*;

pub fn encode_delegate(encoder: &mut Encoder, item: &syntax::Delegate) -> Result<(), Error> {
    let extends = encoder.output.TypeRef("System", "MulticastDelegate");

    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    encoder.generics = item
        .sig
        .generics
        .params
        .iter()
        .map(|generic| {
            let syn::GenericParam::Type(generic) = generic else {
                todo!("syntax parsing should not allow anything else");
            };

            generic.ident.to_string()
        })
        .collect();

    let mut name = encoder.name.to_string();

    if !encoder.generics.is_empty() {
        name = format!("{name}`{}", encoder.generics.len());
    }

    let delegate = encoder.output.TypeDef(
        encoder.namespace,
        &name,
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        flags,
    );

    for (number, name) in encoder.generics.iter().enumerate() {
        encoder.output.GenericParam(
            name,
            metadata::writer::TypeOrMethodDef::TypeDef(delegate),
            number.try_into().unwrap(),
            metadata::GenericParamAttributes::None,
        );
    }

    let flags = metadata::MethodAttributes::Public
        | metadata::MethodAttributes::HideBySig
        | metadata::MethodAttributes::Abstract
        | metadata::MethodAttributes::NewSlot
        | metadata::MethodAttributes::Virtual;

    let mut params = vec![];

    for arg in &item.sig.inputs {
        match arg {
            syn::FnArg::Receiver(receiver) => {
                return encoder.err(receiver, "`unexpected `self` parameter");
            }
            syn::FnArg::Typed(pt) => {
                params.push(param(encoder, pt)?);
            }
        }
    }

    let types = params.iter().map(|param| param.ty.clone()).collect();

    let signature = metadata::Signature {
        flags: Default::default(),
        return_type: encode_return_type(encoder, &item.sig.output)?,
        types,
    };

    encoder
        .output
        .MethodDef("Invoke", &signature, flags, Default::default());

    for (sequence, param) in params.iter().enumerate() {
        encoder.output.Param(
            &param.name,
            (sequence + 1).try_into().unwrap(),
            param.attributes,
        );
    }

    Ok(())
}
