use crate::*;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use winmd::*;

pub(crate) fn write_modules(reader: &Reader, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    write_namespace_set(reader.namespaces(), scope)
}

pub(crate) fn write_namespace_set(namespaces: NamespaceSet, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let mut tokens = quote! {};

    for namespace in namespaces {
        if scope.contains(namespace.full_name()) {
            let namespace = write_namespace(&namespace, scope);

            tokens = quote! {
                #tokens
                #namespace
            };
        }
    }

    tokens
}

fn write_namespace(namespace: &Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let module = format_ident!("{}", namespace.name().to_lowercase());
    let types = write_namespace_types(namespace, scope);
    let namespaces = write_namespace_set(namespace.namespaces(), scope);

    quote! {
        pub mod #module {
            #types
            #namespaces
        }
    }
}

fn write_namespace_types(namespace: &Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let mut tokens = Vec::new();

    for t in namespace.types() {
        tokens.push(match t.category() {
            TypeCategory::Interface => write_interface(&t, scope),
            TypeCategory::Class => write_class(&t, scope),
            TypeCategory::Enum => write_enum(&t, scope),
            TypeCategory::Struct => write_struct(&t, scope),
            TypeCategory::Delegate => write_delegate(&t, scope),
            _ => continue,
        });
    }

    TokenStream::from_iter(tokens)
}

fn write_class(class: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    // TODO: don't define struct here if the class is static - only declare.

    let name = format_ident!("{}", class.name());
    let functions = write_class_functions(&class);
    let string_name = format!("{}.{}", class.namespace(), class.name());

    quote! {
        pub struct #name { ptr: *mut std::ffi::c_void }
        impl #name { #functions }
        impl winrt::TypeName for #name {
            fn type_name() -> &'static str {
                #string_name
            }
        }
        impl winrt::AsAbi for #name {
            type In = *const std::ffi::c_void;
            type Out = *mut *mut std::ffi::c_void;
            fn as_abi_in(&self) -> Self::In {
                self.ptr
            }
            fn as_abi_out(&mut self) -> Self::Out {
                debug_assert!(self.ptr.is_null());
                &mut self.ptr
            }
        }
        impl From<*mut std::ffi::c_void> for #name {
            fn from(ptr: *mut std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }
    }
}

fn write_class_functions(class: &TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for attribute in class.attributes() {
        let (_, name) = attribute.name();

        if name == "StaticAttribute" {
            for (_, sig) in attribute.arguments() {
                if let ArgumentSig::Type(interface) = sig {
                    let class_name = format_ident!("{}", class.name());
                    let interface_name = format_ident!("{}", interface.name());

                    for method in interface.methods() {
                        let method_name = format_ident!("r#{}", method.name());
                        let signature = method.signature();
                        let params = write_consume_params(&signature);
                        let args = signature.params().iter().map(|param| format_ident!("{}", param.name()));

                        if let Some(result) = signature.return_type() {
                            let result = write_type_sig(result.sig_type());

                            tokens = quote! {
                                #tokens
                                pub fn #method_name(#params) -> winrt::Result<#result> {
                                    winrt::factory::<#class_name, #interface_name>()?.#method_name(#(#args),*)
                                }
                            };
                        } else {
                            tokens = quote! {
                                #tokens
                                pub fn #method_name(#params) -> winrt::Result<()> {
                                        panic!();
                                }
                            };
                        };
                    }
                }
            }
        }
    }

    tokens
}

fn guid_u32(sig: &mut std::slice::Iter<(&str, ArgumentSig)>) -> u32 {
    match sig.next().unwrap().1 {
        ArgumentSig::U32(value) => value,
        _ => panic!(),
    }
}

fn guid_u16(sig: &mut std::slice::Iter<(&str, ArgumentSig)>) -> u16 {
    match sig.next().unwrap().1 {
        ArgumentSig::U16(value) => value,
        _ => panic!(),
    }
}

fn guid_u8(sig: &mut std::slice::Iter<(&str, ArgumentSig)>) -> u8 {
    match sig.next().unwrap().1 {
        ArgumentSig::U8(value) => value,
        _ => panic!(),
    }
}

