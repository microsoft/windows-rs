use super::*;

pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    pub attributes: metadata::ParamAttributes,
    pub attrs: Vec<syn::Attribute>,
}

pub fn param(encoder: &mut Encoder, param: &syn::PatType) -> Result<Param, Error> {
    let syn::Pat::Ident(ref name) = *param.pat else {
        return encoder.err(param, "param name not found");
    };

    let name = name.ident.to_string();

    let ty = encode_type(encoder, &param.ty)?;

    let mut attributes = match ty {
        metadata::Type::RefMut(..) | metadata::Type::PtrMut(..) => metadata::ParamAttributes::Out,
        _ => metadata::ParamAttributes::In,
    };

    for attr in &param.attrs {
        if attr.path().is_ident("out") {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return encoder.err(attr, "`out` attribute does not accept arguments");
            }
            attributes = metadata::ParamAttributes::Out;
        }
    }

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

    Ok(Param {
        name: name.unraw_to_string(),
        ty: encode_type(encoder, &param.ty)?,
        attributes: metadata::ParamAttributes::In,
        attrs: vec![],
    })
}
