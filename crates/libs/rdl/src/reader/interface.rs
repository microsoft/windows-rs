use super::guid;
use super::*;

syn::custom_keyword!(interface);

#[derive(Debug)]
pub struct Interface {
    pub attrs: Vec<syn::Attribute>,
    pub token: interface,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub requires: Vec<syn::Path>,
    pub methods: Vec<Method>,
    pub winrt: bool,
}

impl syn::parse::Parse for Interface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;

        let requires = if input.parse::<syn::Token![:]>().is_ok() {
            let mut requires = vec![input.parse::<syn::Path>()?];
            while input.parse::<syn::Token![+]>().is_ok() {
                requires.push(input.parse::<syn::Path>()?);
            }
            requires
        } else {
            vec![]
        };

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];

        while !content.is_empty() {
            methods.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            token,
            name,
            generics,
            requires,
            methods,
            winrt: false,
        })
    }
}

impl Interface {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let mut flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::Interface;

        if self.winrt {
            flags |= metadata::TypeAttributes::WindowsRuntime;
        }

        encoder.generics = self
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

        let interface = encoder.output.TypeDef(
            encoder.namespace,
            &name,
            metadata::writer::TypeDefOrRef::default(),
            flags,
        );

        for (number, name) in encoder.generics.iter().enumerate() {
            encoder.output.GenericParam(
                name,
                metadata::writer::TypeOrMethodDef::TypeDef(interface),
                number.try_into().unwrap(),
                metadata::GenericParamAttributes::None,
            );
        }

        // Emit any Named attributes (defined in metadata or RDL) attached to this interface.
        // Skip GUID derivation if an explicit GuidAttribute is already present.
        let already_has_guid = self
            .attrs
            .iter()
            .any(|attr| is_guid_attribute(encoder, attr));

        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(interface),
            &self.attrs,
            &[],
        )?;

        if !self.winrt && self.requires.len() > 1 {
            return encoder.err(
                &self.requires[1],
                "non-WinRT interface can only inherit from one interface",
            );
        }

        for require in &self.requires {
            let ty = encode_path(encoder, require)?;
            encoder.output.InterfaceImpl(interface, &ty);
        }

        let base_flags = metadata::MethodAttributes::Public
            | metadata::MethodAttributes::HideBySig
            | metadata::MethodAttributes::Abstract
            | metadata::MethodAttributes::NewSlot
            | metadata::MethodAttributes::Virtual;

        // Collect method signatures for GUID derivation (for any interface without an explicit
        // GuidAttribute — both WinRT and Win32 interfaces benefit from this).
        let mut method_signatures: Vec<(String, Vec<metadata::Type>, metadata::Type)> = Vec::new();

        for method in &self.methods {
            let mut params = vec![];

            if method.sig.inputs.is_empty() {
                return encoder.err(&method.sig.ident, "`&self` parameter not found");
            }

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

            let types: Vec<metadata::Type> = params.iter().map(|param| param.ty.clone()).collect();
            let return_type = encode_return_type(encoder, &method.sig.output)?;

            if !already_has_guid {
                method_signatures.push((
                    method.sig.ident.to_string(),
                    types.clone(),
                    return_type.clone(),
                ));
            }

            let signature = metadata::Signature {
                flags: metadata::MethodCallAttributes::HASTHIS,
                return_type,
                types,
            };

            // Check for the built-in `#[special]` pseudo-attribute which sets the
            // SpecialName bit on the MethodDef, preserving properties and events.
            let mut is_special = false;
            for attr in &method.attrs {
                if attr.path().is_ident("special") {
                    if !matches!(attr.meta, syn::Meta::Path(_)) {
                        return encoder.err(attr, "`special` attribute does not accept arguments");
                    }
                    is_special = true;
                }
            }

            let mut flags = base_flags;
            if is_special {
                flags |= metadata::MethodAttributes::SpecialName;
            }

            let method_def = encoder.output.MethodDef(
                &method.sig.ident.to_string(),
                &signature,
                flags,
                Default::default(),
            );

            // Emit any Named attributes attached to this method.
            // `special` is a built-in pseudo-attribute and must not be emitted as metadata.
            encode_attrs(
                encoder,
                metadata::writer::HasAttribute::MethodDef(method_def),
                &method.attrs,
                &["special"],
            )?;

            for (sequence, param) in params.iter().enumerate() {
                let param_id = encoder.output.Param(
                    &param.name,
                    (sequence + 1).try_into().unwrap(),
                    param.attributes,
                );

                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Param(param_id),
                    &param.attrs,
                    &["out"],
                )?;
            }
        }

        // For interfaces without an explicit GuidAttribute (both WinRT and Win32), derive the GUID
        // from the interface name and method signatures using the midlrt algorithm (RFC 4122 UUID v5).
        if !already_has_guid {
            let methods: Vec<(&str, &[metadata::Type], &metadata::Type)> = method_signatures
                .iter()
                .map(|(name, types, ret)| (name.as_str(), types.as_slice(), ret))
                .collect();

            let interface_string =
                guid::build_interface_string(encoder.namespace, encoder.name, &methods);
            let (data1, data2, data3, data4) = guid::guid_from_interface_string(&interface_string);

            guid::emit_guid_attribute(
                encoder.output,
                metadata::writer::HasAttribute::TypeDef(interface),
                data1,
                data2,
                data3,
                data4,
            );
        }

        Ok(())
    }
}

#[test]
#[should_panic(
    expected = "error: non-WinRT interface can only inherit from one interface\n --> .rdl:4:28"
)]
fn win32_multiple_required_interfaces() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo: IBar + IBaz {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:19")]
fn missing_self_typed_first_param() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(a: i32);
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:19")]
fn missing_self_wrong_receiver() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&mut self);
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: `&self` parameter not found\n --> .rdl:5:12")]
fn missing_self_no_params() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method();
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: `out` attribute does not accept arguments\n --> .rdl:5:26")]
fn out_with_args() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo {
        fn Method(&self, #[out(42)] output: i32);
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: `special` attribute does not accept arguments\n --> .rdl:5:9")]
fn special_with_args() {
    Reader::new()
        .input_str(
            r#"
#[win32]
mod Test {
    interface IFoo {
        #[special(42)]
        fn Method(&self);
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
