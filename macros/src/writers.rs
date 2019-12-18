use crate::*;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

pub(crate) fn write_modules(reader: &winmd::Reader, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    write_namespace_set(reader.namespaces(), scope)
}

pub(crate) fn write_namespace_set(namespaces: winmd::NamespaceSet, scope: &std::collections::BTreeSet<String>) -> TokenStream {
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

fn write_namespace(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
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

fn write_namespace_types(namespace: &winmd::Namespace, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for t in namespace.types() {
        match t.category() {
            winmd::TypeCategory::Interface => tokens.push(write_interface(&t, scope)),
            winmd::TypeCategory::Class => tokens.push(write_class(&t, scope)),
            winmd::TypeCategory::Enum => tokens.push(write_enum(&t, scope)),
            winmd::TypeCategory::Struct => tokens.push(write_struct(&t, scope)),
            //winmd::TypeCategory::Delegate => write_delegate(t, scope),
            _ => {}
        };
    }

    TokenStream::from_iter(tokens)
}

fn write_class(class: &winmd::TypeDef, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = format_ident!("{}", class.name());
    let functions = write_class_functions(&class);
    let mut string_name = String::new();
    string_name.push_str(class.namespace());
    string_name.push('.');
    string_name.push_str(class.name());
    // TODO: don't define struct here if the class is static - only declare.
    quote! {
        pub struct #name { ptr: *const std::ffi::c_void }
        impl #name { #functions }
        impl winrt::TypeName for #name {
            fn type_name() -> &'static str {
                #string_name
            }
        }
    }
}