fn write_interface(interface: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    if interface.name().starts_with("IAsync") || interface.name() == "IMemoryBufferReference" || interface.name() == "IMemoryBuffer" {
        return TokenStream::new();
    }

    //let generics = interface.generics();
    let name = interface.name();
    let name_ident = format_ident!("{}", name);
    let abi_name_ident = format_ident!("abi_{}", name);
    let abi_methods = write_abi_methods(&interface);
    let consume_methods = write_consume_methods(&interface);

    let guid = interface.find_attribute("Windows.Foundation.Metadata.GuidAttribute").unwrap();
    let guid = guid.arguments();
    let mut guid = guid.iter();
    let g1 = guid_u32(&mut guid);
    let g2 = guid_u16(&mut guid);
    let g3 = guid_u16(&mut guid);
    let g4 = guid_u8(&mut guid);
    let g5 = guid_u8(&mut guid);
    let g6 = guid_u8(&mut guid);
    let g7 = guid_u8(&mut guid);
    let g8 = guid_u8(&mut guid);
    let g9 = guid_u8(&mut guid);
    let g10 = guid_u8(&mut guid);
    let g11 = guid_u8(&mut guid);

    quote! {
        #[repr(C)]
        pub struct #name_ident { ptr: *mut std::ffi::c_void }
        #[repr(C)]
        struct #abi_name_ident {
            __0: usize,
            __1: usize,
            __2: usize,
            __3: usize,
            __4: usize,
            __5: usize,
            #abi_methods
        }
        impl #name_ident {
            #consume_methods
        }
        impl winrt::TypeInterface for #name_ident {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    #g1,
                    #g2,
                    #g3,
                    &[#g4, #g5, #g6, #g7, #g8, #g9, #g10, #g11],
                );
                &GUID
            }
        }
        impl winrt::AsAbi for #name_ident {
            type In = *const std::ffi::c_void;
            type Out = *mut *mut std::ffi::c_void;
            fn as_abi_in(&self) -> Self::In {
                self.ptr
            }
            fn as_abi_out(&mut self) -> Self::Out {
                debug_assert!(self.ptr.is_null());
                &mut self.ptr
            }
        }
        impl From<*mut std::ffi::c_void> for #name_ident {
            fn from(ptr: *mut std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }
    }
}

fn write_abi_methods(interface: &TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for method in interface.methods() {
        let name = format_ident!("r#{}", method.name());
        let params = write_abi_params(&method.signature());
        tokens = quote! {
            #tokens
            #name: extern "system" fn(*const std::ffi::c_void, #params) -> winrt::ErrorCode,
        };
    }

    tokens
}

