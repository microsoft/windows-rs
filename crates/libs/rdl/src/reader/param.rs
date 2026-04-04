use super::*;

pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    pub attributes: metadata::ParamAttributes,
    pub attrs: Vec<syn::Attribute>,
}

impl Encoder<'_> {
    fn parse_param_attributes(
        &mut self,
        attrs: &[syn::Attribute],
        ty: &metadata::Type,
    ) -> Result<metadata::ParamAttributes, Error> {
        let mut attributes = metadata::ParamAttributes::default();

        for attr in attrs {
            if attr.path().is_ident("out") {
                if !matches!(attr.meta, syn::Meta::Path(_)) {
                    return self.err(attr, "`out` attribute does not accept arguments");
                }
                attributes |= metadata::ParamAttributes::Out;
            } else if attr.path().is_ident("r#in") {
                if !matches!(attr.meta, syn::Meta::Path(_)) {
                    return self.err(attr, "`in` attribute does not accept arguments");
                }
                attributes |= metadata::ParamAttributes::In;
            } else if attr.path().is_ident("opt") {
                if !matches!(attr.meta, syn::Meta::Path(_)) {
                    return self.err(attr, "`opt` attribute does not accept arguments");
                }
                attributes |= metadata::ParamAttributes::Optional;
            }
        }

        if !attributes.contains(metadata::ParamAttributes::Out)
            && !attributes.contains(metadata::ParamAttributes::In)
        {
            if matches!(ty, metadata::Type::RefMut(_) | metadata::Type::PtrMut(..)) {
                attributes |= metadata::ParamAttributes::Out;
            } else {
                attributes |= metadata::ParamAttributes::In;
            }
        }

        Ok(attributes)
    }

    pub fn param(&mut self, param: &syn::PatType) -> Result<Param, Error> {
        let syn::Pat::Ident(ref name) = *param.pat else {
            return self.err(param, "param name not found");
        };

        let name = name.ident.to_string();
        let ty = self.encode_type(&param.ty)?;
        let attributes = self.parse_param_attributes(&param.attrs, &ty)?;

        Ok(Param {
            name,
            ty,
            attributes,
            attrs: param.attrs.clone(),
        })
    }

    pub fn bare_param(&mut self, param: &syn::BareFnArg) -> Result<Param, Error> {
        let Some((ref name, _)) = param.name else {
            return self.err(param, "param name not found");
        };

        let ty = self.encode_type(&param.ty)?;
        let attributes = self.parse_param_attributes(&param.attrs, &ty)?;

        Ok(Param {
            name: name.unraw_to_string(),
            ty,
            attributes,
            attrs: param.attrs.clone(),
        })
    }

    pub fn encode_return_attrs(&mut self, return_attrs: &[syn::Attribute]) -> Result<(), Error> {
        if !return_attrs.is_empty() {
            let param_id = self
                .output
                .Param("", 0, metadata::ParamAttributes::default());
            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                return_attrs,
                &[],
            )?;
        }
        Ok(())
    }

    pub fn encode_params(&mut self, params: &[Param]) -> Result<(), Error> {
        for (sequence, param) in params.iter().enumerate() {
            let param_id = self.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );
            self.encode_attrs(
                metadata::writer::HasAttribute::Param(param_id),
                &param.attrs,
                &["r#in", "out", "opt"],
            )?;
        }
        Ok(())
    }

    pub fn collect_params(&mut self, sig: &syn::Signature) -> Result<Vec<Param>, Error> {
        let mut params = vec![];
        let mut param_names = HashSet::new();

        for arg in &sig.inputs {
            match arg {
                syn::FnArg::Receiver(receiver) => {
                    return self.err(receiver, "unexpected `self` parameter");
                }
                syn::FnArg::Typed(pt) => {
                    let syn::Pat::Ident(ref name) = *pt.pat else {
                        return self.err(pt, "param name not found");
                    };

                    if !param_names.insert(name.ident.to_string()) {
                        return self.err(&name.ident, "param names must be unique");
                    }

                    params.push(self.param(pt)?);
                }
            }
        }

        Ok(params)
    }
}
