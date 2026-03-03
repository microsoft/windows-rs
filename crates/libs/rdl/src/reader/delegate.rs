use super::*;

syn::custom_keyword!(delegate);

#[derive(Debug)]
pub struct Delegate {
    pub attrs: Vec<syn::Attribute>,
    pub token: delegate,
    pub sig: syn::Signature,
    pub winrt: bool,
}

impl syn::parse::Parse for Delegate {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            token,
            sig,
            winrt: false,
        })
    }
}

impl Delegate {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = encoder.output.TypeRef("System", "MulticastDelegate");

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        if self.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

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

        for arg in &self.sig.inputs {
            if let syn::FnArg::Typed(pt) = arg {
                params.push(param(encoder, pt)?);
            }
        }

        let types = params.iter().map(|param| param.ty.clone()).collect();

        let signature = metadata::Signature {
            flags: Default::default(),
            return_type: encode_return_type(encoder, &self.sig.output)?,
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

    pub fn validate(
        &self,
        source_file: &str,
        _index: &Index,
        _reference: &metadata::reader::TypeIndex,
    ) -> Result<(), Error> {
        let mut param_names = HashSet::new();

        for arg in &self.sig.inputs {
            match arg {
                syn::FnArg::Receiver(receiver) => {
                    return err(receiver, source_file, "unexpected `self` parameter");
                }
                syn::FnArg::Typed(pt) => {
                    let syn::Pat::Ident(ref name) = *pt.pat else {
                        return err(pt, source_file, "param name not found");
                    };

                    if !param_names.insert(name.ident.to_string()) {
                        return err(&name.ident, source_file, "param names must be unique");
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
#[should_panic(
    expected = r#"{ message: "unexpected `self` parameter", file_name: ".rdl", line: 4, column: 24 }"#
)]
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
#[should_panic(
    expected = r#"{ message: "param names must be unique", file_name: ".rdl", line: 4, column: 32 }"#
)]
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
