windows_core::imp::define_interface!(
    Handler,
    Handler_Vtbl,
    0x9df939b9_b0e1_58dc_89ec_fc84175d6ef4
);
impl windows_core::RuntimeType for Handler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl Handler {
    pub fn new<F: Fn(i32) + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&HandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
}
#[repr(C)]
pub struct Handler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        value: i32,
    ) -> windows_core::HRESULT,
}
struct HandlerBox<F: Fn(i32) + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(i32) + 'static> HandlerBox<F> {
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
        value: i32,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<Handler, F>);
            (this.invoke)(value);
            windows_core::HRESULT(0)
        }
    }
}
windows_core::imp::define_interface!(Sink, Sink_Vtbl, 0xfbb9e298_5149_534a_843a_fce157f2a8f2);
impl windows_core::RuntimeType for Sink {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(Sink, windows_core::IUnknown, windows_core::IInspectable);
impl Sink {
    pub fn Subscribe<P0>(&self, handler: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Handler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subscribe)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for Sink {
    const NAME: &'static str = "Test.Sink";
}
pub trait Sink_Impl: windows_core::IUnknownImpl {
    fn Subscribe(&self, handler: windows_core::Ref<Handler>) -> windows_core::Result<i32>;
}
impl Sink_Vtbl {
    pub const fn new<Identity: Sink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Subscribe<Identity: Sink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Sink_Impl::Subscribe(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Sink, OFFSET>(),
            Subscribe: Subscribe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Sink as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Sink_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Subscribe: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
}
