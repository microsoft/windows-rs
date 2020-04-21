use crate::tables::*;
use crate::types::*;
use crate::TypeReader;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

/// A WinRT Class
#[derive(Debug)]
pub struct Class {
    pub name: TypeName,
    pub bases: Vec<TypeName>,
    pub interfaces: Vec<RequiredInterface>,
    pub default_constructor: bool,
}

impl Class {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut interfaces = Vec::new();
        RequiredInterface::append(reader, &name, &mut interfaces);
        let mut bases = Vec::new();
        let mut base = def;

        loop {
            let (namespace, name) = base.extends(reader).name(reader);

            if (namespace, name) == ("System", "Object") {
                break;
            }

            base = reader.resolve_type_def((namespace, name));
            let namespace = namespace.to_string();
            let name = name.to_string();
            let generics = Vec::new();

            let base = TypeName {
                namespace,
                name,
                generics,
                def: base,
            };

            RequiredInterface::append(reader, &base, &mut interfaces);
            bases.push(base);
        }

        let mut default_constructor = false;

        for attribute in def.attributes(reader) {
            match attribute.name(reader) {
                ("Windows.Foundation.Metadata", "StaticAttribute") => {
                    let mut interface = RequiredInterface::from_type_def(
                        reader,
                        attribute_factory(reader, attribute).unwrap(),
                    );
                    interface.kind = InterfaceKind::Statics;
                    interfaces.push(interface);
                }
                ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    match attribute_factory(reader, attribute) {
                        Some(def) => {
                            let mut interface = RequiredInterface::from_type_def(reader, def);
                            interface.kind = InterfaceKind::Statics;
                            interfaces.push(interface);
                        }
                        None => default_constructor = true,
                    }
                }
                _ => {}
            }
        }

        Self {
            name,
            interfaces,
            bases,
            default_constructor,
        }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.bases.iter().map(|i| i.def))
            .collect()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = self.name.to_tokens(&self.name.namespace);
        let type_name = self.type_name(&name);
        let methods = to_method_tokens(&self.name.namespace, &self.interfaces);

        if self.interfaces[0].kind == InterfaceKind::Default {
            let guid = self.interfaces[0].guid.to_tokens();
            let conversions = TokenStream::from_iter(self.interfaces.iter().map(|interface| {
                interface.to_conversions_tokens(&self.name.namespace, &name, &TokenStream::new())
            }));

            let new = if self.default_constructor {
                quote! {
                    pub fn new() -> ::winrt::Result<Self> {
                        ::winrt::factory::<Self, ::winrt::IActivationFactory>()?.activate_instance::<Self>()
                    }
                }
            } else {
                quote! {}
            };

            quote! {
                #[repr(transparent)]
                #[derive(Default, Clone)]
                pub struct #name { ptr: ::winrt::IUnknown }
                impl #name {
                    #new
                    #methods
                }
                #type_name
                unsafe impl ::winrt::ComInterface for #name {
                    const GUID: ::winrt::Guid = ::winrt::Guid::from_values(#guid);
                }
                impl ::winrt::RuntimeType for #name {
                    type Abi = ::winrt::RawPtr;
                    fn abi(&self) -> Self::Abi {
                        self.ptr.get()
                    }
                    fn set_abi(&mut self) -> *mut Self::Abi {
                        self.ptr.set()
                    }
                }
                #conversions
            }
        } else {
            quote! {
                pub struct #name {}
                impl #name { #methods }
                #type_name
            }
        }
    }

    fn type_name(&self, class_name: &TokenStream) -> TokenStream {
        let runtime_name = self.name.runtime_name();

        quote! {
            impl ::winrt::RuntimeName for #class_name {
                const NAME: &'static str = #runtime_name;
            }
        }
    }
}

fn attribute_factory(reader: &TypeReader, attribute: Attribute) -> Option<TypeDef> {
    for (_, arg) in attribute.args(reader) {
        if let AttributeArg::TypeDef(def) = arg {
            return Some(def);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class((namespace, type_name): (&str, &str)) -> Class {
        let reader = &TypeReader::from_os();
        let def = reader.resolve_type_def((namespace, type_name));

        match def.into_type(reader) {
            Type::Class(t) => t,
            _ => panic!("Type not an interface"),
        }
    }

    #[test]
    fn test_uri() {
        let t = class(("Windows.Foundation", "Uri"));
        assert!(t.default_constructor == false);
        assert!(t.bases.is_empty());
        assert!(t.interfaces.len() == 5);

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IUriRuntimeClass")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IUriRuntimeClass");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IUriRuntimeClassWithAbsoluteCanonicalUri")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(
            interface.name.runtime_name()
                == "Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri"
        );

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IStringable")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IStringable");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IUriRuntimeClassFactory")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Statics);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IUriRuntimeClassFactory");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IUriEscapeStatics")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Statics);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IUriEscapeStatics");
    }

    #[test]
    fn test_url_decoder() {
        let t = class(("Windows.Foundation", "WwwFormUrlDecoder"));
        assert!(t.default_constructor == false);

        assert!(t.name.runtime_name() == "Windows.Foundation.WwwFormUrlDecoder");

        assert!(t.interfaces.len() == 4);

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IWwwFormUrlDecoderRuntimeClassFactory")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Statics);
        assert!(
            interface.name.runtime_name()
                == "Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory"
        );

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IWwwFormUrlDecoderRuntimeClass")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(
            interface.name.runtime_name() == "Windows.Foundation.IWwwFormUrlDecoderRuntimeClass"
        );

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IIterable`1")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.runtime_name() == "Windows.Foundation.Collections.IIterable`1<Windows.Foundation.IWwwFormUrlDecoderEntry>");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IVectorView`1")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.runtime_name() == "Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.IWwwFormUrlDecoderEntry>");
    }

    #[test]
    fn test_class_with_bases() {
        let t = class(("Windows.UI.Composition", "SpriteVisual"));
        assert!(t.default_constructor == false);
        assert!(t.name.runtime_name() == "Windows.UI.Composition.SpriteVisual");
        assert!(t.bases.len() == 3);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.ContainerVisual");
        assert!(t.bases[1].runtime_name() == "Windows.UI.Composition.Visual");
        assert!(t.bases[2].runtime_name() == "Windows.UI.Composition.CompositionObject");
    }

    #[test]
    fn test_class_with_default_constructor() {
        let t = class(("Windows.UI.Composition", "Compositor"));
        assert!(t.default_constructor == true);
        assert!(t.name.runtime_name() == "Windows.UI.Composition.Compositor");
        assert!(t.bases.is_empty());

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "ICompositor")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(interface.name.runtime_name() == "Windows.UI.Composition.ICompositor");
    }
}