fn write_consume_methods(interface: &TypeDef) -> TokenStream {
    let mut tokens = quote! {};
    let abi_interface_name = format_ident!("abi_{}", interface.name());

    for method in interface.methods() {
        let name = format_ident!("r#{}", method.name());
        let signature = method.signature();
        let params = write_consume_params(&signature);
        let args = write_abi_args(&signature);

        if let Some(result) = signature.return_type() {
            let projected_result = write_type_sig(result.sig_type());
            let receive_result = write_consume_receive_type(result.sig_type());

            tokens = quote! {
                #tokens
                pub fn #name(&self, #params) -> winrt::Result<#projected_result> {
                    unsafe {
                        #receive_result
                        ((*(*(self.ptr as *const *const #abi_interface_name))).#name)(
                            self.ptr, #args &mut __ok,
                        )
                        .ok_or(From::from(__ok))
                    }
                }
            };
        } else {
            tokens = quote! {
                #tokens
                pub fn #name(&self, #params) -> winrt::Result<()> {
                    unsafe {
                        panic!();
                    }
                }
            };
        };
    }

    tokens
}

fn write_consume_receive_type(value: &TypeSig) -> TokenStream {
    match value.category() {
        ParamCategory::Object | ParamCategory::String => quote! {
            let mut __ok = std::ptr::null_mut();
        },
        ParamCategory::Enum => quote! {
            let mut __ok;
        },
        _ => quote! {
            let mut __ok = Default::default();
        },
    }
}

fn write_abi_params(signature: &MethodSig) -> TokenStream {
    let mut tokens = Vec::new();

    for param in signature.params() {
        tokens.push(write_abi_type_sig(param));
        tokens.push(quote! {,}); // TODO: surely there's a simpler/more efficient way to do this?
    }

    if let Some(param) = signature.return_type() {
        tokens.push(write_abi_type_sig(param));
    }

    TokenStream::from_iter(tokens)
}

fn write_consume_params(signature: &MethodSig) -> TokenStream {
    let mut tokens = Vec::new();

    for param in signature.params() {
        tokens.push(write_consume_param(param));
    }

    TokenStream::from_iter(tokens)
}

fn write_abi_args(signature: &MethodSig) -> TokenStream {
    let mut tokens = Vec::new();

    for param in signature.params() {
        tokens.push(write_abi_arg(param));
    }

    TokenStream::from_iter(tokens)
}

fn write_consume_param(param: &ParamSig) -> TokenStream {
    let name = format_ident!("{}", param.name());
    let category = param.sig_type().category();
    let tokens = write_type_sig(param.sig_type());

    if param.input() {
        match category {
            // TODO: exclude non-trivial structs
            ParamCategory::Enum | ParamCategory::Primitive | ParamCategory::Struct => quote! { #name: #tokens, },
            _ => quote! { #name: &#tokens, },
        }
    } else {
        quote! { #name: &mut #tokens, }
    }
}

fn write_abi_arg(param: &ParamSig) -> TokenStream {
    let name = format_ident!("{}", param.name());
    let category = param.sig_type().category();

    if param.input() {
        match category {
            ParamCategory::Enum | ParamCategory::Primitive | ParamCategory::Struct => quote! { #name, },
            _ => quote! { winrt::AsAbi::as_abi_in(#name), },
        }
    } else {
        match category {
            ParamCategory::Enum | ParamCategory::Primitive | ParamCategory::Struct => quote! { &mut #name, },
            _ => quote! { winrt::AsAbi::as_abi_out(#name), },
        }
    }
}

fn write_abi_type_sig(param: &ParamSig) -> TokenStream {
    match param.sig_type().sig_type() {
        TypeSigType::ElementType(value) => write_abi_element_type(param, value),
        TypeSigType::TypeDefOrRef(value) => write_abi_type_def_or_ref(param, value),
        TypeSigType::GenericSig(_value) => quote! {bool},
        TypeSigType::GenericTypeIndex(_value) => quote! {bool},
    }
}

fn write_type_sig(value: &TypeSig) -> TokenStream {
    match value.sig_type() {
        TypeSigType::ElementType(value) => write_element_type(value),
        TypeSigType::TypeDefOrRef(value) => write_type_def_or_ref(value),
        TypeSigType::GenericSig(_value) => quote! {bool},
        TypeSigType::GenericTypeIndex(_value) => quote! {bool},
    }
}

fn write_abi_element_type(param: &ParamSig, value: &ElementType) -> TokenStream {
    if param.input() {
        match value {
            ElementType::Bool => quote! { bool },
            ElementType::Char => quote! { char },
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
            ElementType::String => quote! { *const std::ffi::c_void },
            ElementType::Object => quote! { *const std::ffi::c_void },
        }
    } else {
        match value {
            ElementType::Bool => quote! { &mut bool },
            ElementType::Char => quote! { &mut char },
            ElementType::I8 => quote! { &mut i8 },
            ElementType::U8 => quote! { &mut u8 },
            ElementType::I16 => quote! { &mut i16 },
            ElementType::U16 => quote! { &mut u16 },
            ElementType::I32 => quote! { &mut i32 },
            ElementType::U32 => quote! { &mut u32 },
            ElementType::I64 => quote! { &mut i64 },
            ElementType::U64 => quote! { &mut u64 },
            ElementType::F32 => quote! { &mut f32 },
            ElementType::F64 => quote! { &mut f64 },
            ElementType::String => quote! { &mut *mut std::ffi::c_void },
            ElementType::Object => quote! { &mut *mut std::ffi::c_void },
        }
    }
}

fn write_element_type(value: &ElementType) -> TokenStream {
    match value {
        ElementType::Bool => quote! { bool },
        ElementType::Char => quote! { char },
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

fn write_abi_type_def_or_ref(param: &ParamSig, value: &TypeDefOrRef) -> TokenStream {
    match value {
        TypeDefOrRef::TypeDef(value) => write_abi_type_def(param, value),
        TypeDefOrRef::TypeRef(value) => write_abi_type_ref(param, value),
        _ => panic!("write_abi_type_def_or_ref"),
    }
}

fn write_type_def_or_ref(value: &TypeDefOrRef) -> TokenStream {
    match value {
        TypeDefOrRef::TypeDef(value) => write_type_def(value),
        TypeDefOrRef::TypeRef(value) => write_type_ref(value),
        _ => panic!("write_type_def_or_ref"),
    }
}

fn write_abi_type_def(param: &ParamSig, value: &TypeDef) -> TokenStream {
    if param.input() {
        match value.category() {
            TypeCategory::Enum | TypeCategory::Struct => {
                let name = format_ident!("{}", value.name());
                quote! { #name }
            }
            _ => quote! { *const std::ffi::c_void },
        }
    } else {
        match value.category() {
            TypeCategory::Enum | TypeCategory::Struct => {
                let name = format_ident!("{}", value.name());
                quote! { &mut #name }
            }
            _ => quote! { &mut *mut std::ffi::c_void },
        }
    }
}

fn write_type_def(value: &TypeDef) -> TokenStream {
    let name = format_ident!("{}", value.name());
    quote! { #name }
}

fn write_abi_type_ref(param: &ParamSig, value: &TypeRef) -> TokenStream {
    if value.name() == "Guid" && value.namespace() == "System" {
        if param.input() {
            quote! { winrt::Guid }
        } else {
            quote! { &mut winrt::Guid }
        }
    } else {
        write_abi_type_def(param, &value.resolve())
    }
}

fn write_type_ref(value: &TypeRef) -> TokenStream {
    if value.name() == "Guid" && value.namespace() == "System" {
        quote! { winrt::Guid }
    } else {
        write_type_def(&value.resolve())
    }
}

fn write_enum(t: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = format_ident!("{}", t.name());
    let fields = write_enum_fields(&t);
    quote! {
        pub enum #name { #fields }
    }
}

fn write_enum_fields(t: &TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        for _c in f.constants() {
            let name = format_ident!("{}", f.name());
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

fn write_delegate(interface: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    //let generics = interface.generics();
    let name = interface.name();
    let name_ident = format_ident!("{}", name);
    let abi_name_ident = format_ident!("abi_{}", name);

    let guid = interface.find_attribute("Windows.Foundation.Metadata.GuidAttribute").unwrap();
    let guid = guid.arguments();
    let mut guid = guid.iter();
    let g1 = guid_u32(&mut guid);
    let g2 = guid_u16(&mut guid);
    let g3 = guid_u16(&mut guid);
    let g4 = guid_u8(&mut guid);
    let g5 = guid_u8(&mut guid);
    let g6 = guid_u8(&mut guid);
    let g7 = guid_u8(&mut guid);
    let g8 = guid_u8(&mut guid);
    let g9 = guid_u8(&mut guid);
    let g10 = guid_u8(&mut guid);
    let g11 = guid_u8(&mut guid);

    quote! {
        #[repr(C)]
        pub struct #name_ident { ptr: *mut std::ffi::c_void }
        #[repr(C)]
        struct #abi_name_ident {
            __0: usize,
            __1: usize,
            __2: usize,
        }
        impl #name_ident {
        }
        impl winrt::TypeInterface for #name_ident {
            fn type_guid() -> &'static winrt::Guid {
                static GUID: winrt::Guid = winrt::Guid::from_values(
                    #g1,
                    #g2,
                    #g3,
                    &[#g4, #g5, #g6, #g7, #g8, #g9, #g10, #g11],
                );
                &GUID
            }
        }
        impl winrt::AsAbi for #name_ident {
            type In = *const std::ffi::c_void;
            type Out = *mut *mut std::ffi::c_void;
            fn as_abi_in(&self) -> Self::In {
                self.ptr
            }
            fn as_abi_out(&mut self) -> Self::Out {
                debug_assert!(self.ptr.is_null());
                &mut self.ptr
            }
        }
        impl From<*mut std::ffi::c_void> for #name_ident {
            fn from(ptr: *mut std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }
    }
}

fn write_struct(t: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = format_ident!("{}", t.name());
    let fields = write_struct_fields(&t);
    quote! {
        #[repr(C)]
        #[derive(Default, Debug, PartialEq)]
        pub struct #name { #fields }
    }
}

fn write_struct_fields(t: &TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        let name = format_ident!("r#{}", to_snake(f.name()));

        tokens = quote! {
            #tokens
            pub #name: u8,
            // TODO: write out field type
        };
    }

    tokens
}

fn to_snake(camel: &str) -> String {
    let mut result = String::new();

    for c in camel.chars() {
        if c.is_uppercase() {
            if !result.is_empty() {
                result.push('_');
            }
            for c in c.to_lowercase() {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    result
}
