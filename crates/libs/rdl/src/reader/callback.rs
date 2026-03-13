use super::*;

#[derive(Debug)]
pub struct Callback {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![extern],
    pub abi: Option<syn::LitStr>, // "system" is default
    pub sig: syn::Signature,
}

impl syn::parse::Parse for Callback {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let abi = input.parse()?;
        let sig = input.parse()?;
        input.parse::<syn::Token![;]>()?;

        Ok(Self {
            attrs,
            token,
            abi,
            sig,
        })
    }
}

impl Callback {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let extends = encoder.output.TypeRef("System", "MulticastDelegate");

        let flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::Sealed;

        let name = encoder.name.to_string();

        let callback = encoder.output.TypeDef(
            encoder.namespace,
            &name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(callback),
            &self.attrs,
            &[],
        )?;

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

        let mut abi = 1; // "system"

        if let Some(value) = &self.abi {
            abi = match value.value().as_str() {
                "system" => 1,
                "C" => 2,
                _ => return encoder.err(abi, "callback abi not supported"),
            };
        }

        let attribute = encoder.output.TypeRef(
            "System.Runtime.InteropServices",
            "UnmanagedFunctionPointerAttribute",
        );

        let signature = windows_metadata::Signature {
            flags: windows_metadata::MethodCallAttributes::HASTHIS,
            return_type: windows_metadata::Type::Void,
            types: vec![windows_metadata::Type::named(
                "System.Runtime.InteropServices",
                "CallingConvention",
            )],
        };

        let ctor = encoder.output.MemberRef(
            ".ctor",
            &signature,
            windows_metadata::writer::MemberRefParent::TypeRef(attribute),
        );

        encoder.output.Attribute(
            metadata::writer::HasAttribute::TypeDef(callback),
            windows_metadata::writer::AttributeType::MemberRef(ctor),
            &[(String::new(), windows_metadata::Value::I32(abi))],
        );

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
