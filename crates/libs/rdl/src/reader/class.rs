use super::*;

syn::custom_keyword!(class);

#[derive(Debug)]
pub struct Class {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub colon_list: Vec<syn::Path>,
}

impl syn::parse::Parse for Class {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<class>()?;
        let name = input.parse()?;

        let colon_list = if input.parse::<syn::Token![:]>().is_ok() {
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_separated_nonempty(
                input,
            )?
            .into_iter()
            .collect()
        } else {
            vec![]
        };

        let content;
        syn::braced!(content in input);

        if !content.is_empty() {
            return Err(syn::Error::new(content.span(), "class body must be empty"));
        }

        Ok(Self {
            attrs,
            name,
            colon_list,
        })
    }
}

impl Class {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let mut base_type: Option<metadata::Type> = None;
        let mut interfaces: Vec<metadata::Type> = vec![];

        for path in &self.colon_list {
            let ty = encode_path(encoder, path)?;

            let metadata::Type::Name(ref tn) = ty else {
                return encoder.err(path, "invalid type in class declaration");
            };

            if is_class_type(encoder, tn) {
                if base_type.is_some() {
                    return encoder.err(path, "only one base class is allowed");
                }
                if !interfaces.is_empty() {
                    return encoder.err(path, "base class must come before interfaces");
                }
                base_type = Some(ty);
            } else {
                interfaces.push(ty);
            }
        }

        let extends_ref = if let Some(metadata::Type::Name(tn)) = base_type {
            encoder.output.TypeRef(&tn.namespace, &tn.name)
        } else {
            encoder.output.TypeRef("System", "Object")
        };

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        let class = encoder.output.TypeDef(
            encoder.namespace,
            encoder.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends_ref),
            flags,
        );

        // Emit any Named attributes (defined in metadata or RDL) attached to this class.
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(class),
            &self.attrs,
            &[],
        )?;

        for (index, ty) in interfaces.iter().enumerate() {
            let interface_impl = encoder.output.InterfaceImpl(class, ty);

            // The first interface in the list is implicitly the default interface.
            if index == 0 {
                let default_attribute = metadata::writer::MemberRefParent::TypeRef(
                    encoder
                        .output
                        .TypeRef("Windows.Foundation.Metadata", "DefaultAttribute"),
                );

                let default_ctor = encoder.output.MemberRef(
                    ".ctor",
                    &metadata::Signature {
                        flags: metadata::MethodCallAttributes::HASTHIS,
                        ..Default::default()
                    },
                    default_attribute,
                );

                encoder.output.Attribute(
                    metadata::writer::HasAttribute::InterfaceImpl(interface_impl),
                    metadata::writer::AttributeType::MemberRef(default_ctor),
                    &[],
                );
            }
        }

        Ok(())
    }
}

fn is_class_type(encoder: &Encoder, type_name: &metadata::TypeName) -> bool {
    if let Some(ns) = encoder.index.namespaces.get(&type_name.namespace) {
        if let Some((_, item)) = ns.types.get(&type_name.name) {
            return matches!(item, Item::Class(_));
        }
    }

    for def in encoder.reference.get(&type_name.namespace, &type_name.name) {
        if def.category() == metadata::reader::TypeCategory::Class {
            return true;
        }
    }

    false
}

#[test]
#[should_panic(expected = "error: class body must be empty\n --> .rdl:6:9")]
fn class_body_must_be_empty() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface IFoo {}
    class MyClass: IFoo {
        IFoo,
    }
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: only one base class is allowed\n --> .rdl:6:26")]
fn multiple_base_classes_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    class Base {}
    class Other {}
    class Derived: Base, Other {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: base class must come before interfaces\n --> .rdl:7:26")]
fn base_class_after_interface_errors() {
    Reader::new()
        .input_str(
            r#"
#[winrt]
mod Test {
    class Base {}
    interface IFoo {}
    interface IBar {}
    class Derived: IFoo, Base {}
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
