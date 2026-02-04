use super::*;

pub fn encode_interface(encoder: &mut Encoder, item: &syntax::Interface) -> Result<(), Error> {
    let mut flags = metadata::TypeAttributes::Public
        | metadata::TypeAttributes::Abstract
        | metadata::TypeAttributes::Interface;

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::default(),
        flags,
    );

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

struct Param {
    name: String,
    ty: metadata::Type,
    attributes: metadata::ParamAttributes,
}

fn param(encoder: &mut Encoder, param: &syn::PatType) -> Result<Param, Error> {
    let syn::Pat::Ident(ref name) = *param.pat else {
        return encoder.err(param, "param name not found");
    };

    let name = name.ident.to_string();

    let ty = encode_type(encoder, &param.ty)?;

    let attributes = match ty {
        metadata::Type::RefMut(..) | metadata::Type::PtrMut(..) => metadata::ParamAttributes::Out,
        _ => metadata::ParamAttributes::In,
    };

    Ok(Param {
        name,
        ty,
        attributes,
    })
}
