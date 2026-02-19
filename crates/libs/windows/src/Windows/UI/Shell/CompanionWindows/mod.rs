#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompanionWindowCoordinator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompanionWindowCoordinator, windows_core::IUnknown, windows_core::IInspectable);
impl CompanionWindowCoordinator {
    pub fn RequestWindowFromAppAsync(&self, appid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<CompanionWindowRequestResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestWindowFromAppAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(appid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachCompanionWindow(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DetachCompanionWindow)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CompanionWindowId(&self) -> windows_core::Result<super::super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompanionWindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Changed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CompanionWindowCoordinator, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Changed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForWindow(windowid: super::super::WindowId) -> windows_core::Result<CompanionWindowCoordinator> {
        Self::ICompanionWindowCoordinatorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindow)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICompanionWindowCoordinatorStatics<R, F: FnOnce(&ICompanionWindowCoordinatorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompanionWindowCoordinator, ICompanionWindowCoordinatorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompanionWindowCoordinator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompanionWindowCoordinator>();
}
unsafe impl windows_core::Interface for CompanionWindowCoordinator {
    type Vtable = <ICompanionWindowCoordinator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompanionWindowCoordinator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompanionWindowCoordinator {
    const NAME: &'static str = "Windows.UI.Shell.CompanionWindows.CompanionWindowCoordinator";
}
unsafe impl Send for CompanionWindowCoordinator {}
unsafe impl Sync for CompanionWindowCoordinator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompanionWindowRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompanionWindowRequest, windows_core::IUnknown, windows_core::IInspectable);
impl CompanionWindowRequest {
    pub fn Accept(&self, windowid: super::super::WindowId) -> windows_core::Result<CompanionWindowCoordinator> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Accept)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Reject(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Reject)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDeferral(&self) -> windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDeferral)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RequestingWindowId(&self) -> windows_core::Result<super::super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestingWindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetFromLaunchUri<P0>(launchuri: P0) -> windows_core::Result<CompanionWindowRequest>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::ICompanionWindowRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromLaunchUri)(windows_core::Interface::as_raw(this), launchuri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICompanionWindowRequestStatics<R, F: FnOnce(&ICompanionWindowRequestStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompanionWindowRequest, ICompanionWindowRequestStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompanionWindowRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompanionWindowRequest>();
}
unsafe impl windows_core::Interface for CompanionWindowRequest {
    type Vtable = <ICompanionWindowRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompanionWindowRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompanionWindowRequest {
    const NAME: &'static str = "Windows.UI.Shell.CompanionWindows.CompanionWindowRequest";
}
unsafe impl Send for CompanionWindowRequest {}
unsafe impl Sync for CompanionWindowRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompanionWindowRequestResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompanionWindowRequestResult, windows_core::IUnknown, windows_core::IInspectable);
impl CompanionWindowRequestResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Succeeded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<CompanionWindowRequestResultStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ExtendedError(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExtendedError)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CompanionWindowId(&self) -> windows_core::Result<super::super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CompanionWindowId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CompanionWindowRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompanionWindowRequestResult>();
}
unsafe impl windows_core::Interface for CompanionWindowRequestResult {
    type Vtable = <ICompanionWindowRequestResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompanionWindowRequestResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompanionWindowRequestResult {
    const NAME: &'static str = "Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResult";
}
unsafe impl Send for CompanionWindowRequestResult {}
unsafe impl Sync for CompanionWindowRequestResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompanionWindowRequestResultStatus(pub i32);
impl CompanionWindowRequestResultStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const RegistrationNotFound: Self = Self(2i32);
    pub const ActivationTimedOut: Self = Self(3i32);
    pub const RejectedByCompanionApp: Self = Self(4i32);
}
impl windows_core::TypeKind for CompanionWindowRequestResultStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CompanionWindowRequestResultStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.CompanionWindows.CompanionWindowRequestResultStatus;i4)");
}
windows_core::imp::define_interface!(ICompanionWindowCoordinator, ICompanionWindowCoordinator_Vtbl, 0x05620e87_b0f7_59ba_b3a5_d614bdc1ebe3);
impl windows_core::RuntimeType for ICompanionWindowCoordinator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompanionWindowCoordinator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestWindowFromAppAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachCompanionWindow: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CompanionWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::WindowId) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompanionWindowCoordinatorStatics, ICompanionWindowCoordinatorStatics_Vtbl, 0x964022fa_380e_518c_bfc8_0f3b84fafea3);
impl windows_core::RuntimeType for ICompanionWindowCoordinatorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompanionWindowCoordinatorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompanionWindowRequest, ICompanionWindowRequest_Vtbl, 0xd92c351a_2d66_59a8_b345_78489562c4d8);
impl windows_core::RuntimeType for ICompanionWindowRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompanionWindowRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Accept: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reject: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestingWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompanionWindowRequestResult, ICompanionWindowRequestResult_Vtbl, 0xd728d2ef_e6d4_5cc0_9ff4_20c17a2ce72d);
impl windows_core::RuntimeType for ICompanionWindowRequestResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompanionWindowRequestResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CompanionWindowRequestResultStatus) -> windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub CompanionWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompanionWindowRequestStatics, ICompanionWindowRequestStatics_Vtbl, 0x585e4544_d474_506a_96c2_3597a44882da);
impl windows_core::RuntimeType for ICompanionWindowRequestStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompanionWindowRequestStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFromLaunchUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
