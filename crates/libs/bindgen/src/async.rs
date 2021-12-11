use super::*;

pub fn gen_async(def: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let kind = def.async_kind();

    if kind != AsyncKind::None {
        return gen_async_kind(kind, def, def, cfg, gen);
    }

    let interfaces = if def.kind() == TypeKind::Class { def.class_interfaces().iter().map(|(def, _)| def.clone()).collect() } else { def.required_interfaces() };

    for interface in interfaces {
        let kind = interface.async_kind();

        if kind != AsyncKind::None {
            return gen_async_kind(kind, &interface, def, cfg, gen);
        }
    }

    quote! {}
}

fn gen_async_kind(kind: AsyncKind, name: &TypeDef, self_name: &TypeDef, cfg: &Cfg, gen: &Gen) -> TokenStream {
    let return_sig = match kind {
        AsyncKind::Operation | AsyncKind::OperationWithProgress => gen_element_name(&name.generics[0], gen),
        _ => quote! { () },
    };

    let handler = match kind {
        AsyncKind::Action => quote! { AsyncActionCompletedHandler },
        AsyncKind::ActionWithProgress => quote! { AsyncActionWithProgressCompletedHandler },
        AsyncKind::Operation => quote! { AsyncOperationCompletedHandler },
        AsyncKind::OperationWithProgress => quote! { AsyncOperationWithProgressCompletedHandler },
        _ => unimplemented!(),
    };

    let constraints = gen_type_constraints(self_name, gen);
    let name = gen_type_name(self_name, gen);
    let namespace = gen.namespace("Windows.Foundation");
    let cfg = cfg.and_async().gen(gen);

    quote! {
        #cfg
        impl<#(#constraints)*> #name {
            pub fn get(&self) -> ::windows::core::Result<#return_sig> {
                if self.Status()? == #namespace AsyncStatus::Started {
                    let (_waiter, signaler) = ::windows::core::Waiter::new();
                    self.SetCompleted(#namespace  #handler::new(move |_sender, _args| {
                        // Safe because the waiter will only be dropped after being signaled.
                        unsafe { signaler.signal(); }
                        Ok(())
                    }))?;
                }
                self.GetResults()
            }
        }
        #cfg
        impl<#(#constraints)*> ::std::future::Future for #name {
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
    }
}
