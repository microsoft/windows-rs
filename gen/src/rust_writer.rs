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

    pub fn add_namespace(&mut self, namespace: &str) {
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

struct GenericGuard<'a, 'b> {
    writer: &'a mut Writer<'b>,
    count: usize,
}

impl<'a, 'b> GenericGuard<'a, 'b> {
    fn new(writer: &'a mut Writer<'b>, count: usize) -> GenericGuard<'a, 'b> {
        GenericGuard { writer, count }
    }
}

impl<'a, 'b> std::ops::Deref for GenericGuard<'a, 'b> {
    type Target = Writer<'b>;

    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

impl<'a, 'b> std::ops::DerefMut for GenericGuard<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.writer
    }
}

impl<'a, 'b> Drop for GenericGuard<'a, 'b> {
    fn drop(&mut self) {
        if self.count > 0 {
            self.writer.generics.resize_with(self.writer.generics.len() - self.count, || panic!());
        }
    }
}

struct Writer<'a> {
    pub r: &'a Reader,
    pub namespace: &'a str,
    pub limits: &'a BTreeSet<String>,
    pub generics: Vec<Vec<TokenStream>>,

    // TODO: what about all the writer methods simply return an Option<TokenStream> so we can short circuit
    // automatically (and quickly) instead of stupid flags like this.
    pub drop_last: bool,
    // TODO: keep track of generic specializations that need GUIDs
}

