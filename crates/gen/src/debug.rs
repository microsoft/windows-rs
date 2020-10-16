use crate::*;
use squote::{quote, TokenStream};

pub fn gen_debug(type_name: &TypeName, interfaces: &[RequiredInterface]) -> TokenStream {
    let name = &type_name.name;
    let default_impl = quote! { ::std::format!("{}({:?})", #name, <Self as ::winrt::AbiTransferable>::get_abi(self)) };

    let implements_istringable = interfaces.iter().any(|interface| {
        interface.name.name == "IStringable" && interface.name.namespace == "Windows.Foundation"
    });

    let is_istringable =
        type_name.name == "IStringable" && type_name.namespace == "Windows.Foundation";

    let implementation = if implements_istringable && !is_istringable {
        quote! {
            "{:?}",
            {
                let s: ::winrt::foundation::IStringable = self.into();
                s
            }
        }
    } else if is_istringable {
        quote! {
            "{}",
            match self.to_string() {
                Ok(s) => s.to_string(),
                Err(_) => #default_impl
            }
        }
    } else {
        quote! {
            "{}", #default_impl
        }
    };

    gen(type_name, &implementation)
}

pub fn default_gen_debug(type_name: &TypeName) -> TokenStream {
    let name = &type_name.name;
    let implementation =
        quote! { "{}({:?})", #name, <Self as ::winrt::AbiTransferable>::get_abi(self) };

    gen(type_name, &implementation)
}

fn gen(type_name: &TypeName, implementation: &TokenStream) -> TokenStream {
    let constraints = type_name.gen_constraint();
    let name = type_name.gen();
    quote! {
        impl<#constraints> ::std::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    #implementation
                )
            }
        }
    }
}
