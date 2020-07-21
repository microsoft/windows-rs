use super::object::to_object_tokens;
use crate::format_ident;
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
    pub is_agile: bool,
    pub signature: String,
}

impl Class {
    pub fn from_type_name(reader: &TypeReader, name: TypeName) -> Self {
        let mut interfaces = Vec::new();
        RequiredInterface::append_default(reader, &name, &mut interfaces);
        let mut bases = Vec::new();
        let mut base = name.def;

        let signature = if !interfaces.is_empty() && interfaces[0].kind == InterfaceKind::Default {
            name.class_signature(reader)
        } else {
            "".to_owned()
        };

        loop {
            let (base_namespace, base_name) = base.extends(reader).name(reader);

            if (base_namespace, base_name) == ("System", "Object") {
                break;
            }

            base = reader.resolve_type_def((base_namespace, base_name));
            let base = TypeName::from_type_def(reader, base, &name.namespace);

            RequiredInterface::append_required(reader, &base, &name.namespace, &mut interfaces);
            bases.push(base);
        }

        let mut default_constructor = false;
        let mut is_agile = false;

        for attribute in name.def.attributes(reader) {
            match attribute.name(reader) {
                ("Windows.Foundation.Metadata", "StaticAttribute") => {
                    let mut interface = RequiredInterface::from_type_def(
                        reader,
                        attribute_factory(reader, attribute).unwrap(),
                        &name.namespace,
                    );
                    interface.kind = InterfaceKind::Statics;
                    interfaces.push(interface);
                }
                ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    match attribute_factory(reader, attribute) {
                        Some(def) => {
                            let mut interface =
                                RequiredInterface::from_type_def(reader, def, &name.namespace);
                            interface.kind = InterfaceKind::Statics;
                            interfaces.push(interface);
                        }
                        None => default_constructor = true,
                    }
                }
                ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    // One of the arguments is a CompositionType enum and the Public variant
                    // has a value of 2 as a signed 32-bit integer.
                    for (_name, arg) in attribute.args(reader) {
                        if let AttributeArg::I32(2) = arg {
                            let mut interface = RequiredInterface::from_type_def(
                                reader,
                                attribute_factory(reader, attribute).unwrap(),
                                &name.namespace,
                            );
                            interface.kind = InterfaceKind::Composable;
                            interfaces.push(interface);
                        }
                    }
                }
                ("Windows.Foundation.Metadata", "MarshalingBehaviorAttribute") => {
                    // The only argument is a MarshalingType enum and the Agile variant
                    // has a value of 2 as a signed 32-bit integer.
                    let (_name, arg) = &attribute.args(reader)[0];

                    if let AttributeArg::I32(2) = arg {
                        is_agile = true;
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
            is_agile,
            signature,
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
        let name = &self.name.tokens;
        let type_name = self.type_name(&name);
        let methods = to_method_tokens(&self.interfaces);
        let call_factory = self.to_call_factory_tokens();

        if self.interfaces[0].kind == InterfaceKind::Default {
            let conversions = TokenStream::from_iter(
                self.interfaces
                    .iter()
                    .map(|interface| interface.to_conversions_tokens(&name, &TokenStream::new())),
            );

            let new = if self.default_constructor {
                quote! {
                    pub fn new() -> ::winrt::Result<Self> {
                        Self::IActivationFactory(|f| f.activate_instance::<Self>())
                    }
                }
            } else {
                quote! {}
            };

            let object = to_object_tokens(&name, &TokenStream::new());
            let bases = self.to_base_conversions_tokens(&name);
            let iterator = iterator_tokens(&self.name, &self.interfaces);
            let signature = &self.signature;

            let default_name = &self.interfaces[0].name.tokens;
            let abi_name = self.interfaces[0].name.to_abi_tokens();
            let (async_get, future) = get_async_tokens(&self.name, &self.interfaces);
            let debug = debug::debug_tokens(&self.name, &self.interfaces);

            let send_sync = if self.is_agile {
                let constraints = &self.name.constraints;
                let name = &self.name.tokens;
                quote! {
                    unsafe impl<#constraints> ::std::marker::Send for #name {}
                    unsafe impl<#constraints> ::std::marker::Sync for #name {}
                }
            } else {
                quote! {}
            };

            quote! {
                #[repr(transparent)]
                #[derive(Default, Clone, PartialEq)]
                pub struct #name { ptr: ::winrt::ComPtr<#default_name> }
                impl #name {
                    #new
                    #methods
                    #async_get
                    #call_factory
                }
                #type_name
                unsafe impl ::winrt::ComInterface for #name {
                    type VTable = #abi_name;
                    fn iid() -> ::winrt::Guid {
                        <#default_name as ::winrt::ComInterface>::iid()
                    }
                }
                unsafe impl ::winrt::RuntimeType for #name {
                    fn signature() -> String {
                        #signature.to_owned()
                    }
                }
                unsafe impl ::winrt::AbiTransferable for #name {
                    type Abi = ::winrt::RawComPtr<#default_name>;
                    fn get_abi(&self) -> Self::Abi {
                        <::winrt::ComPtr<#default_name> as ::winrt::AbiTransferable>::get_abi(&self.ptr)
                    }
                    fn set_abi(&mut self) -> *mut Self::Abi {
                        <::winrt::ComPtr<#default_name> as ::winrt::AbiTransferable>::set_abi(&mut self.ptr)
                    }
                }
                #debug
                #conversions
                #object
                #bases
                #iterator
                #send_sync
                #future
            }
        } else {
            quote! {
                pub struct #name {}
                impl #name {
                    #methods
                    #call_factory
                }
                #type_name
            }
        }
    }

    pub fn to_base_conversions_tokens(&self, from: &TokenStream) -> TokenStream {
        TokenStream::from_iter(self.bases.iter().map(|base| {
            let into = &base.tokens;
            quote! {
                impl ::std::convert::From<#from> for #into {
                    fn from(value: #from) -> Self {
                        ::std::convert::Into::<#into>::into(&value)
                    }
                }
                impl ::std::convert::From<&#from> for #into {
                    fn from(value: &#from) -> Self {
                        <#from as ::winrt::ComInterface>::query(value)
                    }
                }
                impl<'a> ::std::convert::Into<::winrt::Param<'a, #into>> for #from {
                    fn into(self) -> ::winrt::Param<'a, #into> {
                        ::winrt::Param::Owned(::std::convert::Into::<#into>::into(self))
                    }
                }
                impl<'a> ::std::convert::Into<::winrt::Param<'a, #into>> for &'a #from {
                    fn into(self) -> ::winrt::Param<'a, #into> {
                        ::winrt::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                    }
                }
            }
        }))
    }

    fn to_call_factory_tokens(&self) -> TokenStream {
        let mut tokens = Vec::new();

        if self.default_constructor {
            let interface_tokens = quote! { ::winrt::IActivationFactory };
            tokens.push(self.to_named_call_factory("IActivationFactory", &interface_tokens));
        }

        for interface in &self.interfaces {
            if (interface.kind != InterfaceKind::Statics
                && interface.kind != InterfaceKind::Composable)
                || interface.methods.len() == 0
            {
                continue;
            }

            let interface_namespace =
                to_namespace_tokens(&interface.name.namespace, &self.name.namespace);

            let interface_name = format_ident(&interface.name.name);
            let interface_tokens = quote! { #interface_namespace #interface_name };
            tokens.push(self.to_named_call_factory(&interface.name.name, &interface_tokens));
        }

        TokenStream::from_iter(tokens)
    }

    fn to_named_call_factory(&self, method_name: &str, interface: &TokenStream) -> TokenStream {
        let self_name = &self.name.tokens;
        let method_name = format_ident(method_name);

        quote! {
            #[allow(non_snake_case)]
            fn #method_name<R, F: FnOnce(&#interface) -> ::winrt::Result<R>>(
                callback: F,
            ) -> ::winrt::Result<R> {
                static mut SHARED: ::winrt::FactoryCache<#self_name, #interface> =
                    ::winrt::FactoryCache::new();
                unsafe { SHARED.call(callback) }
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

    fn interface<'a>(class: &'a Class, name: &str) -> &'a RequiredInterface {
        class
            .interfaces
            .iter()
            .find(|interface| interface.name.name == name)
            .unwrap()
    }

    fn count_default(class: &Class) -> usize {
        class
            .interfaces
            .iter()
            .filter(|interface| interface.kind == InterfaceKind::Default)
            .count()
    }

    #[test]
    fn test_uri() {
        let t = class(("Windows.Foundation", "Uri"));
        assert!(t.default_constructor == false);
        assert!(t.bases.is_empty());
        assert!(t.interfaces.len() == 5);
        assert!(t.is_agile);

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
    fn test_media_core() {
        let t = class(("Windows.Media.Core", "TimedMetadataStreamDescriptor"));
        assert!(t.default_constructor == false);
        assert!(t.name.runtime_name() == "Windows.Media.Core.TimedMetadataStreamDescriptor");
        assert!(t.interfaces[0].name.runtime_name() == "Windows.Media.Core.IMediaStreamDescriptor");
        assert!(t.interfaces[0].kind == InterfaceKind::Default);
    }

    #[test]
    fn test_class_with_bases() {
        let t = class(("Windows.UI.Composition", "SpriteVisual"));
        assert!(t.default_constructor == false);
        assert!(t.name.runtime_name() == "Windows.UI.Composition.SpriteVisual");
        assert!(t.interfaces[0].name.runtime_name() == "Windows.UI.Composition.ISpriteVisual");
        assert!(t.bases.len() == 3);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.ContainerVisual");
        assert!(t.bases[1].runtime_name() == "Windows.UI.Composition.Visual");
        assert!(t.bases[2].runtime_name() == "Windows.UI.Composition.CompositionObject");
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "ISpriteVisual").kind == InterfaceKind::Default);
        assert!(interface(&t, "IContainerVisual").kind == InterfaceKind::NonDefault);
        assert!(interface(&t, "IVisual").kind == InterfaceKind::NonDefault);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::NonDefault);

        let t = class(("Windows.UI.Composition", "ContainerVisual"));
        assert!(t.interfaces[0].name.runtime_name() == "Windows.UI.Composition.IContainerVisual");
        assert!(t.bases.len() == 2);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.Visual");
        assert!(t.bases[1].runtime_name() == "Windows.UI.Composition.CompositionObject");
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "IContainerVisual").kind == InterfaceKind::Default);
        assert!(interface(&t, "IVisual").kind == InterfaceKind::NonDefault);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::NonDefault);

        let t = class(("Windows.UI.Composition", "Visual"));
        assert!(t.interfaces[0].name.runtime_name() == "Windows.UI.Composition.IVisual");
        assert!(t.bases.len() == 1);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.CompositionObject");
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "IVisual").kind == InterfaceKind::Default);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::NonDefault);

        let t = class(("Windows.UI.Composition", "CompositionObject"));
        assert!(t.interfaces[0].name.runtime_name() == "Windows.UI.Composition.ICompositionObject");
        assert!(t.bases.is_empty());
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::Default);
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

    #[test]
    fn test_is_agile() {
        // MarshalType.Standard
        let t = class(("Windows.UI.Core", "CoreWindow"));
        assert!(t.is_agile == false);

        // MarshalType.None
        let t = class(("Windows.System.Display", "DisplayRequest"));
        assert!(t.is_agile == false);

        // MarshalType.Agile
        let t = class(("Windows.Foundation", "Uri"));
        assert!(t.is_agile == true);
    }
}
