use super::*;
use tokens::*;

pub fn write_idl(module: &Module, path: &str) -> Result<()> {
    write_to_file(path, format_idl(&module_to_idl("", module).to_string()).map_err(|error| error.with_path(path))?)
}

fn module_to_idl(name: &str, module: &Module) -> TokenStream {
    let modules = module.modules.iter().map(|(name, module)| module_to_idl(name, module));

    if name.is_empty() {
        quote! { #(#modules)* }
    } else {
        let name = to_ident(name);
        let types = module.types.iter().flat_map(|(name, ty)| ty.iter().map(move |ty| (name, ty))).map(|(name, ty)| type_def_to_idl(module, name, ty));

        quote! {
            mod #name {
                #(#modules)*
                #(#types)*
            }
        }
    }
}

fn type_def_to_idl(module: &Module, name: &str, ty: &TypeDef) -> TokenStream {
    let attributes = attributes_to_idl(module, &ty.attributes);

    let ty = if let Some(extends) = &ty.extends {
        if extends.namespace == "System" {
            if extends.name == "Enum" {
                enum_to_idl(module, name, ty)
            } else if extends.name == "ValueType" {
                struct_to_idl(module, name, ty)
            } else if extends.name == "MulticastDelegate" {
                delegate_to_idl(module, name, ty)
            } else {
                class_to_idl(module, name, ty)
            }
        } else {
            class_to_idl(module, name, ty)
        }
    } else {
        interface_to_idl(module, name, ty)
    };

    quote! {
        #attributes
        #ty
    }
}

fn enum_to_idl(module: &Module, name: &str, ty: &TypeDef) -> TokenStream {
    let name = to_ident(name);

    let constants = ty.fields.iter().filter_map(|field| {
        let Some(value) = &field.value else {
            return None;
        };

        let name = to_ident(&field.name);
        let value = value_to_idl(module, value);

        Some(quote! {
            #name = #value
        })
    });

    quote! {
        enum #name {
            #(#constants),*
        }
    }
}

fn value_to_idl(module: &Module, value: &Value) -> TokenStream {
        match value {
            Value::Bool(value) => value.to_string().into(),
            Value::U8(value) => format!("{value}u8").into(),
            Value::I8(value) => format!("{value}i8").into(),
            Value::U16(value) => format!("{value}u16").into(),
            Value::I16(value) => format!("{value}i16").into(),
            Value::U32(value) => format!("{value}u32").into(),
            Value::I32(value) => format!("{value}i32").into(),
            Value::U64(value) => format!("{value}u64").into(),
            Value::I64(value) => format!("{value}i64").into(),
            Value::F32(value) => format!("{value}f32").into(),
            Value::F64(value) => format!("{value}f64").into(),
            Value::String(value) => value.into(),
            Value::TypeName(type_name) => {
                let type_name = reader::TypeName::parse(type_name);
                let namespace = namespace_to_idl(&module.namespace, &type_name.namespace);
                let name = to_ident(&type_name.name);
                quote! { #namespace#name }
            }
            rest => todo!("{:?}", rest),
        }
}

fn struct_to_idl(module: &Module, name: &str, ty: &TypeDef) -> TokenStream {
    let name = to_ident(name);

    let fields = ty.fields.iter().map(|field| {
        let name = to_ident(&field.name);
        let ty = type_to_idl(module, &field.ty);
        quote! {
            #name: #ty
        }
    });

    quote! {
        struct #name {
            #(#fields),*
        }
    }
}

fn attributes_to_idl(module: &Module, attributes: &[Attribute]) -> TokenStream {
    let attributes = attributes.iter().map(|attribute| {
        let namespace = namespace_to_idl(&module.namespace, &attribute.ty.namespace);
        let name = to_ident(&attribute.ty.name);

        if attribute.args.is_empty() {
            quote! {
                #[#namespace#name]
            }
        } else {
            let args = attribute.args.iter().map(|(name, value)| {
                if name.is_empty() {
                    value_to_idl(module, value)
                } else {
                    let name = to_ident(&name);
                    let value = value_to_idl(module, value);
                    quote! {
                        #name: #value
                    }
                }
            });

            quote! {
                #[#namespace#name(#(#args),*)]
            }
        }
    });

    quote! {
        #(#attributes)*
    }
}

fn delegate_to_idl(_module: &Module, name: &str, _ty: &TypeDef) -> TokenStream {
    let name = to_ident(name);

    quote! {
        struct #name();
    }
}

fn class_to_idl(_module: &Module, name: &str, _ty: &TypeDef) -> TokenStream {
    let name = to_ident(name);

    quote! {
        struct #name {
        }
    }
}

