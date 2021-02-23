use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Class(pub GenericType);

impl Class {
    pub fn type_signature(&self) -> String {
        let default = self
            .0
            .interfaces()
            .find(|(_, kind)| *kind == parser::InterfaceKind::Default)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    format!(
                        "Class {}.{} does not have a default interface.",
                        self.0.def.namespace(),
                        self.0.def.name()
                    )
                )
            });

        format!(
            "rc({}.{};{})",
            self.0.def.namespace(),
            self.0.def.name(),
            default.0 .0.interface_signature()
        )
    }

    pub fn bases(&self) -> impl Iterator<Item = Self> + '_ {
        self.0
            .def
            .bases()
            .map(|def| Self(GenericType::from_type_def(def, Vec::new())))
    }

    // TODO: fold into GenericType::interfaces
    pub fn factories(&self) -> impl Iterator<Item = (Interface, parser::InterfaceKind)> + '_ {
        self.0.def.attributes().filter_map(|attribute| {
            match attribute.full_name() {
                ("Windows.Foundation.Metadata", "StaticAttribute")
                | ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
                    for (_, arg) in attribute.args() {
                        if let parser::ConstantValue::TypeDef(def) = arg {
                            return Some((
                                Interface(GenericType::from_type_def(def, Vec::new())),
                                parser::InterfaceKind::Static,
                            ));
                        }
                    }

                    None
                }
                ("Windows.Foundation.Metadata", "ComposableAttribute") => {
                    // One of the arguments is a CompositionType enum and the Public variant
                    // has a value of 2 as a signed 32-bit integer.

                    let mut public = false;
                    let mut interface = None;

                    for (_, arg) in attribute.args() {
                        match arg {
                            parser::ConstantValue::I32(2) => {
                                public = true;
                            }
                            parser::ConstantValue::TypeDef(def) => {
                                interface = Some((
                                    Interface(GenericType::from_type_def(def, Vec::new())),
                                    parser::InterfaceKind::Composable,
                                ));
                            }
                            _ => {}
                        }
                    }

                    if public {
                        interface
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        let generics = self.0.generics.iter().filter_map(|g| g.definition());
        let interfaces = self.0.interfaces().filter_map(|i| i.0.definition());
        let bases = self.bases().filter_map(|b| b.definition());
        let factories = self.factories().filter_map(|(i, _)| i.definition());

        generics
            .chain(interfaces)
            .chain(bases)
            .chain(factories)
            .collect()
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        Some(self.0.def)
    }

    pub fn gen(&self, _: &Gen) -> TokenStream {
        quote! {}
    }
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
        let mut i: Vec<(Interface, parser::InterfaceKind)> = c.0.interfaces().collect();
        assert_eq!(i.len(), 3);

        i.sort_by(|a, b| {
            a.0 .0
                .gen_name(&Gen::Absolute)
                .as_str()
                .cmp(b.0 .0.gen_name(&Gen::Absolute).as_str())
        });

        assert_eq!(
            i[0].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IIterable :: < windows :: foundation :: collections :: IKeyValuePair :: < windows :: HString , windows :: HString > >"
        );

        assert_eq!(i[0].1, InterfaceKind::NonDefault);

        assert_eq!(
            i[1].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IMap :: < windows :: HString , windows :: HString >"
        );

        assert_eq!(i[1].1, InterfaceKind::Default);

        assert_eq!(
            i[2].0.0.gen_name(&Gen::Absolute).as_str(),
            "windows :: foundation :: collections :: IObservableMap :: < windows :: HString , windows :: HString >"
        );

        assert_eq!(i[2].1, InterfaceKind::NonDefault);
    }

    #[test]
    fn test_bases() {
        let c = TypeReader::get_class("Windows.Foundation", "Uri");
        assert_eq!(c.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "CompositionObject");
        assert_eq!(c.bases().count(), 0);

        let c = TypeReader::get_class("Windows.UI.Composition", "Visual");
        let bases: Vec<Class> = c.bases().collect();
        assert_eq!(bases.len(), 1);
        assert_eq!(bases[0].0.def.name(), "CompositionObject");

        let c = TypeReader::get_class("Windows.UI.Composition", "SpriteVisual");
        let bases: Vec<Class> = c.bases().collect();
        assert_eq!(bases.len(), 3);
        assert_eq!(bases[0].0.def.name(), "ContainerVisual");
        assert_eq!(bases[1].0.def.name(), "Visual");
        assert_eq!(bases[2].0.def.name(), "CompositionObject");
    }
}

// #[derive(Debug)]
// pub struct Class {
//     pub name: TypeName,
//     pub bases: Vec<TypeName>,
//     pub interfaces: Vec<RequiredInterface>,
//     pub default_constructor: bool,
//     pub is_agile: bool,
//     pub signature: String,
// }

// impl Class {
//     pub fn from_type_name(name: TypeName) -> Self {
//         let mut interfaces = Vec::new();
//         add_dependencies(&mut interfaces, &name, &name.namespace, false);
//         let mut bases = Vec::new();
//         let mut base = name.def;

//         let signature = if interfaces.iter().any(|i| i.kind == InterfaceKind::Default) {
//             name.class_signature()
//         } else {
//             String::new()
//         };

//         loop {
//             let extends = base.extends();

//             if extends.full_name() == ("System", "Object") {
//                 break;
//             }

//             base = extends.resolve();
//             let base = TypeName::from_type_def(&base, &name.namespace);

//             add_dependencies(&mut interfaces, &base, &name.namespace, true);
//             bases.push(base);
//         }

//         let mut default_constructor = false;
//         let mut is_agile = false;

//         for attribute in name.def.attributes() {
//             match attribute.full_name() {
//                 ("Windows.Foundation.Metadata", "StaticAttribute") => {
//                     add_type(
//                         &mut interfaces,
//                         &attribute_factory(&attribute).unwrap(),
//                         &name.namespace,
//                         InterfaceKind::Statics,
//                     );
//                 }
//                 ("Windows.Foundation.Metadata", "ActivatableAttribute") => {
//                     match attribute_factory(&attribute) {
//                         Some(def) => {
//                             add_type(
//                                 &mut interfaces,
//                                 &def,
//                                 &name.namespace,
//                                 InterfaceKind::Statics,
//                             );
//                         }
//                         None => default_constructor = true,
//                     }
//                 }
//                 ("Windows.Foundation.Metadata", "ComposableAttribute") => {
//                     // One of the arguments is a CompositionType enum and the Public variant
//                     // has a value of 2 as a signed 32-bit integer.
//                     for (_name, arg) in attribute.args() {
//                         if let winmd::AttributeArg::I32(2) = arg {
//                             add_type(
//                                 &mut interfaces,
//                                 &attribute_factory(&attribute).unwrap(),
//                                 &name.namespace,
//                                 InterfaceKind::Composable,
//                             );
//                         }
//                     }
//                 }
//                 ("Windows.Foundation.Metadata", "MarshalingBehaviorAttribute") => {
//                     // The only argument is a MarshalingType enum and the Agile variant
//                     // has a value of 2 as a signed 32-bit integer.
//                     let (_name, arg) = &attribute.args()[0];

//                     if let winmd::AttributeArg::I32(2) = arg {
//                         is_agile = true;
//                     }
//                 }
//                 _ => {}
//             }
//         }

//         rename_collisions(&mut interfaces);

//         Self {
//             name,
//             interfaces,
//             bases,
//             default_constructor,
//             is_agile,
//             signature,
//         }
//     }

//     pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
//         self.interfaces
//             .iter()
//             .flat_map(|i| i.name.dependencies())
//             .chain(self.bases.iter().map(|i| i.def))
//             .collect()
//     }

//     pub fn gen(&self) -> TokenStream {
//         let name = self.name.gen();
//         let type_name = self.type_name(&name);
//         let methods = gen_method(&self.interfaces);
//         let call_factory = self.gen_call_factory();

//         if let Some(default_interface) = self
//             .interfaces
//             .iter()
//             .find(|i| i.kind == InterfaceKind::Default)
//         {
//             let conversions = self
//                 .interfaces
//                 .iter()
//                 .map(|interface| interface.gen_conversions(&name, &TokenStream::new()));

//             let new = if self.default_constructor {
//                 quote! {
//                     pub fn new() -> ::windows::Result<Self> {
//                         Self::IActivationFactory(|f| f.activate_instance::<Self>())
//                     }
//                 }
//             } else {
//                 TokenStream::new()
//             };

//             let bases = self.gen_base_conversions(&name);
//             let iterator = gen_iterator(&self.name, &self.interfaces);
//             let signature = Literal::byte_string(&self.signature.as_bytes());

//             let default_name = default_interface.name.gen();
//             let abi_name = default_interface.name.gen_abi();
//             let (async_get, future) = gen_async(&self.name, &self.interfaces);

//             let send_sync = if self.is_agile {
//                 let constraints = self.name.gen_constraint();
//                 quote! {
//                     unsafe impl<#constraints> ::std::marker::Send for #name {}
//                     unsafe impl<#constraints> ::std::marker::Sync for #name {}
//                 }
//             } else {
//                 TokenStream::new()
//             };

//             quote! {
//                 #[repr(transparent)]
//                 pub struct #name(::windows::Object);
//                 impl #name {
//                     #new
//                     #methods
//                     #async_get
//                     #call_factory
//                 }
//                 impl ::std::clone::Clone for #name {
//                     fn clone(&self) -> Self {
//                         Self(self.0.clone())
//                     }
//                 }
//                 impl ::std::cmp::PartialEq for #name {
//                     fn eq(&self, other: &Self) -> bool {
//                         self.0 == other.0
//                     }
//                 }
//                 impl ::std::cmp::Eq for #name {}
//                 impl ::std::fmt::Debug for #name {
//                     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//                         write!(f, "{:?}", self.0)
//                     }
//                 }
//                 #type_name
//                 unsafe impl ::windows::Interface for #name {
//                     type Vtable = #abi_name;
//                     const IID: ::windows::Guid = <#default_name as ::windows::Interface>::IID;
//                 }
//                 unsafe impl ::windows::RuntimeType for #name {
//                     type DefaultType = ::std::option::Option<Self>;
//                     const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
//                 }
//                 impl ::std::convert::From<#name> for ::windows::Object {
//                     fn from(value: #name) -> Self {
//                         value.0
//                     }
//                 }
//                 impl ::std::convert::From<&#name> for ::windows::Object {
//                     fn from(value: &#name) -> Self {
//                         ::std::convert::From::from(::std::clone::Clone::clone(value))
//                     }
//                 }
//                 impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for #name {
//                     fn into(self) -> ::windows::Param<'a, ::windows::Object> {
//                         ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(self))
//                     }
//                 }
//                 impl<'a> ::std::convert::Into<::windows::Param<'a, ::windows::Object>> for &'a #name {
//                     fn into(self) -> ::windows::Param<'a, ::windows::Object> {
//                         ::windows::Param::Owned(::std::convert::Into::<::windows::Object>::into(::std::clone::Clone::clone(self)))
//                     }
//                 }
//                 #(#conversions)*
//                 #bases
//                 #iterator
//                 #send_sync
//                 #future
//             }
//         } else {
//             quote! {
//                 pub struct #name {}
//                 impl #name {
//                     #methods
//                     #call_factory
//                 }
//                 #type_name
//             }
//         }
//     }

//     pub fn gen_base_conversions(&self, from: &TokenStream) -> TokenStream {
//         TokenStream::from_iter(self.bases.iter().map(|base| {
//             let into = base.gen();
//             quote! {
//                 impl ::std::convert::From<#from> for #into {
//                     fn from(value: #from) -> Self {
//                         ::std::convert::Into::<#into>::into(&value)
//                     }
//                 }
//                 impl ::std::convert::From<&#from> for #into {
//                     fn from(value: &#from) -> Self {
//                         ::windows::Interface::cast(value).unwrap()
//                     }
//                 }
//                 impl<'a> ::std::convert::Into<::windows::Param<'a, #into>> for #from {
//                     fn into(self) -> ::windows::Param<'a, #into> {
//                         ::windows::Param::Owned(::std::convert::Into::<#into>::into(self))
//                     }
//                 }
//                 impl<'a> ::std::convert::Into<::windows::Param<'a, #into>> for &'a #from {
//                     fn into(self) -> ::windows::Param<'a, #into> {
//                         ::windows::Param::Owned(::std::convert::Into::<#into>::into(::std::clone::Clone::clone(self)))
//                     }
//                 }
//             }
//         }))
//     }

//     fn gen_call_factory(&self) -> TokenStream {
//         let mut tokens = TokenStream::new();

//         if self.default_constructor {
//             let interface_tokens = quote! { ::windows::IActivationFactory };
//             tokens.combine(&self.to_named_call_factory("IActivationFactory", &interface_tokens));
//         }

//         for interface in &self.interfaces {
//             if (interface.kind != InterfaceKind::Statics
//                 && interface.kind != InterfaceKind::Composable)
//                 || interface.methods.is_empty()
//             {
//                 continue;
//             }

//             let interface_namespace =
//                 gen_namespace(&interface.name.namespace, &self.name.namespace);

//             let interface_name = to_ident(&interface.name.name);
//             let interface_tokens = quote! { #interface_namespace #interface_name };
//             tokens.combine(&self.to_named_call_factory(&interface.name.name, &interface_tokens));
//         }

//         tokens
//     }

//     fn to_named_call_factory(&self, method_name: &str, interface: &TokenStream) -> TokenStream {
//         let self_name = self.name.gen();
//         let method_name = to_ident(method_name);

//         quote! {
//             #[allow(non_snake_case)]
//             fn #method_name<R, F: FnOnce(&#interface) -> ::windows::Result<R>>(
//                 callback: F,
//             ) -> ::windows::Result<R> {
//                 static mut SHARED: ::windows::FactoryCache<#self_name, #interface> =
//                     ::windows::FactoryCache::new();
//                 unsafe { SHARED.call(callback) }
//             }
//         }
//     }

//     fn type_name(&self, class_name: &TokenStream) -> TokenStream {
//         let runtime_name = self.name.runtime_name();

//         quote! {
//             impl ::windows::RuntimeName for #class_name {
//                 const NAME: &'static str = #runtime_name;
//             }
//         }
//     }
// }

// fn attribute_factory(attribute: &winmd::Attribute) -> Option<winmd::TypeDef> {
//     for (_, arg) in attribute.args() {
//         if let winmd::AttributeArg::TypeDef(def) = arg {
//             return Some(def);
//         }
//     }

//     None
// }
