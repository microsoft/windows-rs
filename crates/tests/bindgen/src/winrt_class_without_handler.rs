#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IClosable,
    IClosable_Vtbl,
    0x30d5a829_7fa4_4026_83bb_d75bae4ea99e
);
windows_core::imp::interface_hierarchy!(
    IClosable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IClosable {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeferral,
    IDeferral_Vtbl,
    0xd6269732_3b7f_46a7_b40b_4fdca2a2c693
);
windows_core::imp::interface_hierarchy!(
    IDeferral,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IDeferral {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IDeferralFactory,
    IDeferralFactory_Vtbl,
    0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a
);
windows_core::imp::interface_hierarchy!(
    IDeferralFactory,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl core::ops::Deref for IDeferralFactory {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl windows_core::RuntimeType for IDeferralFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeferralFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Create: usize,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Deferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Deferral,
    windows_core::IUnknown,
    windows_core::IInspectable
);
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
