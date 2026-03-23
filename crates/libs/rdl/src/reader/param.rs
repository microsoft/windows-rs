use super::*;

pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    pub attributes: metadata::ParamAttributes,
    pub attrs: Vec<syn::Attribute>,
}

fn parse_param_attributes(
    encoder: &mut Encoder,
    attrs: &[syn::Attribute],
    ty: &metadata::Type,
) -> Result<metadata::ParamAttributes, Error> {
    let mut attributes = metadata::ParamAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("output") {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return encoder.err(attr, "`output` attribute does not accept arguments");
            }
            attributes |= metadata::ParamAttributes::Out;
        } else if attr.path().is_ident("input") {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return encoder.err(attr, "`input` attribute does not accept arguments");
            }
            attributes |= metadata::ParamAttributes::In;
        } else if attr.path().is_ident("optional") {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return encoder.err(attr, "`optional` attribute does not accept arguments");
            }
            attributes |= metadata::ParamAttributes::Optional;
        }
    }

    if !attributes.contains(metadata::ParamAttributes::Out)
        && !attributes.contains(metadata::ParamAttributes::In)
        && matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..))
    {
        attributes |= metadata::ParamAttributes::Out;
    } else {
        attributes |= metadata::ParamAttributes::In;
    }

    Ok(attributes)
}

pub fn param(encoder: &mut Encoder, param: &syn::PatType) -> Result<Param, Error> {
    let syn::Pat::Ident(ref name) = *param.pat else {
        return encoder.err(param, "param name not found");
    };

    let name = name.ident.to_string();
    let ty = encode_type(encoder, &param.ty)?;
    let attributes = parse_param_attributes(encoder, &param.attrs, &ty)?;

    Ok(Param {
        name,
        ty,
        attributes,
        attrs: param.attrs.clone(),
    })
}

pub fn bare_param(encoder: &mut Encoder, param: &syn::BareFnArg) -> Result<Param, Error> {
    let Some((ref name, _)) = param.name else {
        return encoder.err(param, "param name not found");
    };

    let ty = encode_type(encoder, &param.ty)?;
    let attributes = parse_param_attributes(encoder, &param.attrs, &ty)?;

    Ok(Param {
        name: name.unraw_to_string(),
        ty,
        attributes,
        attrs: param.attrs.clone(),
    })
}

/// Collects parameters from a function signature that has no `self` parameter.
/// Returns an error if a `self` parameter is found or if parameter names are not unique.
pub fn collect_params(encoder: &mut Encoder, sig: &syn::Signature) -> Result<Vec<Param>, Error> {
    let mut params = vec![];
    let mut param_names = HashSet::new();

    for arg in &sig.inputs {
        match arg {
            syn::FnArg::Receiver(receiver) => {
                return encoder.err(receiver, "unexpected `self` parameter");
            }
            syn::FnArg::Typed(pt) => {
                let syn::Pat::Ident(ref name) = *pt.pat else {
                    return encoder.err(pt, "param name not found");
                };

                if !param_names.insert(name.ident.to_string()) {
                    return encoder.err(&name.ident, "param names must be unique");
                }

                params.push(param(encoder, pt)?);
            }
        }
    }

    Ok(params)
}
