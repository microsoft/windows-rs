windows_core::imp::define_interface!(
    Callable,
    Callable_Vtbl,
    0xc6166a85_543a_5b5a_9324_0fd34a07698b
);
impl windows_core::RuntimeType for Callable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    Callable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Callable {
    pub fn Invoke(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Invoke)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct Callable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Constructed(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Constructed,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IConstructed
);
impl Constructed {
    pub fn new() -> windows_core::Result<Self> {
        Self::IConstructedFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IConstructedFactory<R, F: FnOnce(&IConstructedFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Constructed, IConstructedFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Constructed {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IConstructed>();
}
unsafe impl windows_core::Interface for Constructed {
    type Vtable = <IConstructed as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IConstructed as windows_core::Interface>::IID;
}
impl core::ops::Deref for Constructed {
    type Target = IConstructed;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for Constructed {
    const NAME: &'static str = "Test.Constructed";
}
windows_core::imp::define_interface!(
    IConstructed,
    IConstructed_Vtbl,
    0xe372f62b_40ef_530c_a87a_efcdd0ceab3b
);
impl windows_core::RuntimeType for IConstructed {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IConstructed,
    windows_core::IUnknown,
    windows_core::IInspectable
);
#[repr(C)]
pub struct IConstructed_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(
    IConstructedFactory,
    IConstructedFactory_Vtbl,
    0x212e4b12_3d77_5712_8556_c828917a66a0
);
impl windows_core::RuntimeType for IConstructedFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IConstructedFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IConstructedFactory {
    pub fn CreateInstance<P0>(
        &self,
        outer: P0,
        inner: &mut Option<windows_core::IInspectable>,
    ) -> windows_core::Result<Constructed>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                outer.param().abi(),
                inner as *mut _ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IConstructedFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IMemberOnly,
    IMemberOnly_Vtbl,
    0x1b61d9b4_47b8_5fea_9b61_d77788fcb639
);
impl windows_core::RuntimeType for IMemberOnly {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IMemberOnly,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IMemberOnly {
    pub fn Value(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetValue(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetValue)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMemberOnly_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    Implemented,
    Implemented_Vtbl,
    0x6df5563f_dd6b_5b44_bf76_ca328373a0a3
);
impl windows_core::RuntimeType for Implemented {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Implemented");
}
windows_core::imp::interface_hierarchy!(
    Implemented,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl windows_core::RuntimeName for Implemented {
    const NAME: &'static str = "Test.Implemented";
}
pub trait Implemented_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<i32>;
}
impl Implemented_Vtbl {
    pub const fn new<Identity: Implemented_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: Implemented_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match Implemented_Impl::Invoke(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, Implemented, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<Implemented as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct Implemented_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Invoke:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberOnly(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    MemberOnly,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IMemberOnly
);
impl windows_core::RuntimeType for MemberOnly {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IMemberOnly>();
}
unsafe impl windows_core::Interface for MemberOnly {
    type Vtable = <IMemberOnly as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMemberOnly as windows_core::Interface>::IID;
}
impl core::ops::Deref for MemberOnly {
    type Target = IMemberOnly;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeName for MemberOnly {
    const NAME: &'static str = "Test.MemberOnly";
}