impl<'a> Writer<'a> {
    pub fn write(r: &Reader, limits: &BTreeSet<String>) -> TokenStream {
        let mut namespaces = BTreeMap::new();

        // TODO: parallalelize this loop
        for namespace in limits {
            let mut w = Writer { r, namespace, limits, generics: Default::default(), drop_last: false };
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
        let namespace = class.namespace(self.r);
        let name = class.name(self.r);
        let string_name = format!("{}.{}", namespace, name);
        let name = format_ident!("{}", name);
        let name = quote! { #name };
        let functions = self.write_class_functions(class);
        let interfaces = self.interfaces(class);
        let empty = TokenStream::new();
        let froms = self.write_from_traits(&name, &empty, &empty, &interfaces);

        if let Some(interface) = interfaces.iter().find(|interface| interface.default) {
            let mut guard = self.push_generic_required_interface(&interface);
            let default_interface = guard.write_required_interface(&interface);

            quote! {
                #[repr(C)]
                #[derive(Default, Clone)]
                pub struct #name { ptr: winrt::ComPtr }
                impl #name { #functions }
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

    fn write_from_traits(&mut self, from: &TokenStream, constraints: &TokenStream, generics: &TokenStream, interfaces: &Vec<Interface>) -> TokenStream {
        let mut tokens = Vec::<TokenStream>::new();

        for interface in interfaces {
            let into = self.write_required_interface(interface);

            if interface.default {
                tokens.push(quote! {
                    impl<#constraints> From<#from<#generics>> for #into {
                        fn from(value: #from<#generics>) -> #into {
                            unsafe { std::mem::transmute(value) }
                        }
                    }
                    impl<#constraints> From<&#from<#generics>> for #into {
                        fn from(value: &#from<#generics>) -> #into {
                            unsafe { std::mem::transmute(value.clone()) }
                        }
                    }
                    impl<'a, #constraints> Into<winrt::Param<'a, #into>> for #from<#generics> {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                    impl<'a, #constraints> Into<winrt::Param<'a, #into>> for &'a #from<#generics> {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                });
            } else {
                tokens.push(quote! {
                    impl<#constraints> From<#from<#generics>> for #into {
                        fn from(value: #from<#generics>) -> #into {
                            #into::from(&value)
                        }
                    }
                    impl<#constraints> From<&#from<#generics>> for #into {
                        fn from(value: &#from<#generics>) -> #into {
                            winrt::QueryType::query(value)
                        }
                    }
                    impl<'a, #constraints> Into<winrt::Param<'a, #into>> for #from<#generics> {
                        fn into(self) -> winrt::Param<'a, #into> {
                            winrt::Param::Value(self.into())
                        }
                    }
                    impl<'a, #constraints> Into<winrt::Param<'a, #into>> for &'a #from<#generics> {
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
            let mut guard = self.push_generic_required_interface(&interface);

            if interface.default {
                tokens.push(guard.write_consume_methods(&interface.definition));
            } else {
                tokens.push(guard.write_forward_methods(&interface));
            }
        }

        TokenStream::from_iter(tokens)
    }

    fn write_interface_methods(&mut self, interface: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        tokens.push(self.write_consume_methods(&interface));

        for interface in self.interfaces(interface) {
            let mut guard = self.push_generic_required_interface(&interface);
            tokens.push(guard.write_forward_methods(&interface));
        }

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
            let args = self.write_consume_args(&signature);

            if let Some(result) = signature.return_type() {
                let result = self.write_type(result.definition());

                tokens.push(quote! {
                    pub fn #method_name<#into_params>(#params) -> winrt::Result<#result> {
                        winrt::factory::<#class_name, #interface_name>()?.#method_name(#args)
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

    fn write_required_interface(&mut self, info: &Interface) -> TokenStream {
        if let Some(generics) = info.generics.last() {
            let namespace = self.write_namespace_name(info.definition.namespace(self.r));
            let name = info.definition.name(self.r);
            let name = &name[..name.len() - 2];
            let name = format_ident!("{}", name);
            quote! { #namespace#name::<#(#generics),*> }
        } else {
            self.write_type_def(&info.definition)
        }
    }

    fn write_interface(&mut self, interface: &TypeDef) -> TokenStream {
        let mut guard = self.push_generic_interface(interface);
        guard.write_generic_interface(interface)
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
        let interfaces = self.interfaces(interface);
        let empty = TokenStream::new();
        let froms = self.write_from_traits(&name, &constraints, &generics, &interfaces);

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
            #froms
        }
    }

    fn write_abi_methods(&mut self, interface: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        for method in interface.methods(self.r) {
            let name = method.name(self.r);
            if name == ".ctor" {
                continue;
            }
            let name = self.write_method_name(&method);
            let params = self.write_abi_params(&method.signature(self.r));
            tokens.push(quote! {
                #name: extern "system" fn(winrt::RawPtr, #params) -> winrt::ErrorCode,
            });
        }

        TokenStream::from_iter(tokens)
    }

    fn write_forward_methods(&mut self, interface: &Interface) -> TokenStream {
        let mut tokens = Vec::new();
        let into = self.write_required_interface(&interface);

        for method in interface.definition.methods(self.r) {
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
            let args = self.write_consume_args(&signature);

            let projected_result = match signature.return_type() {
                Some(result) => self.write_type(result.definition()),
                None => quote! { () },
            };

            tokens.push(quote! {
                pub fn #name<#into_params>(&self, #params) -> winrt::Result<#projected_result> {
                    <#into as From<&Self>>::from(self).#name(#args)
                }
            });
        }

        TokenStream::from_iter(tokens)
    }

    fn write_consume_methods(&mut self, interface: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();
        let generics = self.write_generics();
        let abi_name = self.write_generic_abi_name(interface);

        for method in interface.methods(self.r) {
            let name = method.name(self.r);
            if name == ".ctor" {
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
                let projected_result = self.write_type(result.definition());
                let receive_expression = self.write_consume_receive_expression(result.definition());

                tokens.push(quote! {
                    pub fn #name<#into_params>(&self, #params) -> winrt::Result<#projected_result> {
                        unsafe {
                            let mut __ok = std::mem::zeroed();
                            ((*(*(self.ptr.get() as *const *const #abi_name<#generics>))).#name)(
                                self.ptr.get(), #args #receive_expression
                            )
                            .ok_or(std::mem::transmute_copy(&__ok))
                        }
                    }
                });
            } else {
                tokens.push(quote! {
                    pub fn #name<#into_params>(&self, #params) -> winrt::Result<()> {
                        unsafe {
                            ((*(*(self.ptr.get() as *const *const #abi_name<#generics>))).#name)(
                                self.ptr.get(), #args
                            )
                            .ok()
                        }
                    }
                });
            }
        }

        TokenStream::from_iter(tokens)
    }

    fn write_consume_args(&self, signature: &MethodSig) -> TokenStream {
        TokenStream::from_iter(signature.params().iter().map(|param| {
            let name = format_ident!("r#{}", param.name());
            quote! { #name, }
        }))
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
        let namespace = t.namespace(self.r);
        let name = t.name(self.r);
        let type_name = format_ident!("{}", name);

        let mut fields = t.fields(self.r);

        // The first field holds the underlying type (either i32 or u32).
        let repr = match fields.next().unwrap().signature(self.r).definition() {
            TypeSigType::ElementType(ElementType::I32) => format_ident!("i32"),
            _ => format_ident!("u32"),
        };

        // The second field is the first or default variant.
        let default = format_ident!("{}", fields.next().unwrap().name(self.r));

        let fields = self.write_enum_fields(&t);

        quote! {
            #[repr(#repr)]
            //#[winrt::winrt_enum("Hello2")]
            #[derive(PartialEq)]
            pub enum #type_name { #fields }
            impl Default for #type_name {
                fn default() -> Self {
                    Self::#default
                }
            }
        }
    }

    fn write_enum_fields(&mut self, t: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        for f in t.fields(self.r) {
            for _c in f.constants(self.r) {
                let name = format_ident!("{}", f.name(self.r));

                tokens.push(quote! {
                    #name,
                    // TODO: write out the enum value
                });
            }
        }

        TokenStream::from_iter(tokens)
    }

    fn write_delegate(&mut self, interface: &TypeDef) -> TokenStream {
        let mut guard = self.push_generic_interface(interface);
        guard.write_generic_delegate(interface)
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
        // TODO: need namespace if ABI is called from different namespace (e.g. default interface is not in same namespace as class)
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

        let namespace = t.namespace(self.r);
        let name = t.name(self.r);
        let type_name = format_ident!("{}", name);

        let fields = self.write_struct_fields(&t);

        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, Default, Debug, PartialEq)]
            pub struct #type_name { #fields }
            impl winrt::RuntimeCopy for #type_name {}
        }
        // TODO: RuntimeType for non-blittable structs needs to be customized
    }

    fn write_struct_fields(&mut self, t: &TypeDef) -> TokenStream {
        let mut tokens = Vec::new();

        for f in t.fields(self.r) {
            let name = format_ident!("r#{}", to_snake(f.name(self.r)));

            tokens.push(quote! {
                pub #name: u8,
                // TODO: write out field type
            });
        }

        TokenStream::from_iter(tokens)
    }

    //
    // write_abi_params
    //

    fn write_abi_params(&mut self, signature: &MethodSig) -> TokenStream {
        let mut tokens: Vec<TokenStream> = signature.params().iter().map(|param| self.write_abi_param(param)).collect();

        if let Some(param) = signature.return_type() {
            tokens.push(self.write_abi_param(param));
        }

        TokenStream::from_iter(tokens)
    }

    fn write_abi_param(&mut self, param: &ParamSig) -> TokenStream {
        let tokens = match param.definition().definition() {
            TypeSigType::ElementType(value) => self.write_abi_param_element_type(value),
            TypeSigType::TypeDefOrRef(value) => self.write_abi_param_type(value),
            TypeSigType::GenericSig(_) => quote! { winrt::RawPtr, },
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

        for (count, param) in signature.params().iter().enumerate() {
            // TODO: don't do convertible for array params?
            if param.array() {
                continue;
            }

            // TODO: use ParamCategory here

            let type_param = format_ident!("__{}", count);

            if let TypeSigType::ElementType(ElementType::String) = param.definition().definition() {
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
            let name = format_ident!("r#{}", param.name());
            tokens.push(quote! { #name: });
            tokens.push(self.write_consume_param(count, param));
        }

        TokenStream::from_iter(tokens)
    }

    fn write_consume_param(&mut self, count: usize, param: &ParamSig) -> TokenStream {
        let tokens = self.write_type(param.definition());

        if param.array() {
            if param.input() {
                quote! { &[#tokens], }
            } else if param.by_ref() {
                quote! { &mut winrt::Array<#tokens>, }
            } else {
                quote! { &mut [#tokens], }
            }
        } else if param.input() {
            match param.definition().category(self.r) {
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
        TokenStream::from_iter(signature.params().iter().map(|param| self.write_abi_arg(param)))
    }

    fn write_abi_arg(&mut self, param: &ParamSig) -> TokenStream {
        let name = format_ident!("r#{}", param.name());
        let category = param.definition().category(self.r);

        if param.array() {
            if param.input() {
                quote! { #name.len() as u32, std::mem::transmute(#name.as_ptr()), }
            } else if param.by_ref() {
                quote! { #name.set_abi_len(), #name.set_abi(), }
            } else {
                quote! { #name.len() as u32, std::mem::transmute_copy(&#name), }
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
        match value.definition() {
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
        let namespace = self.write_namespace_name(value.definition().namespace(self.r));
        let name = value.definition().name(self.r);
        let name = name.get(..name.len() - 2).unwrap();
        let name = format_ident!("{}", name);
        let args = value.args().iter().map(|arg| self.write_type(arg));

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

    fn push_generic_interface<'g>(&'g mut self, interface: &TypeDef) -> GenericGuard<'g, 'a> {
        let generics = interface.generics(self.r);

        if generics.is_empty() {
            GenericGuard::new(self, 0)
        } else {
            self.generics.push(
                generics
                    .map(|g| {
                        let name = format_ident!("{}", g.name(self.r));
                        quote! { #name }
                    })
                    .collect(),
            );

            GenericGuard::new(self, 1)
        }
    }

    fn push_generic_required_interface<'g>(&'g mut self, interface: &Interface) -> GenericGuard<'g, 'a> {
        if interface.generics.len() > 0 {
            self.generics.append(&mut interface.generics.clone());
        }

        GenericGuard::new(self, interface.generics.len())
    }

    fn add_interfaces(&mut self, result: &mut Vec<Interface>, parent_generics: &Vec<Vec<TokenStream>>, children: RowIterator<InterfaceImpl>) {
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
                    let definition = sig.definition().resolve(self.r);
                    let args: Vec<TokenStream> = sig.args().iter().map(|arg| self.write_type(arg)).collect();
                    self.generics.push(args.to_vec());
                    generics.push(args);
                    pop_generics = true;

                    definition
                }
            };

            if let Err(index) = result.binary_search_by_key(&definition, |info| info.definition) {
                if self.limits.contains(definition.namespace(self.r)) {
                    let exclusive = definition.has_attribute(self.r, "Windows.Foundation.Metadata", "ExclusiveToAttribute");
                    // TODO: ideally we don't need to clone here but we need to insert before calling add_interfaces
                    result.insert(index, Interface { definition, generics: generics.clone(), default, overridable, exclusive });
                    self.add_interfaces(result, &generics, definition.interfaces(self.r));
                }
            }

            if pop_generics {
                self.generics.pop();
            }
        }
    }

    fn interfaces(&mut self, t: &TypeDef) -> Vec<Interface> {
        let mut result = Vec::new();

        self.add_interfaces(&mut result, &Vec::new(), t.interfaces(self.r));

        // TODO: add base class interfaces

        result
    }
}

struct Interface {
    definition: TypeDef,
    generics: Vec<Vec<TokenStream>>,
    default: bool,
    overridable: bool,
    exclusive: bool,
}
