use super::guid;
use super::*;

syn::custom_keyword!(delegate);

#[derive(Debug)]
pub struct Delegate {
    pub attrs: Vec<syn::Attribute>,
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Delegate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<delegate>()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self { attrs, sig })
    }
}

impl Delegate {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = encoder.output.TypeRef("System", "MulticastDelegate");

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        encoder.generics = self
            .sig
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

        let delegate = encoder.output.TypeDef(
            encoder.namespace,
            &name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        // Emit any Named attributes (defined in metadata or RDL) attached to this delegate.
        // Skip GUID derivation if an explicit GuidAttribute is already present.
        let already_has_guid = self
            .attrs
            .iter()
            .any(|attr| is_guid_attribute(encoder, attr));

        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(delegate),
            &self.attrs,
            &[],
        )?;

        for (number, name) in encoder.generics.iter().enumerate() {
            encoder.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(delegate),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        let flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

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

        let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
        let return_type = encode_return_type(encoder, &self.sig.output)?;

        // For WinRT delegates without an explicit GuidAttribute, derive the GUID from the
        // delegate name and Invoke method signature using the midlrt algorithm.
        if !already_has_guid {
            guid::derive_and_emit_guid(
                encoder.output,
                metadata::writer::HasAttribute::TypeDef(delegate),
                encoder.namespace,
                encoder.name,
                &[("Invoke", types.as_slice(), &return_type)],
            );
        }

        let signature = metadata::Signature {
            flags: Default::default(),
            return_type,
            types,
        };

        encoder
            .output
            .MethodDef("Invoke", &signature, flags, Default::default());

        for (sequence, param) in params.iter().enumerate() {
            encoder.output.Param(
                &param.name,
                (sequence + 1).try_into().unwrap(),
                param.attributes,
            );
        }

        Ok(())
    }
}

#[test]
#[should_panic(expected = "error: unexpected `self` parameter\n --> .rdl:4:25")]
fn unexpected_self() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    delegate fn Handler(&self);
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: param names must be unique\n --> .rdl:4:33")]
fn param_name_unique() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    delegate fn Handler(a: i32, a: i32);
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
