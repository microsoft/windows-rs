use super::*;

pub fn gen_async(
    def: &GenericType,
    interfaces: &[InterfaceInfo],
    gen: &Gen,
) -> (TokenStream, TokenStream) {
    let kind = async_kind(def);

    if kind != AsyncKind::None {
        return gen_async_kind(kind, def, def, gen);
    }

    for interface in interfaces {
        let kind = async_kind(&interface.def);

        if kind != AsyncKind::None {
            return gen_async_kind(kind, &interface.def, def, gen);
        }
    }

    (TokenStream::new(), TokenStream::new())
}

#[derive(PartialEq)]
pub enum AsyncKind {
    None,
    Action,
    ActionWithProgress,
    Operation,
    OperationWithProgress,
}

pub fn async_kind(def: &GenericType) -> AsyncKind {
    if def.def.namespace() != "Windows.Foundation" {
        return AsyncKind::None;
    }

    match def.def.name() {
        "IAsyncAction" => AsyncKind::Action,
        "IAsyncActionWithProgress`1" => AsyncKind::ActionWithProgress,
        "IAsyncOperation`1" => AsyncKind::Operation,
        "IAsyncOperationWithProgress`2" => AsyncKind::OperationWithProgress,
        _ => AsyncKind::None,
    }
}

fn gen_async_kind(
    kind: AsyncKind,
    name: &GenericType,
    self_name: &GenericType,
    gen: &Gen,
) -> (TokenStream, TokenStream) {
    let return_type = match kind {
        AsyncKind::Operation | AsyncKind::OperationWithProgress => name.generics[0].gen_name(gen),
        _ => quote! { () },
    };

    let handler = match kind {
        AsyncKind::Action => quote! { AsyncActionCompletedHandler },
        AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
        AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
        AsyncKind::OperationWithProgress => quote! { AsyncOperationWithProgressCompletedHandler },
        _ => panic!("Unexpected AsyncKind"),
    };

    let constraints = self_name.gen_constraints(gen);
    let name = self_name.gen_name(gen);
    let namespace = gen.namespace("Windows.Foundation");

    (
        quote! {
            pub fn get(&self) -> ::windows::Result<#return_type> {
                if self.Status()? == #namespace AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::Waiter::new();
                    self.SetCompleted(#namespace  #handler::new(move |_sender, _args| {
                        // Safe because the waiter will only be dropped after being signaled.
                        unsafe { signaler.signal(); }
                        Ok(())
                    }))?;
                }
                self.GetResults()
            }
        },
        quote! {
            impl<#constraints> ::std::future::Future for #name {
                type Output = ::windows::Result<#return_type>;

                fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
                    if self.Status()? == #namespace AsyncStatus::Started {
                        let waker = context.waker().clone();

                        let _ = self.SetCompleted(#namespace #handler::new(move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        }));

                        ::std::task::Poll::Pending
                    } else {
                        ::std::task::Poll::Ready(self.GetResults())
                    }
                }
            }
        },
    )
}
