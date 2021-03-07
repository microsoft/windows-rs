use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Class(pub GenericType);

impl Class {
    // TODO: can't this be private and use the interfaces collection?
    pub fn type_signature(&self) -> String {
        let default = self.0.default_interface();

        format!(
            "rc({}.{};{})",
            self.0.def.namespace(),
            self.0.def.name(),
            default.interface_signature()
        )
    }

    fn has_default_constructor(&self) -> bool {
        for attribute in self.0.def.attributes() {
            if attribute.full_name() == ("Windows.Foundation.Metadata", "ActivatableAttribute") {
                if attribute.args().iter().any(|arg| {
                    if let parser::ConstantValue::TypeDef(_) = arg.1 {
                        true
                    } else {
                        false
                    }
                }) {
                    continue;
                } else {
                    return true;
                }
            }
        }

        false
    }

    pub fn interfaces(&self) -> Vec<InterfaceInfo> {
        fn add_interfaces(result: &mut Vec<InterfaceInfo>, parent: &GenericType, is_base: bool) {
            for child in parent.def.interface_impls() {
                if let Some(def) = child.generic_interface(&parent.generics) {
                        let kind = if !is_base && child.is_default() {
                            InterfaceKind::Default
                        } else if child.is_overridable() {
                            InterfaceKind::Overridable
                        } else {
                            InterfaceKind::NonDefault
                        };

                        let mut found = false;

                        for existing in result.iter_mut() {
                            if existing.def == def {
                                found = true;

                                if kind == InterfaceKind::Default {
                                    existing.kind = kind;
                                }
                            }
                        }

                        if !found {
                            add_interfaces(result, &def, is_base);

                            let version = def.def.version();

                            result.push(InterfaceInfo {
                                def,
                                kind,
                                is_base,
                                version,
                            });
                        }
                    }
                }
            }
        

        let mut result = Vec::new();
        add_interfaces(&mut result, &self.0, false);

        for base in self.0.bases() {
            add_interfaces(&mut result, &base, true);
        }

        for attribute in self.0.def.attributes() {
            match attribute.full_name() {
                ("Windows.Foundation.Metadata", "StaticAttribute")
                | ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    for (_, arg) in attribute.args() {
                        if let parser::ConstantValue::TypeDef(def) = arg {
                            let def = GenericType::from_type_def(def, Vec::new());
                            let version = def.def.version();

                            result.push(InterfaceInfo {
                                def,
                                kind: InterfaceKind::Static,
                                is_base: false,
                                version,
                            });

                            break;
                        }
                    }
                }
                ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    // One of the arguments is a CompositionType enum and the Public variant
                    // has a value of 2 as a signed 32-bit integer.

                    let mut public = false;
                    let mut def = None;

                    for (_, arg) in attribute.args() {
                        match arg {
                            parser::ConstantValue::I32(2) => public = true,

                            parser::ConstantValue::TypeDef(value) => {
                                def = Some(GenericType::from_type_def(value, Vec::new()))
                            }

                            _ => {}
                        }
                    }

                    if let Some(def) = def {
                        if public {
                            let version = def.def.version();

                            result.push(InterfaceInfo {
                                def,
                                kind: InterfaceKind::Composable,
                                is_base: false,
                                version,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        InterfaceInfo::sort(&mut result);
        result
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        let generics = self.0.generics.iter().map(|g| g.definition());
        let interfaces = self.0.interfaces().map(|i| i.definition());
        let bases = self.0.bases().map(|b| b.definition());

        let factories = self.0.def.attributes().filter_map(|attribute| {
            match attribute.full_name() {
                ("Windows.Foundation.Metadata", "StaticAttribute")
                | ("Windows.Foundation.Metadata", "ActivatableAttribute")
                | ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    for (_, arg) in attribute.args() {
                        if let parser::ConstantValue::TypeDef(def) = arg {
                            return Some(def);
                        }
                    }
                }
                _ => {}
            }

            None
        });

        generics
            .chain(interfaces)
            .chain(bases)
            .flatten()
            .chain(factories)
            .collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        vec![self.0.def]
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let interfaces = self.interfaces();
        let methods = InterfaceInfo::gen_methods(&interfaces, gen);
        let runtime_name = format!("{}.{}", self.0.def.namespace(), self.0.def.name());

        let factories = interfaces.iter().filter_map(|interface| {
            match interface.kind {
                InterfaceKind::Static | InterfaceKind::Composable => {
                    if interface.def.def.methods().next().is_some() {
                        let interface_name = format_ident!("{}", interface.def.def.name());
                        let interface_type = interface.def.gen_name(gen);

                        Some(quote! {
                            #[allow(non_snake_case)]
                            fn #interface_name<R, F: FnOnce(&#interface_type) -> ::windows::Result<R>>(
                                callback: F,
                            ) -> ::windows::Result<R> {
                                static mut SHARED: ::windows::FactoryCache<#name, #interface_type> =
                                    ::windows::FactoryCache::new();
                                unsafe { SHARED.call(callback) }
                            }
                        })
                    } else {
                        None
                    }
                }
                _ => None
            }
        });

        if let Some(default_interface) =
            interfaces.iter().find(|i| i.kind == InterfaceKind::Default)
        {
            let guid = default_interface.def.gen_guid(gen);
            let default_abi_name = default_interface.def.gen_abi_name(gen);
            let type_signature = Literal::byte_string(self.type_signature().as_bytes());
            let object = gen_object(&name, &TokenStream::new());
            let (async_get, future) = gen_async(&self.0, &interfaces, gen);

            let new = if self.has_default_constructor() {
                quote! {
                    pub fn new() -> ::windows::Result<Self> {
                        Self::IActivationFactory(|f| f.activate_instance::<Self>())
                    }
                    #[allow(non_snake_case)]
                    fn IActivationFactory<R, F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>>(
                        callback: F,
                    ) -> ::windows::Result<R> {
                        static mut SHARED: ::windows::FactoryCache<#name, ::windows::IActivationFactory> =
                            ::windows::FactoryCache::new();
                        unsafe { SHARED.call(callback) }
                    }
                }
            } else {
                quote! {}
            };

            let conversions = interfaces
                .iter()
                .map(|interface| interface.gen_conversion(&name, &TokenStream::new(), gen));

            let send_sync = if self.0.def.is_agile() {
                quote! {
                    unsafe impl ::std::marker::Send for #name {}
                    unsafe impl ::std::marker::Sync for #name {}
                }
            } else {
                TokenStream::new()
            };

            let bases = self.gen_base_conversions(&name, gen);
            let iterator = gen_iterator(&self.0, &interfaces, gen);

            quote! {
                #[repr(transparent)]
                #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
                pub struct #name(::windows::Object);
                impl #name {
                    #new
                    #methods
                    #async_get
                    #(#factories)*
                }
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = ::std::option::Option<Self>;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#type_signature);
                }
                unsafe impl ::windows::Interface for #name {
                    type Vtable = #default_abi_name;
                    const IID: ::windows::Guid = #guid;
                }
                impl ::windows::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
                #future
                #object
                #(#conversions)*
                #(#bases)*
                #send_sync
                #iterator
            }
        } else {
            quote! {
                pub struct #name {}
                impl #name {
                    #methods
                    #(#factories)*
                }
                impl ::windows::RuntimeName for #name {
                    const NAME: &'static str = #runtime_name;
                }
            }
        }
    }

    fn gen_base_conversions<'a>(
        &'a self,
        from: &'a TokenStream,
        gen: Gen,
    ) -> impl Iterator<Item = TokenStream> + 'a {
        self.0.bases().map(move |base| {
            let into = base.gen_name(gen);

            quote! {
                impl ::std::convert::From<#from> for #into {
                    fn from(value: #from) -> Self {
                        ::std::convert::Into::<#into>::into(&value)
                    }
                }
                impl ::std::convert::From<&#from> for #into {
                    fn from(value: &#from) -> Self {
                        ::windows::Interface::cast(value).unwrap()
                    }
                }
                impl<'a> ::windows::IntoParam<'a, #into> for #from {
                    fn into_param(self) -> ::windows::Param<'a, #into> {
                        ::windows::Param::Owned(::std::convert::Into::<#into>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, #into> for &'a #from {
                    fn into_param(self) -> ::windows::Param<'a, #into> {
                        ::windows::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
                    }
                }
            }
        })
    }

    // TODO: don't generate conversions for exclusive interfaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let c = TypeReader::get_class("Windows.Foundation", "Uri");
        assert_eq!(
            c.type_signature(),
            "rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})"
        )
    }

    #[test]
    fn test_class() {
        let c = TypeReader::get_class("Windows.Foundation.Collections", "StringMap");
        let i = c.interfaces();
        assert_eq!(i.len(), 3);

        assert_eq!(
            i[0].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IMap :: < :: windows :: HString , :: windows :: HString >"
        );
        assert_eq!(i[0].kind, InterfaceKind::Default);

        assert_eq!(
            i[1].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < :: windows :: HString , :: windows :: HString > >"
        );
        assert_eq!(i[1].kind, InterfaceKind::NonDefault);

        assert_eq!(
            i[2].def.gen_name(Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IObservableMap :: < :: windows :: HString , :: windows :: HString >"
        );
        assert_eq!(i[2].kind, InterfaceKind::NonDefault);
    }

    #[test]
    fn test_bases() {
        let c = TypeReader::get_class("Windows.Foundation", "Uri");
        assert_eq!(c.0.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "CompositionObject");
        assert_eq!(c.0.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "Visual");
        let bases: Vec<GenericType> = c.0.bases().collect();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].def.name(), "CompositionObject");

        let c = TypeReader::get_class("Windows.UI.Composition", "SpriteVisual");
        let bases: Vec<GenericType> = c.0.bases().collect();
        assert_eq!(bases.len(), 3);
        assert_eq!(bases[0].def.name(), "ContainerVisual");
        assert_eq!(bases[1].def.name(), "Visual");
        assert_eq!(bases[2].def.name(), "CompositionObject");
    }

    #[test]
    fn test_double_default() {
        // TimedMetadataStreamDescriptor is a wonky class. It has IMediaStreamDescriptor as its default interface, but also implements
        // IMediaStreamDescriptor2 which also implements IMediaStreamDescriptor. This is unfortunately legal but rather unusual and
        // language projections need to be careful not to be confused by this.

        let c = TypeReader::get_class("Windows.Media.Core", "TimedMetadataStreamDescriptor");
        let mut i = c.interfaces();
        assert_eq!(i.len(), 4);

        i.sort_by(|a, b| a.def.def.name().cmp(b.def.def.name()));

        assert_eq!(i[0].def.def.name(), "IMediaStreamDescriptor");
        assert_eq!(i[0].kind, InterfaceKind::Default);

        assert_eq!(i[1].def.def.name(), "IMediaStreamDescriptor2");
        assert_eq!(i[1].kind, InterfaceKind::NonDefault);

        assert_eq!(i[2].def.def.name(), "ITimedMetadataStreamDescriptor");
        assert_eq!(i[2].kind, InterfaceKind::NonDefault);

        assert_eq!(i[3].def.def.name(), "ITimedMetadataStreamDescriptorFactory");
        assert_eq!(i[3].kind, InterfaceKind::Static);
    }
}
