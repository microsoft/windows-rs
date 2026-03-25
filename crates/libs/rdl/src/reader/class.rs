use super::*;

syn::custom_keyword!(class);

#[derive(Debug)]
pub struct Class {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub extends: Option<syn::Path>,
    pub interfaces: Vec<ClassInterface>,
}

impl syn::parse::Parse for Class {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        input.parse::<class>()?;
        let name = input.parse()?;

        let extends = if input.parse::<syn::Token![:]>().is_ok() {
            Some(input.parse()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);

        let interfaces = content
            .parse_terminated(ClassInterface::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            name,
            extends,
            interfaces,
        })
    }
}

#[derive(Debug)]
pub struct ClassInterface {
    pub attrs: Vec<syn::Attribute>,
    pub ty: syn::Path,
}

impl syn::parse::Parse for ClassInterface {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let ty = input.parse()?;

        Ok(Self { attrs, ty })
    }
}

impl Encoder<'_> {
    pub fn encode_class(&mut self, item: &Class) -> Result<(), Error> {
        let extends = if let Some(path) = &item.extends {
            let extends = self.encode_path(path)?;
            if let metadata::Type::ClassName(extends) = extends {
                self.output.TypeRef(&extends.namespace, &extends.name)
            } else {
                return self.err(&item.extends, "invalid base type");
            }
        } else {
            self.output.TypeRef("System", "Object")
        };

        let flags = metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Sealed
            | metadata::TypeAttributes::WindowsRuntime;

        let class = self.output.TypeDef(
            self.namespace,
            self.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(class),
            &item.attrs,
            &[],
        )?;

        for interface in &item.interfaces {
            self.encode_implement(class, interface)?;
        }

        Ok(())
    }

    fn encode_implement(
        &mut self,
        class: metadata::writer::TypeDef,
        interface: &ClassInterface,
    ) -> Result<(), Error> {
        let ty = self.encode_path(&interface.ty)?;

        let interface_impl = self.output.InterfaceImpl(class, &ty);

        for attr in &interface.attrs {
            let path = attr.path();

            if path.is_ident("default") {
                if !matches!(attr.meta, syn::Meta::Path(_)) {
                    return self.err(attr, "`default` attribute does not accept arguments");
                }

                let default_attribute = metadata::writer::MemberRefParent::TypeRef(
                    self.output
                        .TypeRef("Windows.Foundation.Metadata", "DefaultAttribute"),
                );

                let default_ctor = self.output.MemberRef(
                    ".ctor",
                    &metadata::Signature {
                        flags: metadata::MethodCallAttributes::HASTHIS,
                        ..Default::default()
                    },
                    default_attribute,
                );

                self.output.Attribute(
                    metadata::writer::HasAttribute::InterfaceImpl(interface_impl),
                    metadata::writer::AttributeType::MemberRef(default_ctor),
                    &[],
                );
            }
        }

        Ok(())
    }
}
