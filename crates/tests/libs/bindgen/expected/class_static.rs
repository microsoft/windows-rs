#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Class(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Class, windows_core::IUnknown, windows_core::IInspectable);
impl Class {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, windows_core::imp::IGenericFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Method(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Method)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Create(value: i32) -> windows_core::Result<Self> {
        Self::IClassStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(
                windows_core::Interface::as_raw(this),
                value,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn Value() -> i32 {
        Self::IClassStatics(|this| {
            Ok(unsafe {
                let mut result__ = core::mem::zeroed();
                let hresult__ = (windows_core::Interface::vtable(this).Value)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                );
                debug_assert!(hresult__.0 == 0);
                result__
            })
        })
        .unwrap()
    }
    pub fn SetValue(value: i32) {
        Self::IClassStatics(|this| unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).SetValue)(
                windows_core::Interface::as_raw(this),
                value,
            );
            debug_assert!(hresult__.0 == 0);
            Ok(())
        })
        .unwrap();
    }
    pub fn Text() -> windows_core::HSTRING {
        Self::IClassStatics(|this| {
            Ok(unsafe {
                let mut result__ = core::mem::zeroed();
                let hresult__ = (windows_core::Interface::vtable(this).Text)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                );
                debug_assert!(hresult__.0 == 0);
                core::mem::transmute(result__)
            })
        })
        .unwrap()
    }
    pub fn Next() -> Option<IClass> {
        Self::IClassStatics(|this| {
            Ok(unsafe {
                let mut result__ = core::mem::zeroed();
                let hresult__ = (windows_core::Interface::vtable(this).Next)(
                    windows_core::Interface::as_raw(this),
                    &mut result__,
                );
                debug_assert!(hresult__.0 == 0);
                core::mem::transmute(result__)
            })
        })
        .unwrap()
    }
    pub fn Numbers() -> windows_core::Array<i32> {
        Self::IClassStatics(|this| {
            Ok(unsafe {
                let mut result__ = core::mem::MaybeUninit::zeroed();
                let hresult__ = (windows_core::Interface::vtable(this).Numbers)(
                    windows_core::Interface::as_raw(this),
                    windows_core::Array::<i32>::set_abi_len(core::mem::transmute(&mut result__)),
                    result__.as_mut_ptr() as *mut _ as _,
                );
                debug_assert!(hresult__.0 == 0);
                result__.assume_init()
            })
        })
        .unwrap()
    }
    fn IClassStatics<R, F: FnOnce(&IClassStatics) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Class, IClassStatics> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Class {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IClass>();
}
unsafe impl windows_core::Interface for Class {
    type Vtable = <IClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Class {
    const NAME: &'static str = "Test.Class";
}
windows_core::imp::define_interface!(IClass, IClass_Vtbl, 0x09c2c798_8aad_52ab_a9d9_33725f7248f3);
impl windows_core::RuntimeType for IClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IClass");
}
#[repr(C)]
pub struct IClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Method:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IClassStatics,
    IClassStatics_Vtbl,
    0xe859dd18_1e99_5da7_a6b0_bbb6011bd19d
);
impl windows_core::RuntimeType for IClassStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IClassStatics");
}
#[repr(C)]
pub struct IClassStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Text: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Numbers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut *mut i32,
    ) -> windows_core::HRESULT,
}
