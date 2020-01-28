use crate::*;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::collections::*;
use std::iter::FromIterator;

#[derive(Default)]
struct Namespace {
    types: TokenStream,
    namespaces: BTreeMap<String, Namespace>,
}

impl Namespace {
    fn write(&self) -> TokenStream {
        let types = &self.types;
        let namespaces = self.namespaces.write_namespaces();

        quote! {
            #types
            #namespaces
        }
    }
}

trait NamespaceWriter {
    fn insert_namespace(&mut self, namespace: &str, types: TokenStream);
    fn write_namespaces(&self) -> TokenStream;
}

impl NamespaceWriter for BTreeMap<String, Namespace> {
    fn insert_namespace(&mut self, namespace: &str, types: TokenStream) {
        if let Some(pos) = namespace.find('.') {
            self.entry(namespace[..pos].to_string()).or_insert_with(|| Default::default()).namespaces.insert_namespace(&namespace[pos + 1..], types);
        } else {
            self.entry(namespace.to_string()).or_insert_with(|| Default::default()).types = types;
        }
    }

    fn write_namespaces(&self) -> TokenStream {
        let mut tokens = Vec::new();

        for (name, namespace) in self {
            let name = format_ident!("{}", name.to_lowercase());
            let namespace = namespace.write();

            tokens.push(quote! {
                pub mod #name {
                    #namespace
                }
            });
        }

        TokenStream::from_iter(tokens)
    }
}

pub struct RustWriter {
    r: Reader,
    limits: BTreeSet<String>,
}

impl RustWriter {
    pub fn new() -> RustWriter {
        RustWriter { r: Reader::from_os().unwrap(), limits: BTreeSet::new() }
    }

    pub fn from_files<'a, P: IntoIterator<Item = &'a String>>(filenames: P) -> RustWriter {
        RustWriter { r: Reader::from_files(filenames).unwrap(), limits: BTreeSet::new() }
    }

    pub fn add_namespace(&mut self, mut namespace: &str) {
        if let Some(found) = self.r.namespaces().keys().find(|name| name.to_lowercase() == namespace) {
            let mut namespace = found.as_str();
            self.limits.insert(namespace.to_string());

            while let Some(index) = namespace.rfind('.') {
                namespace = namespace.get(0..index).unwrap();

                if self.r.namespaces().contains_key(namespace) {
                    self.limits.insert(namespace.to_string());
                }
            }
        } else {
            panic!("Namespace `{}` not found in winmd files", namespace);
        }
    }

    pub fn write(&self) -> TokenStream {
        Writer::write(&self.r, &self.limits)
    }
}

struct GenericGuard<'a> {
    generics: &'a mut Vec<Vec<TokenStream>>,
    count: usize
}

impl<'a> Drop for GenericGuard<'a> {
    fn drop(&mut self) {
        self.generics.resize(self.generics.len() - self.count, Vec::new());
    }
}

struct Writer<'a> {
    pub r: &'a Reader,
    pub namespace: &'a str,
    pub limits: &'a BTreeSet<String>,
    pub generics: Vec<Vec<TokenStream>>,
}

impl<'a> Writer<'a> {
    pub fn write(r: &Reader, limits: &BTreeSet<String>) -> TokenStream {
        let mut namespaces = BTreeMap::new();

        // TODO: parallalelize this loop
        for namespace in limits {
            let mut w = Writer { r, namespace, limits, generics: Default::default() };
            namespaces.insert_namespace(namespace, w.write_namespace(namespace));
        }

        namespaces.write_namespaces()
    }

    fn write_namespace(&mut self, namespace: &str) -> TokenStream {
        let mut tokens = Vec::new();

        for t in self.r.namespace_types(namespace) {
            tokens.push(match t.category(self.r) {
                TypeCategory::Interface => self.write_interface(t),
                TypeCategory::Class => self.write_class(t),
                TypeCategory::Enum => self.write_enum(t),
                TypeCategory::Struct => self.write_struct(t),
                TypeCategory::Delegate => self.write_delegate(t),
                _ => continue,
            });
        }

        TokenStream::from_iter(tokens)
    }

