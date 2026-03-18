#[cfg(feature = "ApplicationModel_Preview_Holographic")]
pub mod Holographic;
#[cfg(feature = "ApplicationModel_Preview_InkWorkspace")]
pub mod InkWorkspace;
#[cfg(feature = "ApplicationModel_Preview_Notes")]
pub mod Notes;
windows_core::imp::define_interface!(IStartupAppInfoPreview, IStartupAppInfoPreview_Vtbl, 0xc3a147db_09fa_5aa5_b3bd_119a09963d58);
impl windows_core::RuntimeType for IStartupAppInfoPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupAppInfoPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Impact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StartupAppImpactPreview) -> windows_core::HRESULT,
    pub ExecutablePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStartupAppsManagerPreview, IStartupAppsManagerPreview_Vtbl, 0x7197b9c1_03bb_5693_87c3_6f983cc70fb3);
impl windows_core::RuntimeType for IStartupAppsManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupAppsManagerPreview_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetStartupAppInfos: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStartupAppsManagerPreviewStatics, IStartupAppsManagerPreviewStatics_Vtbl, 0x9d0331f5_343f_5cd7_9d66_762cfa2c0380);
impl windows_core::RuntimeType for IStartupAppsManagerPreviewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IStartupAppsManagerPreviewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StartupAppImpactPreview(pub i32);
impl StartupAppImpactPreview {
    pub const Unknown: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
    pub const Medium: Self = Self(3i32);
    pub const High: Self = Self(4i32);
}
impl windows_core::TypeKind for StartupAppImpactPreview {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for StartupAppImpactPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Preview.StartupAppImpactPreview;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartupAppInfoPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StartupAppInfoPreview, windows_core::IUnknown, windows_core::IInspectable);
impl StartupAppInfoPreview {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Publisher(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Publisher)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Impact(&self) -> windows_core::Result<StartupAppImpactPreview> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Impact)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExecutablePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExecutablePath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for StartupAppInfoPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStartupAppInfoPreview>();
}
unsafe impl windows_core::Interface for StartupAppInfoPreview {
    type Vtable = <IStartupAppInfoPreview as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStartupAppInfoPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StartupAppInfoPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.StartupAppInfoPreview";
}
unsafe impl Send for StartupAppInfoPreview {}
unsafe impl Sync for StartupAppInfoPreview {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartupAppsManagerPreview(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StartupAppsManagerPreview, windows_core::IUnknown, windows_core::IInspectable);
impl StartupAppsManagerPreview {
    pub fn GetStartupAppInfos(&self) -> windows_core::Result<windows_collections::IVectorView<StartupAppInfoPreview>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetStartupAppInfos)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDefault() -> windows_core::Result<StartupAppsManagerPreview> {
        Self::IStartupAppsManagerPreviewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStartupAppsManagerPreviewStatics<R, F: FnOnce(&IStartupAppsManagerPreviewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StartupAppsManagerPreview, IStartupAppsManagerPreviewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StartupAppsManagerPreview {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStartupAppsManagerPreview>();
}
unsafe impl windows_core::Interface for StartupAppsManagerPreview {
    type Vtable = <IStartupAppsManagerPreview as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStartupAppsManagerPreview as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StartupAppsManagerPreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.StartupAppsManagerPreview";
}
unsafe impl Send for StartupAppsManagerPreview {}
unsafe impl Sync for StartupAppsManagerPreview {}
