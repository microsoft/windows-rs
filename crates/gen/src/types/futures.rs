use crate::types::*;
use squote::{quote, TokenStream};

pub fn get_async_tokens(
    name: &TypeName,
    interfaces: &[RequiredInterface],
) -> (TokenStream, TokenStream) {
    let kind = async_kind(name);
    if kind != AsyncKind::None {
        return to_async_tokens(kind, name, name);
    }

    for interface in interfaces {
        let kind = async_kind(&interface.name);

        if kind != AsyncKind::None {
            return to_async_tokens(kind, &interface.name, name);
        }
    }

    (TokenStream::new(), TokenStream::new())
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

fn to_async_tokens(
    kind: AsyncKind,
    name: &TypeName,
    self_name: &TypeName,
) -> (TokenStream, TokenStream) {
    let namespace = to_namespace_tokens("Windows.Foundation", &self_name.namespace);

    let return_type = match kind {
        AsyncKind::Operation | AsyncKind::OperationWithProgress => name.generics[0].to_tokens(),
        _ => quote! { () },
    };

    let handler = match kind {
        AsyncKind::Action => quote! { AsyncActionCompletedHandler },
        AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
        AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
        AsyncKind::OperationWithProgress => quote! { AsyncOperationWithProgressCompletedHandler },
        _ => panic!("Unexpected AsyncKind"),
    };

    let constraints = &self_name.constraints;
    let name = &self_name.tokens;

    (
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
        },
        quote! {
            impl<#constraints> ::std::future::Future for #name {
                type Output = ::winrt::Result<#return_type>;

                fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
                    if self.status()? == #namespace AsyncStatus::Started {
                        let waker = context.waker().clone();

                        self.set_completed(#namespace #handler::new(move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        }))?;

                        ::std::task::Poll::Pending
                    } else {
                        ::std::task::Poll::Ready(self.get_results())
                    }
                }
            }
        },
    )
}
