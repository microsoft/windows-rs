#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcquireEnvironmentInfoResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AcquireEnvironmentInfoResult, windows_core::IUnknown, windows_core::IInspectable);
impl AcquireEnvironmentInfoResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn EnvironmentInfo(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnvironmentInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance<P0>(result: P0, environmentinfo: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
    {
        Self::IAcquireEnvironmentInfoResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), core::mem::transmute_copy(environmentinfo), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAcquireEnvironmentInfoResultFactory<R, F: FnOnce(&IAcquireEnvironmentInfoResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AcquireEnvironmentInfoResult, IAcquireEnvironmentInfoResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AcquireEnvironmentInfoResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAcquireEnvironmentInfoResult>();
}
unsafe impl windows_core::Interface for AcquireEnvironmentInfoResult {
    type Vtable = <IAcquireEnvironmentInfoResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAcquireEnvironmentInfoResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AcquireEnvironmentInfoResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.AcquireEnvironmentInfoResult";
}
unsafe impl Send for AcquireEnvironmentInfoResult {}
unsafe impl Sync for AcquireEnvironmentInfoResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AreRebootsPendingResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AreRebootsPendingResult, windows_core::IUnknown, windows_core::IInspectable);
impl AreRebootsPendingResult {
    pub fn Result(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsRebootPending(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRebootPending)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance<P0>(result: P0, isrebootpending: bool) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateOperationResult>,
    {
        Self::IAreRebootsPendingResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), isrebootpending, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAreRebootsPendingResultFactory<R, F: FnOnce(&IAreRebootsPendingResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AreRebootsPendingResult, IAreRebootsPendingResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AreRebootsPendingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAreRebootsPendingResult>();
}
unsafe impl windows_core::Interface for AreRebootsPendingResult {
    type Vtable = <IAreRebootsPendingResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAreRebootsPendingResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AreRebootsPendingResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.AreRebootsPendingResult";
}
unsafe impl Send for AreRebootsPendingResult {}
unsafe impl Sync for AreRebootsPendingResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClusterNativeUpdateCompletionStatus(pub i32);
impl ClusterNativeUpdateCompletionStatus {
    pub const Completed: Self = Self(0);
    pub const Failed: Self = Self(1);
    pub const Canceled: Self = Self(2);
}
impl windows_core::TypeKind for ClusterNativeUpdateCompletionStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClusterNativeUpdateCompletionStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.Cluster.ClusterNativeUpdateCompletionStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.ClusterNativeUpdateCompletionStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClusterNativeUpdateCredentialStatus(pub i32);
impl ClusterNativeUpdateCredentialStatus {
    pub const Success: Self = Self(0);
    pub const NoSuchCredential: Self = Self(1);
    pub const ErrorRetrievingCredential: Self = Self(2);
}
impl windows_core::TypeKind for ClusterNativeUpdateCredentialStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClusterNativeUpdateCredentialStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.Cluster.ClusterNativeUpdateCredentialStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.ClusterNativeUpdateCredentialStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClusterNativeUpdateEnvironmentValidationStatus(pub i32);
impl ClusterNativeUpdateEnvironmentValidationStatus {
    pub const Approved: Self = Self(0);
    pub const ApprovedWithWarnings: Self = Self(1);
    pub const Rejected: Self = Self(2);
}
impl windows_core::TypeKind for ClusterNativeUpdateEnvironmentValidationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClusterNativeUpdateEnvironmentValidationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.Cluster.ClusterNativeUpdateEnvironmentValidationStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.ClusterNativeUpdateEnvironmentValidationStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClusterNativeUpdateLogLevel(pub i32);
impl ClusterNativeUpdateLogLevel {
    pub const Debug: Self = Self(0);
    pub const Verbose: Self = Self(1);
    pub const Info: Self = Self(2);
    pub const Warning: Self = Self(3);
    pub const Error: Self = Self(4);
}
impl windows_core::TypeKind for ClusterNativeUpdateLogLevel {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClusterNativeUpdateLogLevel {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.Cluster.ClusterNativeUpdateLogLevel;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.ClusterNativeUpdateLogLevel");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClusterNativeUpdateOperationStatus(pub i32);
impl ClusterNativeUpdateOperationStatus {
    pub const Success: Self = Self(0);
    pub const Failed: Self = Self(1);
    pub const Pending: Self = Self(2);
    pub const Canceled: Self = Self(3);
}
impl windows_core::TypeKind for ClusterNativeUpdateOperationStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ClusterNativeUpdateOperationStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Management.Update.Cluster.ClusterNativeUpdateOperationStatus;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.ClusterNativeUpdateOperationStatus");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClusterUpdateServices(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ClusterUpdateServices, windows_core::IUnknown, windows_core::IInspectable);
impl ClusterUpdateServices {
    pub fn WriteLogEntry(&self, level: ClusterNativeUpdateLogLevel, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteLogEntry)(windows_core::Interface::as_raw(self), level, core::mem::transmute_copy(message)).ok() }
    }
    pub fn ReportProgress(&self, currentstep: i32, maxsteps: i32, message: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReportProgress)(windows_core::Interface::as_raw(self), currentstep, maxsteps, core::mem::transmute_copy(message)).ok() }
    }
    pub fn GetPluginCredential(&self, credentialid: &windows_core::HSTRING) -> windows_core::Result<UpdateCredential> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPluginCredential)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(credentialid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsOperationMarkedAsCanceled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOperationMarkedAsCanceled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SaveRunStateInformation(&self, persistentinformation: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveRunStateInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(persistentinformation)).ok() }
    }
    pub fn GetRunStateInformation(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunStateInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SaveRunIndependentInformation(&self, persistentinformation: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SaveRunIndependentInformation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(persistentinformation)).ok() }
    }
    pub fn GetRunIndependentInformation(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunIndependentInformation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for ClusterUpdateServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IClusterUpdateServices>();
}
unsafe impl windows_core::Interface for ClusterUpdateServices {
    type Vtable = <IClusterUpdateServices as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IClusterUpdateServices as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ClusterUpdateServices {
    const NAME: &'static str = "Windows.Management.Update.Cluster.ClusterUpdateServices";
}
unsafe impl Send for ClusterUpdateServices {}
unsafe impl Sync for ClusterUpdateServices {}
windows_core::imp::define_interface!(IAcquireEnvironmentInfoResult, IAcquireEnvironmentInfoResult_Vtbl, 0xb720e3a4_9a34_51c1_a1fa_0c6673bef689);
impl windows_core::RuntimeType for IAcquireEnvironmentInfoResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IAcquireEnvironmentInfoResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcquireEnvironmentInfoResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnvironmentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAcquireEnvironmentInfoResultFactory, IAcquireEnvironmentInfoResultFactory_Vtbl, 0xbbe87f45_0125_5b6d_b9de_05effc98becb);
impl windows_core::RuntimeType for IAcquireEnvironmentInfoResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IAcquireEnvironmentInfoResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcquireEnvironmentInfoResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAreRebootsPendingResult, IAreRebootsPendingResult_Vtbl, 0xa0c93ffd_8409_5fe1_876b_d59d5d9951b5);
impl windows_core::RuntimeType for IAreRebootsPendingResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IAreRebootsPendingResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAreRebootsPendingResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRebootPending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAreRebootsPendingResultFactory, IAreRebootsPendingResultFactory_Vtbl, 0x801c7304_1fdf_59b4_9ef6_adbd0544d98f);
impl windows_core::RuntimeType for IAreRebootsPendingResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IAreRebootsPendingResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAreRebootsPendingResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClusterNativeEnvironmentOperations, IClusterNativeEnvironmentOperations_Vtbl, 0xa6d3d05e_1cfe_5363_b745_c6981d0e6b25);
impl windows_core::RuntimeType for IClusterNativeEnvironmentOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IClusterNativeEnvironmentOperations");
}
windows_core::imp::interface_hierarchy!(IClusterNativeEnvironmentOperations, windows_core::IUnknown, windows_core::IInspectable);
impl IClusterNativeEnvironmentOperations {
    pub fn AcquireNodeEnvironmentInfo(&self) -> windows_core::Result<AcquireEnvironmentInfoResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AcquireNodeEnvironmentInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValidateClusterEnvironment<P0>(&self, nodeenvironmentinfo: P0) -> windows_core::Result<ValidateClusterEnvironmentResult>
    where
        P0: windows_core::Param<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateClusterEnvironment)(windows_core::Interface::as_raw(self), nodeenvironmentinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPreUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPreUpdateRunOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPreUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreUpdateRunOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPostUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPostUpdateRunOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPostUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPostUpdateRunOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IClusterNativeEnvironmentOperations {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IClusterNativeEnvironmentOperations";
}
pub trait IClusterNativeEnvironmentOperations_Impl: windows_core::IUnknownImpl {
    fn AcquireNodeEnvironmentInfo(&self) -> windows_core::Result<AcquireEnvironmentInfoResult>;
    fn ValidateClusterEnvironment(&self, nodeEnvironmentInfo: windows_core::Ref<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>) -> windows_core::Result<ValidateClusterEnvironmentResult>;
    fn StartPreUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPreUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPostUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPostUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
}
impl IClusterNativeEnvironmentOperations_Vtbl {
    pub const fn new<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AcquireNodeEnvironmentInfo<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::AcquireNodeEnvironmentInfo(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateClusterEnvironment<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeenvironmentinfo: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::ValidateClusterEnvironment(this, core::mem::transmute_copy(&nodeenvironmentinfo)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPreUpdateRunOperation<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::StartPreUpdateRunOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPreUpdateRunOperationComplete<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::IsPreUpdateRunOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPostUpdateRunOperation<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::StartPostUpdateRunOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPostUpdateRunOperationComplete<Identity: IClusterNativeEnvironmentOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperations_Impl::IsPostUpdateRunOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClusterNativeEnvironmentOperations, OFFSET>(),
            AcquireNodeEnvironmentInfo: AcquireNodeEnvironmentInfo::<Identity, OFFSET>,
            ValidateClusterEnvironment: ValidateClusterEnvironment::<Identity, OFFSET>,
            StartPreUpdateRunOperation: StartPreUpdateRunOperation::<Identity, OFFSET>,
            IsPreUpdateRunOperationComplete: IsPreUpdateRunOperationComplete::<Identity, OFFSET>,
            StartPostUpdateRunOperation: StartPostUpdateRunOperation::<Identity, OFFSET>,
            IsPostUpdateRunOperationComplete: IsPostUpdateRunOperationComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClusterNativeEnvironmentOperations as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterNativeEnvironmentOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AcquireNodeEnvironmentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateClusterEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPreUpdateRunOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPreUpdateRunOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPostUpdateRunOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPostUpdateRunOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClusterNativeEnvironmentOperationsPlugin, IClusterNativeEnvironmentOperationsPlugin_Vtbl, 0x534b9984_3201_5a90_a42b_42d9c98c961a);
impl windows_core::RuntimeType for IClusterNativeEnvironmentOperationsPlugin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IClusterNativeEnvironmentOperationsPlugin");
}
windows_core::imp::interface_hierarchy!(IClusterNativeEnvironmentOperationsPlugin, windows_core::IUnknown, windows_core::IInspectable);
impl IClusterNativeEnvironmentOperationsPlugin {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEnvironmentOperations<P2, P3>(&self, clustername: &windows_core::HSTRING, runid: &windows_core::HSTRING, options: P2, updateservices: P3) -> windows_core::Result<IClusterNativeEnvironmentOperations>
    where
        P2: windows_core::Param<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>,
        P3: windows_core::Param<ClusterUpdateServices>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEnvironmentOperations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clustername), core::mem::transmute_copy(runid), options.param().abi(), updateservices.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IClusterNativeEnvironmentOperationsPlugin {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IClusterNativeEnvironmentOperationsPlugin";
}
pub trait IClusterNativeEnvironmentOperationsPlugin_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo>;
    fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>;
    fn CreateEnvironmentOperations(&self, clusterName: &windows_core::HSTRING, runId: &windows_core::HSTRING, options: windows_core::Ref<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>, updateServices: windows_core::Ref<ClusterUpdateServices>) -> windows_core::Result<IClusterNativeEnvironmentOperations>;
}
impl IClusterNativeEnvironmentOperationsPlugin_Vtbl {
    pub const fn new<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Version<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::Version(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultOptions<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::DefaultOptions(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEnvironmentOperations<Identity: IClusterNativeEnvironmentOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clustername: *mut core::ffi::c_void, runid: *mut core::ffi::c_void, options: *mut core::ffi::c_void, updateservices: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeEnvironmentOperationsPlugin_Impl::CreateEnvironmentOperations(this, core::mem::transmute(&clustername), core::mem::transmute(&runid), core::mem::transmute_copy(&options), core::mem::transmute_copy(&updateservices)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClusterNativeEnvironmentOperationsPlugin, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            DefaultOptions: DefaultOptions::<Identity, OFFSET>,
            CreateEnvironmentOperations: CreateEnvironmentOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClusterNativeEnvironmentOperationsPlugin as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterNativeEnvironmentOperationsPlugin_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEnvironmentOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClusterNativeNodeOperations, IClusterNativeNodeOperations_Vtbl, 0xce23db69_b72b_5204_9d1c_c53d0b2d803d);
impl windows_core::RuntimeType for IClusterNativeNodeOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IClusterNativeNodeOperations");
}
windows_core::imp::interface_hierarchy!(IClusterNativeNodeOperations, windows_core::IUnknown, windows_core::IInspectable);
impl IClusterNativeNodeOperations {
    pub fn AreAdditionalRebootsPending(&self) -> windows_core::Result<AreRebootsPendingResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreAdditionalRebootsPending)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPreRebootOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPreRebootOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPreRebootOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreRebootOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPostRebootOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPostRebootOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPostRebootOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPostRebootOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPreDrainOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPreDrainOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPreDrainOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreDrainOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPostResumeOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPostResumeOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPostResumeOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPostResumeOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPreUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPreUpdateRunOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPreUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPreUpdateRunOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StartPostUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPostUpdateRunOperation)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsPostUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPostUpdateRunOperationComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IClusterNativeNodeOperations {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IClusterNativeNodeOperations";
}
pub trait IClusterNativeNodeOperations_Impl: windows_core::IUnknownImpl {
    fn AreAdditionalRebootsPending(&self) -> windows_core::Result<AreRebootsPendingResult>;
    fn StartPreRebootOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPreRebootOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPostRebootOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPostRebootOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPreDrainOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPreDrainOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPostResumeOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPostResumeOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPreUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPreUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
    fn StartPostUpdateRunOperation(&self) -> windows_core::Result<UpdateOperationResult>;
    fn IsPostUpdateRunOperationComplete(&self) -> windows_core::Result<UpdatePendingOperationResult>;
}
impl IClusterNativeNodeOperations_Vtbl {
    pub const fn new<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AreAdditionalRebootsPending<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::AreAdditionalRebootsPending(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPreRebootOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPreRebootOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPreRebootOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPreRebootOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPostRebootOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPostRebootOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPostRebootOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPostRebootOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPreDrainOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPreDrainOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPreDrainOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPreDrainOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPostResumeOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPostResumeOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPostResumeOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPostResumeOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPreUpdateRunOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPreUpdateRunOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPreUpdateRunOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPreUpdateRunOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartPostUpdateRunOperation<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::StartPostUpdateRunOperation(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPostUpdateRunOperationComplete<Identity: IClusterNativeNodeOperations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperations_Impl::IsPostUpdateRunOperationComplete(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClusterNativeNodeOperations, OFFSET>(),
            AreAdditionalRebootsPending: AreAdditionalRebootsPending::<Identity, OFFSET>,
            StartPreRebootOperation: StartPreRebootOperation::<Identity, OFFSET>,
            IsPreRebootOperationComplete: IsPreRebootOperationComplete::<Identity, OFFSET>,
            StartPostRebootOperation: StartPostRebootOperation::<Identity, OFFSET>,
            IsPostRebootOperationComplete: IsPostRebootOperationComplete::<Identity, OFFSET>,
            StartPreDrainOperation: StartPreDrainOperation::<Identity, OFFSET>,
            IsPreDrainOperationComplete: IsPreDrainOperationComplete::<Identity, OFFSET>,
            StartPostResumeOperation: StartPostResumeOperation::<Identity, OFFSET>,
            IsPostResumeOperationComplete: IsPostResumeOperationComplete::<Identity, OFFSET>,
            StartPreUpdateRunOperation: StartPreUpdateRunOperation::<Identity, OFFSET>,
            IsPreUpdateRunOperationComplete: IsPreUpdateRunOperationComplete::<Identity, OFFSET>,
            StartPostUpdateRunOperation: StartPostUpdateRunOperation::<Identity, OFFSET>,
            IsPostUpdateRunOperationComplete: IsPostUpdateRunOperationComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClusterNativeNodeOperations as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterNativeNodeOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreAdditionalRebootsPending: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPreRebootOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPreRebootOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPostRebootOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPostRebootOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPreDrainOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPreDrainOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPostResumeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPostResumeOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPreUpdateRunOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPreUpdateRunOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPostUpdateRunOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsPostUpdateRunOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClusterNativeNodeOperationsPlugin, IClusterNativeNodeOperationsPlugin_Vtbl, 0x5f7a457c_e9d3_5cec_a967_0da9d95fd6f4);
impl windows_core::RuntimeType for IClusterNativeNodeOperationsPlugin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IClusterNativeNodeOperationsPlugin");
}
windows_core::imp::interface_hierarchy!(IClusterNativeNodeOperationsPlugin, windows_core::IUnknown, windows_core::IInspectable);
impl IClusterNativeNodeOperationsPlugin {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateNodeOperations<P2, P3>(&self, clustername: &windows_core::HSTRING, runid: &windows_core::HSTRING, options: P2, updateservices: P3) -> windows_core::Result<IClusterNativeNodeOperations>
    where
        P2: windows_core::Param<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>,
        P3: windows_core::Param<ClusterUpdateServices>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNodeOperations)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clustername), core::mem::transmute_copy(runid), options.param().abi(), updateservices.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IClusterNativeNodeOperationsPlugin {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IClusterNativeNodeOperationsPlugin";
}
pub trait IClusterNativeNodeOperationsPlugin_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo>;
    fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>;
    fn CreateNodeOperations(&self, clusterName: &windows_core::HSTRING, runId: &windows_core::HSTRING, options: windows_core::Ref<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>, updateServices: windows_core::Ref<ClusterUpdateServices>) -> windows_core::Result<IClusterNativeNodeOperations>;
}
impl IClusterNativeNodeOperationsPlugin_Vtbl {
    pub const fn new<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Version<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::Version(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultOptions<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::DefaultOptions(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNodeOperations<Identity: IClusterNativeNodeOperationsPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clustername: *mut core::ffi::c_void, runid: *mut core::ffi::c_void, options: *mut core::ffi::c_void, updateservices: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IClusterNativeNodeOperationsPlugin_Impl::CreateNodeOperations(this, core::mem::transmute(&clustername), core::mem::transmute(&runid), core::mem::transmute_copy(&options), core::mem::transmute_copy(&updateservices)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IClusterNativeNodeOperationsPlugin, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            DefaultOptions: DefaultOptions::<Identity, OFFSET>,
            CreateNodeOperations: CreateNodeOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClusterNativeNodeOperationsPlugin as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterNativeNodeOperationsPlugin_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNodeOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IClusterUpdateServices, IClusterUpdateServices_Vtbl, 0x38461e68_1445_53e3_b914_ce19e3b730c4);
impl windows_core::RuntimeType for IClusterUpdateServices {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IClusterUpdateServices");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClusterUpdateServices_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WriteLogEntry: unsafe extern "system" fn(*mut core::ffi::c_void, ClusterNativeUpdateLogLevel, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReportProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPluginCredential: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOperationMarkedAsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SaveRunStateInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRunStateInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SaveRunIndependentInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRunIndependentInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInstallUpdateTaskResult, IInstallUpdateTaskResult_Vtbl, 0xd519090a_4e93_5916_92bb_9356da52344b);
impl windows_core::RuntimeType for IInstallUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IInstallUpdateTaskResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallUpdateTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShouldRollback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInstallUpdateTaskResultFactory, IInstallUpdateTaskResultFactory_Vtbl, 0x07473356_1a87_554d_9c14_489c0d69b258);
impl windows_core::RuntimeType for IInstallUpdateTaskResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IInstallUpdateTaskResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstallUpdateTaskResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRollbackUpdateTaskResult, IRollbackUpdateTaskResult_Vtbl, 0x477e2a95_bcb6_5044_ad3d_e4180efcf1be);
impl windows_core::RuntimeType for IRollbackUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IRollbackUpdateTaskResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IRollbackUpdateTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InstallRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRollbackUpdateTaskResultFactory, IRollbackUpdateTaskResultFactory_Vtbl, 0x2dbd5bc3_efdf_5046_8954_78981796f016);
impl windows_core::RuntimeType for IRollbackUpdateTaskResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IRollbackUpdateTaskResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IRollbackUpdateTaskResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScanUpdateTaskResult, IScanUpdateTaskResult_Vtbl, 0x24e9f302_a976_5148_b109_5100ac1a50ed);
impl windows_core::RuntimeType for IScanUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IScanUpdateTaskResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IScanUpdateTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScanRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScanUpdateTaskResultFactory, IScanUpdateTaskResultFactory_Vtbl, 0x12594ad2_5312_5f8a_8fa2_2da4a34e3319);
impl windows_core::RuntimeType for IScanUpdateTaskResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IScanUpdateTaskResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IScanUpdateTaskResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStageUpdateTaskResult, IStageUpdateTaskResult_Vtbl, 0xc926add1_335c_57f7_8dac_1ce2fce65060);
impl windows_core::RuntimeType for IStageUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IStageUpdateTaskResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IStageUpdateTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StageRecords: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStageUpdateTaskResultFactory, IStageUpdateTaskResultFactory_Vtbl, 0x2256548a_f175_5143_888a_f49f27a34ed3);
impl windows_core::RuntimeType for IStageUpdateTaskResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IStageUpdateTaskResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IStageUpdateTaskResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateCredential, IUpdateCredential_Vtbl, 0xde6d09b1_bcdb_5d6d_9d46_7250b853fffe);
impl windows_core::RuntimeType for IUpdateCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateCredential");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateCredential_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateCredentialStatus) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateCredentialFactory, IUpdateCredentialFactory_Vtbl, 0xe251a298_44b1_5455_8b44_5971bd516f09);
impl windows_core::RuntimeType for IUpdateCredentialFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateCredentialFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateCredentialFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, ClusterNativeUpdateCredentialStatus, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateInstallRecord, IUpdateInstallRecord_Vtbl, 0x38118797_dc5d_5727_89ff_5b8a44b3f53b);
impl windows_core::RuntimeType for IUpdateInstallRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateInstallRecord");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallRecord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateCompletionStatus) -> windows_core::HRESULT,
    pub FailureMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateInstallRecordFactory, IUpdateInstallRecordFactory_Vtbl, 0x9b6c54b5_d229_5147_9d6e_16e47f2317db);
