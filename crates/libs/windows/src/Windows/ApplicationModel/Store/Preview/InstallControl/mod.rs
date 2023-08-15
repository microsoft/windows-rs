#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
}
impl ::core::clone::Clone for IAppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49d3dfab_168a_4cbf_a93a_9e448c82737d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub InstallType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows_core::HRESULT,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub GetCurrentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub StatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem2 {
    type Vtable = IAppInstallItem2_Vtbl;
}
impl ::core::clone::Clone for IAppInstallItem2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallItem2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3972af8_40c0_4fd7_aa6c_0aa13ca6188c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem3 {
    type Vtable = IAppInstallItem3_Vtbl;
}
impl ::core::clone::Clone for IAppInstallItem3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallItem3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6f3dc998_dd47_433c_9234_560172d67a45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub ItemOperationsMightAffectOtherItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem4 {
    type Vtable = IAppInstallItem4_Vtbl;
}
impl ::core::clone::Clone for IAppInstallItem4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallItem4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2d1ce12_71ff_4fc8_b540_453d4b37e1d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallItem5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallItem5 {
    type Vtable = IAppInstallItem5_Vtbl;
}
impl ::core::clone::Clone for IAppInstallItem5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallItem5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5510e7cc_4076_4a0b_9472_c21d9d380e55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallItem5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9353e170_8441_4b45_bd72_7c2fa925beee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItems: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Restart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub ItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemStatusChanged: usize,
    pub AutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows_core::HRESULT,
    pub SetAutoUpdateSetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows_core::HRESULT,
    pub AcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAcquisitionIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetIsApplicableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsApplicableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsStoreBlockedByPolicyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeclientname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, storeclientpublisher: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsStoreBlockedByPolicyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager2 {
    type Vtable = IAppInstallManager2_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16937851_ed37_480d_8314_52e27c03f04a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAppInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, bundleid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAppInstallWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAppByPackageFamilyNameWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAppByPackageFamilyNameWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithTelemetryAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsAppAllowedToInstallWithTelemetryAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsAppAllowedToInstallWithTelemetryAsync: usize,
    pub CancelWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub PauseWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RestartWithTelemetry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager3 {
    type Vtable = IAppInstallManager3_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95b24b17_e96a_4d0e_84e1_c8cb417a0178);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub StartProductInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, flightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment")))]
    StartProductInstallAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub StartProductInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, flightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetvolume: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System")))]
    StartProductInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub UpdateAppByPackageFamilyNameForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    UpdateAppByPackageFamilyNameForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsAppAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, catalogid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsAppAllowedToInstallForUserAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsApplicableForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsApplicableForUserAsync: usize,
    pub MoveToFrontOfDownloadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager4 {
    type Vtable = IAppInstallManager4_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x260a2a16_5a9e_4ebd_b944_f2ba75c31159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetFreeUserEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, campaignid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeUserEntitlementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetFreeUserEntitlementForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, storeid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, campaignid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetFreeUserEntitlementForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFreeDeviceEntitlementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, campaignid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFreeDeviceEntitlementAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager5 {
    type Vtable = IAppInstallManager5_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3cd7be4c_1be9_4f7f_b675_aa1d64a529b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AppInstallItemsWithGroupSupport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AppInstallItemsWithGroupSupport: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager6 {
    type Vtable = IAppInstallManager6_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9e7d408_f27a_4471_b2f4_e76efcbebcca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SearchForAllUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SearchForAllUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub SearchForAllUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    SearchForAllUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SearchForUpdatesWithUpdateOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SearchForUpdatesWithUpdateOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub SearchForUpdatesWithUpdateOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, skuid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, updateoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    SearchForUpdatesWithUpdateOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StartProductInstallWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, flightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartProductInstallWithOptionsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub StartProductInstallWithOptionsForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, productid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, flightid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, clientid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, installoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "System")))]
    StartProductInstallWithOptionsForUserAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetIsPackageIdentityAllowedToInstallAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, packageidentityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, publishercertificatename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIsPackageIdentityAllowedToInstallAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub GetIsPackageIdentityAllowedToInstallForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, correlationvector: ::std::mem::MaybeUninit<::windows_core::HSTRING>, packageidentityname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, publishercertificatename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "System")))]
    GetIsPackageIdentityAllowedToInstallForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManager7(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManager7 {
    type Vtable = IAppInstallManager7_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManager7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManager7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5ee7b30_d5e4_49a3_9853_3db03203321d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManager7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallManagerItemEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallManagerItemEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc505743_4674_4dd1_957e_c25682086a14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallManagerItemEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
}
impl ::core::clone::Clone for IAppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9808300_1cb8_4eb6_8c9f_6a30c64a5b51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetForceUseOfNonRemovableStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetRepair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Management_Deployment")]
    pub TargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    TargetVolume: usize,
    #[cfg(feature = "Management_Deployment")]
    pub SetTargetVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Management_Deployment"))]
    SetTargetVolume: usize,
    pub LaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetLaunchAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallOptions2 {
    type Vtable = IAppInstallOptions2_Vtbl;
}
impl ::core::clone::Clone for IAppInstallOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a04c0d7_c94b_425e_95b4_bf27faeaee89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToDesktopAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToStartAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub PinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetPinToTaskbarAfterInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetCompletedInstallToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub SetInstallInProgressToastNotificationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows_core::HRESULT,
    pub InstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetInstallForAllUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub StageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetStageButDoNotInstall: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetExtendedCampaignId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
}
impl ::core::clone::Clone for IAppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallStatus {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x936dccfa_2450_4126_88b1_6127a644dd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstallState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows_core::HRESULT,
    pub DownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesDownloaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub PercentComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus2 {
    type Vtable = IAppInstallStatus2_Vtbl;
}
impl ::core::clone::Clone for IAppInstallStatus2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallStatus2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x96e7818a_5e92_4aa9_8edc_58fed4b87e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
    pub ReadyForLaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppInstallStatus3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppInstallStatus3 {
    type Vtable = IAppInstallStatus3_Vtbl;
}
impl ::core::clone::Clone for IAppInstallStatus3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppInstallStatus3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb880c56_837b_4b4c_9ebb_6d44a0a96307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppInstallStatus3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsStaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
}
impl ::core::clone::Clone for IAppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppUpdateOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x26f0b02f_c2f3_4aea_af8c_6308dd9db85f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetCatalogId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowForcedAppRestart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppUpdateOptions2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppUpdateOptions2 {
    type Vtable = IAppUpdateOptions2_Vtbl;
}
impl ::core::clone::Clone for IAppUpdateOptions2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAppUpdateOptions2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4646e08_ed26_4bf9_9679_48f628e53df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppUpdateOptions2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAutomaticallyDownloadAndInstallUpdateIfFound: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetEntitlementResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
}
impl ::core::clone::Clone for IGetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGetEntitlementResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74fc843f_1a9e_4609_8e4d_819086d08a3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetEntitlementResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallItem(::windows_core::IUnknown);
impl AppInstallItem {
    pub fn ProductId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProductId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PackageFamilyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PackageFamilyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstallType(&self) -> ::windows_core::Result<AppInstallType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsUserInitiated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserInitiated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCurrentStatus(&self) -> ::windows_core::Result<AppInstallStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetCurrentStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Restart(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Restart)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CancelWithTelemetry(&self, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CancelWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PauseWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RestartWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Children)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ItemOperationsMightAffectOtherItems(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemOperationsMightAffectOtherItems)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLaunchAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToStartAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallItem5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AppInstallItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallItem {}
impl ::core::fmt::Debug for AppInstallItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallItem").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem;{49d3dfab-168a-4cbf-a93a-9e448c82737d})");
}
impl ::core::clone::Clone for AppInstallItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppInstallItem {
    type Vtable = IAppInstallItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstallItem {
    const IID: ::windows_core::GUID = <IAppInstallItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallItem";
}
::windows_core::imp::interface_hierarchy!(AppInstallItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallItem {}
unsafe impl ::core::marker::Sync for AppInstallItem {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManager(::windows_core::IUnknown);
impl AppInstallManager {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppInstallManager, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItems(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallItems)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Cancel(&self, productid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Cancel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Pause(&self, productid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    pub fn Restart(&self, productid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Restart)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemCompleted(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemStatusChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemStatusChanged(&self, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemStatusChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn AutoUpdateSetting(&self) -> ::windows_core::Result<AutoUpdateSetting> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutoUpdateSetting)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAutoUpdateSetting)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AcquisitionIdentity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AcquisitionIdentity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAcquisitionIdentity(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAcquisitionIdentity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsApplicableAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsApplicableAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAppInstallAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameAsync(&self, packagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesAsync(&self) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsStoreBlockedByPolicyAsync(&self, storeclientname: &::windows_core::HSTRING, storeclientpublisher: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStoreBlockedByPolicyAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(storeclientname), ::core::mem::transmute_copy(storeclientpublisher), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallAsync(&self, productid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAppInstallWithTelemetryAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &::windows_core::HSTRING, bundleid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAppInstallWithTelemetryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(bundleid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&self, packagefamilyname: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameWithTelemetryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithTelemetryAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithTelemetryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithTelemetryAsync(&self, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithTelemetryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsAppAllowedToInstallWithTelemetryAsync(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallWithTelemetryAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    pub fn CancelWithTelemetry(&self, productid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).CancelWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn PauseWithTelemetry(&self, productid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).PauseWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    pub fn RestartWithTelemetry(&self, productid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RestartWithTelemetry)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment"))]
    pub fn StartProductInstallAsync<P0>(&self, productid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, flightid: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows_core::HSTRING, targetvolume: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(correlationvector), targetvolume.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Management_Deployment\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System"))]
    pub fn StartProductInstallForUserAsync<P0, P1>(&self, user: P0, productid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, flightid: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows_core::HSTRING, targetvolume: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::IntoParam<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), repair, forceuseofnonremovablestorage, ::core::mem::transmute_copy(correlationvector), targetvolume.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn UpdateAppByPackageFamilyNameForUserAsync<P0>(&self, user: P0, packagefamilyname: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateAppByPackageFamilyNameForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(packagefamilyname), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesForUserAsync<P0>(&self, user: P0, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesForUserAsync<P0>(&self, user: P0, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsAppAllowedToInstallForUserAsync<P0>(&self, user: P0, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, catalogid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsAppAllowedToInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(catalogid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsApplicableForUserAsync<P0>(&self, user: P0, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsApplicableForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), &mut result__).from_abi(result__)
        }
    }
    pub fn MoveToFrontOfDownloadQueue(&self, productid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).MoveToFrontOfDownloadQueue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(correlationvector)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeUserEntitlementAsync(&self, storeid: &::windows_core::HSTRING, campaignid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeUserEntitlementAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetFreeUserEntitlementForUserAsync<P0>(&self, user: P0, storeid: &::windows_core::HSTRING, campaignid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeUserEntitlementForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFreeDeviceEntitlementAsync(&self, storeid: &::windows_core::HSTRING, campaignid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFreeDeviceEntitlementAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(storeid), ::core::mem::transmute_copy(campaignid), ::core::mem::transmute_copy(correlationvector), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AppInstallItemsWithGroupSupport(&self) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppInstallItemsWithGroupSupport)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SearchForAllUpdatesWithUpdateOptionsAsync<P0>(&self, correlationvector: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, updateoptions: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<AppUpdateOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), updateoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<P0, P1>(&self, user: P0, correlationvector: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, updateoptions: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::IntoParam<AppUpdateOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForAllUpdatesWithUpdateOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), updateoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SearchForUpdatesWithUpdateOptionsAsync<P0>(&self, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, updateoptions: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: ::windows_core::IntoParam<AppUpdateOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), updateoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn SearchForUpdatesWithUpdateOptionsForUserAsync<P0, P1>(&self, user: P0, productid: &::windows_core::HSTRING, skuid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, updateoptions: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::IntoParam<AppUpdateOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SearchForUpdatesWithUpdateOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(skuid), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(clientid), updateoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartProductInstallWithOptionsAsync<P0>(&self, productid: &::windows_core::HSTRING, flightid: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING, installoptions: P0) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<AppInstallOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(correlationvector), installoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "System"))]
    pub fn StartProductInstallWithOptionsForUserAsync<P0, P1>(&self, user: P0, productid: &::windows_core::HSTRING, flightid: &::windows_core::HSTRING, clientid: &::windows_core::HSTRING, correlationvector: &::windows_core::HSTRING, installoptions: P1) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
        P1: ::windows_core::IntoParam<AppInstallOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartProductInstallWithOptionsForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(productid), ::core::mem::transmute_copy(flightid), ::core::mem::transmute_copy(clientid), ::core::mem::transmute_copy(correlationvector), installoptions.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIsPackageIdentityAllowedToInstallAsync(&self, correlationvector: &::windows_core::HSTRING, packageidentityname: &::windows_core::HSTRING, publishercertificatename: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(packageidentityname), ::core::mem::transmute_copy(publishercertificatename), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "Foundation", feature = "System"))]
    pub fn GetIsPackageIdentityAllowedToInstallForUserAsync<P0>(&self, user: P0, correlationvector: &::windows_core::HSTRING, packageidentityname: &::windows_core::HSTRING, publishercertificatename: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::System::User>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetIsPackageIdentityAllowedToInstallForUserAsync)(::windows_core::Interface::as_raw(this), user.into_param().abi(), ::core::mem::transmute_copy(correlationvector), ::core::mem::transmute_copy(packageidentityname), ::core::mem::transmute_copy(publishercertificatename), &mut result__).from_abi(result__)
        }
    }
    pub fn CanInstallForAllUsers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallManager7>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanInstallForAllUsers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppInstallManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManager {}
impl ::core::fmt::Debug for AppInstallManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManager").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager;{9353e170-8441-4b45-bd72-7c2fa925beee})");
}
impl ::core::clone::Clone for AppInstallManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppInstallManager {
    type Vtable = IAppInstallManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstallManager {
    const IID: ::windows_core::GUID = <IAppInstallManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManager";
}
::windows_core::imp::interface_hierarchy!(AppInstallManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallManager {}
unsafe impl ::core::marker::Sync for AppInstallManager {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallManagerItemEventArgs(::windows_core::IUnknown);
impl AppInstallManagerItemEventArgs {
    pub fn Item(&self) -> ::windows_core::Result<AppInstallItem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Item)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppInstallManagerItemEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallManagerItemEventArgs {}
impl ::core::fmt::Debug for AppInstallManagerItemEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallManagerItemEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallManagerItemEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs;{bc505743-4674-4dd1-957e-c25682086a14})");
}
impl ::core::clone::Clone for AppInstallManagerItemEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppInstallManagerItemEventArgs {
    type Vtable = IAppInstallManagerItemEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstallManagerItemEventArgs {
    const IID: ::windows_core::GUID = <IAppInstallManagerItemEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallManagerItemEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppInstallManagerItemEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallManagerItemEventArgs {}
unsafe impl ::core::marker::Sync for AppInstallManagerItemEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallOptions(::windows_core::IUnknown);
impl AppInstallOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppInstallOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CatalogId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCatalogId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCatalogId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ForceUseOfNonRemovableStorage(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ForceUseOfNonRemovableStorage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForceUseOfNonRemovableStorage)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowForcedAppRestart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Repair(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Repair)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRepair(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepair)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn TargetVolume(&self) -> ::windows_core::Result<super::super::super::super::Management::Deployment::PackageVolume> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetVolume)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Management_Deployment\"`*"]
    #[cfg(feature = "Management_Deployment")]
    pub fn SetTargetVolume<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Management::Deployment::PackageVolume>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetVolume)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn LaunchAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLaunchAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLaunchAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToDesktopAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToDesktopAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToStartAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToStartAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToStartAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn PinToTaskbarAfterInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPinToTaskbarAfterInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletedInstallToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletedInstallToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallInProgressToastNotificationMode(&self) -> ::windows_core::Result<AppInstallationToastNotificationMode> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallInProgressToastNotificationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InstallForAllUsers(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallForAllUsers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInstallForAllUsers(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetInstallForAllUsers)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StageButDoNotInstall(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StageButDoNotInstall)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStageButDoNotInstall(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStageButDoNotInstall)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CampaignId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CampaignId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCampaignId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCampaignId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendedCampaignId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedCampaignId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExtendedCampaignId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetExtendedCampaignId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for AppInstallOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallOptions {}
impl ::core::fmt::Debug for AppInstallOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions;{c9808300-1cb8-4eb6-8c9f-6a30c64a5b51})");
}
impl ::core::clone::Clone for AppInstallOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppInstallOptions {
    type Vtable = IAppInstallOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstallOptions {
    const IID: ::windows_core::GUID = <IAppInstallOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallOptions";
}
::windows_core::imp::interface_hierarchy!(AppInstallOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallOptions {}
unsafe impl ::core::marker::Sync for AppInstallOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppInstallStatus(::windows_core::IUnknown);
impl AppInstallStatus {
    pub fn InstallState(&self) -> ::windows_core::Result<AppInstallState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DownloadSizeInBytes(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadSizeInBytes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BytesDownloaded(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BytesDownloaded)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PercentComplete(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PercentComplete)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadyForLaunch(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallStatus2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadyForLaunch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsStaged(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppInstallStatus3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsStaged)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppInstallStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallStatus {}
impl ::core::fmt::Debug for AppInstallStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus;{936dccfa-2450-4126-88b1-6127a644dd5c})");
}
impl ::core::clone::Clone for AppInstallStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppInstallStatus {
    type Vtable = IAppInstallStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppInstallStatus {
    const IID: ::windows_core::GUID = <IAppInstallStatus as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallStatus";
}
::windows_core::imp::interface_hierarchy!(AppInstallStatus, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppInstallStatus {}
unsafe impl ::core::marker::Sync for AppInstallStatus {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct AppUpdateOptions(::windows_core::IUnknown);
impl AppUpdateOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppUpdateOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CatalogId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CatalogId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCatalogId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCatalogId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowForcedAppRestart(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowForcedAppRestart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowForcedAppRestart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppUpdateOptions2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AutomaticallyDownloadAndInstallUpdateIfFound)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppUpdateOptions2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAutomaticallyDownloadAndInstallUpdateIfFound)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for AppUpdateOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppUpdateOptions {}
impl ::core::fmt::Debug for AppUpdateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppUpdateOptions").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppUpdateOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions;{26f0b02f-c2f3-4aea-af8c-6308dd9db85f})");
}
impl ::core::clone::Clone for AppUpdateOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AppUpdateOptions {
    type Vtable = IAppUpdateOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppUpdateOptions {
    const IID: ::windows_core::GUID = <IAppUpdateOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.AppUpdateOptions";
}
::windows_core::imp::interface_hierarchy!(AppUpdateOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppUpdateOptions {}
unsafe impl ::core::marker::Sync for AppUpdateOptions {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
pub struct GetEntitlementResult(::windows_core::IUnknown);
impl GetEntitlementResult {
    pub fn Status(&self) -> ::windows_core::Result<GetEntitlementStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GetEntitlementResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetEntitlementResult {}
impl ::core::fmt::Debug for GetEntitlementResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GetEntitlementResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult;{74fc843f-1a9e-4609-8e4d-819086d08a3d})");
}
impl ::core::clone::Clone for GetEntitlementResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for GetEntitlementResult {
    type Vtable = IGetEntitlementResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GetEntitlementResult {
    const IID: ::windows_core::GUID = <IGetEntitlementResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementResult";
}
::windows_core::imp::interface_hierarchy!(GetEntitlementResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GetEntitlementResult {}
unsafe impl ::core::marker::Sync for GetEntitlementResult {}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallState(pub i32);
impl AppInstallState {
    pub const Pending: Self = Self(0i32);
    pub const Starting: Self = Self(1i32);
    pub const AcquiringLicense: Self = Self(2i32);
    pub const Downloading: Self = Self(3i32);
    pub const RestoringData: Self = Self(4i32);
    pub const Installing: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
    pub const Canceled: Self = Self(7i32);
    pub const Paused: Self = Self(8i32);
    pub const Error: Self = Self(9i32);
    pub const PausedLowBattery: Self = Self(10i32);
    pub const PausedWiFiRecommended: Self = Self(11i32);
    pub const PausedWiFiRequired: Self = Self(12i32);
    pub const ReadyToDownload: Self = Self(13i32);
}
impl ::core::marker::Copy for AppInstallState {}
impl ::core::clone::Clone for AppInstallState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppInstallState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallType(pub i32);
impl AppInstallType {
    pub const Install: Self = Self(0i32);
    pub const Update: Self = Self(1i32);
    pub const Repair: Self = Self(2i32);
}
impl ::core::marker::Copy for AppInstallType {}
impl ::core::clone::Clone for AppInstallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppInstallType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppInstallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppInstallationToastNotificationMode(pub i32);
impl AppInstallationToastNotificationMode {
    pub const Default: Self = Self(0i32);
    pub const Toast: Self = Self(1i32);
    pub const ToastWithoutPopup: Self = Self(2i32);
    pub const NoToast: Self = Self(3i32);
}
impl ::core::marker::Copy for AppInstallationToastNotificationMode {}
impl ::core::clone::Clone for AppInstallationToastNotificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppInstallationToastNotificationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppInstallationToastNotificationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppInstallationToastNotificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallationToastNotificationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppInstallationToastNotificationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AppInstallationToastNotificationMode;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AutoUpdateSetting(pub i32);
impl AutoUpdateSetting {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const EnabledByPolicy: Self = Self(3i32);
}
impl ::core::marker::Copy for AutoUpdateSetting {}
impl ::core::clone::Clone for AutoUpdateSetting {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AutoUpdateSetting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AutoUpdateSetting {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AutoUpdateSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSetting").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutoUpdateSetting {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.AutoUpdateSetting;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Store_Preview_InstallControl\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GetEntitlementStatus(pub i32);
impl GetEntitlementStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const NoStoreAccount: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
}
impl ::core::marker::Copy for GetEntitlementStatus {}
impl ::core::clone::Clone for GetEntitlementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GetEntitlementStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GetEntitlementStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GetEntitlementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetEntitlementStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for GetEntitlementStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Store.Preview.InstallControl.GetEntitlementStatus;i4)");
}
