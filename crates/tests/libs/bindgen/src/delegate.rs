#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    DeferralCompletedHandler,
    DeferralCompletedHandler_Vtbl,
    0xed32a372_f3c8_4faa_9cfb_470148da3888
);
impl windows_core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DeferralCompletedHandler {
    pub fn new<F: Fn() -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<DeferralCompletedHandler, F>::new(
            &DeferralCompletedHandlerBox::<F>::VTABLE,
            invoke,
        );
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeferralCompletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct DeferralCompletedHandlerBox<F: Fn() -> windows_core::Result<()> + Send + 'static>(
    core::marker::PhantomData<(fn() -> F,)>,
);
impl<F: Fn() -> windows_core::Result<()> + Send + 'static> DeferralCompletedHandlerBox<F> {
    const VTABLE: DeferralCompletedHandler_Vtbl = DeferralCompletedHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface:
                windows_core::imp::DelegateBox::<DeferralCompletedHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DeferralCompletedHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DeferralCompletedHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void
                as *mut windows_core::imp::DelegateBox<DeferralCompletedHandler, F>);
            (this.invoke)().into()
        }
    }
}
