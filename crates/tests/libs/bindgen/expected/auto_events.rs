windows_core::imp::define_interface!(
    CompletedCallback,
    CompletedCallback_Vtbl,
    0xf1cd8bd1_dcc8_59e4_bcf4_ef29f644772c
);
impl windows_core::RuntimeType for CompletedCallback {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl CompletedCallback {
    pub fn new<F: Fn(i32) + Send + Sync + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(
            &CompletedCallbackBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct CompletedCallback_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        result: i32,
    ) -> windows_core::HRESULT,
}
struct CompletedCallbackBox<F: Fn(i32) + Send + Sync + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn(i32) + Send + Sync + 'static> CompletedCallbackBox<F> {
    const VTABLE: CompletedCallback_Vtbl = CompletedCallback_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<CompletedCallback, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<CompletedCallback, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<CompletedCallback, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        result: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<CompletedCallback, F>);
            (this.invoke)(result);
            windows_core::HRESULT(0)
        }
    }
}
windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0xd8ccb2eb_f4d4_55c6_977f_a23ed0a7d401
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: i32,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<F: Fn(windows_core::Ref<IFoo>, i32) + Send + Sync + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn(windows_core::Ref<IFoo>, i32) + Send + Sync + 'static> HandlerBox<F> {
    const VTABLE: Handler_Vtbl = Handler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<Handler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<Handler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<Handler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &*(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), args);
            windows_core::HRESULT(0)
        }
    }
}
windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x822a1799_57f1_55f1_83d3_8c32e23dea74);
impl windows_core::RuntimeType for IFoo {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IFoo, windows_core::IUnknown, windows_core::IInspectable);
impl IFoo {
    pub fn Bar(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bar)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, i32) + Send + Sync + 'static,
    {
        let handler: Handler = {
            let com = windows_core::imp::DelegateBox::<Handler, F>::new(
                &HandlerBox::<F>::VTABLE,
                handler,
            );
            unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
        };
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(
                windows_core::Interface::as_raw(self),
                windows_core::Interface::as_raw(&handler),
                &mut result__,
            )
            .map(|| result__)?;
            Ok(windows_core::EventRevoker::new(
                self.clone(),
                token__,
                windows_core::Interface::vtable(self).RemoveClick,
            ))
        }
    }
    pub fn SetCompleted<P0>(&self, callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<CompletedCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompleted)(
                windows_core::Interface::as_raw(self),
                callback.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFoo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Click: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i64,
    ) -> windows_core::HRESULT,
    pub RemoveClick:
        unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
