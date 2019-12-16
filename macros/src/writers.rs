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
    let enums = write_enums(namespace);
    let structs = write_structs(namespace);
    let interfaces = write_interfaces(namespace);
    let classes = write_classes(namespace);
    let namespaces = write_namespace_set(namespace.namespaces(), scope);

    quote! {
        pub mod #module {
            #enums
            #structs
            #interfaces
            #classes
            #namespaces
        }
    }
}

fn write_classes(namespace: &winmd::Namespace) -> TokenStream {
    let mut tokens = quote! {};

    for t in namespace.classes() {
        let name = format_ident!("{}", t.name());
        tokens = quote! {
            #tokens
            pub struct #name { ptr: *const std::ffi::c_void }
        };
    }

    tokens
}

fn write_interfaces(namespace: &winmd::Namespace) -> TokenStream {
    let mut tokens = quote! {};

    for interface in namespace.interfaces() {
        let name = interface.name();
        let name_ident = format_ident!("{}", name);
        let abi_name_ident = format_ident!("abi_{}", name);
        let abi_methods = write_abi_methods(&interface);
        let consume_methods = write_consume_methods(&interface);
        tokens = quote! {
            #tokens
            #[repr(C)]
            pub struct #name_ident { ptr: *const std::ffi::c_void }
            #[repr(C)]
            struct #abi_name_ident {
                _0: usize,
                _1: usize,
                _2: usize,
                _3: usize,
                _4: usize,
                _5: usize,
                #abi_methods
            }
            impl #name_ident {
                #consume_methods
            }
        };
    }

    tokens
}

// fn write_consume_methods(interface: &winmd::TypeDef) -> TokenStream {
// }
// fn write_produce_methods(interface: &winmd::TypeDef) -> TokenStream {
// }

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

    for method in interface.methods() {
        let name = format_ident!("{}", method.name());
        let signature = method.signature();
        let params = write_consume_params(&signature);
        let args = write_abi_args(&signature);

        if let Some(result) = signature.return_type() {
            let result = write_type_sig(result.sig_type());

            tokens = quote! {
                #tokens
                pub fn #name(&self, #params) -> winrt::Result<#result> {
                    unsafe {
                        let mut winrt_impl_result: #result = Default::default();
                        ((*(*(self.ptr as *const *const #abi_interface_name))).#name)(
                            self.ptr, #args &mut winrt_impl_result,
                        )
                        .ok_or(winrt_impl_result)
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

fn write_abi_params(signature: &winmd::MethodSig) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (param, param_sig) in signature.params() {
        tokens.push(write_abi_param(param, param_sig));
    }

    if let Some(param_sig) = signature.return_type() {
        let name = write_abi_type_sig(param_sig.sig_type());
        tokens.push(quote!{ &mut #name });
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

fn write_abi_arg(param: &winmd::Param, param_sig: &winmd::ParamSig) -> TokenStream {
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
        winmd::TypeSigType::GenericSig(value) => panic!("GenericSig"),
        winmd::TypeSigType::GenericTypeIndex(value) => panic!("GenericTypeIndex"),
        winmd::TypeSigType::GenericMethodIndex(value) => panic!("GenericMethodIndex"),
        _ => panic!("write_abi_type_sig"),
    }
}

fn write_type_sig(value: &winmd::TypeSig) -> TokenStream {
    match value.sig_type() {
        winmd::TypeSigType::ElementType(value) => write_element_type(value),
        winmd::TypeSigType::TypeDefOrRef(value) => write_type_def_or_ref(value),
        winmd::TypeSigType::GenericSig(value) => panic!("GenericSig"),
        winmd::TypeSigType::GenericTypeIndex(value) => panic!("GenericTypeIndex"),
        winmd::TypeSigType::GenericMethodIndex(value) => panic!("GenericMethodIndex"),
        _ => panic!("write_type_sig"),
    }
}

fn write_abi_element_type(value: &winmd::ElementType) -> TokenStream {
    match value {
        winmd::ElementType::Bool => quote! { bool },
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

fn write_enums(namespace: &winmd::Namespace) -> TokenStream {
    let mut tokens = quote! {};

    for t in namespace.enums() {
        let name = format_ident!("{}", t.name());
        let fields = write_enum_fields(&t);
        tokens = quote! {
            #tokens
            pub enum #name { #fields }
        };
    }

    tokens
}

fn write_enum_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        for c in f.constants() {
            let name = format_ident!("{}", f.name());
            let value = c.value();

            tokens = quote! {
                #tokens
                #name,
                // TODO: write out the enum value
            };
        }
    }

    tokens
}

fn write_structs(namespace: &winmd::Namespace) -> TokenStream {
    let mut tokens = quote! {};

    for t in namespace.structs() {
        let name = format_ident!("{}", t.name());
        let fields = write_struct_fields(&t);
        tokens = quote! {
            #tokens
            #[repr(C)]
            #[derive(Default, Debug)]
            pub struct #name { #fields }
        };
    }

    tokens
}

fn write_struct_fields(t: &winmd::TypeDef) -> TokenStream {
    let mut tokens = quote! {};

    for f in t.fields() {
        let name = format_ident!("{}", to_snake(f.name()));

        tokens = quote! {
            #tokens
            #name: u32,
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
