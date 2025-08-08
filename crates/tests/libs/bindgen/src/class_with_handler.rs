#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Deferral,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(Deferral, IClosable);
impl Deferral {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Create<P0>(handler: P0) -> windows_core::Result<Deferral>
    where
        P0: windows_core::Param<DeferralCompletedHandler>,
    {
        Self::IDeferralFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                handler.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Deferral, IDeferralFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Deferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IDeferral>();
}
unsafe impl windows_core::Interface for Deferral {
    type Vtable = <IDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
unsafe impl Send for Deferral {}
unsafe impl Sync for Deferral {}
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
windows_core::imp::define_interface!(
    IClosable,
    IClosable_Vtbl,
    0x30d5a829_7fa4_4026_83bb_d75bae4ea99e
);
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IClosable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
pub trait IClosable_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl IClosable_Vtbl {
    pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClosable_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClosable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeferral,
    IDeferral_Vtbl,
    0xd6269732_3b7f_46a7_b40b_4fdca2a2c693
);
impl windows_core::RuntimeType for IDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeferralFactory,
    IDeferralFactory_Vtbl,
    0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a
);
impl windows_core::RuntimeType for IDeferralFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferralFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