fn interface_to_idl(module: &Module, name: &str, ty: &TypeDef) -> TokenStream {
    let name = to_ident(name);

    let methods = ty.methods.iter().map(|method| {
        let name = to_ident(&method.name);
        let params = method.params.iter().map(|param| {
            let name = to_ident(&param.name);
            let ty = type_to_idl(module, &param.ty);
            quote! { #name: #ty }
        });
        let return_type = if method.return_type.ty != Type::Void {
            let ty = type_to_idl(module, &method.return_type.ty);
            quote! { -> #ty }
        } else {
            quote! {}
        };
        quote! {
            fn #name(#(#params),*) #return_type;
        }
    });

    quote! {
        interface #name {
            #(#methods)*
        }
    }
}

fn type_to_idl(module: &Module, ty: &Type) -> TokenStream {
    match ty {
        Type::Void => quote! { ::core::ffi::c_void },
        Type::Bool => quote! { bool },
        Type::Char => quote! { u16 },
        Type::I8 => quote! { i8 },
        Type::U8 => quote! { u8 },
        Type::I16 => quote! { i16 },
        Type::U16 => quote! { u16 },
        Type::I32 => quote! { i32 },
        Type::U32 => quote! { u32 },
        Type::I64 => quote! { i64 },
        Type::U64 => quote! { u64 },
        Type::F32 => quote! { f32 },
        Type::F64 => quote! { f64 },
        Type::ISize => quote! { isize },
        Type::USize => quote! { usize },
        Type::String => {
            quote! { HSTRING }
        }
        Type::BSTR => {
            quote! { BSTR }
        }
        Type::IInspectable => {
            quote! { IInspectable }
        }
        Type::GUID => {
            quote! { GUID }
        }
        Type::IUnknown => {
            quote! { IUnknown }
        }
        Type::HRESULT => {
            quote! { HRESULT }
        }
        Type::PSTR => {
            quote! { PSTR }
        }
        Type::PWSTR => {
            quote! { PWSTR }
        }
        Type::PCSTR => {
            quote! { PCSTR }
        }
        Type::PCWSTR => {
            quote! { PCWSTR }
        }
        Type::Win32Array((ty, len)) => {
            let ty = type_to_idl(module, ty);
            let len = Literal::usize_unsuffixed(*len);
            quote! { [#ty; #len] }
        }
        Type::GenericParam(generic) => {
            let generic = to_ident(generic);
            quote! { #generic }
        }
        Type::TypeRef(ty) => {
            let namespace = namespace_to_idl(&module.namespace, &ty.namespace);
            let name = to_ident(&ty.name);
            if ty.generics.is_empty() {
                quote! { #namespace#name }
            } else {
                let generics = ty.generics.iter().map(|ty| type_to_idl(module, ty));
                quote! { #namespace#name<#(#generics,)*> }
            }
        }
        Type::MutPtr((ty, pointers)) => {
            let pointers = mut_ptrs(*pointers);
            let ty = type_to_idl(module, ty);
            quote! { #pointers #ty }
        }
        Type::ConstPtr((ty, pointers)) => {
            let pointers = const_ptrs(*pointers);
            let ty = type_to_idl(module, ty);
            quote! { #pointers #ty }
        }
        Type::WinrtArray(ty) => type_to_idl(module, ty),
        Type::WinrtArrayRef(ty) => type_to_idl(module, ty),
        Type::ConstRef(ty) => type_to_idl(module, ty),
        _ => todo!(),
    }
}

// TODO: maybe move these functions into `tokens` crate as they duplicated in bindgen

fn namespace_to_idl(relative: &str, namespace: &str) -> TokenStream {
    // TODO: handle nested structs?
    if namespace.is_empty() || relative == namespace {
        quote! {}
    } else {
        let mut relative = relative.split('.').peekable();
        let mut namespace = namespace.split('.').peekable();
        let mut related = false;

        while relative.peek() == namespace.peek() {
            related = true;

            if relative.next().is_none() {
                break;
            }

            namespace.next();
        }

        let mut tokens = TokenStream::new();

        if related {
            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }
        }

        for namespace in namespace {
            tokens.push_str(namespace);
            tokens.push_str("::");
        }

        tokens
    }
}

pub fn to_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await" | "dyn" => format!("r#{name}").into(),
        "Self" | "self" => format!("{name}_").into(),
        "_" => "unused".into(),
        _ => reader::trim_tick(name).into(),
    }
}

fn mut_ptrs(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn const_ptrs(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}