impl windows_core::RuntimeType for IUpdateInstallRecordFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateInstallRecordFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallRecordFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, ClusterNativeUpdateCompletionStatus, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateInstaller, IUpdateInstaller_Vtbl, 0xc8cfe973_daaf_57d3_8d3b_59eec0b082ea);
impl windows_core::RuntimeType for IUpdateInstaller {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateInstaller");
}
windows_core::imp::interface_hierarchy!(IUpdateInstaller, windows_core::IUnknown, windows_core::IInspectable);
impl IUpdateInstaller {
    pub fn Scan(&self) -> windows_core::Result<ScanUpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scan)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Stage<P0>(&self, updatestostage: P0) -> windows_core::Result<StageUpdateTaskResult>
    where
        P0: windows_core::Param<windows_collections::IVectorView<UpdateScanRecord>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stage)(windows_core::Interface::as_raw(self), updatestostage.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Install<P0>(&self, updatestoinstall: P0) -> windows_core::Result<InstallUpdateTaskResult>
    where
        P0: windows_core::Param<windows_collections::IVectorView<UpdateScanRecord>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Install)(windows_core::Interface::as_raw(self), updatestoinstall.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Rollback<P0>(&self, updatestorollback: P0) -> windows_core::Result<RollbackUpdateTaskResult>
    where
        P0: windows_core::Param<windows_collections::IVectorView<UpdateScanRecord>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rollback)(windows_core::Interface::as_raw(self), updatestorollback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShouldUsePluginSpecificReboot(&self, usepluginreboot: &mut bool) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldUsePluginSpecificReboot)(windows_core::Interface::as_raw(self), usepluginreboot, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RebootNode(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RebootNode)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AcquireUpdateListValidationInfo(&self, recipevalidationinfo: &mut windows_core::HSTRING) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AcquireUpdateListValidationInfo)(windows_core::Interface::as_raw(self), recipevalidationinfo as *mut _ as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ValidateAllNodesUpdateRecipe<P0>(&self, allnodesrecipevalidationinfo: P0) -> windows_core::Result<ValidateAllNodesUpdateRecipeResult>
    where
        P0: windows_core::Param<windows_collections::IVectorView<UpdateRecipeNodeValidationInfoRecord>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidateAllNodesUpdateRecipe)(windows_core::Interface::as_raw(self), allnodesrecipevalidationinfo.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IUpdateInstaller {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IUpdateInstaller";
}
pub trait IUpdateInstaller_Impl: windows_core::IUnknownImpl {
    fn Scan(&self) -> windows_core::Result<ScanUpdateTaskResult>;
    fn Stage(&self, updatesToStage: windows_core::Ref<windows_collections::IVectorView<UpdateScanRecord>>) -> windows_core::Result<StageUpdateTaskResult>;
    fn Install(&self, updatesToInstall: windows_core::Ref<windows_collections::IVectorView<UpdateScanRecord>>) -> windows_core::Result<InstallUpdateTaskResult>;
    fn Rollback(&self, updatesToRollback: windows_core::Ref<windows_collections::IVectorView<UpdateScanRecord>>) -> windows_core::Result<RollbackUpdateTaskResult>;
    fn ShouldUsePluginSpecificReboot(&self, usePluginReboot: &mut bool) -> windows_core::Result<UpdateTaskResult>;
    fn RebootNode(&self) -> windows_core::Result<UpdateTaskResult>;
    fn AcquireUpdateListValidationInfo(&self, recipeValidationInfo: &mut windows_core::HSTRING) -> windows_core::Result<UpdateTaskResult>;
    fn ValidateAllNodesUpdateRecipe(&self, allNodesRecipeValidationInfo: windows_core::Ref<windows_collections::IVectorView<UpdateRecipeNodeValidationInfoRecord>>) -> windows_core::Result<ValidateAllNodesUpdateRecipeResult>;
}
impl IUpdateInstaller_Vtbl {
    pub const fn new<Identity: IUpdateInstaller_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scan<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Scan(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Stage<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatestostage: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Stage(this, core::mem::transmute_copy(&updatestostage)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Install<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatestoinstall: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Install(this, core::mem::transmute_copy(&updatestoinstall)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Rollback<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatestorollback: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::Rollback(this, core::mem::transmute_copy(&updatestorollback)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShouldUsePluginSpecificReboot<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usepluginreboot: *mut bool, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::ShouldUsePluginSpecificReboot(this, core::mem::transmute_copy(&usepluginreboot)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RebootNode<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::RebootNode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AcquireUpdateListValidationInfo<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recipevalidationinfo: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::AcquireUpdateListValidationInfo(this, core::mem::transmute_copy(&recipevalidationinfo)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ValidateAllNodesUpdateRecipe<Identity: IUpdateInstaller_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allnodesrecipevalidationinfo: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstaller_Impl::ValidateAllNodesUpdateRecipe(this, core::mem::transmute_copy(&allnodesrecipevalidationinfo)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUpdateInstaller, OFFSET>(),
            Scan: Scan::<Identity, OFFSET>,
            Stage: Stage::<Identity, OFFSET>,
            Install: Install::<Identity, OFFSET>,
            Rollback: Rollback::<Identity, OFFSET>,
            ShouldUsePluginSpecificReboot: ShouldUsePluginSpecificReboot::<Identity, OFFSET>,
            RebootNode: RebootNode::<Identity, OFFSET>,
            AcquireUpdateListValidationInfo: AcquireUpdateListValidationInfo::<Identity, OFFSET>,
            ValidateAllNodesUpdateRecipe: ValidateAllNodesUpdateRecipe::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstaller as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstaller_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Scan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Install: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Rollback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShouldUsePluginSpecificReboot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RebootNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AcquireUpdateListValidationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ValidateAllNodesUpdateRecipe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateInstallerPlugin, IUpdateInstallerPlugin_Vtbl, 0x61122faa_0cd8_5a98_8938_5c1f88c632c6);
impl windows_core::RuntimeType for IUpdateInstallerPlugin {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateInstallerPlugin");
}
windows_core::imp::interface_hierarchy!(IUpdateInstallerPlugin, windows_core::IUnknown, windows_core::IInspectable);
impl IUpdateInstallerPlugin {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Version)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IsUpdateRollbackSupported(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUpdateRollbackSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateUpdateInstaller<P2, P3>(&self, clustername: &windows_core::HSTRING, runid: &windows_core::HSTRING, options: P2, updateservices: P3) -> windows_core::Result<IUpdateInstaller>
    where
        P2: windows_core::Param<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>,
        P3: windows_core::Param<ClusterUpdateServices>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateUpdateInstaller)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(clustername), core::mem::transmute_copy(runid), options.param().abi(), updateservices.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IUpdateInstallerPlugin {
    const NAME: &'static str = "Windows.Management.Update.Cluster.IUpdateInstallerPlugin";
}
pub trait IUpdateInstallerPlugin_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Description(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Version(&self) -> windows_core::Result<UpdatePluginVersionInfo>;
    fn IsUpdateRollbackSupported(&self) -> windows_core::Result<bool>;
    fn DefaultOptions(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::HSTRING>>;
    fn CreateUpdateInstaller(&self, clusterName: &windows_core::HSTRING, runId: &windows_core::HSTRING, options: windows_core::Ref<windows_collections::IMap<windows_core::HSTRING, windows_core::HSTRING>>, updateServices: windows_core::Ref<ClusterUpdateServices>) -> windows_core::Result<IUpdateInstaller>;
}
impl IUpdateInstallerPlugin_Vtbl {
    pub const fn new<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::FriendlyName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::Description(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Version<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::Version(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUpdateRollbackSupported<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::IsUpdateRollbackSupported(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultOptions<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::DefaultOptions(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateUpdateInstaller<Identity: IUpdateInstallerPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clustername: *mut core::ffi::c_void, runid: *mut core::ffi::c_void, options: *mut core::ffi::c_void, updateservices: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUpdateInstallerPlugin_Impl::CreateUpdateInstaller(this, core::mem::transmute(&clustername), core::mem::transmute(&runid), core::mem::transmute_copy(&options), core::mem::transmute_copy(&updateservices)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUpdateInstallerPlugin, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            IsUpdateRollbackSupported: IsUpdateRollbackSupported::<Identity, OFFSET>,
            DefaultOptions: DefaultOptions::<Identity, OFFSET>,
            CreateUpdateInstaller: CreateUpdateInstaller::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUpdateInstallerPlugin as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateInstallerPlugin_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Version: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsUpdateRollbackSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUpdateInstaller: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateOperationResult, IUpdateOperationResult_Vtbl, 0x9fc35275_3eb7_5785_bb32_0e37bb52cf41);
impl windows_core::RuntimeType for IUpdateOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateOperationResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StatusCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateOperationStatus) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateOperationResultFactory, IUpdateOperationResultFactory_Vtbl, 0xff53c580_e92c_50a9_8811_a057d5a2a523);
impl windows_core::RuntimeType for IUpdateOperationResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateOperationResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateOperationResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, ClusterNativeUpdateOperationStatus, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, ClusterNativeUpdateOperationStatus, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdatePendingOperationResult, IUpdatePendingOperationResult_Vtbl, 0x0f83b557_e96a_5f19_9492_99fc8bd83c5d);
impl windows_core::RuntimeType for IUpdatePendingOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdatePendingOperationResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdatePendingOperationResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SuggestedRecheckInterval: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdatePendingOperationResultFactory, IUpdatePendingOperationResultFactory_Vtbl, 0x45a3b731_d513_5baf_bb44_68b97a938308);
impl windows_core::RuntimeType for IUpdatePendingOperationResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdatePendingOperationResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdatePendingOperationResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_time::TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdatePluginVersionInfo, IUpdatePluginVersionInfo_Vtbl, 0x0901af2e_b340_5e7e_9898_516fc96add89);
impl windows_core::RuntimeType for IUpdatePluginVersionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdatePluginVersionInfo");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdatePluginVersionInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MajorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MinorVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub PatchLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdatePluginVersionInfoFactory, IUpdatePluginVersionInfoFactory_Vtbl, 0x56bb98ce_543c_59f2_810d_4d2cf84ee40e);
impl windows_core::RuntimeType for IUpdatePluginVersionInfoFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdatePluginVersionInfoFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdatePluginVersionInfoFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateRecipeNodeValidationInfoRecord, IUpdateRecipeNodeValidationInfoRecord_Vtbl, 0x55dfff15_5556_51c4_80c2_af03f385d429);
impl windows_core::RuntimeType for IUpdateRecipeNodeValidationInfoRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateRecipeNodeValidationInfoRecord");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateRecipeNodeValidationInfoRecord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NodeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecipeValidationInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScanResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateRecipeNodeValidationInfoRecordFactory, IUpdateRecipeNodeValidationInfoRecordFactory_Vtbl, 0xe0fa2776_dd6c_5139_a93e_23e2188921af);
impl windows_core::RuntimeType for IUpdateRecipeNodeValidationInfoRecordFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateRecipeNodeValidationInfoRecordFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateRecipeNodeValidationInfoRecordFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateScanRecord, IUpdateScanRecord_Vtbl, 0xc54af4fb_d316_5480_954f_d1eb51961de5);
impl windows_core::RuntimeType for IUpdateScanRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateScanRecord");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateScanRecord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRebootRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PluginSpecificData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateScanRecordFactory, IUpdateScanRecordFactory_Vtbl, 0xd9a3860e_a05a_58c3_b368_07bb350072d0);
impl windows_core::RuntimeType for IUpdateScanRecordFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateScanRecordFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateScanRecordFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateStageRecord, IUpdateStageRecord_Vtbl, 0x60d8edf9_eb18_59d1_badb_5d862f9352e9);
impl windows_core::RuntimeType for IUpdateStageRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateStageRecord");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateStageRecord_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UpdateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateCompletionStatus) -> windows_core::HRESULT,
    pub FailureMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdatedPluginSpecificData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateStageRecordFactory, IUpdateStageRecordFactory_Vtbl, 0xb5c12a84_ebf5_505b_872f_4de71fdda7e8);
impl windows_core::RuntimeType for IUpdateStageRecordFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateStageRecordFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateStageRecordFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ClusterNativeUpdateCompletionStatus, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ClusterNativeUpdateCompletionStatus, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateTaskResult, IUpdateTaskResult_Vtbl, 0x3fdd9274_7686_5cb9_89e4_6d618ed3a47a);
impl windows_core::RuntimeType for IUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateTaskResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateTaskResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateTaskResultFactory, IUpdateTaskResultFactory_Vtbl, 0xc5ae4ce3_7d1e_50bf_a64a_95f34783fc8c);
impl windows_core::RuntimeType for IUpdateTaskResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateTaskResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateTaskResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstance2: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateValidationLogMessage, IUpdateValidationLogMessage_Vtbl, 0x1e55bbd5_648f_584c_904d_ae6a053e80c8);
impl windows_core::RuntimeType for IUpdateValidationLogMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateValidationLogMessage");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateValidationLogMessage_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Severity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateLogLevel) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUpdateValidationLogMessageFactory, IUpdateValidationLogMessageFactory_Vtbl, 0x9005a873_4c0b_5c78_a433_be5095b0226e);
impl windows_core::RuntimeType for IUpdateValidationLogMessageFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IUpdateValidationLogMessageFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUpdateValidationLogMessageFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, ClusterNativeUpdateLogLevel, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValidateAllNodesUpdateRecipeResult, IValidateAllNodesUpdateRecipeResult_Vtbl, 0x4a6adcd4_b621_5ca6_b82e_6ce96d2a3b93);
impl windows_core::RuntimeType for IValidateAllNodesUpdateRecipeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IValidateAllNodesUpdateRecipeResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidateAllNodesUpdateRecipeResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AreUpdatesApproved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ValidationMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValidateAllNodesUpdateRecipeResultFactory, IValidateAllNodesUpdateRecipeResultFactory_Vtbl, 0x00a73e76_0643_5793_ba1c_6e7aa89fdba6);
impl windows_core::RuntimeType for IValidateAllNodesUpdateRecipeResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IValidateAllNodesUpdateRecipeResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidateAllNodesUpdateRecipeResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValidateClusterEnvironmentResult, IValidateClusterEnvironmentResult_Vtbl, 0x4df22928_9fde_5c16_961c_aab26f4a7780);
impl windows_core::RuntimeType for IValidateClusterEnvironmentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IValidateClusterEnvironmentResult");
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidateClusterEnvironmentResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ApprovalStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ClusterNativeUpdateEnvironmentValidationStatus) -> windows_core::HRESULT,
    pub ValidationMessages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IValidateClusterEnvironmentResultFactory, IValidateClusterEnvironmentResultFactory_Vtbl, 0x9222e2a5_7106_5dc4_860e_6634e47299fc);
