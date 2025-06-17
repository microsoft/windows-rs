use windows_bindgen::*;
use windows_idl as idl;
mod fmt;

fn main() {
    let file = std::include_str!("WebView2.idl");
    let file = idl::parse(file).unwrap();

    let mut tokens = quote! {};

    for item in &file.items {
        let item = write_item(item);

        tokens = quote! {
            #tokens
            #item
        };
    }

    let tokens = fmt::fmt(tokens.as_str());
    write_to_file("crates/libs/webview/src/bindings.rs", tokens);
}

fn write_item(item: &idl::Item) -> TokenStream {
    match item {
        idl::Item::Enum(item) => write_enum(item),
        idl::Item::Interface(item) => write_interface(item),
        idl::Item::Struct(item) => write_struct(item),
        idl::Item::Library(item) => write_library(item),
        idl::Item::CppQuote(item) => write_cpp_quote(item),
        _ => quote! {},
    }
}

fn write_enum(item: &idl::Enum) -> TokenStream {
    let type_name = to_ident(&item.name);
    let mut next = 0;

    let variants = item.variants.iter().map(|variant| {
        let name = to_ident(&variant.name);

        let value = if let Some(value) = variant.value {
            value
        } else {
            next
        };

        next = value + 1;
        let value = Literal::i64_unsuffixed(value);

        quote! {
            pub const #name: #type_name = #type_name(#value);
        }
    });

    quote! {
        #[repr(transparent)]
        pub struct #type_name(pub u32);
        #(#variants)*
    }
}

fn write_interface(item: &idl::Interface) -> TokenStream {
    let type_name = to_ident(&item.name);

    let methods = item.methods.iter().map(|method| {
        let mut name = to_ident(&method.name);

        if method.attributes.iter().any(|a| a.name == "propput") {
            name = name.prefix("put_");
        } else if method.attributes.iter().any(|a| a.name == "propget") {
            name = name.prefix("get_");
        }

        debug_assert_eq!(method.return_type, "HRESULT");

        quote! {
            pub fn #name(&self) -> crate::HRESULT {
                todo!()
            }
        }
    });

    quote! {
        #[repr(transparent)]
        pub struct #type_name(windows_core::IUnknown);
        impl #type_name {
            #(#methods)*
        }
    }
}

fn write_type(ty: &str) -> TokenStream {
    match ty {
        "BOOL" => quote! { windows_core::BOOL },
        "UINT32" => quote! { u32 },
        "BYTE" => quote! { u8 },
        _ => to_ident(ty),
    }
}

fn write_struct(item: &idl::Struct) -> TokenStream {
    let type_name = to_ident(&item.name);

    let fields = item.fields.iter().map(|field| {
        let name = to_ident(&field.name);
        let ty = write_type(&field.field_type);

        quote! {
            pub #name: #ty,
        }
    });

    quote! {
        #[repr(C)]
        pub struct #type_name {
            #(#fields)*
        }
    }
}

fn write_library(item: &idl::Library) -> TokenStream {
    let mut tokens = quote! {};

    for item in &item.items {
        tokens.combine(write_item(item));
    }

    tokens
}

fn write_cpp_quote(_item: &str) -> TokenStream {
    quote! {}
}
