use crate::types::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn async_get_tokens(name: &TypeName, interfaces: &Vec<RequiredInterface>) -> TokenStream {
    let kind = async_kind(name);
    if kind != AsyncKind::None {
        return to_async_get_tokens(kind, name, &name.namespace);
    }

    for interface in interfaces {
        let kind = async_kind(&interface.name);

        if kind != AsyncKind::None {
            return to_async_get_tokens(kind, &interface.name, &name.namespace);
        }
    }

    TokenStream::new()
}

#[derive(PartialEq)]
enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}

fn async_kind(name: &TypeName) -> AsyncKind {
    if name.namespace != "Windows.Foundation" {
        return AsyncKind::None;
    }

    match name.name.as_ref() {
        "IAsyncAction" => AsyncKind::Action,
        "IAsyncActionWithProgress`1" => AsyncKind::ActionWithProgress,
        "IAsyncOperation`1" => AsyncKind::Operation,
        "IAsyncOperationWithProgress`2" => AsyncKind::OperationWithProgress,
        _ => AsyncKind::None,
    }
}

fn to_async_get_tokens(kind: AsyncKind, name: &TypeName, calling_namespace: &str) -> TokenStream {
    let namespace = to_namespace_tokens("Windows.Foundation", calling_namespace);

    let return_type = match kind {
        AsyncKind::Operation | AsyncKind::OperationWithProgress => name.generics[0].to_tokens(),
        _ => quote! { () },
    };

    let handler = match kind {
        AsyncKind::Action => quote! { AsyncActionCompletedHandler },
        AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
        AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
        AsyncKind::OperationWithProgress => quote! { AsyncOperationWithProgressCompletedHandler },
        _ => panic!(),
    };

    quote! {
        pub fn get(&self) -> ::winrt::Result<#return_type> {
            if self.status()? == #namespace AsyncStatus::Started {
                unsafe {
                    let event = ::winrt::CreateEventW(::std::ptr::null_mut(), 1, 0, ::std::ptr::null_mut());
                    self.set_completed(#namespace #handler::new(move |_sender, _args| {
                        ::winrt::SetEvent(event);
                        Ok(())
                    }))?;
                    ::winrt::WaitForSingleObject(event, 0xFFFFFFFF);
                    ::winrt::CloseHandle(event);
                }
            }
            self.get_results()
        }
    }
}
