windows_core::imp::define_winrt_interface!(
    IWidget,
    IWidget_Vtbl,
    0x05cda87c_844a_5686_8429_f335c8ee6276,
    b"Test.IWidget"
);
windows_core::imp::interface_hierarchy!(
    IWidget,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidget {
    pub fn Read(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IWidget {
    const NAME: &'static str = "Test.IWidget";
}
pub trait IWidget_Impl: windows_core::IUnknownImpl {
    fn Read(&self) -> windows_core::Result<i32>;
}
impl IWidget_Vtbl {
    pub const fn new<Identity: IWidget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IWidget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidget_Impl::Read(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Read: Read::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidget as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IWidget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_winrt_interface!(
    IWidgetStatics,
    IWidgetStatics_Vtbl,
    0x06f114ad_1fde_5492_8c0e_7f775964db69,
    b"Test.IWidgetStatics"
);
windows_core::imp::interface_hierarchy!(
    IWidgetStatics,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWidgetStatics {
    pub fn Create(&self) -> windows_core::Result<Widget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Create)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IWidgetStatics {
    const NAME: &'static str = "Test.IWidgetStatics";
}
pub trait IWidgetStatics_Impl: windows_core::IUnknownImpl {
    fn Create(&self) -> windows_core::Result<Widget>;
}
impl IWidgetStatics_Vtbl {
    pub const fn new<Identity: IWidgetStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IWidgetStatics_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWidgetStatics_Impl::Create(this) {
                    Ok(ok__) => {
                        core::ptr::write(result__.cast(), ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWidgetStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IWidgetStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Widget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Widget,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IWidget
);
impl Widget {
    pub fn Read(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Read)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Create() -> windows_core::Result<Self> {
        Self::IWidgetStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::from_abi(result__))
        })
    }
    fn IWidgetStatics<R, F: FnOnce(&IWidgetStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Widget, IWidgetStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWidget>();
}
unsafe impl windows_core::Interface for Widget {
    type Vtable = <IWidget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWidget as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for Widget {
    const NAME: &'static str = "Test.Widget";
}