fn write_class_functions(class: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for attribute in class.attributes() {
        let (_, name) = attribute.name();

        if name == "StaticAttribute" {
            for (_, sig) in attribute.arguments() {
                if let winmd::ArgumentSig::Type(interface) = sig {
                    let class_name = format_ident!("{}", class.name());
                    let interface_name = format_ident!("{}", interface.name());

                    if interface.name() != "IColorHelperStatics" && interface.name() != "IColorHelperStatics2" && interface.name() != "IUIContentRoot" {
                        for method in interface.methods() {
                            let method_name = format_ident!("{}", method.name());
                            let signature = method.signature();
                            let params = write_consume_params(&signature);

                            if let Some(result) = signature.return_type() {
                                let result = write_type_sig(result.sig_type());

                                tokens = quote! {
                                    #tokens
                                    pub fn #method_name(#params) -> winrt::Result<#result> {
                                        winrt::factory::<#class_name, #interface_name>()?.#method_name()
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
    }

    tokens
}

fn write_interface(interface: &winmd::TypeDef, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = interface.name();
    let name_ident = format_ident!("{}", name);
    let abi_name_ident = format_ident!("abi_{}", name);
    let abi_methods = write_abi_methods(&interface);
    let consume_methods = write_consume_methods(&interface);
    quote! {
        #[repr(C)]
        pub struct #name_ident { ptr: *const std::ffi::c_void }
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
                    0xCFF52E04,
                    0xCCA6,
                    0x4614,
                    &[0xA1, 0x7E, 0x75, 0x49, 0x10, 0xC8, 0x4A, 0x99],
                );
                &GUID
            }
        }
        impl winrt::TakeOwnership for #name_ident {
            fn take_ownership(ptr: *const std::ffi::c_void) -> Self {
                Self { ptr }
            }
        }
    }
}

fn write_abi_methods(interface: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for method in interface.methods() {
        let name = format_ident!("{}", method.name());
        let params = write_abi_params(&method.signature());
        tokens = quote! {
            #tokens
            #name: extern "system" fn(*const std::ffi::c_void, #params) -> winrt::ErrorCode,
        };
    }

    tokens
}

fn write_consume_methods(interface: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};
    let abi_interface_name = format_ident!("abi_{}", interface.name());

    if interface.name() != "IColorHelperStatics2" && interface.name() != "IUIContentRoot" {
        for method in interface.methods() {
            let name = format_ident!("{}", method.name());
            let signature = method.signature();
            let params = write_consume_params(&signature);
            let args = write_abi_args(&signature);

            if let Some(result) = signature.return_type() {
                let result_type = write_type_sig(result.sig_type());
                let result_local = write_consume_result_local(result);
                let result_take_ownership = write_consume_result_take_ownership(result);

                tokens = quote! {
                    #tokens
                    pub fn #name(&self, #params) -> winrt::Result<#result_type> {
                        unsafe {
                            #result_local
                            ((*(*(self.ptr as *const *const #abi_interface_name))).#name)(
                                self.ptr, #args &mut __ok,
                            )
                            .ok_or(#result_take_ownership)
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
    }

    tokens
}

fn write_consume_result_local(sig: &winmd::ParamSig) -> TokenStream {
    let result = write_type_sig(sig.sig_type());
    quote! {
        let mut __ok: #result = Default::default();
    }
}

fn write_consume_result_take_ownership(sig: &winmd::ParamSig) -> TokenStream {
    let result = write_type_sig(sig.sig_type());
    quote! {
        __ok
    }
}

fn write_abi_params(signature: &winmd::MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_abi_param(param, param_sig));
    }

    if let Some(param_sig) = signature.return_type() {
        let name = write_abi_type_sig(param_sig.sig_type());
        tokens.push(quote! { &mut #name });
    }

    TokenStream::from_iter(tokens)
}

fn write_consume_params(signature: &winmd::MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_consume_param(param, param_sig));
    }

    TokenStream::from_iter(tokens)
}

fn write_abi_args(signature: &winmd::MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_abi_arg(param, param_sig));
    }

    TokenStream::from_iter(tokens)
}

fn write_abi_param(param: &winmd::Param, param_sig: &winmd::ParamSig) -> TokenStream {
    let tokens = write_abi_type_sig(param_sig.sig_type());

    if param.flags().input() {
        quote! {
             #tokens,
        }
    } else {
        quote! {
            &mut #tokens,
        }
    }
}

fn write_consume_param(param: &winmd::Param, param_sig: &winmd::ParamSig) -> TokenStream {
    let name = format_ident!("{}", param.name());
    let tokens = write_type_sig(param_sig.sig_type());

    if param.flags().input() {
        quote! {
             #name: #tokens,
        }
    } else {
        quote! {
            #name: &mut #tokens,
        }
    }
}

fn write_abi_arg(param: &winmd::Param, _param_sig: &winmd::ParamSig) -> TokenStream {
    let name = format_ident!("{}", param.name());

    if param.flags().input() {
        quote! {
             #name,
        }
    } else {
        quote! {
            &mut #name,
        }
    }
}

fn write_abi_type_sig(value: &winmd::TypeSig) -> TokenStream {
    match value.sig_type() {
        winmd::TypeSigType::ElementType(value) => write_abi_element_type(value),
        winmd::TypeSigType::TypeDefOrRef(value) => write_abi_type_def_or_ref(value),
        winmd::TypeSigType::GenericSig(_value) => panic!("GenericSig"),
        winmd::TypeSigType::GenericTypeIndex(_value) => panic!("GenericTypeIndex"),
        winmd::TypeSigType::GenericMethodIndex(_value) => panic!("GenericMethodIndex"),
    }
}

fn write_type_sig(value: &winmd::TypeSig) -> TokenStream {
    match value.sig_type() {
        winmd::TypeSigType::ElementType(value) => write_element_type(value),
        winmd::TypeSigType::TypeDefOrRef(value) => write_type_def_or_ref(value),
        winmd::TypeSigType::GenericSig(_value) => panic!("GenericSig"),
        winmd::TypeSigType::GenericTypeIndex(_value) => panic!("GenericTypeIndex"),
        winmd::TypeSigType::GenericMethodIndex(_value) => panic!("GenericMethodIndex"),
    }
}

fn write_abi_element_type(value: &winmd::ElementType) -> TokenStream {
    match value {
        winmd::ElementType::Bool => quote! { bool },
        winmd::ElementType::Char => quote! { char },
        winmd::ElementType::I8 => quote! { i8 },
        winmd::ElementType::U8 => quote! { u8 },
        winmd::ElementType::I16 => quote! { i16 },
        winmd::ElementType::U16 => quote! { u16 },
        winmd::ElementType::I32 => quote! { i32 },
        winmd::ElementType::U32 => quote! { u32 },
        winmd::ElementType::I64 => quote! { i64 },
        winmd::ElementType::U64 => quote! { u64 },
        winmd::ElementType::F32 => quote! { f32 },
        winmd::ElementType::F64 => quote! { f64 },
        winmd::ElementType::String => quote! { *mut std::ffi::c_void },
        _ => panic!("write_abi_element_type"),
    }
}

fn write_element_type(value: &winmd::ElementType) -> TokenStream {
    match value {
        winmd::ElementType::Bool => quote! { bool },
        winmd::ElementType::Char => quote! { char },
        winmd::ElementType::I8 => quote! { i8 },
        winmd::ElementType::U8 => quote! { u8 },
        winmd::ElementType::I16 => quote! { i16 },
        winmd::ElementType::U16 => quote! { u16 },
        winmd::ElementType::I32 => quote! { i32 },
        winmd::ElementType::U32 => quote! { u32 },
        winmd::ElementType::I64 => quote! { i64 },
        winmd::ElementType::U64 => quote! { u64 },
        winmd::ElementType::F32 => quote! { f32 },
        winmd::ElementType::F64 => quote! { f64 },
        winmd::ElementType::String => quote! { winrt::String },
        _ => panic!("write_element_type"),
    }
}

fn write_abi_type_def_or_ref(value: &winmd::TypeDefOrRef) -> TokenStream {
    match value {
        winmd::TypeDefOrRef::TypeDef(value) => write_abi_type_def(value),
        winmd::TypeDefOrRef::TypeRef(value) => write_abi_type_ref(value),
        _ => panic!("write_abi_type_def_or_ref"),
    }
}

fn write_type_def_or_ref(value: &winmd::TypeDefOrRef) -> TokenStream {
    match value {
        winmd::TypeDefOrRef::TypeDef(value) => write_type_def(value),
        winmd::TypeDefOrRef::TypeRef(value) => write_type_ref(value),
        _ => panic!("write_type_def_or_ref"),
    }
}

fn write_abi_type_def(value: &winmd::TypeDef) -> TokenStream {
    match value.category() {
        winmd::TypeCategory::Struct => {
            let name = format_ident!("{}", value.name());
            quote! { #name }
        }
        _ => quote! { *mut std::ffi::c_void },
    }
}

fn write_type_def(value: &winmd::TypeDef) -> TokenStream {
    let name = format_ident!("{}", value.name());
    quote! { #name }
}

fn write_abi_type_ref(value: &winmd::TypeRef) -> TokenStream {
    // TODO: handle "System.Guid" directly
    write_abi_type_def(&value.find_def())
}

fn write_type_ref(value: &winmd::TypeRef) -> TokenStream {
    // TODO: handle "System.Guid" directly
    write_type_def(&value.find_def())
}

fn write_enum(t: &winmd::TypeDef, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = format_ident!("{}", t.name());
    let fields = write_enum_fields(&t);
    quote! {
        pub enum #name { #fields }
    }
}

fn write_enum_fields(t: &winmd::TypeDef) -> TokenStream {
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

fn write_struct(t: &winmd::TypeDef, scope: &std::collections::BTreeSet<String>) -> TokenStream {
    let name = format_ident!("{}", t.name());
    let fields = write_struct_fields(&t);
    quote! {
        #[repr(C)]
        #[derive(Default, Debug)]
        pub struct #name { #fields }
    }
}

fn write_struct_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        let name = format_ident!("{}", to_snake(f.name()));

        tokens = quote! {
            #tokens
            #name: u8,
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

    // TODO: go through all keywords and prepend `r#` if result is keyword
    if result == "type" {
        result.insert_str(0, "r#");
    }

    result
}
