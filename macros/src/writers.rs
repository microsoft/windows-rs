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
    let mut tokens = Vec::<TokenStream>::new();

    for t in namespace.types() {
        match t.category() {
            TypeCategory::Interface => tokens.push(write_interface(&t, scope)),
            TypeCategory::Class => tokens.push(write_class(&t, scope)),
            TypeCategory::Enum => tokens.push(write_enum(&t, scope)),
            TypeCategory::Struct => tokens.push(write_struct(&t, scope)),
            _ => {}
        };
    }

    TokenStream::from_iter(tokens)
}

fn write_class(class: &TypeDef, _scope: &std::collections::BTreeSet<String>) -> TokenStream {
    // TODO: don't define struct here if the class is static - only declare.

    let name = format_ident!("{}", class.name());
    let functions = write_class_functions(&class);
    let string_name = format!("{}.{}", class.namespace(), class.name());

    quote! {
        pub struct #name { ptr: *const std::ffi::c_void }
        impl #name { #functions }
        impl winrt::TypeName for #name {
            fn type_name() -> &'static str {
                #string_name
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
                        let method_name = format_ident!("{}", method.name());
                        let signature = method.signature();
                        let params = write_consume_params(&signature);
                        let args = signature.params().iter().map(|(param, _)| format_ident!("{}", param.name()));

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
    let generics = interface.generics();

    let name = if generics.is_empty() {
        interface.name()
    }
    else {
        let name = interface.name();
        name.get(..name.len() - 2).unwrap()
    };

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
                    #g1,
                    #g2,
                    #g3,
                    &[#g4, #g5, #g6, #g7, #g8, #g9, #g10, #g11],
                );
                &GUID
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
        let name = format_ident!("{}", method.name());
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
        let name = format_ident!("{}", method.name());
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
        _ => quote! {
            let mut __ok = Default::default();
        },
    }
}

fn write_abi_params(signature: &MethodSig) -> TokenStream {
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

fn write_consume_params(signature: &MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_consume_param(param, param_sig));
    }

    TokenStream::from_iter(tokens)
}

fn write_abi_args(signature: &MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_abi_arg(param, param_sig));
    }

    TokenStream::from_iter(tokens)
}

fn write_abi_param(param: &Param, param_sig: &ParamSig) -> TokenStream {
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

fn write_consume_param(param: &Param, param_sig: &ParamSig) -> TokenStream {
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

fn write_abi_arg(param: &Param, _param_sig: &ParamSig) -> TokenStream {
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

fn write_abi_type_sig(value: &TypeSig) -> TokenStream {
    match value.sig_type() {
        TypeSigType::ElementType(value) => write_abi_element_type(value),
        TypeSigType::TypeDefOrRef(value) => write_abi_type_def_or_ref(value),
        TypeSigType::GenericSig(_value) => panic!("GenericSig"),
        TypeSigType::GenericTypeIndex(_value) => panic!("GenericTypeIndex"),
        TypeSigType::GenericMethodIndex(_value) => panic!("GenericMethodIndex"),
    }
}

fn write_type_sig(value: &TypeSig) -> TokenStream {
    match value.sig_type() {
        TypeSigType::ElementType(value) => write_element_type(value),
        TypeSigType::TypeDefOrRef(value) => write_type_def_or_ref(value),
        TypeSigType::GenericSig(_value) => panic!("GenericSig"),
        TypeSigType::GenericTypeIndex(_value) => panic!("GenericTypeIndex"),
        TypeSigType::GenericMethodIndex(_value) => panic!("GenericMethodIndex"),
    }
}

fn write_abi_element_type(value: &ElementType) -> TokenStream {
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
        ElementType::String => quote! { *mut std::ffi::c_void },
        ElementType::Object => quote! { *mut std::ffi::c_void },
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

fn write_abi_type_def_or_ref(value: &TypeDefOrRef) -> TokenStream {
    match value {
        TypeDefOrRef::TypeDef(value) => write_abi_type_def(value),
        TypeDefOrRef::TypeRef(value) => write_abi_type_ref(value),
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

fn write_abi_type_def(value: &TypeDef) -> TokenStream {
    match value.category() {
        TypeCategory::Struct => {
            let name = format_ident!("{}", value.name());
            quote! { #name }
        }
        _ => quote! { *mut std::ffi::c_void },
    }
}

fn write_type_def(value: &TypeDef) -> TokenStream {
    let name = format_ident!("{}", value.name());
    quote! { #name }
}

fn write_abi_type_ref(value: &TypeRef) -> TokenStream {
    if value.name() == "Guid" && value.namespace() == "System" {
        quote! { winrt::Guid }
    } else {
        write_abi_type_def(&value.resolve())
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
        let name = format_ident!("{}", to_snake(f.name()));

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

    // TODO: go through all keywords and prepend `r#` if result is keyword
    if result == "type" {
        result.insert_str(0, "r#");
    }

    result
}
