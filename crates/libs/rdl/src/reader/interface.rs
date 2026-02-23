use super::*;

pub fn encode_interface(encoder: &mut Encoder, item: &syntax::Interface) -> Result<(), Error> {
    let mut flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::Abstract
        | metadata::TypeAttributes::Interface;

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    encoder.generics = item
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

    let interface = encoder.output.TypeDef(
        encoder.namespace,
        &name,
        metadata::writer::TypeDefOrRef::default(),
        flags,
    );

    for (number, name) in encoder.generics.iter().enumerate() {
        encoder.output.GenericParam(
            name,
            metadata::writer::TypeOrMethodDef::TypeDef(interface),
            number.try_into().unwrap(),
            metadata::GenericParamAttributes::None,
        );
    }

    let flags = metadata::MethodAttributes::Public
        | metadata::MethodAttributes::HideBySig
        | metadata::MethodAttributes::Abstract
        | metadata::MethodAttributes::NewSlot
        | metadata::MethodAttributes::Virtual;

    for method in &item.methods {
        let mut params = vec![];

        for (sequence, arg) in method.sig.inputs.iter().enumerate() {
            match arg {
                syn::FnArg::Receiver(receiver) => {
                    if *receiver != syn::parse_quote! { &self } {
                        return encoder.err(receiver, "`&self` parameter not found");
                    }
                }
                syn::FnArg::Typed(pt) => {
                    if sequence == 0 {
                        // This may seems a little redundant but is consistent with Rust
                        // and leaves room for WinRT classes to model static methods.
                        return encoder.err(arg, "`&self` parameter not found");
                    }
                    params.push(param(encoder, pt)?);
                }
            }
        }

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let signature = metadata::Signature {
            flags: metadata::MethodCallAttributes::HASTHIS,
            return_type: encode_return_type(encoder, &method.sig.output)?,
            types,
        };

        encoder.output.MethodDef(
            &method.sig.ident.to_string(),
            &signature,
            flags,
            Default::default(),
        );

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