impl windows_core::RuntimeType for IValidateClusterEnvironmentResultFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Management.Update.Cluster.IValidateClusterEnvironmentResultFactory");
}
#[repr(C)]
#[doc(hidden)]
pub struct IValidateClusterEnvironmentResultFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, ClusterNativeUpdateEnvironmentValidationStatus, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallUpdateTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InstallUpdateTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl InstallUpdateTaskResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstallRecords(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateInstallRecord>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ShouldRollback(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldRollback)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance<P0, P1>(result: P0, installrecords: P1, shouldrollback: bool) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P1: windows_core::Param<windows_collections::IVectorView<UpdateInstallRecord>>,
    {
        Self::IInstallUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), installrecords.param().abi(), shouldrollback, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IInstallUpdateTaskResultFactory<R, F: FnOnce(&IInstallUpdateTaskResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InstallUpdateTaskResult, IInstallUpdateTaskResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InstallUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInstallUpdateTaskResult>();
}
unsafe impl windows_core::Interface for InstallUpdateTaskResult {
    type Vtable = <IInstallUpdateTaskResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInstallUpdateTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InstallUpdateTaskResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.InstallUpdateTaskResult";
}
unsafe impl Send for InstallUpdateTaskResult {}
unsafe impl Sync for InstallUpdateTaskResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RollbackUpdateTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RollbackUpdateTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl RollbackUpdateTaskResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InstallRecords(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateInstallRecord>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InstallRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1>(result: P0, installrecords: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P1: windows_core::Param<windows_collections::IVectorView<UpdateInstallRecord>>,
    {
        Self::IRollbackUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), installrecords.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IRollbackUpdateTaskResultFactory<R, F: FnOnce(&IRollbackUpdateTaskResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RollbackUpdateTaskResult, IRollbackUpdateTaskResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RollbackUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRollbackUpdateTaskResult>();
}
unsafe impl windows_core::Interface for RollbackUpdateTaskResult {
    type Vtable = <IRollbackUpdateTaskResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRollbackUpdateTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RollbackUpdateTaskResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.RollbackUpdateTaskResult";
}
unsafe impl Send for RollbackUpdateTaskResult {}
unsafe impl Sync for RollbackUpdateTaskResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanUpdateTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScanUpdateTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl ScanUpdateTaskResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ScanRecords(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateScanRecord>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1>(result: P0, scanrecords: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P1: windows_core::Param<windows_collections::IVectorView<UpdateScanRecord>>,
    {
        Self::IScanUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), scanrecords.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IScanUpdateTaskResultFactory<R, F: FnOnce(&IScanUpdateTaskResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScanUpdateTaskResult, IScanUpdateTaskResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ScanUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScanUpdateTaskResult>();
}
unsafe impl windows_core::Interface for ScanUpdateTaskResult {
    type Vtable = <IScanUpdateTaskResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScanUpdateTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScanUpdateTaskResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.ScanUpdateTaskResult";
}
unsafe impl Send for ScanUpdateTaskResult {}
unsafe impl Sync for ScanUpdateTaskResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageUpdateTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StageUpdateTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl StageUpdateTaskResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn StageRecords(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateStageRecord>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StageRecords)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P1>(result: P0, stagerecords: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P1: windows_core::Param<windows_collections::IVectorView<UpdateStageRecord>>,
    {
        Self::IStageUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), stagerecords.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IStageUpdateTaskResultFactory<R, F: FnOnce(&IStageUpdateTaskResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StageUpdateTaskResult, IStageUpdateTaskResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StageUpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStageUpdateTaskResult>();
}
unsafe impl windows_core::Interface for StageUpdateTaskResult {
    type Vtable = <IStageUpdateTaskResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStageUpdateTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StageUpdateTaskResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.StageUpdateTaskResult";
}
unsafe impl Send for StageUpdateTaskResult {}
unsafe impl Sync for StageUpdateTaskResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateCredential(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateCredential, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateCredential {
    pub fn Status(&self) -> windows_core::Result<ClusterNativeUpdateCredentialStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(status: ClusterNativeUpdateCredentialStatus, username: &windows_core::HSTRING, password: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateCredentialFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), status, core::mem::transmute_copy(username), core::mem::transmute_copy(password), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateCredentialFactory<R, F: FnOnce(&IUpdateCredentialFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateCredential, IUpdateCredentialFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateCredential {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateCredential>();
}
unsafe impl windows_core::Interface for UpdateCredential {
    type Vtable = <IUpdateCredential as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateCredential as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateCredential {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateCredential";
}
unsafe impl Send for UpdateCredential {}
unsafe impl Sync for UpdateCredential {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateInstallRecord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateInstallRecord, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateInstallRecord {
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsRebootRequired(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<ClusterNativeUpdateCompletionStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FailureMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FailureMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(updateid: &windows_core::HSTRING, isrebootrequired: bool, status: ClusterNativeUpdateCompletionStatus, failuremessage: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateInstallRecordFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), isrebootrequired, status, core::mem::transmute_copy(failuremessage), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateInstallRecordFactory<R, F: FnOnce(&IUpdateInstallRecordFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateInstallRecord, IUpdateInstallRecordFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateInstallRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateInstallRecord>();
}
unsafe impl windows_core::Interface for UpdateInstallRecord {
    type Vtable = <IUpdateInstallRecord as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateInstallRecord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateInstallRecord {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateInstallRecord";
}
unsafe impl Send for UpdateInstallRecord {}
unsafe impl Sync for UpdateInstallRecord {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateOperationResult {
    pub fn StatusCode(&self) -> windows_core::Result<ClusterNativeUpdateOperationStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StatusCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Reason(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(status: ClusterNativeUpdateOperationStatus, reason: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateOperationResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), status, core::mem::transmute_copy(reason), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(status: ClusterNativeUpdateOperationStatus, reason: &windows_core::HSTRING, errorcode: i32) -> windows_core::Result<Self> {
        Self::IUpdateOperationResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), status, core::mem::transmute_copy(reason), errorcode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateOperationResultFactory<R, F: FnOnce(&IUpdateOperationResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateOperationResult, IUpdateOperationResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateOperationResult>();
}
unsafe impl windows_core::Interface for UpdateOperationResult {
    type Vtable = <IUpdateOperationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateOperationResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateOperationResult";
}
unsafe impl Send for UpdateOperationResult {}
unsafe impl Sync for UpdateOperationResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePendingOperationResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdatePendingOperationResult, windows_core::IUnknown, windows_core::IInspectable);
impl UpdatePendingOperationResult {
    pub fn Result(&self) -> windows_core::Result<UpdateOperationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SuggestedRecheckInterval(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SuggestedRecheckInterval)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance<P0>(result: P0, suggestedrecheckinterval: windows_time::TimeSpan) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateOperationResult>,
    {
        Self::IUpdatePendingOperationResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), suggestedrecheckinterval, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdatePendingOperationResultFactory<R, F: FnOnce(&IUpdatePendingOperationResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdatePendingOperationResult, IUpdatePendingOperationResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdatePendingOperationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdatePendingOperationResult>();
}
unsafe impl windows_core::Interface for UpdatePendingOperationResult {
    type Vtable = <IUpdatePendingOperationResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdatePendingOperationResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdatePendingOperationResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdatePendingOperationResult";
}
unsafe impl Send for UpdatePendingOperationResult {}
unsafe impl Sync for UpdatePendingOperationResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdatePluginVersionInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdatePluginVersionInfo, windows_core::IUnknown, windows_core::IInspectable);
impl UpdatePluginVersionInfo {
    pub fn MajorVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MajorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MinorVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinorVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PatchLevel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PatchLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(majorversion: u32, minorversion: u32, patchlevel: u32) -> windows_core::Result<Self> {
        Self::IUpdatePluginVersionInfoFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), majorversion, minorversion, patchlevel, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdatePluginVersionInfoFactory<R, F: FnOnce(&IUpdatePluginVersionInfoFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdatePluginVersionInfo, IUpdatePluginVersionInfoFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdatePluginVersionInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdatePluginVersionInfo>();
}
unsafe impl windows_core::Interface for UpdatePluginVersionInfo {
    type Vtable = <IUpdatePluginVersionInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdatePluginVersionInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdatePluginVersionInfo {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdatePluginVersionInfo";
}
unsafe impl Send for UpdatePluginVersionInfo {}
unsafe impl Sync for UpdatePluginVersionInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateRecipeNodeValidationInfoRecord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateRecipeNodeValidationInfoRecord, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateRecipeNodeValidationInfoRecord {
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NodeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RecipeValidationInfo(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecipeValidationInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ScanResults(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateScanRecord>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScanResults)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P2>(nodename: &windows_core::HSTRING, recipevalidationinfo: &windows_core::HSTRING, scanresults: P2) -> windows_core::Result<Self>
    where
        P2: windows_core::Param<windows_collections::IVectorView<UpdateScanRecord>>,
    {
        Self::IUpdateRecipeNodeValidationInfoRecordFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(nodename), core::mem::transmute_copy(recipevalidationinfo), scanresults.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateRecipeNodeValidationInfoRecordFactory<R, F: FnOnce(&IUpdateRecipeNodeValidationInfoRecordFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateRecipeNodeValidationInfoRecord, IUpdateRecipeNodeValidationInfoRecordFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateRecipeNodeValidationInfoRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateRecipeNodeValidationInfoRecord>();
}
unsafe impl windows_core::Interface for UpdateRecipeNodeValidationInfoRecord {
    type Vtable = <IUpdateRecipeNodeValidationInfoRecord as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateRecipeNodeValidationInfoRecord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateRecipeNodeValidationInfoRecord {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateRecipeNodeValidationInfoRecord";
}
unsafe impl Send for UpdateRecipeNodeValidationInfoRecord {}
unsafe impl Sync for UpdateRecipeNodeValidationInfoRecord {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateScanRecord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateScanRecord, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateScanRecord {
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateTitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateTitle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdateDescription(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsRebootRequired(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRebootRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PluginSpecificData(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PluginSpecificData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(updateid: &windows_core::HSTRING, updatetitle: &windows_core::HSTRING, updatedescription: &windows_core::HSTRING, isrebootrequired: bool, pluginspecificdata: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateScanRecordFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), core::mem::transmute_copy(updatetitle), core::mem::transmute_copy(updatedescription), isrebootrequired, core::mem::transmute_copy(pluginspecificdata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateScanRecordFactory<R, F: FnOnce(&IUpdateScanRecordFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateScanRecord, IUpdateScanRecordFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateScanRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateScanRecord>();
}
unsafe impl windows_core::Interface for UpdateScanRecord {
    type Vtable = <IUpdateScanRecord as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateScanRecord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateScanRecord {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateScanRecord";
}
unsafe impl Send for UpdateScanRecord {}
unsafe impl Sync for UpdateScanRecord {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateStageRecord(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateStageRecord, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateStageRecord {
    pub fn UpdateId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Status(&self) -> windows_core::Result<ClusterNativeUpdateCompletionStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Status)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FailureMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FailureMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UpdatedPluginSpecificData(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UpdatedPluginSpecificData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(updateid: &windows_core::HSTRING, status: ClusterNativeUpdateCompletionStatus, failuremessage: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateStageRecordFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), status, core::mem::transmute_copy(failuremessage), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(updateid: &windows_core::HSTRING, status: ClusterNativeUpdateCompletionStatus, failuremessage: &windows_core::HSTRING, updatedpluginspecificdata: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateStageRecordFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(updateid), status, core::mem::transmute_copy(failuremessage), core::mem::transmute_copy(updatedpluginspecificdata), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateStageRecordFactory<R, F: FnOnce(&IUpdateStageRecordFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateStageRecord, IUpdateStageRecordFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateStageRecord {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateStageRecord>();
}
unsafe impl windows_core::Interface for UpdateStageRecord {
    type Vtable = <IUpdateStageRecord as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateStageRecord as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateStageRecord {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateStageRecord";
}
unsafe impl Send for UpdateStageRecord {}
unsafe impl Sync for UpdateStageRecord {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateTaskResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateTaskResult, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateTaskResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Succeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Reason(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CreateInstance(succeeded: bool, reason: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), succeeded, core::mem::transmute_copy(reason), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInstance2(succeeded: bool, reason: &windows_core::HSTRING, errorcode: i32) -> windows_core::Result<Self> {
        Self::IUpdateTaskResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance2)(windows_core::Interface::as_raw(this), succeeded, core::mem::transmute_copy(reason), errorcode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateTaskResultFactory<R, F: FnOnce(&IUpdateTaskResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateTaskResult, IUpdateTaskResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateTaskResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateTaskResult>();
}
unsafe impl windows_core::Interface for UpdateTaskResult {
    type Vtable = <IUpdateTaskResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateTaskResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateTaskResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateTaskResult";
}
unsafe impl Send for UpdateTaskResult {}
unsafe impl Sync for UpdateTaskResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpdateValidationLogMessage(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UpdateValidationLogMessage, windows_core::IUnknown, windows_core::IInspectable);
impl UpdateValidationLogMessage {
    pub fn Severity(&self) -> windows_core::Result<ClusterNativeUpdateLogLevel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Severity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstance(severity: ClusterNativeUpdateLogLevel, message: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUpdateValidationLogMessageFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), severity, core::mem::transmute_copy(message), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUpdateValidationLogMessageFactory<R, F: FnOnce(&IUpdateValidationLogMessageFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UpdateValidationLogMessage, IUpdateValidationLogMessageFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UpdateValidationLogMessage {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUpdateValidationLogMessage>();
}
unsafe impl windows_core::Interface for UpdateValidationLogMessage {
    type Vtable = <IUpdateValidationLogMessage as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUpdateValidationLogMessage as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UpdateValidationLogMessage {
    const NAME: &'static str = "Windows.Management.Update.Cluster.UpdateValidationLogMessage";
}
unsafe impl Send for UpdateValidationLogMessage {}
unsafe impl Sync for UpdateValidationLogMessage {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidateAllNodesUpdateRecipeResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValidateAllNodesUpdateRecipeResult, windows_core::IUnknown, windows_core::IInspectable);
impl ValidateAllNodesUpdateRecipeResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AreUpdatesApproved(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreUpdatesApproved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ValidationMessages(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateValidationLogMessage>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidationMessages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P2>(result: P0, areupdatesapproved: bool, validationmessages: P2) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P2: windows_core::Param<windows_collections::IVectorView<UpdateValidationLogMessage>>,
    {
        Self::IValidateAllNodesUpdateRecipeResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), areupdatesapproved, validationmessages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IValidateAllNodesUpdateRecipeResultFactory<R, F: FnOnce(&IValidateAllNodesUpdateRecipeResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValidateAllNodesUpdateRecipeResult, IValidateAllNodesUpdateRecipeResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ValidateAllNodesUpdateRecipeResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IValidateAllNodesUpdateRecipeResult>();
}
unsafe impl windows_core::Interface for ValidateAllNodesUpdateRecipeResult {
    type Vtable = <IValidateAllNodesUpdateRecipeResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IValidateAllNodesUpdateRecipeResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ValidateAllNodesUpdateRecipeResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.ValidateAllNodesUpdateRecipeResult";
}
unsafe impl Send for ValidateAllNodesUpdateRecipeResult {}
unsafe impl Sync for ValidateAllNodesUpdateRecipeResult {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidateClusterEnvironmentResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ValidateClusterEnvironmentResult, windows_core::IUnknown, windows_core::IInspectable);
impl ValidateClusterEnvironmentResult {
    pub fn Result(&self) -> windows_core::Result<UpdateTaskResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ApprovalStatus(&self) -> windows_core::Result<ClusterNativeUpdateEnvironmentValidationStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ApprovalStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ValidationMessages(&self) -> windows_core::Result<windows_collections::IVectorView<UpdateValidationLogMessage>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidationMessages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance<P0, P2>(result: P0, approvalstatus: ClusterNativeUpdateEnvironmentValidationStatus, validationmessages: P2) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<UpdateTaskResult>,
        P2: windows_core::Param<windows_collections::IVectorView<UpdateValidationLogMessage>>,
    {
        Self::IValidateClusterEnvironmentResultFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), result.param().abi(), approvalstatus, validationmessages.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IValidateClusterEnvironmentResultFactory<R, F: FnOnce(&IValidateClusterEnvironmentResultFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ValidateClusterEnvironmentResult, IValidateClusterEnvironmentResultFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ValidateClusterEnvironmentResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IValidateClusterEnvironmentResult>();
}
unsafe impl windows_core::Interface for ValidateClusterEnvironmentResult {
    type Vtable = <IValidateClusterEnvironmentResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IValidateClusterEnvironmentResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ValidateClusterEnvironmentResult {
    const NAME: &'static str = "Windows.Management.Update.Cluster.ValidateClusterEnvironmentResult";
}
unsafe impl Send for ValidateClusterEnvironmentResult {}
unsafe impl Sync for ValidateClusterEnvironmentResult {}
