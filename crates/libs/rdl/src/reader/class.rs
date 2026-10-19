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
            if let metadata::Type::ClassName(ref tn) = extends {
                // Classes are always WinRT - the base class must also be WinRT.
                self.validate_type_is_winrt(path, &extends)?;
                if self.is_sealed_class(&tn.namespace, &tn.name) {
                    return self.err(path, "base class must be `#[unsealed]`");
                }
                self.output.TypeRef(&tn.namespace, &tn.name)
            } else {
                return self.err(&item.extends, "invalid base type");
            }
        } else {
            self.output.TypeRef("System", "Object")
        };

        let mut flags = metadata::TypeAttributes::Public | metadata::TypeAttributes::WindowsRuntime;

        if let Some(attr) = item.attrs.iter().find(|a| a.path().is_ident("unsealed")) {
            if !matches!(attr.meta, syn::Meta::Path(_)) {
                return self.err(attr, "`unsealed` attribute does not accept arguments");
            }
        } else {
            flags |= metadata::TypeAttributes::Sealed;
        }

        // A class without interfaces, such as a static class, has no instances.
        if item.interfaces.is_empty() {
            flags |= metadata::TypeAttributes::Abstract;
        }

        let class = self.output.TypeDef(
            self.namespace,
            self.name,
            metadata::writer::TypeDefOrRef::TypeRef(extends),
            flags,
        );

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(class),
            &item.attrs,
            &["unsealed"],
        )?;

        for (index, interface) in item.interfaces.iter().enumerate() {
            self.encode_implement(class, interface, index == 0)?;
        }

        Ok(())
    }

    /// Returns whether a base class is known to be sealed, from the RDL input or the reference
    /// metadata. Unknown classes are left for the metadata consumer to diagnose.
    fn is_sealed_class(&self, namespace: &str, name: &str) -> bool {
        if let Some(item) = self.index.get(namespace, name).next() {
            return match item {
                Item::Class(class) => !class.attrs.iter().any(|a| a.path().is_ident("unsealed")),
                _ => false,
            };
        }

        self.output.reference().is_some_and(|reference| {
            reference
                .get(namespace, name)
                .next()
                .is_some_and(|def| def.flags().contains(metadata::TypeAttributes::Sealed))
        })
    }

    fn encode_implement(
        &mut self,
        class: metadata::writer::TypeDef,
        interface: &ClassInterface,
        default: bool,
    ) -> Result<(), Error> {
        let ty = self.encode_path(&interface.ty)?;
        if !matches!(&ty, metadata::Type::ClassName(_)) {
            return self.err(
                &interface.ty,
                "implemented interface must resolve to an interface type",
            );
        }

        // Classes are always WinRT - every implemented interface must also be WinRT.
        self.validate_type_is_winrt(&interface.ty, &ty)?;

        let interface_impl = self.output.InterfaceImpl(class, &ty);

        if default {
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

        if let Some(attr) = interface.attrs.first() {
            return self.err(attr, "class interface does not support attributes");
        }

        Ok(())
    }
}
