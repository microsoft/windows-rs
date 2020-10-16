use crate::*;
use squote::{quote, Literal, TokenStream};
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
    pub fn from_type_name(reader: &winmd::TypeReader, name: TypeName) -> Self {
        let mut interfaces = Vec::new();
        add_dependencies(&mut interfaces, reader, &name, &name.namespace, false);
        let mut bases = Vec::new();
        let mut base = name.def;

        let signature = if interfaces.iter().any(|i| i.kind == InterfaceKind::Default) {
            name.class_signature(reader)
        } else {
            String::new()
        };

        loop {
            let (base_namespace, base_name) = base.extends(reader).name(reader);

            if (base_namespace, base_name) == ("System", "Object") {
                break;
            }

            base = reader.resolve_type_def((base_namespace, base_name));
            let base = TypeName::from_type_def(reader, base, &name.namespace);

            add_dependencies(&mut interfaces, reader, &base, &name.namespace, true);
            bases.push(base);
        }

        let mut default_constructor = false;
        let mut is_agile = false;

        for attribute in name.def.attributes(reader) {
            match attribute.name(reader) {
                ("Windows.Foundation.Metadata", "StaticAttribute") => {
                    add_type(
                        &mut interfaces,
                        reader,
                        attribute_factory(reader, attribute).unwrap(),
                        &name.namespace,
                        InterfaceKind::Statics,
                    );
                }
                ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    match attribute_factory(reader, attribute) {
                        Some(def) => {
                            add_type(
                                &mut interfaces,
                                reader,
                                def,
                                &name.namespace,
                                InterfaceKind::Statics,
                            );
                        }
                        None => default_constructor = true,
                    }
                }
                ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    // One of the arguments is a CompositionType enum and the Public variant
                    // has a value of 2 as a signed 32-bit integer.
                    for (_name, arg) in attribute.args(reader) {
                        if let winmd::AttributeArg::I32(2) = arg {
                            add_type(
                                &mut interfaces,
                                reader,
                                attribute_factory(reader, attribute).unwrap(),
                                &name.namespace,
                                InterfaceKind::Composable,
                            );
                        }
                    }
                }
                ("Windows.Foundation.Metadata", "MarshalingBehaviorAttribute") => {
                    // The only argument is a MarshalingType enum and the Agile variant
                    // has a value of 2 as a signed 32-bit integer.
                    let (_name, arg) = &attribute.args(reader)[0];

                    if let winmd::AttributeArg::I32(2) = arg {
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

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.interfaces
            .iter()
            .flat_map(|i| i.name.dependencies())
            .chain(self.bases.iter().map(|i| i.def))
            .collect()
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let type_name = self.type_name(&name);
        let methods = gen_method(&self.interfaces);
        let call_factory = self.gen_call_factory();

        if let Some(default_interface) = self
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
        {
            let conversions = TokenStream::from_iter(
                self.interfaces
                    .iter()
                    .map(|interface| interface.gen_conversions(&name, &TokenStream::new())),
            );

            let new = if self.default_constructor {
                quote! {
                    pub fn new() -> ::winrt::Result<Self> {
                        Self::IActivationFactory(|f| f.activate_instance::<Self>())
                    }
                }
            } else {
                TokenStream::new()
            };

            let bases = self.gen_base_conversions(&name);
            let iterator = gen_iterator(&self.name, &self.interfaces);
            let signature = Literal::byte_string(&self.signature.as_bytes());

            let default_name = default_interface.name.gen();
            let abi_name = default_interface.name.gen_abi();
            let (async_get, future) = gen_async(&self.name, &self.interfaces);
            let debug = gen_debug(&self.name, &self.interfaces);

            let send_sync = if self.is_agile {
                let constraints = self.name.gen_constraint();
                quote! {
                    unsafe impl<#constraints> ::std::marker::Send for #name {}
                    unsafe impl<#constraints> ::std::marker::Sync for #name {}
                }
            } else {
                TokenStream::new()
            };

            quote! {
                #[repr(transparent)]
                #[derive(::std::clone::Clone, ::std::default::Default, ::std::cmp::PartialEq)]
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
                    const IID: ::winrt::Guid = <#default_name as ::winrt::ComInterface>::IID;
                }
                unsafe impl ::winrt::RuntimeType for #name {
                    const SIGNATURE: ::winrt::ConstBuffer = ::winrt::ConstBuffer::from_slice(#signature);
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
                impl ::std::convert::From<#name> for ::winrt::Object {
                    fn from(value: #name) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&#name> for ::winrt::Object {
                    fn from(value: &#name) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::std::convert::Into<::winrt::Param<'a, ::winrt::Object>> for #name {
                    fn into(self) -> ::winrt::Param<'a, ::winrt::Object> {
                        ::winrt::Param::Owned(::std::convert::Into::<::winrt::Object>::into(self))
                    }
                }
                impl<'a> ::std::convert::Into<::winrt::Param<'a, ::winrt::Object>> for &'a #name {
                    fn into(self) -> ::winrt::Param<'a, ::winrt::Object> {
                        ::winrt::Param::Owned(::std::convert::Into::<::winrt::Object>::into(::std::clone::Clone::clone(self)))
                    }
                }
                #debug
                #conversions
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

    pub fn gen_base_conversions(&self, from: &TokenStream) -> TokenStream {
        TokenStream::from_iter(self.bases.iter().map(|base| {
            let into = base.gen();
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

    fn gen_call_factory(&self) -> TokenStream {
        let mut tokens = Vec::new();

        if self.default_constructor {
            let interface_tokens = quote! { ::winrt::IActivationFactory };
            tokens.push(self.to_named_call_factory("IActivationFactory", &interface_tokens));
        }

        for interface in &self.interfaces {
            if (interface.kind != InterfaceKind::Statics
                && interface.kind != InterfaceKind::Composable)
                || interface.methods.is_empty()
            {
                continue;
            }

            let interface_namespace =
                gen_namespace(&interface.name.namespace, &self.name.namespace);

            let interface_name = format_ident(&interface.name.name);
            let interface_tokens = quote! { #interface_namespace #interface_name };
            tokens.push(self.to_named_call_factory(&interface.name.name, &interface_tokens));
        }

        TokenStream::from_iter(tokens)
    }

    fn to_named_call_factory(&self, method_name: &str, interface: &TokenStream) -> TokenStream {
        let self_name = self.name.gen();
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

fn attribute_factory(
    reader: &winmd::TypeReader,
    attribute: winmd::Attribute,
) -> Option<winmd::TypeDef> {
    for (_, arg) in attribute.args(reader) {
        if let winmd::AttributeArg::TypeDef(def) = arg {
            return Some(def);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn class((namespace, type_name): (&str, &str)) -> Class {
        let reader = &winmd::TypeReader::from_os();
        let def = reader.resolve_type_def((namespace, type_name));

        match Type::from_type_def(reader, def) {
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

        let default = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(
            1 == t
                .interfaces
                .iter()
                .filter(|i| i.kind == InterfaceKind::Default)
                .count()
        );

        assert!(t.default_constructor == false);
        assert!(t.name.runtime_name() == "Windows.Media.Core.TimedMetadataStreamDescriptor");
        assert!(default.name.runtime_name() == "Windows.Media.Core.IMediaStreamDescriptor");
        assert!(default.kind == InterfaceKind::Default);
    }

    #[test]
    fn test_class_with_bases() {
        let t = class(("Windows.UI.Composition", "SpriteVisual"));

        let default = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(t.default_constructor == false);
        assert!(t.name.runtime_name() == "Windows.UI.Composition.SpriteVisual");
        assert!(default.name.runtime_name() == "Windows.UI.Composition.ISpriteVisual");
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

        let default = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(default.name.runtime_name() == "Windows.UI.Composition.IContainerVisual");
        assert!(t.bases.len() == 2);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.Visual");
        assert!(t.bases[1].runtime_name() == "Windows.UI.Composition.CompositionObject");
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "IContainerVisual").kind == InterfaceKind::Default);
        assert!(interface(&t, "IVisual").kind == InterfaceKind::NonDefault);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::NonDefault);

        let t = class(("Windows.UI.Composition", "Visual"));

        let default = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(default.name.runtime_name() == "Windows.UI.Composition.IVisual");
        assert!(t.bases.len() == 1);
        assert!(t.bases[0].runtime_name() == "Windows.UI.Composition.CompositionObject");
        assert!(count_default(&t) == 1);
        assert!(interface(&t, "IVisual").kind == InterfaceKind::Default);
        assert!(interface(&t, "ICompositionObject").kind == InterfaceKind::NonDefault);

        let t = class(("Windows.UI.Composition", "CompositionObject"));

        let default = t
            .interfaces
            .iter()
            .find(|i| i.kind == InterfaceKind::Default)
            .unwrap();

        assert!(default.name.runtime_name() == "Windows.UI.Composition.ICompositionObject");
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