    fn write_class(&mut self, class: &TypeDef) -> TokenStream {
        // TODO: don't define struct here if the class is static - only declare.

        // if class.name(self.r) == "PropertyValue" {
        //     return TokenStream::new();
        // }
        if class.name(self.r) == "StringMap" {
            return TokenStream::new();
        }
        if class.name(self.r) == "StringMap" {
            return TokenStream::new();
        }

        let name = format_ident!("{}", class.name(self.r));
        let functions = self.write_class_functions(class);
        let string_name = format!("{}.{}", class.namespace(self.r), class.name(self.r));
        let interfaces = self.interfaces(class);
        let froms = self.write_from_traits(&name, &interfaces);

        // TODO: use bool.then when stable
        if let Some(default_interface) = interfaces.iter().find(|interface| interface.default) {
            
            let default_interface = self.write_type_def(&default_interface.definition);

            quote! {
                #[repr(C)]
                #[derive(Default, Clone)]
                pub struct #name { ptr: winrt::ComPtr }
                impl #name { #functions }
                impl winrt::ClassType for #name {}
                impl winrt::QueryType for #name {
                    fn type_guid() -> &'static winrt::Guid {
                        <#default_interface as winrt::QueryType>::type_guid()
                    }
                }
                impl winrt::TypeName for #name {
                    fn type_name() -> &'static str {
                        #string_name
                    }
                }
                impl winrt::RuntimeType for #name {
                    type Abi = winrt::RawPtr;
                    fn abi(&self) -> Self::Abi {
                        self.ptr.get()
                    }
                    fn set_abi(&mut self) -> *mut Self::Abi {
                        self.ptr.set()
                    }
                }
                impl<'a> Into<winrt::Param<'a, #name>> for #name {
                    fn into(self) -> winrt::Param<'a, #name> {
                        winrt::Param::Value(self)
                    }
                }
                impl<'a> Into<winrt::Param<'a, #name>> for &'a #name {
                    fn into(self) -> winrt::Param<'a, #name> {
                        winrt::Param::Ref(self)
                    }
                }
                #froms
            }
        } else {
            quote! {
                pub struct #name { }
                impl #name { #functions }
                impl winrt::TypeName for #name {
                    fn type_name() -> &'static str {
                        #string_name
                    }
                }
            }
        }
    }

    fn write_from_traits(&mut self, from: &Ident, interfaces: &Vec<InterfaceInfo>) -> TokenStream {
        let mut tokens = Vec::<TokenStream>::new();

        for interface in interfaces {
            // TODO: support generic interfaces
            if !interface.generics.is_empty() {
                continue;
            }

            let namespace = self.write_namespace_name(interface.definition.namespace(self.r));
            let name = format_ident!("{}", interface.definition.name(self.r));
            let into = quote! { #namespace#name };

            if interface.default {
                tokens.push(quote! {
                    impl From<#from> for #into {
                        fn from(value: #from) -> #into {
                            unsafe { std::mem::transmute(value) }
                        }
                    }
                    impl From<&#from> for #into {
                        fn from(value: &#from) -> #into {
                            unsafe { std::mem::transmute(value.clone()) }
                        }
                    }
                    impl<'a> Into<winrt::Param<'a, #into>> for #from {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                    impl<'a> Into<winrt::Param<'a, #into>> for &'a #from {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                });
            } else {
                tokens.push(quote! {
                    impl From<#from> for #into {
                        fn from(value: #from) -> #into {
                            #into::from(&value)
                        }
                    }
                    impl From<&#from> for #into {
                        fn from(value: &#from) -> #into {
                            winrt::QueryType::query(value)
                        }
                    }
                    impl<'a> Into<winrt::Param<'a, #into>> for #from {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                    impl<'a> Into<winrt::Param<'a, #into>> for &'a #from {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                });
            }
        }

        TokenStream::from_iter(tokens)
    }

    fn write_class_functions(&mut self, class: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        for attribute in class.attributes(self.r) {
            let (_, name) = attribute.name(self.r);

            if name == "StaticAttribute" {
                let interface = self.factory_type(&attribute).unwrap();
                tokens.push(self.write_class_statics(class, &interface));
            } else if name == "ActivatableAttribute" {
                if let Some(interface) = self.factory_type(&attribute) {
                    tokens.push(self.write_class_statics(class, &interface));
                } else {
                    // TODO: code default constructor "new"
                }
            }
        }

        for interface in self.interfaces(class) {
            // TODO: this needs some kind of scope guard to push and pop automatically
            // Not sure how to do that since self is already a &mut and we need to use it below
            let count = interface.generics.len();

            if count > 0 {
                self.generics.append(&mut interface.generics.clone());
            }

            if interface.default {
                tokens.push(self.write_consume_methods(&interface.definition));
            } else {
                // TODO: write forwarding consume methods for non-default interfaces
                // e.g. self.into::<IOther>().method()
            }

            if count > 0 {
                self.generics.resize(self.generics.len() - count, Vec::new());
            }
        }

        // TODO: 1. write default interface consume methods directly into class impl (not calling through default interface)

        // TODO: 2. write non-default interface consume methods as calls through those interfaces e.g.
        // self.into<INonDefault>().NonDefaultMethod()

        TokenStream::from_iter(tokens)
    }

    fn write_class_statics(&mut self, class: &TypeDef, interface: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        let class_name = format_ident!("{}", class.name(self.r));
        let interface_name = format_ident!("{}", interface.name(self.r));

        for method in interface.methods(self.r) {
            let method_name = self.write_method_name(&method);
            let signature = method.signature(self.r);
            let params = self.write_consume_params(&signature);
            let into_params = self.write_consume_into_params(&signature);
            let args = signature.params().iter().map(|param| format_ident!("{}", param.name()));

            if let Some(result) = signature.return_type() {
                let result = self.write_type(result.sig_type());

                tokens.push(quote! {
                    pub fn #method_name<#into_params>(#params) -> winrt::Result<#result> {
                        winrt::factory::<#class_name, #interface_name>()?.#method_name(#(#args),*)
                    }
                });
            } else {
                tokens.push(quote! {
                    pub fn #method_name<#into_params>(#params) -> winrt::Result<()> {
                            panic!();
                    }
                });
            };
        }

        TokenStream::from_iter(tokens)
    }

    fn write_guid(&mut self, t: &TypeDef) -> TokenStream {
        let guid = t.find_attribute(self.r, "Windows.Foundation.Metadata", "GuidAttribute").unwrap();
        let args = guid.arguments(self.r);

        let mut iter = args.iter().map(|(_, value)| match value {
            ArgumentSig::U8(value) => Literal::u8_unsuffixed(*value),
            ArgumentSig::U16(value) => Literal::u16_unsuffixed(*value),
            ArgumentSig::U32(value) => Literal::u32_unsuffixed(*value),
            _ => panic!(),
        });

        let three = iter.by_ref().take(3);

        quote! {
            #(#three,)* &[#(#iter),*],
        }
    }

    // fn write_required_interface(&mut self, info: &InterfaceInfo) -> TokenStream {
    //     let count = info.generics.len();

    //     if count > 0 {
    //         self.generics.append(&mut info.generics.clone());
    //     }

    //     if interface.default {
    //         tokens.push(self.write_consume_methods(&interface.definition));
    //     } else {
    //         // TODO: write forwarding consume methods for non-default interfaces
    //         // e.g. self.into::<IOther>().method()
    //     }

    //     if count > 0 {
    //         self.generics.resize(self.generics.len() - count, Vec::new());
    //     }
    // }

    fn write_interface(&mut self, interface: &TypeDef) -> TokenStream {
        // tODO: should be able to code this entire function as:
        //      let guard = self.push_generic_params(generics);
        //      self.write_generic_interface(interface)
        // done - just don't know how to get the lifetimes to work.
        
        let generics = interface.generics(self.r);

        if generics.is_empty() {
            self.write_generic_interface(interface)
        } else {
            self.push_generic_params(generics);
            let tokens = self.write_generic_interface(interface);
            self.generics.pop();
            tokens
        }
    }

    fn write_generic_interface(&mut self, interface: &TypeDef) -> TokenStream {
        let guid = self.write_guid(interface);
        let phantoms = self.write_generic_phantoms();
        let abi_methods = self.write_abi_methods(&interface);
        let consume_methods = self.write_consume_methods(&interface);

        let generics = self.write_generics();
        let constraints = self.write_generic_constraints();
        let name = self.write_generic_name(interface);
        let abi_name = self.write_generic_abi_name(interface);

        quote! {
            #[repr(C)]
            #[derive(Default, Clone)]
            pub struct #name<#constraints> { ptr: winrt::ComPtr, #phantoms }
            #[repr(C)]
            struct #abi_name<#constraints> {
                __0: usize,
                __1: usize,
                __2: usize,
                __3: usize,
                __4: usize,
                __5: usize,
                #abi_methods
                #phantoms
            }
            impl<#constraints> #name<#generics> {
                #consume_methods
            }
            impl<#constraints> winrt::InterfaceType for #name<#generics> {}
            impl<#constraints> winrt::QueryType for #name<#generics> {
                fn type_guid() -> &'static winrt::Guid {
                    static GUID: winrt::Guid = winrt::Guid::from_values(
                        #guid
                    );
                    &GUID
                }
            }
            impl<#constraints> winrt::RuntimeType for #name<#generics> {
                type Abi = winrt::RawPtr;
                fn abi(&self) -> Self::Abi {
                    self.ptr.get()
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self.ptr.set()
                }
            }
            impl<'a, #constraints> Into<winrt::Param<'a, #name<#generics>>> for #name<#generics> {
                fn into(self) -> winrt::Param<'a, #name<#generics>> {
                    winrt::Param::Value(self)
                }
            }
            impl<'a, #constraints> Into<winrt::Param<'a, #name<#generics>>> for &'a #name<#generics> {
                fn into(self) -> winrt::Param<'a, #name<#generics>> {
                    winrt::Param::Ref(self)
                }
            }
        }
    }

    fn write_abi_methods(&mut self, interface: &TypeDef) -> TokenStream {
        let mut tokens = quote! {};

        for method in interface.methods(self.r) {
            let name = method.name(self.r);
            if name == ".ctor" {
                continue;
            }
            let name = self.write_method_name(&method);
            let params = self.write_abi_params(&method.signature(self.r));
            tokens = quote! {
                #tokens
                #name: extern "system" fn(winrt::RawPtr, #params) -> winrt::ErrorCode,
            };
        }

        tokens
    }

    fn write_consume_methods(&mut self, interface: &TypeDef) -> TokenStream {
        let mut tokens = quote! {};
        let generics = self.write_generics();
        let abi_name = self.write_generic_abi_name(interface);

        for method in interface.methods(self.r) {
            let name = method.name(self.r);
            if name == ".ctor" {
                continue;
            }

            if method.name(self.r) == "GetMany" {
                continue;
            }

            if method.is_remove_overload(self.r) {
                // We don't project this method at all - the ABI is called internally by the EventGuard
                continue;
            }
            if method.is_add_overload(self.r) {
                // TODO: define this using an EventToken<T> return type
                continue;
            }

            let name = self.write_method_name(&method);
            let signature = method.signature(self.r);
            let params = self.write_consume_params(&signature);
            let into_params = self.write_consume_into_params(&signature);
            let args = self.write_abi_args(&signature);

            if let Some(result) = signature.return_type() {
                let projected_result = self.write_type(result.sig_type());
                let receive_expression = self.write_consume_receive_expression(result.sig_type());

                tokens = quote! {
                    #tokens
                    pub fn #name<#into_params>(&self, #params) -> winrt::Result<#projected_result> {
                        unsafe {
                            let mut __ok = std::mem::zeroed();
                            ((*(*(self.ptr.get() as *const *const #abi_name<#generics>))).#name)(
                                self.ptr.get(), #args #receive_expression
                            )
                            .ok_or(std::mem::transmute_copy(&__ok))
                        }
                    }
                };
            } else {
                tokens = quote! {
                    #tokens
                    pub fn #name<#into_params>(&self, #params) -> winrt::Result<()> {
                        unsafe {
                            ((*(*(self.ptr.get() as *const *const #abi_name<#generics>))).#name)(
                                self.ptr.get(), #args
                            )
                            .ok()
                        }
                    }
                };
            };
        }

        tokens
    }

    fn write_consume_receive_expression(&mut self, value: &TypeSig) -> TokenStream {
        let projected_type = self.write_type(value);
        match value.category(self.r) {
            ParamCategory::Generic => {
                quote! {
                        <#projected_type as winrt::RuntimeType>::set_abi(&mut __ok)
                }
            }
            ParamCategory::Array => {
                quote! { winrt::Array::<#projected_type>::set_abi_len(&mut __ok), winrt::Array::<#projected_type>::set_abi(&mut __ok), }
            }
            _ => quote! {
                &mut __ok
            },
        }
    }

    fn write_enum(&mut self, t: &TypeDef) -> TokenStream {
        let name = format_ident!("{}", t.name(self.r));
        let fields = self.write_enum_fields(&t);
        let first = format_ident!("{}", t.fields(self.r).skip(1).next().unwrap().name(self.r));
        quote! {
            pub enum #name { #fields }
            impl Default for #name {
                fn default() -> Self {
                    Self::#first
                }
            }
        }
    }

    fn write_enum_fields(&mut self, t: &TypeDef) -> TokenStream {
        let mut tokens = quote! {};

        for f in t.fields(self.r) {
            for _c in f.constants(self.r) {
                let name = format_ident!("{}", f.name(self.r));
                //let value = c.value();

                tokens = quote! {
                    #tokens
                    #name,
                    // TODO: write out the enum value
                };
            }
        }

        tokens
    }

    fn write_delegate(&mut self, interface: &TypeDef) -> TokenStream {
        let generics = interface.generics(self.r);

        if generics.is_empty() {
            self.write_generic_delegate(interface)
        } else {
            self.push_generic_params(generics);
            let tokens = self.write_generic_delegate(interface);
            self.generics.pop();
            tokens
        }
    }

    fn write_generic_params(&mut self) -> TokenStream {
        let generics = self.generics.last().unwrap();

        let mut tokens = Vec::new();

        for generic in generics {
            tokens.push(quote! { #generic : winrt::RuntimeType })
        }

        quote! { <#(#tokens),*> }
    }

    fn write_generic_phantoms(&mut self) -> TokenStream {
        if let Some(generics) = self.generics.last() {
            let mut tokens = Vec::new();

            for (count, generic) in generics.iter().enumerate() {
                let name = format_ident!("__{}", count + 6);
                tokens.push(quote! { #name: std::marker::PhantomData::<#generic>, })
            }

            TokenStream::from_iter(tokens)
        } else {
            TokenStream::new()
        }
    }

    fn write_generic_impl(&self, interface: &TypeDef) -> TokenStream {
        if let Some(generics) = self.generics.last() {
            let mut tokens = Vec::new();

            for generic in generics {
                tokens.push(quote! { #generic : winrt::RuntimeType })
            }

            quote! { impl <#(#tokens),*> }
        } else {
            quote! { impl }
        }
    }

    fn write_generics(&self) -> TokenStream {
        if let Some(generics) = self.generics.last() {
            quote! { #(#generics),* }
        } else {
            TokenStream::new()
        }
    }

    fn write_generic_constraints(&self) -> TokenStream {
        let mut tokens = Vec::new();

        if let Some(generics) = self.generics.last() {
            for generic in generics {
                tokens.push(quote! { #generic : winrt::RuntimeType, })
            }
        }

        TokenStream::from_iter(tokens)
    }

    fn write_generic_name(&self, interface: &TypeDef) -> TokenStream {
        if let Some(_) = self.generics.last() {
            let name = interface.name(self.r);
            let name = &name[..name.len() - 2];
            let name = format_ident!("{}", name);
            quote! { #name }
        } else {
            let name = interface.name(self.r);
            let name = format_ident!("{}", name);
            quote! { #name }
        }
    }

    fn write_generic_abi_name(&self, interface: &TypeDef) -> TokenStream {
        if let Some(_) = self.generics.last() {
            let name = interface.name(self.r);
            let name = &name[..name.len() - 2];
            let name = format_ident!("abi_{}", name);
            quote! { #name }
        } else {
            let name = interface.name(self.r);
            let name = format_ident!("abi_{}", name);
            quote! { #name }
        }
    }

    fn write_generic_delegate(&mut self, interface: &TypeDef) -> TokenStream {
        let guid = self.write_guid(interface);
        let phantoms = self.write_generic_phantoms();
        let abi_methods = self.write_abi_methods(&interface);
        let consume_methods = self.write_consume_methods(&interface);

        let generics = self.write_generics();
        let constraints = self.write_generic_constraints();
        let name = self.write_generic_name(interface);
        let abi_name = self.write_generic_abi_name(interface);

        quote! {
            #[repr(C)]
            #[derive(Default, Clone)]
            pub struct #name<#constraints> { ptr: winrt::ComPtr, #phantoms }
            #[repr(C)]
            struct #abi_name<#constraints> {
                __0: usize,
                __1: usize,
                __2: usize,
                #abi_methods
                #phantoms
            }
            impl<#constraints> #name<#generics> {
                #consume_methods
            }
            impl<#constraints> winrt::DelegateType for #name<#generics> {}
            impl<#constraints> winrt::QueryType for #name<#generics> {
                fn type_guid() -> &'static winrt::Guid {
                    static GUID: winrt::Guid = winrt::Guid::from_values(
                        #guid
                    );
                    &GUID
                }
            }
            impl<#constraints> winrt::RuntimeType for #name<#generics> {
                type Abi = winrt::RawPtr;
                fn abi(&self) -> Self::Abi {
                    self.ptr.get()
                }
                fn set_abi(&mut self) -> *mut Self::Abi {
                    self.ptr.set()
                }
            }
            impl<'a, #constraints> Into<winrt::Param<'a, #name<#generics>>> for #name<#generics> {
                fn into(self) -> winrt::Param<'a, #name<#generics>> {
                    winrt::Param::Value(self)
                }
            }
            impl<'a, #constraints> Into<winrt::Param<'a, #name<#generics>>> for &'a #name<#generics> {
                fn into(self) -> winrt::Param<'a, #name<#generics>> {
                    winrt::Param::Ref(self)
                }
            }
        }
    }

    fn write_struct(&mut self, t: &TypeDef) -> TokenStream {
        // TODO: skip EventRegistrationToken

        let name = format_ident!("{}", t.name(self.r));
        let fields = self.write_struct_fields(&t);
        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, Default, Debug, PartialEq)]
            pub struct #name { #fields }
            impl winrt::RuntimeCopy for #name {}
        }
        // TODO: RuntimeType for non-blittable structs needs to be customized
    }

    fn write_struct_fields(&mut self, t: &TypeDef) -> TokenStream {
        let mut tokens = quote! {};

        for f in t.fields(self.r) {
            let name = format_ident!("r#{}", to_snake(f.name(self.r)));

            tokens = quote! {
                #tokens
                pub #name: u8,
                // TODO: write out field type
            };
        }

        tokens
    }

    //
    // write_abi_params
    //

    fn write_abi_params(&mut self, signature: &MethodSig) -> TokenStream {
        let mut tokens = Vec::new();

        for param in signature.params() {
            tokens.push(self.write_abi_param(param));
        }

        if let Some(param) = signature.return_type() {
            tokens.push(self.write_abi_param(param));
        }

        TokenStream::from_iter(tokens)
    }

    fn write_abi_param(&mut self, param: &ParamSig) -> TokenStream {
        let tokens = match param.sig_type().sig_type() {
            TypeSigType::ElementType(value) => self.write_abi_param_element_type(value),
            TypeSigType::TypeDefOrRef(value) => self.write_abi_param_type(value),
            TypeSigType::GenericSig(value) => quote! { winrt::RawPtr, },
            TypeSigType::GenericTypeIndex(value) => self.write_abi_param_generic_index(*value),
        };

        if param.array() {
            if param.input() {
                quote! { u32, *const #tokens }
            } else if param.by_ref() {
                quote! { *mut u32, *mut *mut #tokens }
            } else {
                quote! { u32, *mut #tokens }
            }
        } else if param.input() {
            tokens
        } else {
            quote! { *mut #tokens }
        }
    }

    fn write_abi_param_element_type(&mut self, value: &ElementType) -> TokenStream {
        match value {
            ElementType::Bool => quote! { bool, },
            ElementType::Char => quote! { u16, },
            ElementType::I8 => quote! { i8, },
            ElementType::U8 => quote! { u8, },
            ElementType::I16 => quote! { i16, },
            ElementType::U16 => quote! { u16, },
            ElementType::I32 => quote! { i32, },
            ElementType::U32 => quote! { u32, },
            ElementType::I64 => quote! { i64, },
            ElementType::U64 => quote! { u64, },
            ElementType::F32 => quote! { f32, },
            ElementType::F64 => quote! { f64, },
            ElementType::String => quote! { winrt::RawPtr, },
            ElementType::Object => quote! { winrt::RawPtr, },
        }
    }

    fn write_abi_param_type(&mut self, value: &TypeDefOrRef) -> TokenStream {
        match value {
            TypeDefOrRef::TypeDef(value) => self.write_abi_param_type_def(value),
            TypeDefOrRef::TypeRef(value) => self.write_abi_param_type_ref(value),
            _ => panic!("write_abi_param_type"),
        }
    }

    fn write_abi_param_type_def(&mut self, value: &TypeDef) -> TokenStream {
        match value.category(self.r) {
            TypeCategory::Enum => {
                let name = format_ident!("{}", value.name(self.r));
                quote! { #name, }
            }
            TypeCategory::Struct => {
                let name = value.name(self.r);
                let namespace = value.namespace(self.r);
                if name == "EventRegistrationToken" && namespace == "Windows.Foundation" {
                    quote! { i64, }
                } else {
                    let name = format_ident!("{}", value.name(self.r));
                    quote! { #name, }
                }
            }
            _ => quote! { winrt::RawPtr, },
        }
    }

    fn write_abi_param_type_ref(&mut self, value: &TypeRef) -> TokenStream {
        if value.name(self.r) == "Guid" && value.namespace(self.r) == "System" {
            quote! { winrt::Guid, }
        } else {
            self.write_abi_param_type_def(&value.resolve(self.r))
        }
    }

    fn write_abi_param_generic_index(&mut self, value: u32) -> TokenStream {
        let last = self.generics.last().unwrap();
        let type_param = &last[value as usize];

        quote! { <#type_param as winrt::RuntimeType>::Abi, }
    }

    //
    // write_consume_params
    //

    fn write_consume_into_params(&self, signature: &MethodSig) -> TokenStream {
        let mut tokens = Vec::<TokenStream>::new();

        // TODO: don't do convertible for array params

        for (count, param) in signature.params().iter().enumerate() {
            if param.array() {
                continue;
            }

            // TODO: use ParamCategory here

            let type_param = format_ident!("__{}", count);

            if let TypeSigType::ElementType(ElementType::String) = param.sig_type().sig_type() {
                if param.input() {
                    tokens.push(quote! { #type_param: Into<winrt::StringParam<'a>>,});
                }
            }

            // TODO: handle other convertible input types...
        }

        if !tokens.is_empty() {
            tokens.insert(0, quote! {'a,});
        }

        TokenStream::from_iter(tokens)
    }

    fn write_consume_params(&mut self, signature: &MethodSig) -> TokenStream {
        let mut tokens = Vec::new();

        for (count, param) in signature.params().iter().enumerate() {
            let name = format_ident!("{}", param.name());
            tokens.push(quote! { #name: });
            tokens.push(self.write_consume_param(count, param));
        }

        TokenStream::from_iter(tokens)
    }

    fn write_consume_param(&mut self, count: usize, param: &ParamSig) -> TokenStream {
        let tokens = self.write_type(param.sig_type());

        if param.array() {
            if param.input() {
                quote! { &[#tokens], }
            } else if param.by_ref() {
                quote! { &mut winrt::Array<#tokens>, }
            } else {
                quote! { &mut [#tokens], }
            }
        } else if param.input() {
            match param.sig_type().category(self.r) {
                ParamCategory::String => {
                    let type_param = format_ident!("__{}", count);
                    quote! { #type_param, }
                }
                ParamCategory::Primitive => quote! { #tokens, },
                ParamCategory::Enum => quote! { #tokens, },
                _ => quote! { &#tokens, },
            }
        } else {
            quote! { &mut #tokens, }
        }
    }

    //
    // write_abi_args
    //

    fn write_abi_args(&mut self, signature: &MethodSig) -> TokenStream {
        let mut tokens = Vec::new();

        for param in signature.params() {
            tokens.push(self.write_abi_arg(param));
        }

        TokenStream::from_iter(tokens)
    }

    fn write_abi_arg(&mut self, param: &ParamSig) -> TokenStream {
        let name = format_ident!("{}", param.name());
        let category = param.sig_type().category(self.r);

        if param.array() {
            if param.input() {
                quote! { #name.len() as u32, std::mem::transmute(#name.as_ptr()), }
            } else if param.by_ref() {
                quote! { #name.set_abi_len(), #name.set_abi(), }
            } else {
                quote! { #name.len(), #name.set_abi(), }
            }
        } else if param.input() {
            match category {
                ParamCategory::Enum | ParamCategory::Primitive => quote! { #name, },
                ParamCategory::String => quote! { #name.into().abi(), },
                _ => quote! { winrt::RuntimeType::abi(#name), },
            }
        } else {
            match category {
                ParamCategory::Enum | ParamCategory::Primitive | ParamCategory::Struct => quote! { #name, },
                _ => quote! { winrt::RuntimeType::set_abi(#name), },
            }
        }
    }

    //
    // write_type
    //

    fn write_type(&mut self, value: &TypeSig) -> TokenStream {
        match value.sig_type() {
            TypeSigType::ElementType(value) => self.write_type_element(value),
            TypeSigType::TypeDefOrRef(value) => self.write_type_def_or_ref(value),
            TypeSigType::GenericSig(value) => self.write_type_generic(value),
            TypeSigType::GenericTypeIndex(value) => self.write_type_generic_index(*value),
        }
    }

    fn write_type_element(&mut self, value: &ElementType) -> TokenStream {
        match value {
            ElementType::Bool => quote! { bool },
            ElementType::Char => quote! { u16 },
            ElementType::I8 => quote! { i8 },
            ElementType::U8 => quote! { u8 },
            ElementType::I16 => quote! { i16 },
            ElementType::U16 => quote! { u16 },
            ElementType::I32 => quote! { i32 },
            ElementType::U32 => quote! { u32 },
            ElementType::I64 => quote! { i64 },
            ElementType::U64 => quote! { u64 },
            ElementType::F32 => quote! { f32 },
            ElementType::F64 => quote! { f64 },
            ElementType::String => quote! { winrt::String },
            ElementType::Object => quote! { winrt::Object },
        }
    }

    fn write_type_def_or_ref(&mut self, value: &TypeDefOrRef) -> TokenStream {
        match value {
            TypeDefOrRef::TypeDef(value) => self.write_type_def(value),
            TypeDefOrRef::TypeRef(value) => self.write_type_ref(value),
            _ => panic!("write_type_def_or_ref"),
        }
    }

    fn write_type_def(&mut self, value: &TypeDef) -> TokenStream {
        let namespace = self.write_namespace_name(value.namespace(self.r));
        let name = format_ident!("{}", value.name(self.r));
        quote! { #namespace#name }
    }

    fn write_type_ref(&mut self, value: &TypeRef) -> TokenStream {
        if value.name(self.r) == "Guid" && value.namespace(self.r) == "System" {
            quote! { winrt::Guid }
        } else {
            self.write_type_def(&value.resolve(self.r))
        }
    }

    fn write_type_generic(&mut self, value: &GenericSig) -> TokenStream {
        let namespace = self.write_namespace_name(value.sig_type().namespace(self.r));
        let name = value.sig_type().name(self.r);
        let name = name.get(..name.len() - 2).unwrap();
        let name = format_ident!("{}", name);
        let mut args = Vec::new();

        for arg in value.args() {
            args.push(self.write_type(arg));
        }

        quote! {
            #namespace#name<#(#args),*>
        }
    }

    fn write_type_generic_index(&mut self, value: u32) -> TokenStream {
        let last = self.generics.last().unwrap();
        let param = &last[value as usize];
        quote! { #param }
    }

    //
    // Helpers
    //

    fn factory_type(&mut self, attribute: &CustomAttribute) -> Option<TypeDef> {
        for (_, sig) in attribute.arguments(self.r) {
            if let ArgumentSig::TypeDef(interface) = sig {
                return Some(interface);
            }
        }

        None
    }

    fn write_method_name(&self, method: &MethodDef) -> TokenStream {
        let mut source = method.name(self.r);
        let mut result = String::with_capacity(source.len() + 2); // TODO: why 2 again?

        if method.flags(self.r).special() {
            if source.starts_with("get") || source.starts_with("add") {
                source = &source[4..];
            } else if source.starts_with("put") {
                result.push_str("set");
                source = &source[4..];
            } else if source.starts_with("remove") {
                result.push_str("remove");
                source = &source[7..];
            }
        }

        append_snake(&mut result, source);

        let name = format_ident!("r#{}", result);
        quote! { #name }
    }

    fn write_namespace_name(&mut self, other: &str) -> TokenStream {
        let mut tokens = Vec::new();

        let mut source = self.namespace.split('.').peekable();
        let mut destination = other.split('.').peekable();

        while source.peek() == destination.peek() {
            if source.next().is_none() {
                break;
            }
            destination.next();
        }

        let count = source.count();

        if count > 0 {
            tokens.resize(tokens.len() + count, quote! {super::});
        }

        tokens.extend(destination.map(|destination| {
            let destination = format_ident!("{}", destination.to_lowercase());
            quote! { #destination:: }
        }));

        TokenStream::from_iter(tokens)
    }

    fn push_generic_params(&mut self, generics: RowIterator<GenericParam>) {
        self.generics.push(
            generics
                .map(|g| {
                    let name = format_ident!("{}", g.name(self.r));
                    quote! { #name }
                })
                .collect(),
        );
    }

    // fn push_generic_params2(&mut self, generics: RowIterator<GenericParam>) -> GenericGuard {
    //     self.generics.push(
    //         generics
    //             .map(|g| {
    //                 let name = format_ident!("{}", g.name(self.r));
    //                 quote! { #name }
    //             })
    //             .collect(),
    //     );

    //     GenericGuard { generics:&mut self.generics, count: 1 }
    // }

    fn add_interfaces(&mut self, result: &mut Vec<InterfaceInfo>, parent_generics: &Vec<Vec<TokenStream>>, children: RowIterator<InterfaceImpl>) {
        for i in children {
            let default = i.has_attribute(self.r, "Windows.Foundation.Metadata", "DefaultAttribute");
            let overridable = i.has_attribute(self.r, "Windows.Foundation.Metadata", "OverridableAttribute");
            let mut generics = parent_generics.to_vec();

            let mut pop_generics = false;

            let definition = match i.interface(self.r) {
                TypeDefOrRef::TypeDef(value) => value,
                TypeDefOrRef::TypeRef(value) => value.resolve(self.r),
                TypeDefOrRef::TypeSpec(value) => {
                    let sig = value.signature(self.r);
                    let mut args = Vec::new();

                    for arg in sig.args() {
                        args.push(self.write_type(arg));
                    }

                    self.generics.push(args.to_vec());
                    generics.push(args);
                    pop_generics = true;

                    sig.sig_type().resolve(self.r)
                }
            };

            if let Err(index) = result.binary_search_by_key(&definition, |info| info.definition) {
                let exclusive = definition.has_attribute(self.r, "Windows.Foundation.Metadata", "ExclusiveToAttribute");
                self.add_interfaces(result, &generics, definition.interfaces(self.r));
                result.insert(index, InterfaceInfo { definition, generics, default, overridable, exclusive });
            }

            if pop_generics {
                self.generics.pop();
            }
        }
    }

    fn interfaces(&mut self, t: &TypeDef) -> Vec<InterfaceInfo> {
        let mut result = Vec::new();

        self.add_interfaces(&mut result, &Vec::new(), t.interfaces(self.r));

        // TODO: add base class interfaces

        result
    }
}

struct InterfaceInfo {
    definition: TypeDef,
    generics: Vec<Vec<TokenStream>>,
    default: bool,
    overridable: bool,
    exclusive: bool,
}
