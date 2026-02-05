use super::*;

pub fn encode_fn(encoder: &mut Encoder, item: &syntax::Fn) -> Result<(), Error> {
    let flags = metadata::MethodAttributes::Public
        | metadata::MethodAttributes::HideBySig
        | metadata::MethodAttributes::Static
        | metadata::MethodAttributes::PInvokeImpl;

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

    let name = item.sig.ident.to_string();

    let method_def = encoder
        .output
        .MethodDef(&name, &signature, flags, Default::default());

    for (sequence, param) in params.iter().enumerate() {
        encoder.output.Param(
            &param.name,
            (sequence + 1).try_into().unwrap(),
            param.attributes,
        );
    }

    let flags =
        metadata::PInvokeAttributes::CallConvPlatformapi | metadata::PInvokeAttributes::NoMangle;

    encoder
        .output
        .ImplMap(method_def, flags, &name, "kernel32.dll");

    Ok(())
}
