use super::*;

#[derive(Debug)]
pub struct Fn {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Fn {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, sig })
    }
}

impl Fn {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Static
            | metadata::MethodAttributes::PInvokeImpl;

        let mut params = vec![];
        let mut param_names = HashSet::new();

        for arg in &self.sig.inputs {
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

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let signature = metadata::Signature {
            flags: Default::default(),
            return_type: encode_return_type(encoder, &self.sig.output)?,
            types,
        };

        let name = self.sig.ident.to_string();

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

        let Some(attribute) = self
            .attrs
            .iter()
            .find(|attribute| attribute.path().is_ident("link"))
        else {
            return encoder.err(&self.sig, "`link` attribute not found");
        };

        let Ok((library, abi)) = library(attribute) else {
            return encoder.err(attribute, "`link` attribute missing name/abi arguments");
        };

        let mut flags = metadata::PInvokeAttributes::NoMangle;

        match abi.as_str() {
            "system" => flags |= metadata::PInvokeAttributes::CallConvPlatformapi,
            "C" => flags |= metadata::PInvokeAttributes::CallConvCdecl,
            _ => return encoder.err(attribute, "`link` abi not supported"),
        }

        encoder.output.ImplMap(method_def, flags, &name, &library);

        Ok(())
    }
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

#[test]
#[should_panic(
    expected = r#"{ message: "unexpected `self` parameter", file_name: ".rdl", line: 4, column: 9 }"#
)]
fn unexpected_self() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    fn F(&self);
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(
    expected = r#"{ message: "param names must be unique", file_name: ".rdl", line: 4, column: 17 }"#
)]
fn param_name_unique() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    fn F(a: i32, a: i32);
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
