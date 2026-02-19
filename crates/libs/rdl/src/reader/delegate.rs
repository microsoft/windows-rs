use super::*;

pub fn encode_delegate(encoder: &mut Encoder, item: &syntax::Delegate) -> Result<(), Error> {
    let extends = encoder.output.TypeRef("System", "MulticastDelegate");

    let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    let name = item.sig.ident.to_string();

    encoder.output.TypeDef(
        encoder.namespace,
        &name,
        metadata::writer::TypeDefOrRef::TypeRef(extends),
        flags,
    );

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
