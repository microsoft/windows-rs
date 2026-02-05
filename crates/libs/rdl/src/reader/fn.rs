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

    let Some(attribute) = item
        .attrs
        .iter()
        .find(|attribute| attribute.path().is_ident("library"))
    else {
        return encoder.err(&item.sig, "`library` attribute not found");
    };

    let Ok((library, abi)) = library(attribute) else {
        return encoder.err(&attribute, "`library` attribute missing name/abi arguments");
    };

    let mut flags = metadata::PInvokeAttributes::NoMangle;

    match abi.as_str() {
        "system" => flags |= metadata::PInvokeAttributes::CallConvPlatformapi,
        "C" => flags |= metadata::PInvokeAttributes::CallConvCdecl,
        _ => return encoder.err(&attribute, "`library` abi not supported"),
    }

    encoder.output.ImplMap(method_def, flags, &name, &library);

    Ok(())
}

fn library(attr: &syn::Attribute) -> syn::Result<(String, String)> {
    let pairs: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]> =
        attr.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;

    let mut name = None;
    let mut abi = None;

    for pair in pairs {
        let key = pair
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new(pair.path.span(), "expected identifier"))?;

        let lit = match pair.value {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) => s,
            _ => {
                return Err(syn::Error::new(
                    pair.value.span(),
                    "expected string literal",
                ))
            }
        };

        if key == "name" {
            name = Some(lit.value());
        } else if key == "abi" {
            abi = Some(lit.value());
        } else {
            return Err(syn::Error::new(key.span(), "unknown argument"));
        }
    }

    let name = name.ok_or_else(|| syn::Error::new(attr.span(), "missing name"))?;
    let abi = abi.ok_or_else(|| syn::Error::new(attr.span(), "missing abi"))?;

    Ok((name, abi))
}
