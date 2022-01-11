use crate::*;

pub fn gen_async(def: &TypeDef, interfaces: &[InterfaceInfo], gen: &Gen, cfg: &TokenStream) -> (TokenStream, TokenStream) {
    let kind = def.async_kind();

    if kind != AsyncKind::None {
        return gen_async_kind(kind, def, def, gen, cfg);
    }

    for interface in interfaces {
        let kind = interface.def.async_kind();

        if kind != AsyncKind::None {
            return gen_async_kind(kind, &interface.def, def, gen, cfg);
        }
    }

    (TokenStream::new(), TokenStream::new())
}

fn gen_async_kind(kind: AsyncKind, name: &TypeDef, self_name: &TypeDef, gen: &Gen, cfg: &TokenStream) -> (TokenStream, TokenStream) {
    let return_sig = match kind {
        AsyncKind::Operation | AsyncKind::OperationWithProgress => gen_name(&name.generics[0], gen),
        _ => quote! { () },
    };

    let handler = match kind {
        AsyncKind::Action => quote! { AsyncActionCompletedHandler },
        AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
        AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
        AsyncKind::OperationWithProgress => quote! { AsyncOperationWithProgressCompletedHandler },
        _ => unimplemented!(),
    };

    let constraints = gen_constraints(self_name);
    let name = gen_type_name(self_name, gen);
    let namespace = gen.namespace("Windows.Foundation");

    (
        quote! {
            #cfg
            pub fn get(&self) -> ::windows::core::Result<#return_sig> {
                if self.Status()? == #namespace AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::core::Waiter::new();
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
            #cfg
            impl<#constraints> ::std::future::Future for #name {
                type Output = ::windows::core::Result<#return_sig>;

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
