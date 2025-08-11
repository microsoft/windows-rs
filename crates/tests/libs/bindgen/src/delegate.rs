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
        let com = DeferralCompletedHandlerBox {
            vtable: &DeferralCompletedHandlerBox::<F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this))
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
#[repr(C)]
struct DeferralCompletedHandlerBox<F: Fn() -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DeferralCompletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: Fn() -> windows_core::Result<()> + Send + 'static> DeferralCompletedHandlerBox<F> {
    const VTABLE: DeferralCompletedHandler_Vtbl = DeferralCompletedHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(
        this: *mut core::ffi::c_void,
        iid: *const windows_core::GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <DeferralCompletedHandler as windows_core::Interface>::IID
                || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
            {
                &mut (*this).vtable as *mut _ as _
            } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                (*this).count.add_ref();
                return windows_core::imp::marshaler(
                    core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                    interface,
                );
            } else {
                core::ptr::null_mut()
            };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)().into()
        }
    }
}
