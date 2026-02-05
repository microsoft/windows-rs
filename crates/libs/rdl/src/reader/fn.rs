use super::*;

pub fn encode_fn(encoder: &mut Encoder, item: &syntax::Fn) -> Result<(), Error> {
    let flags = metadata::MethodAttributes::Public
        | metadata::MethodAttributes::HideBySig
        | metadata::MethodAttributes::Static;

    let signature = metadata::Signature {
        flags: metadata::MethodCallAttributes::default(),
        return_type: metadata::Type::Void,
        types: vec![],
    };

    encoder.output.MethodDef(
        &item.sig.ident.to_string(),
        &signature,
        flags,
        Default::default(),
    );

    Ok(())
}
