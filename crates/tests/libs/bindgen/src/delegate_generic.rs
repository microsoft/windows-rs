#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<T>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = EventHandlerBox {
            vtable: &EventHandlerBox::<T, F>::VTABLE,
            count: windows_core::imp::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Invoke)(
                windows_core::Interface::as_raw(this),
                sender.param().abi(),
                args.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(C)]
struct EventHandlerBox<
    T,
    F: Fn(
            windows_core::Ref<windows_core::IInspectable>,
            windows_core::Ref<T>,
        ) -> windows_core::Result<()>
        + Send
        + 'static,
> where
    T: windows_core::RuntimeType + 'static,
{
    vtable: *const EventHandler_Vtbl<T>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<
        T: windows_core::RuntimeType + 'static,
        F: Fn(
                windows_core::Ref<windows_core::IInspectable>,
                windows_core::Ref<T>,
            ) -> windows_core::Result<()>
            + Send
            + 'static,
    > EventHandlerBox<T, F>
{
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: Self::QueryInterface,
            AddRef: Self::AddRef,
            Release: Self::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
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
            *interface = if *iid == <EventHandler<T> as windows_core::Interface>::IID
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
    unsafe extern "system" fn Invoke(
        this: *mut core::ffi::c_void,
        sender: *mut core::ffi::c_void,
        args: windows_core::AbiType<T>,
    ) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(
                core::mem::transmute_copy(&sender),
                core::mem::transmute_copy(&args),
            )
            .into()
        }
    }
}
