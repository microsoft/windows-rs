use super::*;

pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    pub attributes: metadata::ParamAttributes,
}

pub fn param(encoder: &mut Encoder, param: &syn::PatType) -> Result<Param, Error> {
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

pub fn bare_param(encoder: &mut Encoder, param: &syn::BareFnArg) -> Result<Param, Error> {
    let Some((ref name, _)) = param.name else {
        return encoder.err(param, "param name not found");
    };

    Ok(Param {
        name: name.unraw_to_string(),
        ty: encode_type(encoder, &param.ty)?,
        attributes: metadata::ParamAttributes::In,
    })
}
