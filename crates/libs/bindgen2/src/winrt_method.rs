use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
pub(super) struct NamedMethod {
    pub(super) name: String,
    pub(super) context_name: String,
    metadata_name: String,
    overloaded: bool,
    pub(super) method: winrt_delegate::Method,
    event: Option<winrt_delegate::EventHandler>,
    public: bool,
}

impl NamedMethod {
    pub(super) fn selected(&self, members: &MemberSelection) -> bool {
        if matches!(members, MemberSelection::All) {
            return true;
        }
        let MemberSelection::Names(names) = members else {
            return false;
        };
        if names.contains(&self.name) || (!self.overloaded && names.contains(&self.metadata_name)) {
            return true;
        }
        self.metadata_name
            .strip_prefix("remove_")
            .is_some_and(|name| names.contains(&format!("add_{name}")) || names.contains(name))
    }

    pub(super) fn selected_factory(&self, members: &MemberSelection) -> bool {
        members.includes(&self.metadata_name, &self.name)
    }

    pub(super) fn write_static(
        &self,
        context: &winrt_delegate::MethodContext<'_>,
        public_name: &str,
        factory_name: &str,
    ) -> Result<Option<TokenStream>, Error> {
        if !self.public {
            return Ok(None);
        }
        if let Some(event) = &self.event {
            return Ok(Some(self.method.write_static_event_method(
                context,
                public_name,
                &format!("Remove{}", self.name),
                factory_name,
                event,
            )?));
        }
        Ok(Some(self.method.write_static_method(
            context,
            public_name,
            &self.name,
            factory_name,
        )?))
    }

    pub(super) fn substitute(&mut self, arguments: &[ty::Type]) {
        self.method.substitute(arguments);
        if let Some(event) = &mut self.event {
            event.substitute(arguments);
        }
    }

    pub(super) const fn is_public(&self) -> bool {
        self.public
    }

    pub(super) fn write_public(
        &self,
        context: &winrt_delegate::MethodContext<'_>,
        public_name: &str,
        interface: Option<TokenStream>,
    ) -> Result<Option<TokenStream>, Error> {
        if !self.public {
            return Ok(None);
        }
        let (receiver, prelude) = if let Some(interface) = &interface {
            (
                quote! { this },
                quote! {
                    let this = &windows_core::Interface::cast::<#interface>(self)?;
                },
            )
        } else {
            (quote! { self }, quote! {})
        };
        if let Some(event) = &self.event {
            return Ok(Some(self.method.write_event_method(
                context,
                public_name,
                &format!("Remove{}", self.name),
                event,
                receiver,
                prelude,
            )?));
        }
        Ok(Some(if let Some(interface) = interface {
            self.method.write_forwarded_public_method(
                context,
                public_name,
                &self.name,
                interface,
            )?
        } else {
            self.method
                .write_public_method(context, public_name, &self.name, receiver)?
        }))
    }
}

#[derive(Clone)]
pub(super) struct RequiredInterface {
    pub(super) entity: Entity<TypeDef>,
    pub(super) namespace: String,
    pub(super) name: String,
    pub(super) arguments: Vec<ty::Type>,
    pub(super) methods: Vec<NamedMethod>,
}

impl RequiredInterface {
    pub(super) fn write_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_named(namespace, layout, generics, &self.name)
    }

    pub(super) fn write_impl_name(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
    ) -> Result<TokenStream, Error> {
        self.write_named(namespace, layout, generics, &format!("{}_Impl", self.name))
    }

    fn write_named(
        &self,
        namespace: &str,
        layout: Layout,
        generics: &[String],
        emitted_name: &str,
    ) -> Result<TokenStream, Error> {
        let path =
            if let Some(crate_name) = layout.winrt_crate(namespace, &self.namespace, &self.name) {
                let crate_name = tokens::ident(crate_name);
                quote! { #crate_name:: }
            } else {
                tokens::namespace(namespace, &self.namespace, layout)
            };
        let name = tokens::ident(emitted_name);
        let arguments = self
            .arguments
            .iter()
            .map(|argument| argument.write_name(namespace, layout, generics))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(if arguments.is_empty() {
            quote! { #path #name }
        } else {
            quote! { #path #name<#(#arguments),*> }
        })
    }
}

pub(super) fn lower_methods(
    database: &Database,
    definition: TypeDefinition<'_>,
    owner: &str,
) -> Result<Vec<NamedMethod>, Error> {
    let mut names = BTreeMap::<String, u32>::new();
    let metadata_names = definition
        .methods()?
        .map(|method| Ok(method.name()?.to_string()))
        .collect::<Result<BTreeSet<_>, Error>>()?;
    definition
        .methods()?
        .map(|method| {
            let metadata_name = method.name()?;
            let event_name = metadata_name.strip_prefix("add_");
            if let Some(event_name) = event_name
                && !metadata_names.contains(&format!("remove_{event_name}"))
            {
                return Err(Error::InvalidType {
                    name: owner.to_string(),
                    message: "event add method has no matching remove method",
                });
            }
            let overload_attribute = method.find_attribute("OverloadAttribute")?;
            let (mut name, context_name) = if method.flags()? & 0x800 != 0 {
                if let Some(name) = metadata_name.strip_prefix("get_") {
                    (name.to_string(), name.to_string())
                } else if let Some(name) = metadata_name.strip_prefix("put_") {
                    (format!("Set{name}"), format!("Set{name}"))
                } else if let Some(name) = metadata_name.strip_prefix("add_") {
                    (name.to_string(), name.to_string())
                } else if let Some(name) = metadata_name.strip_prefix("remove_") {
                    (format!("Remove{name}"), format!("Remove{name}"))
                } else {
                    (metadata_name.to_string(), metadata_name.to_string())
                }
            } else if let Some(attribute) = overload_attribute {
                let arguments = attribute.arguments(&())?;
                let overload = arguments.iter().find_map(|argument| match argument {
                    AttributeArgument::Fixed {
                        value: AttributeValue::String(value),
                        ..
                    } => Some(value.as_str()),
                    _ => None,
                });
                match overload {
                    Some(overload)
                        if overload
                            .strip_prefix(metadata_name)
                            .is_some_and(|suffix| suffix.parse::<u32>().is_ok()) =>
                    {
                        (metadata_name.to_string(), metadata_name.to_string())
                    }
                    Some(overload) => (overload.to_string(), overload.to_string()),
                    None => (metadata_name.to_string(), metadata_name.to_string()),
                }
            } else {
                (metadata_name.to_string(), metadata_name.to_string())
            };
            let count = names.entry(name.clone()).or_default();
            *count += 1;
            if *count > 1 {
                name.push_str(&count.to_string());
            }
            let method = winrt_delegate::Method::lower(
                database,
                definition.entity().file(),
                method,
                owner,
                false,
            )?;
            let event = if event_name.is_some() {
                Some(method.lower_event_handler(database, owner)?)
            } else {
                None
            };
            Ok(NamedMethod {
                name,
                context_name,
                metadata_name: metadata_name.to_string(),
                overloaded: overload_attribute.is_some(),
                method,
                event,
                public: !metadata_name.starts_with("remove_"),
            })
        })
        .collect()
}
