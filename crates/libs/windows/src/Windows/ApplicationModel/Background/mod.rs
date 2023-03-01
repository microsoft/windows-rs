#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0dd4342_e37b_4823_a5fe_6b31dfefdeb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SubscribedActivities: usize,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivitySensorTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivitySensorTriggerFactory {
    type Vtable = IActivitySensorTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IActivitySensorTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IActivitySensorTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa72691c3_3837_44f7_831b_0132cc872bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportintervalinmilliseconds: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAlarmApplicationManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAlarmApplicationManagerStatics {
    type Vtable = IAlarmApplicationManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IAlarmApplicationManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAlarmApplicationManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca03fa3b_cce6_4de2_b09b_9628bd33bbbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmApplicationManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlarmAccessStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74d4f496_8d37_44ec_9481_2a0b9854eb48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerFactory {
    type Vtable = IAppBroadcastTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x280b9f44_22f4_4618_a02e_e7e411eb7238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateAppBroadcastTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerkey: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastTriggerProviderInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastTriggerProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastTriggerProviderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf219352d_9de8_4420_9ce2_5eff8f17376b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerProviderInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLogoResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LogoResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetVideoKeyFrameInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetVideoKeyFrameInterval: usize,
    #[cfg(feature = "Foundation")]
    pub VideoKeyFrameInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoKeyFrameInterval: usize,
    pub SetMaxVideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxVideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetMaxVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationTrigger {
    type Vtable = IApplicationTrigger_Vtbl;
}
impl ::core::clone::Clone for IApplicationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApplicationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b468630_9574_492c_9e93_1a3ae6335fe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IApplicationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IApplicationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97dc6ab2_2219_4a9e_9c5e_41d047f76e82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IAppointmentStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppointmentStoreNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64d4040c_c201_42ad_aa2a_e21ba3425b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundCondition(::windows::core::IUnknown);
impl IBackgroundCondition {}
::windows::imp::interface_hierarchy!(IBackgroundCondition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundCondition {}
impl ::core::fmt::Debug for IBackgroundCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundCondition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundCondition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ae48a1ee-8951-400a-8302-9c9c9a2a3a3b}");
}
unsafe impl ::windows::core::Interface for IBackgroundCondition {
    type Vtable = IBackgroundCondition_Vtbl;
}
impl ::core::clone::Clone for IBackgroundCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae48a1ee_8951_400a_8302_9c9c9a2a3a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCondition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics {
    type Vtable = IBackgroundExecutionManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundExecutionManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe826ea58_66a9_4d41_83d4_b4c18c87b846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForApplicationAsync: usize,
    pub RemoveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAccessForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
    pub GetAccessStatusForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics2 {
    type Vtable = IBackgroundExecutionManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundExecutionManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x469b24ef_9bbb_4e18_999a_fd6512931be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundExecutionManagerStatics3 {
    type Vtable = IBackgroundExecutionManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundExecutionManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x98a5d3f6_5a25_5b6c_9192_d77a43dfedc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindForModernStandbyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindForModernStandbyAsync: usize,
    pub GetAccessStatusForModernStandby: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
    pub GetAccessStatusForModernStandbyForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTask(::windows::core::IUnknown);
impl IBackgroundTask {
    pub fn Run<P0>(&self, taskinstance: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Run)(::windows::core::Interface::as_raw(this), taskinstance.try_into_param()?.abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTask, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTask {}
impl ::core::fmt::Debug for IBackgroundTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTask").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTask {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{7d13d534-fd12-43ce-8c22-ea1ff13c06df}");
}
unsafe impl ::windows::core::Interface for IBackgroundTask {
    type Vtable = IBackgroundTask_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d13d534_fd12_43ce_8c22_ea1ff13c06df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTask_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskinstance: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0351550e_3e64_4572_a93a_84075a37c917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTaskEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TaskEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trigger: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder2 {
    type Vtable = IBackgroundTaskBuilder2_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskBuilder2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskBuilder2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ae7cfb1_104f_406d_8db6_844a570f42bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCancelOnConditionLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CancelOnConditionLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder3 {
    type Vtable = IBackgroundTaskBuilder3_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskBuilder3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskBuilder3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28c74f4a_8ba9_4c09_a24f_19683e2c924c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetIsNetworkRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsNetworkRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder4 {
    type Vtable = IBackgroundTaskBuilder4_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskBuilder4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskBuilder4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4755e522_cba2_4e35_bd16_a6da7f1c19aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskBuilder5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskBuilder5 {
    type Vtable = IBackgroundTaskBuilder5_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskBuilder5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskBuilder5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077103f6_99f5_4af4_bcad_4731d0330d43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetTaskEntryPointClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskentrypoint: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x565d25cf_f209_48f4_9967_2b184f7bfbf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CheckResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93cc156d_af27_4dd3_846e_24ee40cadd25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance(::windows::core::IUnknown);
impl IBackgroundTaskInstance {
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistration>();
            (::windows::core::Interface::vtable(this).Task)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProgress)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).TriggerDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled(&self, cancelhandler: &BackgroundTaskCanceledEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Canceled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cancelhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanceled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).SuspendedCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskInstance, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance {}
impl ::core::fmt::Debug for IBackgroundTaskInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskInstance").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskInstance {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{865bda7a-21d8-4573-8f32-928a1b0641f6}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance {
    type Vtable = IBackgroundTaskInstance_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskInstance {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x865bda7a_21d8_4573_8f32_928a1b0641f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub TriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancelhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
    pub SuspendedCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance2(::windows::core::IUnknown);
impl IBackgroundTaskInstance2 {
    pub fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).GetThrottleCount)(::windows::core::Interface::as_raw(this), counter, &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistration>();
            (::windows::core::Interface::vtable(this).Task)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProgress)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).TriggerDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled(&self, cancelhandler: &BackgroundTaskCanceledEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Canceled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cancelhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanceled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).SuspendedCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskInstance2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IBackgroundTaskInstance> for IBackgroundTaskInstance2 {}
impl ::core::cmp::PartialEq for IBackgroundTaskInstance2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance2 {}
impl ::core::fmt::Debug for IBackgroundTaskInstance2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskInstance2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskInstance2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4f7d0176-0c76-4fb4-896d-5de1864122f6}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance2 {
    type Vtable = IBackgroundTaskInstance2_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskInstance2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskInstance2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f7d0176_0c76_4fb4_896d_5de1864122f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetThrottleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskInstance4(::windows::core::IUnknown);
impl IBackgroundTaskInstance4 {
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistration>();
            (::windows::core::Interface::vtable(this).Task)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProgress)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).TriggerDetails)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled(&self, cancelhandler: &BackgroundTaskCanceledEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Canceled)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(cancelhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanceled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).SuspendedCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskInstance4, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IBackgroundTaskInstance> for IBackgroundTaskInstance4 {}
impl ::core::cmp::PartialEq for IBackgroundTaskInstance4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskInstance4 {}
impl ::core::fmt::Debug for IBackgroundTaskInstance4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskInstance4").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskInstance4 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{7f29f23c-aa04-4b08-97b0-06d874cdabf5}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskInstance4 {
    type Vtable = IBackgroundTaskInstance4_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskInstance4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskInstance4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f29f23c_aa04_4b08_97b0_06d874cdabf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskProgressEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskProgressEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskProgressEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb1468ac_8332_4d0a_9532_03eae684da31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskProgressEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration(::windows::core::IUnknown);
impl IBackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress(&self, handler: &BackgroundTaskProgressEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProgress)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &BackgroundTaskCompletedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskRegistration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration {}
impl ::core::fmt::Debug for IBackgroundTaskRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskRegistration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskRegistration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10654cc2_a26e_43bf_8c12_1fb40dbfbfa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Progress: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgress: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration2(::windows::core::IUnknown);
impl IBackgroundTaskRegistration2 {
    pub fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IBackgroundTrigger>();
            (::windows::core::Interface::vtable(this).Trigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress(&self, handler: &BackgroundTaskProgressEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProgress)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &BackgroundTaskCompletedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskRegistration2, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IBackgroundTaskRegistration> for IBackgroundTaskRegistration2 {}
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration2 {}
impl ::core::fmt::Debug for IBackgroundTaskRegistration2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskRegistration2").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskRegistration2 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{6138c703-bb86-4112-afc3-7f939b166e3b}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration2 {
    type Vtable = IBackgroundTaskRegistration2_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistration2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6138c703_bb86_4112_afc3_7f939b166e3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTaskRegistration3(::windows::core::IUnknown);
impl IBackgroundTaskRegistration3 {
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).TaskGroup)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress(&self, handler: &BackgroundTaskProgressEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProgress)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &BackgroundTaskCompletedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows::imp::interface_hierarchy!(IBackgroundTaskRegistration3, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IBackgroundTaskRegistration> for IBackgroundTaskRegistration3 {}
impl ::core::cmp::PartialEq for IBackgroundTaskRegistration3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTaskRegistration3 {}
impl ::core::fmt::Debug for IBackgroundTaskRegistration3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTaskRegistration3").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTaskRegistration3 {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{fe338195-9423-4d8b-830d-b1dd2c7badd5}");
}
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistration3 {
    type Vtable = IBackgroundTaskRegistration3_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistration3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistration3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe338195_9423_4d8b_830d_b1dd2c7badd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistrationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistrationGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ab1919a_871b_4167_8a76_055cd67b5b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationGroupFactory {
    type Vtable = IBackgroundTaskRegistrationGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistrationGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistrationGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83d92b69_44cf_4631_9740_03c7d8741bc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationStatics {
    type Vtable = IBackgroundTaskRegistrationStatics_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistrationStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistrationStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c542f69_b000_42ba_a093_6a563c65e3f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundTaskRegistrationStatics2 {
    type Vtable = IBackgroundTaskRegistrationStatics2_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTaskRegistrationStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTaskRegistrationStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x174b671e_b20d_4fa9_ad9a_e93ad6c71e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTaskGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTaskGroups: usize,
    pub GetTaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct IBackgroundTrigger(::windows::core::IUnknown);
impl IBackgroundTrigger {}
::windows::imp::interface_hierarchy!(IBackgroundTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IBackgroundTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBackgroundTrigger {}
impl ::core::fmt::Debug for IBackgroundTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBackgroundTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IBackgroundTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{84b3a058-6027-4b87-9790-bdf3f757dbd7}");
}
unsafe impl ::windows::core::Interface for IBackgroundTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
impl ::core::clone::Clone for IBackgroundTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBackgroundWorkCostStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBackgroundWorkCostStatics {
    type Vtable = IBackgroundWorkCostStatics_Vtbl;
}
impl ::core::clone::Clone for IBackgroundWorkCostStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBackgroundWorkCostStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc740a662_c310_4b82_b3e3_3bcfb9e4c77d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundWorkCostStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentBackgroundWorkCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundWorkCostValue) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementPublisherTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab3e2612_25d3_48ae_8724_d81877ae6129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Advertisement: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementPublisherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger2_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTrigger2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementPublisherTrigger2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa28d064_38f4_597d_b597_4e55588c6503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredTransmitPowerLevelInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredTransmitPowerLevelInDBm: usize,
    pub UseExtendedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUseExtendedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementWatcherTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1aab1819_bce1_48eb_a827_59fb7cee52a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOutOfRangeTimeout: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SetSignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    AdvertisementFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    SetAdvertisementFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBluetoothLEAdvertisementWatcherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger2_Vtbl;
}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTrigger2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IBluetoothLEAdvertisementWatcherTrigger2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39b56799_eb39_5ab6_9932_aa9e4549604d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe21caeeb_32f2_4d31_b553_b9e01bde37e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for ICachedFileUpdaterTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICachedFileUpdaterTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71838c13_1314_47b4_9597_dc7e248c17cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateTarget: usize,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateRequest: usize,
    pub CanRequestUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IChatMessageNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IChatMessageNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x513b43bf_1d40_5c5d_78f5_c923fee3739e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReceivedNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IChatMessageReceivedNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IChatMessageReceivedNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ea3760e_baf5_4077_88e9_060cf6f0c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReceivedNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_Vtbl;
}
impl ::core::clone::Clone for ICommunicationBlockingAppSetAsActiveTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICommunicationBlockingAppSetAsActiveTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb91f28a_16a5_486d_974c_7835a8477be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IContactStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactStoreNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc833419b_4705_4571_9a16_06b997bf9c96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetchTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_Vtbl;
}
impl ::core::clone::Clone for IContentPrefetchTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContentPrefetchTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x710627ee_04fa_440b_80c0_173202199e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub WaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitInterval: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentPrefetchTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentPrefetchTriggerFactory {
    type Vtable = IContentPrefetchTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IContentPrefetchTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContentPrefetchTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2643eda_8a03_409e_b8c4_88814c28ccb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitinterval: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSystemEventTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_Vtbl;
}
impl ::core::clone::Clone for ICustomSystemEventTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICustomSystemEventTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3596798_cf6b_4ef4_a0ca_29cf4a278c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICustomSystemEventTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICustomSystemEventTriggerFactory {
    type Vtable = ICustomSystemEventTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ICustomSystemEventTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICustomSystemEventTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bcb16c5_f2dc_41b2_9efd_b96bdcd13ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_Vtbl;
}
impl ::core::clone::Clone for IDeviceConnectionChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceConnectionChangeTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90875e64_3cdd_4efb_ab1c_5b3b6a60ce34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceConnectionChangeTriggerStatics {
    type Vtable = IDeviceConnectionChangeTriggerStatics_Vtbl;
}
impl ::core::clone::Clone for IDeviceConnectionChangeTriggerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceConnectionChangeTriggerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3ea246a_4efd_4498_aa60_a4e4e3b17ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IDeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IDeviceManufacturerNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IDeviceManufacturerNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81278ab5_41ab_16da_86c2_7f7bf0912f5b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TriggerQualifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TriggerQualifier: usize,
    #[cfg(feature = "deprecated")]
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OneShot: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTriggerFactory(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IDeviceManufacturerNotificationTriggerFactory {
    type Vtable = IDeviceManufacturerNotificationTriggerFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for IDeviceManufacturerNotificationTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for IDeviceManufacturerNotificationTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7955de75_25bb_4153_a1a2_3029fcabb652);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerqualifier: ::std::mem::MaybeUninit<::windows::core::HSTRING>, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceServicingTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_Vtbl;
}
impl ::core::clone::Clone for IDeviceServicingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceServicingTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab217ad_6e34_49d3_9e6f_17f1b6dfa881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, arguments: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceUseTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_Vtbl;
}
impl ::core::clone::Clone for IDeviceUseTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceUseTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0da68011_334f_4d57_b6ec_6dca64b412e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, arguments: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceWatcherTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_Vtbl;
}
impl ::core::clone::Clone for IDeviceWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDeviceWatcherTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4617fdd_8573_4260_befc_5bec89cb693d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStoreNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IEmailStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailStoreNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d06da_47eb_4268_a4f2_f3f77188388a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe25f8fc8_0696_474f_a732_f292b0cebc5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTrigger2 {
    type Vtable = IGattCharacteristicNotificationTrigger2_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTrigger2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTrigger2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9322a2c4_ae0e_42f2_b28c_f51372e69245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    EventTriggeringMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerFactory {
    type Vtable = IGattCharacteristicNotificationTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57ba1995_b143_4575_9f6b_fd59d93ace1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattCharacteristicNotificationTriggerFactory2 {
    type Vtable = IGattCharacteristicNotificationTriggerFactory2_Vtbl;
}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattCharacteristicNotificationTriggerFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5998e91f_8a53_4e9f_a32c_23cd33664cee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub CreateWithEventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: *mut ::core::ffi::c_void, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))]
    CreateWithEventTriggeringMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddc6a3e9_1557_4bd8_8542_468aa0c696f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub SetAdvertisingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    SetAdvertisingParameters: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub AdvertisingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    AdvertisingParameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderTriggerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderTriggerResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c4691b1_b198_4e84_bad4_cf4ad299ed3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    Error: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGattServiceProviderTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGattServiceProviderTriggerStatics {
    type Vtable = IGattServiceProviderTriggerStatics_Vtbl;
}
impl ::core::clone::Clone for IGattServiceProviderTriggerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGattServiceProviderTriggerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb413a36a_e294_4591_a5a6_64891a828153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, serviceuuid: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGeovisitTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGeovisitTrigger {
    type Vtable = IGeovisitTrigger_Vtbl;
}
impl ::core::clone::Clone for IGeovisitTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGeovisitTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4818edaa_04e1_4127_9a4c_19351b8a80a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub MonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    MonitoringScope: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetMonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetMonitoringScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocationTrigger {
    type Vtable = ILocationTrigger_Vtbl;
}
impl ::core::clone::Clone for ILocationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILocationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47666a1c_6877_481e_8026_ff7e14a811a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocationTriggerType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILocationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILocationTriggerFactory {
    type Vtable = ILocationTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ILocationTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ILocationTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1106bb07_ff69_4e09_aa8b_1384ea475e98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: LocationTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMaintenanceTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_Vtbl;
}
impl ::core::clone::Clone for IMaintenanceTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMaintenanceTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68184c83_fc22_4ce5_841a_7239a9810047);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMaintenanceTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMaintenanceTriggerFactory {
    type Vtable = IMaintenanceTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IMaintenanceTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMaintenanceTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3ddb2e_97dd_4629_88b0_b06cf9482ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaProcessingTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_Vtbl;
}
impl ::core::clone::Clone for IMediaProcessingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMediaProcessingTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a95be65_8a52_4b30_9011_cf38040ea8b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_Vtbl;
}
impl ::core::clone::Clone for INetworkOperatorHotspotAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkOperatorHotspotAuthenticationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe756c791_3001_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorHotspotAuthenticationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorNotificationTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_Vtbl;
}
impl ::core::clone::Clone for INetworkOperatorNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkOperatorNotificationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90089cc6_63cd_480c_95d1_6e6aef801e4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INetworkOperatorNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INetworkOperatorNotificationTriggerFactory {
    type Vtable = INetworkOperatorNotificationTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for INetworkOperatorNotificationTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetworkOperatorNotificationTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a223e00_27d7_4353_adb9_9265aaea579d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneTrigger {
    type Vtable = IPhoneTrigger_Vtbl;
}
impl ::core::clone::Clone for IPhoneTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dcfe99b_d4c5_49f1_b7d3_82e87a0e9dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    TriggerType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhoneTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhoneTriggerFactory {
    type Vtable = IPhoneTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IPhoneTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPhoneTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0d93cda_5fc1_48fb_a546_32262040157b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPushNotificationTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPushNotificationTriggerFactory {
    type Vtable = IPushNotificationTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IPushNotificationTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPushNotificationTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dd8ed1b_458e_4fc2_bc2e_d5664f77ed19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_Vtbl;
}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRcsEndUserMessageAvailableTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x986d0d6a_b2f6_467f_a978_a44091c11a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRfcommConnectionTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_Vtbl;
}
impl ::core::clone::Clone for IRfcommConnectionTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRfcommConnectionTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8c4cae2_0b53_4464_9394_fd875654de64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub InboundConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    InboundConnection: usize,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub OutboundConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    OutboundConnection: usize,
    pub AllowMultipleConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowMultipleConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    SetProtectionLevel: usize,
    #[cfg(feature = "Networking")]
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    RemoteHostName: usize,
    #[cfg(feature = "Networking")]
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetRemoteHostName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ISecondaryAuthenticationFactorAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf237f327_5181_4f24_96a7_700a4e5fac62);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_Vtbl;
}
impl ::core::clone::Clone for ISensorDataThresholdTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorDataThresholdTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bc0f372_d48b_4b7f_abec_15f9bacc12e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISensorDataThresholdTriggerFactory {
    type Vtable = ISensorDataThresholdTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ISensorDataThresholdTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISensorDataThresholdTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x921fe675_7df0_4da3_97b3_e544ee857fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sensors")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmartCardTrigger {
    type Vtable = ISmartCardTrigger_Vtbl;
}
impl ::core::clone::Clone for ISmartCardTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmartCardTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf53bc5ac_84ca_4972_8ce9_e58f97b37a50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    TriggerType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmartCardTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmartCardTriggerFactory {
    type Vtable = ISmartCardTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ISmartCardTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmartCardTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63bf54c3_89c1_4e00_a9d3_97c629269dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISmsMessageReceivedTriggerFactory {
    type Vtable = ISmsMessageReceivedTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ISmsMessageReceivedTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISmsMessageReceivedTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3ad8c8_6ba4_4ab2_8d21_bc6b09c77564);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sms")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISocketActivityTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISocketActivityTrigger {
    type Vtable = ISocketActivityTrigger_Vtbl;
}
impl ::core::clone::Clone for ISocketActivityTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISocketActivityTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9bbf810_9dde_4f8a_83e3_b0e0e7a50d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTrackerTriggerFactory {
    type Vtable = IStorageLibraryChangeTrackerTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IStorageLibraryChangeTrackerTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageLibraryChangeTrackerTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1eb0ffd0_5a85_499e_a888_824607124f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracker: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_Vtbl;
}
impl ::core::clone::Clone for IStorageLibraryContentChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageLibraryContentChangedTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1637e0a7_829c_45bc_929b_a1e7ea78d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryContentChangedTriggerStatics {
    type Vtable = IStorageLibraryContentChangedTriggerStatics_Vtbl;
}
impl ::core::clone::Clone for IStorageLibraryContentChangedTriggerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStorageLibraryContentChangedTriggerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f9f1b39_5f90_4e12_914e_a7d8e0bbfb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibrary: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub CreateFromLibraries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibraries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    CreateFromLibraries: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemCondition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemCondition {
    type Vtable = ISystemCondition_Vtbl;
}
impl ::core::clone::Clone for ISystemCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemCondition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc15fb476_89c5_420b_abd3_fb3030472128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCondition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ConditionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemConditionType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemConditionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemConditionFactory {
    type Vtable = ISystemConditionFactory_Vtbl;
}
impl ::core::clone::Clone for ISystemConditionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemConditionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd269d1f1_05a7_49ae_87d7_16b2b8b9a553);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemConditionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditiontype: SystemConditionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemTrigger {
    type Vtable = ISystemTrigger_Vtbl;
}
impl ::core::clone::Clone for ISystemTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d80c776_3748_4463_8d7e_276dc139ac1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemTriggerType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISystemTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISystemTriggerFactory {
    type Vtable = ISystemTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ISystemTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISystemTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe80423d4_8791_4579_8126_87ec8aaa407a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: SystemTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeTrigger(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
}
impl ::core::clone::Clone for ITimeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimeTrigger {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x656e5556_0b2a_4377_ba70_3b45a935547f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimeTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ITimeTriggerFactory {
    type Vtable = ITimeTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for ITimeTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ITimeTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38c682fe_9b54_45e6_b2f3_269b87a6f734);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationActionTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationActionTriggerFactory {
    type Vtable = IToastNotificationActionTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationActionTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IToastNotificationActionTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb09dfc27_6480_4349_8125_97b3efaa0a3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IToastNotificationHistoryChangedTriggerFactory {
    type Vtable = IToastNotificationHistoryChangedTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IToastNotificationHistoryChangedTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IToastNotificationHistoryChangedTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x81c6faad_8797_4785_81b4_b0cccb73d1d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserNotificationChangedTriggerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserNotificationChangedTriggerFactory {
    type Vtable = IUserNotificationChangedTriggerFactory_Vtbl;
}
impl ::core::clone::Clone for IUserNotificationChangedTriggerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserNotificationChangedTriggerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcad4436c_69ab_4e18_a48a_5ed2ac435957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedTriggerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    Create: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ActivitySensorTrigger(::windows::core::IUnknown);
impl ActivitySensorTrigger {
    #[doc = "*Required features: `\"Devices_Sensors\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>>();
            (::windows::core::Interface::vtable(this).SubscribedActivities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Sensors\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>>();
            (::windows::core::Interface::vtable(this).SupportedActivities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinimumReportInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(reportintervalinmilliseconds: u32) -> ::windows::core::Result<ActivitySensorTrigger> {
        Self::IActivitySensorTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ActivitySensorTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), reportintervalinmilliseconds, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorTriggerFactory<R, F: FnOnce(&IActivitySensorTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ActivitySensorTrigger, IActivitySensorTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ActivitySensorTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivitySensorTrigger {}
impl ::core::fmt::Debug for ActivitySensorTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivitySensorTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ActivitySensorTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ActivitySensorTrigger;{d0dd4342-e37b-4823-a5fe-6b31dfefdeb0})");
}
impl ::core::clone::Clone for ActivitySensorTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ActivitySensorTrigger {
    const IID: ::windows::core::GUID = <IActivitySensorTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ActivitySensorTrigger";
}
::windows::imp::interface_hierarchy!(ActivitySensorTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ActivitySensorTrigger {}
unsafe impl ::core::marker::Send for ActivitySensorTrigger {}
unsafe impl ::core::marker::Sync for ActivitySensorTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
pub struct AlarmApplicationManager;
impl AlarmApplicationManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatus() -> ::windows::core::Result<AlarmAccessStatus> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AlarmAccessStatus>();
            (::windows::core::Interface::vtable(this).GetAccessStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAlarmApplicationManagerStatics<R, F: FnOnce(&IAlarmApplicationManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AlarmApplicationManager, IAlarmApplicationManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for AlarmApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AlarmApplicationManager";
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct AppBroadcastTrigger(::windows::core::IUnknown);
impl AppBroadcastTrigger {
    pub fn SetProviderInfo(&self, value: &AppBroadcastTriggerProviderInfo) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ProviderInfo(&self) -> ::windows::core::Result<AppBroadcastTriggerProviderInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastTriggerProviderInfo>();
            (::windows::core::Interface::vtable(this).ProviderInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAppBroadcastTrigger(providerkey: &::windows::core::HSTRING) -> ::windows::core::Result<AppBroadcastTrigger> {
        Self::IAppBroadcastTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastTrigger>();
            (::windows::core::Interface::vtable(this).CreateAppBroadcastTrigger)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(providerkey), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastTriggerFactory<R, F: FnOnce(&IAppBroadcastTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppBroadcastTrigger, IAppBroadcastTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTrigger {}
impl ::core::fmt::Debug for AppBroadcastTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppBroadcastTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTrigger;{74d4f496-8d37-44ec-9481-2a0b9854eb48})");
}
impl ::core::clone::Clone for AppBroadcastTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastTrigger {
    const IID: ::windows::core::GUID = <IAppBroadcastTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTrigger";
}
::windows::imp::interface_hierarchy!(AppBroadcastTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for AppBroadcastTrigger {}
unsafe impl ::core::marker::Send for AppBroadcastTrigger {}
unsafe impl ::core::marker::Sync for AppBroadcastTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct AppBroadcastTriggerProviderInfo(::windows::core::IUnknown);
impl AppBroadcastTriggerProviderInfo {
    pub fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayNameResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayNameResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLogoResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLogoResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LogoResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LogoResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetVideoKeyFrameInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVideoKeyFrameInterval)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VideoKeyFrameInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).VideoKeyFrameInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoBitrate(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxVideoBitrate)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoBitrate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxVideoBitrate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxVideoWidth)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxVideoWidth)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxVideoHeight)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxVideoHeight)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppBroadcastTriggerProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastTriggerProviderInfo {}
impl ::core::fmt::Debug for AppBroadcastTriggerProviderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastTriggerProviderInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppBroadcastTriggerProviderInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo;{f219352d-9de8-4420-9ce2-5eff8f17376b})");
}
impl ::core::clone::Clone for AppBroadcastTriggerProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastTriggerProviderInfo {
    const IID: ::windows::core::GUID = <IAppBroadcastTriggerProviderInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo";
}
::windows::imp::interface_hierarchy!(AppBroadcastTriggerProviderInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastTriggerProviderInfo {}
unsafe impl ::core::marker::Sync for AppBroadcastTriggerProviderInfo {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ApplicationTrigger(::windows::core::IUnknown);
impl ApplicationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ApplicationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments(&self, arguments: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncWithArguments)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ApplicationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationTrigger {}
impl ::core::fmt::Debug for ApplicationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ApplicationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTrigger;{0b468630-9574-492c-9e93-1a3ae6335fe9})");
}
impl ::core::clone::Clone for ApplicationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ApplicationTrigger {
    type Vtable = IApplicationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ApplicationTrigger {
    const IID: ::windows::core::GUID = <IApplicationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTrigger";
}
::windows::imp::interface_hierarchy!(ApplicationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ApplicationTrigger {}
unsafe impl ::core::marker::Send for ApplicationTrigger {}
unsafe impl ::core::marker::Sync for ApplicationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ApplicationTriggerDetails(::windows::core::IUnknown);
impl ApplicationTriggerDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::ValueSet>();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ApplicationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationTriggerDetails {}
impl ::core::fmt::Debug for ApplicationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ApplicationTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTriggerDetails;{97dc6ab2-2219-4a9e-9c5e-41d047f76e82})");
}
impl ::core::clone::Clone for ApplicationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ApplicationTriggerDetails {
    const IID: ::windows::core::GUID = <IApplicationTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTriggerDetails";
}
::windows::imp::interface_hierarchy!(ApplicationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ApplicationTriggerDetails {}
unsafe impl ::core::marker::Sync for ApplicationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct AppointmentStoreNotificationTrigger(::windows::core::IUnknown);
impl AppointmentStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppointmentStoreNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppointmentStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppointmentStoreNotificationTrigger {}
impl ::core::fmt::Debug for AppointmentStoreNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppointmentStoreNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger;{64d4040c-c201-42ad-aa2a-e21ba3425b6d})");
}
impl ::core::clone::Clone for AppointmentStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppointmentStoreNotificationTrigger {
    const IID: ::windows::core::GUID = <IAppointmentStoreNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger";
}
::windows::imp::interface_hierarchy!(AppointmentStoreNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for AppointmentStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
pub struct BackgroundExecutionManager;
impl BackgroundExecutionManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>();
            (::windows::core::Interface::vtable(this).RequestAccessAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForApplicationAsync(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>();
            (::windows::core::Interface::vtable(this).RequestAccessForApplicationAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveAccess() -> ::windows::core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAccess)(::windows::core::Interface::as_raw(this)).ok() })
    }
    pub fn RemoveAccessForApplication(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveAccessForApplication)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid)).ok() })
    }
    pub fn GetAccessStatus() -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundAccessStatus>();
            (::windows::core::Interface::vtable(this).GetAccessStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForApplication(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundAccessStatus>();
            (::windows::core::Interface::vtable(this).GetAccessStatusForApplication)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindAsync(requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestAccessKindAsync)(::windows::core::Interface::as_raw(this), requestedaccess, ::core::mem::transmute_copy(reason), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindForModernStandbyAsync(requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).RequestAccessKindForModernStandbyAsync)(::windows::core::Interface::as_raw(this), requestedaccess, ::core::mem::transmute_copy(reason), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForModernStandby() -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundAccessStatus>();
            (::windows::core::Interface::vtable(this).GetAccessStatusForModernStandby)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForModernStandbyForApplication(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundAccessStatus>();
            (::windows::core::Interface::vtable(this).GetAccessStatusForModernStandbyForApplication)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics<R, F: FnOnce(&IBackgroundExecutionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics2<R, F: FnOnce(&IBackgroundExecutionManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics3<R, F: FnOnce(&IBackgroundExecutionManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for BackgroundExecutionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundExecutionManager";
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskBuilder(::windows::core::IUnknown);
impl BackgroundTaskBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundTaskBuilder, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTaskEntryPoint(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTaskEntryPoint)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TaskEntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TaskEntryPoint)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTrigger<P0>(&self, trigger: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IBackgroundTrigger>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTrigger)(::windows::core::Interface::as_raw(this), trigger.try_into_param()?.abi()).ok() }
    }
    pub fn AddCondition<P0>(&self, condition: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IBackgroundCondition>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddCondition)(::windows::core::Interface::as_raw(this), condition.try_into_param()?.abi()).ok() }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Register(&self) -> ::windows::core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistration>();
            (::windows::core::Interface::vtable(this).Register)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCancelOnConditionLoss)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelOnConditionLoss(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CancelOnConditionLoss)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsNetworkRequested(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsNetworkRequested)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNetworkRequested(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsNetworkRequested)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).TaskGroup)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTaskGroup(&self, value: &BackgroundTaskRegistrationGroup) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTaskGroup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetTaskEntryPointClsid(&self, taskentrypoint: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskBuilder5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTaskEntryPointClsid)(::windows::core::Interface::as_raw(this), taskentrypoint).ok() }
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskBuilder {}
impl ::core::fmt::Debug for BackgroundTaskBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskBuilder").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskBuilder {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskBuilder;{0351550e-3e64-4572-a93a-84075a37c917})");
}
impl ::core::clone::Clone for BackgroundTaskBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskBuilder {
    const IID: ::windows::core::GUID = <IBackgroundTaskBuilder as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskBuilder";
}
::windows::imp::interface_hierarchy!(BackgroundTaskBuilder, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventArgs(::windows::core::IUnknown);
impl BackgroundTaskCompletedEventArgs {
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CheckResult(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CheckResult)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCompletedEventArgs {}
impl ::core::fmt::Debug for BackgroundTaskCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs;{565d25cf-f209-48f4-9967-2b184f7bfbf0})");
}
impl ::core::clone::Clone for BackgroundTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskCompletedEventArgs {
    const IID: ::windows::core::GUID = <IBackgroundTaskCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(BackgroundTaskCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskCompletedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskDeferral(::windows::core::IUnknown);
impl BackgroundTaskDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskDeferral {}
impl ::core::fmt::Debug for BackgroundTaskDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskDeferral").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskDeferral;{93cc156d-af27-4dd3-846e-24ee40cadd25})");
}
impl ::core::clone::Clone for BackgroundTaskDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskDeferral {
    const IID: ::windows::core::GUID = <IBackgroundTaskDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskDeferral";
}
::windows::imp::interface_hierarchy!(BackgroundTaskDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskDeferral {}
unsafe impl ::core::marker::Sync for BackgroundTaskDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskProgressEventArgs(::windows::core::IUnknown);
impl BackgroundTaskProgressEventArgs {
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskProgressEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskProgressEventArgs {}
impl ::core::fmt::Debug for BackgroundTaskProgressEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskProgressEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskProgressEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs;{fb1468ac-8332-4d0a-9532-03eae684da31})");
}
impl ::core::clone::Clone for BackgroundTaskProgressEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskProgressEventArgs {
    const IID: ::windows::core::GUID = <IBackgroundTaskProgressEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs";
}
::windows::imp::interface_hierarchy!(BackgroundTaskProgressEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskProgressEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskProgressEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskRegistration(::windows::core::IUnknown);
impl BackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Progress(&self, handler: &BackgroundTaskProgressEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProgress)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, handler: &BackgroundTaskCompletedEventHandler) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Unregister)(::windows::core::Interface::as_raw(this), canceltask).ok() }
    }
    pub fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IBackgroundTrigger>();
            (::windows::core::Interface::vtable(this).Trigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows::core::ComInterface::cast::<IBackgroundTaskRegistration3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).TaskGroup)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>> {
        Self::IBackgroundTaskRegistrationStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>>();
            (::windows::core::Interface::vtable(this).AllTasks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTaskGroups() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>>();
            (::windows::core::Interface::vtable(this).AllTaskGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetTaskGroup(groupid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).GetTaskGroup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(groupid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics<R, F: FnOnce(&IBackgroundTaskRegistrationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics2<R, F: FnOnce(&IBackgroundTaskRegistrationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskRegistration {}
impl ::core::fmt::Debug for BackgroundTaskRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskRegistration").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskRegistration {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistration;{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0})");
}
impl ::core::clone::Clone for BackgroundTaskRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskRegistration {
    const IID: ::windows::core::GUID = <IBackgroundTaskRegistration as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistration";
}
::windows::imp::interface_hierarchy!(BackgroundTaskRegistration, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTaskRegistration> for BackgroundTaskRegistration {}
impl ::windows::core::CanTryInto<IBackgroundTaskRegistration2> for BackgroundTaskRegistration {}
impl ::windows::core::CanTryInto<IBackgroundTaskRegistration3> for BackgroundTaskRegistration {}
unsafe impl ::core::marker::Send for BackgroundTaskRegistration {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistration {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskRegistrationGroup(::windows::core::IUnknown);
impl BackgroundTaskRegistrationGroup {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated(&self, handler: &super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).BackgroundActivated)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBackgroundActivated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>>();
            (::windows::core::Interface::vtable(this).AllTasks)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(id: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithName(id: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundTaskRegistrationGroup>();
            (::windows::core::Interface::vtable(this).CreateWithName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationGroupFactory<R, F: FnOnce(&IBackgroundTaskRegistrationGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundTaskRegistrationGroup, IBackgroundTaskRegistrationGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskRegistrationGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskRegistrationGroup {}
impl ::core::fmt::Debug for BackgroundTaskRegistrationGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskRegistrationGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskRegistrationGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup;{2ab1919a-871b-4167-8a76-055cd67b5b23})");
}
impl ::core::clone::Clone for BackgroundTaskRegistrationGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskRegistrationGroup {
    const IID: ::windows::core::GUID = <IBackgroundTaskRegistrationGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup";
}
::windows::imp::interface_hierarchy!(BackgroundTaskRegistrationGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskRegistrationGroup {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistrationGroup {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
pub struct BackgroundWorkCost;
impl BackgroundWorkCost {
    pub fn CurrentBackgroundWorkCost() -> ::windows::core::Result<BackgroundWorkCostValue> {
        Self::IBackgroundWorkCostStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<BackgroundWorkCostValue>();
            (::windows::core::Interface::vtable(this).CurrentBackgroundWorkCost)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundWorkCostStatics<R, F: FnOnce(&IBackgroundWorkCostStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BackgroundWorkCost, IBackgroundWorkCostStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for BackgroundWorkCost {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundWorkCost";
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTrigger(::windows::core::IUnknown);
impl BluetoothLEAdvertisementPublisherTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BluetoothLEAdvertisementPublisherTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Advertisement(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>();
            (::windows::core::Interface::vtable(this).Advertisement)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<i16>>();
            (::windows::core::Interface::vtable(this).PreferredTransmitPowerLevelInDBm)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<i16>>,
    {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredTransmitPowerLevelInDBm)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn UseExtendedFormat(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).UseExtendedFormat)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUseExtendedFormat(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUseExtendedFormat)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAnonymous)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAnonymous)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IncludeTransmitPowerLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIncludeTransmitPowerLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementPublisherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementPublisherTrigger {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementPublisherTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementPublisherTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger;{ab3e2612-25d3-48ae-8724-d81877ae6129})");
}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BluetoothLEAdvertisementPublisherTrigger {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementPublisherTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger";
}
::windows::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTrigger(::windows::core::IUnknown);
impl BluetoothLEAdvertisementWatcherTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<BluetoothLEAdvertisementWatcherTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MinSamplingInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MaxSamplingInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MinOutOfRangeTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).MaxOutOfRangeTimeout)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>();
            (::windows::core::Interface::vtable(this).SignalStrengthFilter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SetSignalStrengthFilter(&self, value: &super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalStrengthFilter)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn AdvertisementFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>();
            (::windows::core::Interface::vtable(this).AdvertisementFilter)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Advertisement\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn SetAdvertisementFilter(&self, value: &super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdvertisementFilter)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowExtendedAdvertisements)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowExtendedAdvertisements)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BluetoothLEAdvertisementWatcherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BluetoothLEAdvertisementWatcherTrigger {}
impl ::core::fmt::Debug for BluetoothLEAdvertisementWatcherTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BluetoothLEAdvertisementWatcherTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger;{1aab1819-bce1-48eb-a827-59fb7cee52a6})");
}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for BluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for BluetoothLEAdvertisementWatcherTrigger {
    const IID: ::windows::core::GUID = <IBluetoothLEAdvertisementWatcherTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for BluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger";
}
::windows::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterTrigger(::windows::core::IUnknown);
impl CachedFileUpdaterTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CachedFileUpdaterTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterTrigger {}
impl ::core::fmt::Debug for CachedFileUpdaterTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CachedFileUpdaterTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTrigger;{e21caeeb-32f2-4d31-b553-b9e01bde37e0})");
}
impl ::core::clone::Clone for CachedFileUpdaterTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CachedFileUpdaterTrigger {
    const IID: ::windows::core::GUID = <ICachedFileUpdaterTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTrigger";
}
::windows::imp::interface_hierarchy!(CachedFileUpdaterTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for CachedFileUpdaterTrigger {}
unsafe impl ::core::marker::Send for CachedFileUpdaterTrigger {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterTriggerDetails(::windows::core::IUnknown);
impl CachedFileUpdaterTriggerDetails {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateTarget(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Provider::CachedFileTarget>();
            (::windows::core::Interface::vtable(this).UpdateTarget)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateRequest(&self) -> ::windows::core::Result<super::super::Storage::Provider::FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Provider::FileUpdateRequest>();
            (::windows::core::Interface::vtable(this).UpdateRequest)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRequestUserInput(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanRequestUserInput)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterTriggerDetails {}
impl ::core::fmt::Debug for CachedFileUpdaterTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CachedFileUpdaterTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails;{71838c13-1314-47b4-9597-dc7e248c17cc})");
}
impl ::core::clone::Clone for CachedFileUpdaterTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CachedFileUpdaterTriggerDetails {
    const IID: ::windows::core::GUID = <ICachedFileUpdaterTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails";
}
::windows::imp::interface_hierarchy!(CachedFileUpdaterTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CachedFileUpdaterTriggerDetails {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ChatMessageNotificationTrigger(::windows::core::IUnknown);
impl ChatMessageNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ChatMessageNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ChatMessageNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageNotificationTrigger {}
impl ::core::fmt::Debug for ChatMessageNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ChatMessageNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageNotificationTrigger;{513b43bf-1d40-5c5d-78f5-c923fee3739e})");
}
impl ::core::clone::Clone for ChatMessageNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ChatMessageNotificationTrigger {
    const IID: ::windows::core::GUID = <IChatMessageNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageNotificationTrigger";
}
::windows::imp::interface_hierarchy!(ChatMessageNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ChatMessageNotificationTrigger {}
unsafe impl ::core::marker::Send for ChatMessageNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ChatMessageReceivedNotificationTrigger(::windows::core::IUnknown);
impl ChatMessageReceivedNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ChatMessageReceivedNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ChatMessageReceivedNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageReceivedNotificationTrigger {}
impl ::core::fmt::Debug for ChatMessageReceivedNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageReceivedNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ChatMessageReceivedNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger;{3ea3760e-baf5-4077-88e9-060cf6f0c6d5})");
}
impl ::core::clone::Clone for ChatMessageReceivedNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ChatMessageReceivedNotificationTrigger {
    const IID: ::windows::core::GUID = <IChatMessageReceivedNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger";
}
::windows::imp::interface_hierarchy!(ChatMessageReceivedNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::core::marker::Send for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageReceivedNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(::windows::core::IUnknown);
impl CommunicationBlockingAppSetAsActiveTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CommunicationBlockingAppSetAsActiveTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CommunicationBlockingAppSetAsActiveTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CommunicationBlockingAppSetAsActiveTrigger {}
impl ::core::fmt::Debug for CommunicationBlockingAppSetAsActiveTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommunicationBlockingAppSetAsActiveTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger;{fb91f28a-16a5-486d-974c-7835a8477be2})");
}
impl ::core::clone::Clone for CommunicationBlockingAppSetAsActiveTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CommunicationBlockingAppSetAsActiveTrigger {
    const IID: ::windows::core::GUID = <ICommunicationBlockingAppSetAsActiveTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger";
}
::windows::imp::interface_hierarchy!(CommunicationBlockingAppSetAsActiveTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::core::marker::Send for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::core::marker::Sync for CommunicationBlockingAppSetAsActiveTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ContactStoreNotificationTrigger(::windows::core::IUnknown);
impl ContactStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ContactStoreNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ContactStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactStoreNotificationTrigger {}
impl ::core::fmt::Debug for ContactStoreNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactStoreNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactStoreNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContactStoreNotificationTrigger;{c833419b-4705-4571-9a16-06b997bf9c96})");
}
impl ::core::clone::Clone for ContactStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactStoreNotificationTrigger {
    const IID: ::windows::core::GUID = <IContactStoreNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContactStoreNotificationTrigger";
}
::windows::imp::interface_hierarchy!(ContactStoreNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ContactStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for ContactStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for ContactStoreNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ContentPrefetchTrigger(::windows::core::IUnknown);
impl ContentPrefetchTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ContentPrefetchTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).WaitInterval)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(waitinterval: super::super::Foundation::TimeSpan) -> ::windows::core::Result<ContentPrefetchTrigger> {
        Self::IContentPrefetchTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ContentPrefetchTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), waitinterval, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetchTriggerFactory<R, F: FnOnce(&IContentPrefetchTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ContentPrefetchTrigger, IContentPrefetchTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ContentPrefetchTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContentPrefetchTrigger {}
impl ::core::fmt::Debug for ContentPrefetchTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContentPrefetchTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContentPrefetchTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContentPrefetchTrigger;{710627ee-04fa-440b-80c0-173202199e5d})");
}
impl ::core::clone::Clone for ContentPrefetchTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContentPrefetchTrigger {
    const IID: ::windows::core::GUID = <IContentPrefetchTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContentPrefetchTrigger";
}
::windows::imp::interface_hierarchy!(ContentPrefetchTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ContentPrefetchTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentTrigger(::windows::core::IUnknown);
impl ConversationalAgentTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ConversationalAgentTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentTrigger {}
impl ::core::fmt::Debug for ConversationalAgentTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ConversationalAgentTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ConversationalAgentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for ConversationalAgentTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ConversationalAgentTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ConversationalAgentTrigger";
}
::windows::imp::interface_hierarchy!(ConversationalAgentTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ConversationalAgentTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct CustomSystemEventTrigger(::windows::core::IUnknown);
impl CustomSystemEventTrigger {
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TriggerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Recurrence(&self) -> ::windows::core::Result<CustomSystemEventTriggerRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CustomSystemEventTriggerRecurrence>();
            (::windows::core::Interface::vtable(this).Recurrence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggerid: &::windows::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows::core::Result<CustomSystemEventTrigger> {
        Self::ICustomSystemEventTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CustomSystemEventTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerid), recurrence, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomSystemEventTriggerFactory<R, F: FnOnce(&ICustomSystemEventTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<CustomSystemEventTrigger, ICustomSystemEventTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CustomSystemEventTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CustomSystemEventTrigger {}
impl ::core::fmt::Debug for CustomSystemEventTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSystemEventTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CustomSystemEventTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CustomSystemEventTrigger;{f3596798-cf6b-4ef4-a0ca-29cf4a278c87})");
}
impl ::core::clone::Clone for CustomSystemEventTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CustomSystemEventTrigger {
    const IID: ::windows::core::GUID = <ICustomSystemEventTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CustomSystemEventTrigger";
}
::windows::imp::interface_hierarchy!(CustomSystemEventTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for CustomSystemEventTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct DeviceConnectionChangeTrigger(::windows::core::IUnknown);
impl DeviceConnectionChangeTrigger {
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DeviceId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanMaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanMaintainConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaintainConnection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).MaintainConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaintainConnection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>> {
        Self::IDeviceConnectionChangeTriggerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceConnectionChangeTriggerStatics<R, F: FnOnce(&IDeviceConnectionChangeTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeviceConnectionChangeTrigger, IDeviceConnectionChangeTriggerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DeviceConnectionChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceConnectionChangeTrigger {}
impl ::core::fmt::Debug for DeviceConnectionChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceConnectionChangeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceConnectionChangeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger;{90875e64-3cdd-4efb-ab1c-5b3b6a60ce34})");
}
impl ::core::clone::Clone for DeviceConnectionChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceConnectionChangeTrigger {
    const IID: ::windows::core::GUID = <IDeviceConnectionChangeTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger";
}
::windows::imp::interface_hierarchy!(DeviceConnectionChangeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for DeviceConnectionChangeTrigger {}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTrigger {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct DeviceManufacturerNotificationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl DeviceManufacturerNotificationTrigger {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TriggerQualifier(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TriggerQualifier)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OneShot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Create(triggerqualifier: &::windows::core::HSTRING, oneshot: bool) -> ::windows::core::Result<DeviceManufacturerNotificationTrigger> {
        Self::IDeviceManufacturerNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DeviceManufacturerNotificationTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerqualifier), oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDeviceManufacturerNotificationTriggerFactory<R, F: FnOnce(&IDeviceManufacturerNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeviceManufacturerNotificationTrigger, IDeviceManufacturerNotificationTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for DeviceManufacturerNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for DeviceManufacturerNotificationTrigger {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for DeviceManufacturerNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceManufacturerNotificationTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for DeviceManufacturerNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger;{81278ab5-41ab-16da-86c2-7f7bf0912f5b})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for DeviceManufacturerNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for DeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for DeviceManufacturerNotificationTrigger {
    const IID: ::windows::core::GUID = <IDeviceManufacturerNotificationTrigger as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for DeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(DeviceManufacturerNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IBackgroundTrigger> for DeviceManufacturerNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct DeviceServicingTrigger(::windows::core::IUnknown);
impl DeviceServicingTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeviceServicingTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING, expectedduration: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncSimple)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), expectedduration, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, expectedduration: super::super::Foundation::TimeSpan, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncWithArguments)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), expectedduration, ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceServicingTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceServicingTrigger {}
impl ::core::fmt::Debug for DeviceServicingTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceServicingTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceServicingTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceServicingTrigger;{1ab217ad-6e34-49d3-9e6f-17f1b6dfa881})");
}
impl ::core::clone::Clone for DeviceServicingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceServicingTrigger {
    const IID: ::windows::core::GUID = <IDeviceServicingTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceServicingTrigger";
}
::windows::imp::interface_hierarchy!(DeviceServicingTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for DeviceServicingTrigger {}
unsafe impl ::core::marker::Send for DeviceServicingTrigger {}
unsafe impl ::core::marker::Sync for DeviceServicingTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct DeviceUseTrigger(::windows::core::IUnknown);
impl DeviceUseTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DeviceUseTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncSimple)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncWithArguments)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DeviceUseTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUseTrigger {}
impl ::core::fmt::Debug for DeviceUseTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUseTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceUseTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceUseTrigger;{0da68011-334f-4d57-b6ec-6dca64b412e4})");
}
impl ::core::clone::Clone for DeviceUseTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceUseTrigger {
    const IID: ::windows::core::GUID = <IDeviceUseTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceUseTrigger";
}
::windows::imp::interface_hierarchy!(DeviceUseTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for DeviceUseTrigger {}
unsafe impl ::core::marker::Send for DeviceUseTrigger {}
unsafe impl ::core::marker::Sync for DeviceUseTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct DeviceWatcherTrigger(::windows::core::IUnknown);
impl DeviceWatcherTrigger {}
impl ::core::cmp::PartialEq for DeviceWatcherTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherTrigger {}
impl ::core::fmt::Debug for DeviceWatcherTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceWatcherTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceWatcherTrigger;{a4617fdd-8573-4260-befc-5bec89cb693d})");
}
impl ::core::clone::Clone for DeviceWatcherTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DeviceWatcherTrigger {
    const IID: ::windows::core::GUID = <IDeviceWatcherTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceWatcherTrigger";
}
::windows::imp::interface_hierarchy!(DeviceWatcherTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for DeviceWatcherTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct EmailStoreNotificationTrigger(::windows::core::IUnknown);
impl EmailStoreNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailStoreNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailStoreNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStoreNotificationTrigger {}
impl ::core::fmt::Debug for EmailStoreNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStoreNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailStoreNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.EmailStoreNotificationTrigger;{986d06da-47eb-4268-a4f2-f3f77188388a})");
}
impl ::core::clone::Clone for EmailStoreNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailStoreNotificationTrigger {
    const IID: ::windows::core::GUID = <IEmailStoreNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.EmailStoreNotificationTrigger";
}
::windows::imp::interface_hierarchy!(EmailStoreNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for EmailStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for EmailStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for EmailStoreNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct GattCharacteristicNotificationTrigger(::windows::core::IUnknown);
impl GattCharacteristicNotificationTrigger {
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>();
            (::windows::core::Interface::vtable(this).Characteristic)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn EventTriggeringMode(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode> {
        let this = &::windows::core::ComInterface::cast::<IGattCharacteristicNotificationTrigger2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>();
            (::windows::core::Interface::vtable(this).EventTriggeringMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Create(characteristic: &super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic) -> ::windows::core::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCharacteristicNotificationTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(characteristic), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub fn CreateWithEventTriggeringMode(characteristic: &super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::Result<GattCharacteristicNotificationTrigger> {
        Self::IGattCharacteristicNotificationTriggerFactory2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<GattCharacteristicNotificationTrigger>();
            (::windows::core::Interface::vtable(this).CreateWithEventTriggeringMode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(characteristic), eventtriggeringmode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory2<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattCharacteristicNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristicNotificationTrigger {}
impl ::core::fmt::Debug for GattCharacteristicNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattCharacteristicNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger;{e25f8fc8-0696-474f-a732-f292b0cebc5d})");
}
impl ::core::clone::Clone for GattCharacteristicNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattCharacteristicNotificationTrigger {
    const IID: ::windows::core::GUID = <IGattCharacteristicNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger";
}
::windows::imp::interface_hierarchy!(GattCharacteristicNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for GattCharacteristicNotificationTrigger {}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTrigger {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderTrigger(::windows::core::IUnknown);
impl GattServiceProviderTrigger {
    pub fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).TriggerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService>();
            (::windows::core::Interface::vtable(this).Service)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn SetAdvertisingParameters(&self, value: &super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdvertisingParameters)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`*"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn AdvertisingParameters(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>();
            (::windows::core::Interface::vtable(this).AdvertisingParameters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(triggerid: &::windows::core::HSTRING, serviceuuid: ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>> {
        Self::IGattServiceProviderTriggerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>>();
            (::windows::core::Interface::vtable(this).CreateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerid), serviceuuid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderTriggerStatics<R, F: FnOnce(&IGattServiceProviderTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GattServiceProviderTrigger, IGattServiceProviderTriggerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderTrigger {}
impl ::core::fmt::Debug for GattServiceProviderTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTrigger;{ddc6a3e9-1557-4bd8-8542-468aa0c696f6})");
}
impl ::core::clone::Clone for GattServiceProviderTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderTrigger {
    const IID: ::windows::core::GUID = <IGattServiceProviderTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTrigger";
}
::windows::imp::interface_hierarchy!(GattServiceProviderTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for GattServiceProviderTrigger {}
unsafe impl ::core::marker::Send for GattServiceProviderTrigger {}
unsafe impl ::core::marker::Sync for GattServiceProviderTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct GattServiceProviderTriggerResult(::windows::core::IUnknown);
impl GattServiceProviderTriggerResult {
    pub fn Trigger(&self) -> ::windows::core::Result<GattServiceProviderTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<GattServiceProviderTrigger>();
            (::windows::core::Interface::vtable(this).Trigger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth\"`*"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn Error(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::BluetoothError>();
            (::windows::core::Interface::vtable(this).Error)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderTriggerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderTriggerResult {}
impl ::core::fmt::Debug for GattServiceProviderTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderTriggerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GattServiceProviderTriggerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTriggerResult;{3c4691b1-b198-4e84-bad4-cf4ad299ed3a})");
}
impl ::core::clone::Clone for GattServiceProviderTriggerResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GattServiceProviderTriggerResult {
    const IID: ::windows::core::GUID = <IGattServiceProviderTriggerResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTriggerResult";
}
::windows::imp::interface_hierarchy!(GattServiceProviderTriggerResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderTriggerResult {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerResult {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct GeovisitTrigger(::windows::core::IUnknown);
impl GeovisitTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<GeovisitTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn MonitoringScope(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Geolocation::VisitMonitoringScope>();
            (::windows::core::Interface::vtable(this).MonitoringScope)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Geolocation\"`*"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMonitoringScope)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for GeovisitTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GeovisitTrigger {}
impl ::core::fmt::Debug for GeovisitTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeovisitTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for GeovisitTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GeovisitTrigger;{4818edaa-04e1-4127-9a4c-19351b8a80a4})");
}
impl ::core::clone::Clone for GeovisitTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for GeovisitTrigger {
    type Vtable = IGeovisitTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for GeovisitTrigger {
    const IID: ::windows::core::GUID = <IGeovisitTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for GeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GeovisitTrigger";
}
::windows::imp::interface_hierarchy!(GeovisitTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for GeovisitTrigger {}
unsafe impl ::core::marker::Send for GeovisitTrigger {}
unsafe impl ::core::marker::Sync for GeovisitTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct LocationTrigger(::windows::core::IUnknown);
impl LocationTrigger {
    pub fn TriggerType(&self) -> ::windows::core::Result<LocationTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<LocationTriggerType>();
            (::windows::core::Interface::vtable(this).TriggerType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggertype: LocationTriggerType) -> ::windows::core::Result<LocationTrigger> {
        Self::ILocationTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<LocationTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), triggertype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocationTriggerFactory<R, F: FnOnce(&ILocationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<LocationTrigger, ILocationTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LocationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LocationTrigger {}
impl ::core::fmt::Debug for LocationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LocationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.LocationTrigger;{47666a1c-6877-481e-8026-ff7e14a811a0})");
}
impl ::core::clone::Clone for LocationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for LocationTrigger {
    type Vtable = ILocationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for LocationTrigger {
    const IID: ::windows::core::GUID = <ILocationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for LocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.LocationTrigger";
}
::windows::imp::interface_hierarchy!(LocationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for LocationTrigger {}
unsafe impl ::core::marker::Send for LocationTrigger {}
unsafe impl ::core::marker::Sync for LocationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MaintenanceTrigger(::windows::core::IUnknown);
impl MaintenanceTrigger {
    pub fn FreshnessTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).FreshnessTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OneShot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<MaintenanceTrigger> {
        Self::IMaintenanceTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<MaintenanceTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMaintenanceTriggerFactory<R, F: FnOnce(&IMaintenanceTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MaintenanceTrigger, IMaintenanceTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MaintenanceTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MaintenanceTrigger {}
impl ::core::fmt::Debug for MaintenanceTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MaintenanceTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MaintenanceTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MaintenanceTrigger;{68184c83-fc22-4ce5-841a-7239a9810047})");
}
impl ::core::clone::Clone for MaintenanceTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MaintenanceTrigger {
    const IID: ::windows::core::GUID = <IMaintenanceTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MaintenanceTrigger";
}
::windows::imp::interface_hierarchy!(MaintenanceTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MaintenanceTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MediaProcessingTrigger(::windows::core::IUnknown);
impl MediaProcessingTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MediaProcessingTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments(&self, arguments: &super::super::Foundation::Collections::ValueSet) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>();
            (::windows::core::Interface::vtable(this).RequestAsyncWithArguments)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaProcessingTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaProcessingTrigger {}
impl ::core::fmt::Debug for MediaProcessingTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProcessingTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaProcessingTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MediaProcessingTrigger;{9a95be65-8a52-4b30-9011-cf38040ea8b0})");
}
impl ::core::clone::Clone for MediaProcessingTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MediaProcessingTrigger {
    const IID: ::windows::core::GUID = <IMediaProcessingTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MediaProcessingTrigger";
}
::windows::imp::interface_hierarchy!(MediaProcessingTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MediaProcessingTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(::windows::core::IUnknown);
impl MobileBroadbandDeviceServiceNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MobileBroadbandDeviceServiceNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceNotificationTrigger {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MobileBroadbandDeviceServiceNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandDeviceServiceNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MobileBroadbandDeviceServiceNotificationTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandDeviceServiceNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger";
}
::windows::imp::interface_hierarchy!(MobileBroadbandDeviceServiceNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandPcoDataChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MobileBroadbandPcoDataChangeTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPcoDataChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPcoDataChangeTrigger {}
impl ::core::fmt::Debug for MobileBroadbandPcoDataChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPcoDataChangeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MobileBroadbandPcoDataChangeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for MobileBroadbandPcoDataChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPcoDataChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MobileBroadbandPcoDataChangeTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPcoDataChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger";
}
::windows::imp::interface_hierarchy!(MobileBroadbandPcoDataChangeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPcoDataChangeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandPinLockStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MobileBroadbandPinLockStateChangeTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChangeTrigger {}
impl ::core::fmt::Debug for MobileBroadbandPinLockStateChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockStateChangeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MobileBroadbandPinLockStateChangeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandPinLockStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MobileBroadbandPinLockStateChangeTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandPinLockStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger";
}
::windows::imp::interface_hierarchy!(MobileBroadbandPinLockStateChangeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChangeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandRadioStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MobileBroadbandRadioStateChangeTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChangeTrigger {}
impl ::core::fmt::Debug for MobileBroadbandRadioStateChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioStateChangeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MobileBroadbandRadioStateChangeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for MobileBroadbandRadioStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandRadioStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MobileBroadbandRadioStateChangeTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandRadioStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger";
}
::windows::imp::interface_hierarchy!(MobileBroadbandRadioStateChangeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChangeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(::windows::core::IUnknown);
impl MobileBroadbandRegistrationStateChangeTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MobileBroadbandRegistrationStateChangeTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRegistrationStateChangeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRegistrationStateChangeTrigger {}
impl ::core::fmt::Debug for MobileBroadbandRegistrationStateChangeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRegistrationStateChangeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MobileBroadbandRegistrationStateChangeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for MobileBroadbandRegistrationStateChangeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MobileBroadbandRegistrationStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MobileBroadbandRegistrationStateChangeTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MobileBroadbandRegistrationStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger";
}
::windows::imp::interface_hierarchy!(MobileBroadbandRegistrationStateChangeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRegistrationStateChangeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTrigger(::windows::core::IUnknown);
impl NetworkOperatorDataUsageTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<NetworkOperatorDataUsageTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorDataUsageTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorDataUsageTrigger {}
impl ::core::fmt::Debug for NetworkOperatorDataUsageTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NetworkOperatorDataUsageTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for NetworkOperatorDataUsageTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorDataUsageTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for NetworkOperatorDataUsageTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorDataUsageTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger";
}
::windows::imp::interface_hierarchy!(NetworkOperatorDataUsageTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for NetworkOperatorDataUsageTrigger {}
unsafe impl ::core::marker::Send for NetworkOperatorDataUsageTrigger {}
unsafe impl ::core::marker::Sync for NetworkOperatorDataUsageTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(::windows::core::IUnknown);
impl NetworkOperatorHotspotAuthenticationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<NetworkOperatorHotspotAuthenticationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorHotspotAuthenticationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorHotspotAuthenticationTrigger {}
impl ::core::fmt::Debug for NetworkOperatorHotspotAuthenticationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorHotspotAuthenticationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger;{e756c791-3001-4de5-83c7-de61d88831d0})");
}
impl ::core::clone::Clone for NetworkOperatorHotspotAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for NetworkOperatorHotspotAuthenticationTrigger {
    const IID: ::windows::core::GUID = <INetworkOperatorHotspotAuthenticationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger";
}
::windows::imp::interface_hierarchy!(NetworkOperatorHotspotAuthenticationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for NetworkOperatorHotspotAuthenticationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct NetworkOperatorNotificationTrigger(::windows::core::IUnknown);
impl NetworkOperatorNotificationTrigger {
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorNotificationTrigger> {
        Self::INetworkOperatorNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<NetworkOperatorNotificationTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorNotificationTriggerFactory<R, F: FnOnce(&INetworkOperatorNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<NetworkOperatorNotificationTrigger, INetworkOperatorNotificationTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorNotificationTrigger {}
impl ::core::fmt::Debug for NetworkOperatorNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for NetworkOperatorNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger;{90089cc6-63cd-480c-95d1-6e6aef801e4a})");
}
impl ::core::clone::Clone for NetworkOperatorNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for NetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for NetworkOperatorNotificationTrigger {
    const IID: ::windows::core::GUID = <INetworkOperatorNotificationTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for NetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger";
}
::windows::imp::interface_hierarchy!(NetworkOperatorNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for NetworkOperatorNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTrigger(::windows::core::IUnknown);
impl PaymentAppCanMakePaymentTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PaymentAppCanMakePaymentTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PaymentAppCanMakePaymentTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaymentAppCanMakePaymentTrigger {}
impl ::core::fmt::Debug for PaymentAppCanMakePaymentTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaymentAppCanMakePaymentTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PaymentAppCanMakePaymentTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PaymentAppCanMakePaymentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PaymentAppCanMakePaymentTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PaymentAppCanMakePaymentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger";
}
::windows::imp::interface_hierarchy!(PaymentAppCanMakePaymentTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct PhoneTrigger(::windows::core::IUnknown);
impl PhoneTrigger {
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OneShot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn TriggerType(&self) -> ::windows::core::Result<super::Calls::Background::PhoneTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Calls::Background::PhoneTriggerType>();
            (::windows::core::Interface::vtable(this).TriggerType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Calls_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn Create(r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows::core::Result<PhoneTrigger> {
        Self::IPhoneTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PhoneTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), r#type, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneTriggerFactory<R, F: FnOnce(&IPhoneTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PhoneTrigger, IPhoneTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PhoneTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhoneTrigger {}
impl ::core::fmt::Debug for PhoneTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhoneTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PhoneTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PhoneTrigger;{8dcfe99b-d4c5-49f1-b7d3-82e87a0e9dde})");
}
impl ::core::clone::Clone for PhoneTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PhoneTrigger {
    type Vtable = IPhoneTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PhoneTrigger {
    const IID: ::windows::core::GUID = <IPhoneTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PhoneTrigger";
}
::windows::imp::interface_hierarchy!(PhoneTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for PhoneTrigger {}
unsafe impl ::core::marker::Send for PhoneTrigger {}
unsafe impl ::core::marker::Sync for PhoneTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct PushNotificationTrigger(::windows::core::IUnknown);
impl PushNotificationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PushNotificationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<PushNotificationTrigger> {
        Self::IPushNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PushNotificationTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationTriggerFactory<R, F: FnOnce(&IPushNotificationTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PushNotificationTrigger, IPushNotificationTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PushNotificationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PushNotificationTrigger {}
impl ::core::fmt::Debug for PushNotificationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PushNotificationTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PushNotificationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PushNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for PushNotificationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PushNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PushNotificationTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PushNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PushNotificationTrigger";
}
::windows::imp::interface_hierarchy!(PushNotificationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for PushNotificationTrigger {}
unsafe impl ::core::marker::Send for PushNotificationTrigger {}
unsafe impl ::core::marker::Sync for PushNotificationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTrigger(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RcsEndUserMessageAvailableTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableTrigger {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger;{986d0d6a-b2f6-467f-a978-a44091c11a66})");
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RcsEndUserMessageAvailableTrigger {
    const IID: ::windows::core::GUID = <IRcsEndUserMessageAvailableTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger";
}
::windows::imp::interface_hierarchy!(RcsEndUserMessageAvailableTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct RfcommConnectionTrigger(::windows::core::IUnknown);
impl RfcommConnectionTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<RfcommConnectionTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn InboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation>();
            (::windows::core::Interface::vtable(this).InboundConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Bluetooth_Background\"`*"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn OutboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation>();
            (::windows::core::Interface::vtable(this).OutboundConnection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowMultipleConnections(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowMultipleConnections)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowMultipleConnections(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowMultipleConnections)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Networking::Sockets::SocketProtectionLevel>();
            (::windows::core::Interface::vtable(this).ProtectionLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_Sockets\"`*"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectionLevel)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn RemoteHostName(&self) -> ::windows::core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Networking::HostName>();
            (::windows::core::Interface::vtable(this).RemoteHostName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    #[cfg(feature = "Networking")]
    pub fn SetRemoteHostName(&self, value: &super::super::Networking::HostName) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteHostName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for RfcommConnectionTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RfcommConnectionTrigger {}
impl ::core::fmt::Debug for RfcommConnectionTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RfcommConnectionTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for RfcommConnectionTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RfcommConnectionTrigger;{e8c4cae2-0b53-4464-9394-fd875654de64})");
}
impl ::core::clone::Clone for RfcommConnectionTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for RfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for RfcommConnectionTrigger {
    const IID: ::windows::core::GUID = <IRfcommConnectionTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for RfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RfcommConnectionTrigger";
}
::windows::imp::interface_hierarchy!(RfcommConnectionTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for RfcommConnectionTrigger {}
unsafe impl ::core::marker::Send for RfcommConnectionTrigger {}
unsafe impl ::core::marker::Sync for RfcommConnectionTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SecondaryAuthenticationFactorAuthenticationTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::PartialEq for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "deprecated")]
impl ::core::cmp::Eq for SecondaryAuthenticationFactorAuthenticationTrigger {}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SecondaryAuthenticationFactorAuthenticationTrigger").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeType for SecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger;{f237f327-5181-4f24-96a7-700a4e5fac62})");
}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for SecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::ComInterface for SecondaryAuthenticationFactorAuthenticationTrigger {
    const IID: ::windows::core::GUID = <ISecondaryAuthenticationFactorAuthenticationTrigger as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows::core::RuntimeName for SecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger";
}
#[cfg(feature = "deprecated")]
::windows::imp::interface_hierarchy!(SecondaryAuthenticationFactorAuthenticationTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SecondaryAuthenticationFactorAuthenticationTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SensorDataThresholdTrigger(::windows::core::IUnknown);
impl SensorDataThresholdTrigger {
    #[doc = "*Required features: `\"Devices_Sensors\"`*"]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Create<P0>(threshold: P0) -> ::windows::core::Result<SensorDataThresholdTrigger>
    where
        P0: ::windows::core::TryIntoParam<super::super::Devices::Sensors::ISensorDataThreshold>,
    {
        Self::ISensorDataThresholdTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SensorDataThresholdTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), threshold.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISensorDataThresholdTriggerFactory<R, F: FnOnce(&ISensorDataThresholdTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SensorDataThresholdTrigger, ISensorDataThresholdTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SensorDataThresholdTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SensorDataThresholdTrigger {}
impl ::core::fmt::Debug for SensorDataThresholdTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SensorDataThresholdTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SensorDataThresholdTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SensorDataThresholdTrigger;{5bc0f372-d48b-4b7f-abec-15f9bacc12e2})");
}
impl ::core::clone::Clone for SensorDataThresholdTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SensorDataThresholdTrigger {
    const IID: ::windows::core::GUID = <ISensorDataThresholdTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SensorDataThresholdTrigger";
}
::windows::imp::interface_hierarchy!(SensorDataThresholdTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SensorDataThresholdTrigger {}
unsafe impl ::core::marker::Send for SensorDataThresholdTrigger {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SmartCardTrigger(::windows::core::IUnknown);
impl SmartCardTrigger {
    #[doc = "*Required features: `\"Devices_SmartCards\"`*"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn TriggerType(&self) -> ::windows::core::Result<super::super::Devices::SmartCards::SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Devices::SmartCards::SmartCardTriggerType>();
            (::windows::core::Interface::vtable(this).TriggerType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_SmartCards\"`*"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn Create(triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::Result<SmartCardTrigger> {
        Self::ISmartCardTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmartCardTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), triggertype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardTriggerFactory<R, F: FnOnce(&ISmartCardTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmartCardTrigger, ISmartCardTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmartCardTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmartCardTrigger {}
impl ::core::fmt::Debug for SmartCardTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmartCardTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmartCardTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmartCardTrigger;{f53bc5ac-84ca-4972-8ce9-e58f97b37a50})");
}
impl ::core::clone::Clone for SmartCardTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmartCardTrigger {
    type Vtable = ISmartCardTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmartCardTrigger {
    const IID: ::windows::core::GUID = <ISmartCardTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmartCardTrigger";
}
::windows::imp::interface_hierarchy!(SmartCardTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SmartCardTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SmsMessageReceivedTrigger(::windows::core::IUnknown);
impl SmsMessageReceivedTrigger {
    #[doc = "*Required features: `\"Devices_Sms\"`*"]
    #[cfg(feature = "Devices_Sms")]
    pub fn Create(filterrules: &super::super::Devices::Sms::SmsFilterRules) -> ::windows::core::Result<SmsMessageReceivedTrigger> {
        Self::ISmsMessageReceivedTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SmsMessageReceivedTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(filterrules), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsMessageReceivedTriggerFactory<R, F: FnOnce(&ISmsMessageReceivedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SmsMessageReceivedTrigger, ISmsMessageReceivedTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SmsMessageReceivedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SmsMessageReceivedTrigger {}
impl ::core::fmt::Debug for SmsMessageReceivedTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SmsMessageReceivedTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SmsMessageReceivedTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmsMessageReceivedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for SmsMessageReceivedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SmsMessageReceivedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SmsMessageReceivedTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SmsMessageReceivedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmsMessageReceivedTrigger";
}
::windows::imp::interface_hierarchy!(SmsMessageReceivedTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SmsMessageReceivedTrigger {}
unsafe impl ::core::marker::Send for SmsMessageReceivedTrigger {}
unsafe impl ::core::marker::Sync for SmsMessageReceivedTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SocketActivityTrigger(::windows::core::IUnknown);
impl SocketActivityTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SocketActivityTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<ISocketActivityTrigger>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsWakeFromLowPowerSupported)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SocketActivityTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SocketActivityTrigger {}
impl ::core::fmt::Debug for SocketActivityTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SocketActivityTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SocketActivityTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SocketActivityTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for SocketActivityTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SocketActivityTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SocketActivityTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SocketActivityTrigger";
}
::windows::imp::interface_hierarchy!(SocketActivityTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SocketActivityTrigger {}
unsafe impl ::core::marker::Send for SocketActivityTrigger {}
unsafe impl ::core::marker::Sync for SocketActivityTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTrigger(::windows::core::IUnknown);
impl StorageLibraryChangeTrackerTrigger {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn Create(tracker: &super::super::Storage::StorageLibraryChangeTracker) -> ::windows::core::Result<StorageLibraryChangeTrackerTrigger> {
        Self::IStorageLibraryChangeTrackerTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageLibraryChangeTrackerTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(tracker), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryChangeTrackerTriggerFactory<R, F: FnOnce(&IStorageLibraryChangeTrackerTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageLibraryChangeTrackerTrigger, IStorageLibraryChangeTrackerTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for StorageLibraryChangeTrackerTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryChangeTrackerTrigger {}
impl ::core::fmt::Debug for StorageLibraryChangeTrackerTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryChangeTrackerTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageLibraryChangeTrackerTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTrackerTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageLibraryChangeTrackerTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTrackerTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger";
}
::windows::imp::interface_hierarchy!(StorageLibraryChangeTrackerTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::core::marker::Send for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTrackerTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct StorageLibraryContentChangedTrigger(::windows::core::IUnknown);
impl StorageLibraryContentChangedTrigger {
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn Create(storagelibrary: &super::super::Storage::StorageLibrary) -> ::windows::core::Result<StorageLibraryContentChangedTrigger> {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageLibraryContentChangedTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(storagelibrary), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn CreateFromLibraries<P0>(storagelibraries: P0) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>,
    {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<StorageLibraryContentChangedTrigger>();
            (::windows::core::Interface::vtable(this).CreateFromLibraries)(::windows::core::Interface::as_raw(this), storagelibraries.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryContentChangedTriggerStatics<R, F: FnOnce(&IStorageLibraryContentChangedTriggerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<StorageLibraryContentChangedTrigger, IStorageLibraryContentChangedTriggerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for StorageLibraryContentChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageLibraryContentChangedTrigger {}
impl ::core::fmt::Debug for StorageLibraryContentChangedTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageLibraryContentChangedTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for StorageLibraryContentChangedTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger;{1637e0a7-829c-45bc-929b-a1e7ea78d89b})");
}
impl ::core::clone::Clone for StorageLibraryContentChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for StorageLibraryContentChangedTrigger {
    const IID: ::windows::core::GUID = <IStorageLibraryContentChangedTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger";
}
::windows::imp::interface_hierarchy!(StorageLibraryContentChangedTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for StorageLibraryContentChangedTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SystemCondition(::windows::core::IUnknown);
impl SystemCondition {
    pub fn ConditionType(&self) -> ::windows::core::Result<SystemConditionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SystemConditionType>();
            (::windows::core::Interface::vtable(this).ConditionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(conditiontype: SystemConditionType) -> ::windows::core::Result<SystemCondition> {
        Self::ISystemConditionFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SystemCondition>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), conditiontype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemConditionFactory<R, F: FnOnce(&ISystemConditionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SystemCondition, ISystemConditionFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SystemCondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemCondition {}
impl ::core::fmt::Debug for SystemCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemCondition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SystemCondition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemCondition;{c15fb476-89c5-420b-abd3-fb3030472128})");
}
impl ::core::clone::Clone for SystemCondition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SystemCondition {
    type Vtable = ISystemCondition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SystemCondition {
    const IID: ::windows::core::GUID = <ISystemCondition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemCondition";
}
::windows::imp::interface_hierarchy!(SystemCondition, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundCondition> for SystemCondition {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct SystemTrigger(::windows::core::IUnknown);
impl SystemTrigger {
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OneShot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TriggerType(&self) -> ::windows::core::Result<SystemTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SystemTriggerType>();
            (::windows::core::Interface::vtable(this).TriggerType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggertype: SystemTriggerType, oneshot: bool) -> ::windows::core::Result<SystemTrigger> {
        Self::ISystemTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<SystemTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), triggertype, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemTriggerFactory<R, F: FnOnce(&ISystemTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<SystemTrigger, ISystemTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SystemTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SystemTrigger {}
impl ::core::fmt::Debug for SystemTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SystemTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemTrigger;{1d80c776-3748-4463-8d7e-276dc139ac1c})");
}
impl ::core::clone::Clone for SystemTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for SystemTrigger {
    type Vtable = ISystemTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for SystemTrigger {
    const IID: ::windows::core::GUID = <ISystemTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for SystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemTrigger";
}
::windows::imp::interface_hierarchy!(SystemTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for SystemTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct TetheringEntitlementCheckTrigger(::windows::core::IUnknown);
impl TetheringEntitlementCheckTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TetheringEntitlementCheckTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TetheringEntitlementCheckTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TetheringEntitlementCheckTrigger {}
impl ::core::fmt::Debug for TetheringEntitlementCheckTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringEntitlementCheckTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TetheringEntitlementCheckTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for TetheringEntitlementCheckTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TetheringEntitlementCheckTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TetheringEntitlementCheckTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TetheringEntitlementCheckTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger";
}
::windows::imp::interface_hierarchy!(TetheringEntitlementCheckTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for TetheringEntitlementCheckTrigger {}
unsafe impl ::core::marker::Send for TetheringEntitlementCheckTrigger {}
unsafe impl ::core::marker::Sync for TetheringEntitlementCheckTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct TimeTrigger(::windows::core::IUnknown);
impl TimeTrigger {
    pub fn FreshnessTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).FreshnessTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).OneShot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<TimeTrigger> {
        Self::ITimeTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<TimeTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeTriggerFactory<R, F: FnOnce(&ITimeTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<TimeTrigger, ITimeTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TimeTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimeTrigger {}
impl ::core::fmt::Debug for TimeTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimeTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TimeTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TimeTrigger;{656e5556-0b2a-4377-ba70-3b45a935547f})");
}
impl ::core::clone::Clone for TimeTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for TimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for TimeTrigger {
    const IID: ::windows::core::GUID = <ITimeTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for TimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TimeTrigger";
}
::windows::imp::interface_hierarchy!(TimeTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for TimeTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ToastNotificationActionTrigger(::windows::core::IUnknown);
impl ToastNotificationActionTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ToastNotificationActionTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationActionTrigger> {
        Self::IToastNotificationActionTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ToastNotificationActionTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationActionTriggerFactory<R, F: FnOnce(&IToastNotificationActionTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ToastNotificationActionTrigger, IToastNotificationActionTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ToastNotificationActionTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationActionTrigger {}
impl ::core::fmt::Debug for ToastNotificationActionTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationActionTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ToastNotificationActionTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationActionTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for ToastNotificationActionTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ToastNotificationActionTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ToastNotificationActionTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationActionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationActionTrigger";
}
::windows::imp::interface_hierarchy!(ToastNotificationActionTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ToastNotificationActionTrigger {}
unsafe impl ::core::marker::Send for ToastNotificationActionTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationActionTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTrigger(::windows::core::IUnknown);
impl ToastNotificationHistoryChangedTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationHistoryChangedTrigger> {
        Self::IToastNotificationHistoryChangedTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<ToastNotificationHistoryChangedTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationHistoryChangedTriggerFactory<R, F: FnOnce(&IToastNotificationHistoryChangedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, IToastNotificationHistoryChangedTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ToastNotificationHistoryChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ToastNotificationHistoryChangedTrigger {}
impl ::core::fmt::Debug for ToastNotificationHistoryChangedTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ToastNotificationHistoryChangedTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ToastNotificationHistoryChangedTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ToastNotificationHistoryChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ToastNotificationHistoryChangedTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ToastNotificationHistoryChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger";
}
::windows::imp::interface_hierarchy!(ToastNotificationHistoryChangedTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::core::marker::Send for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationHistoryChangedTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct UserNotificationChangedTrigger(::windows::core::IUnknown);
impl UserNotificationChangedTrigger {
    #[doc = "*Required features: `\"UI_Notifications\"`*"]
    #[cfg(feature = "UI_Notifications")]
    pub fn Create(notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows::core::Result<UserNotificationChangedTrigger> {
        Self::IUserNotificationChangedTriggerFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UserNotificationChangedTrigger>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), notificationkinds, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserNotificationChangedTriggerFactory<R, F: FnOnce(&IUserNotificationChangedTriggerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UserNotificationChangedTrigger, IUserNotificationChangedTriggerFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for UserNotificationChangedTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserNotificationChangedTrigger {}
impl ::core::fmt::Debug for UserNotificationChangedTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserNotificationChangedTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UserNotificationChangedTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.UserNotificationChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for UserNotificationChangedTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserNotificationChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserNotificationChangedTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserNotificationChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.UserNotificationChangedTrigger";
}
::windows::imp::interface_hierarchy!(UserNotificationChangedTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for UserNotificationChangedTrigger {}
unsafe impl ::core::marker::Send for UserNotificationChangedTrigger {}
unsafe impl ::core::marker::Sync for UserNotificationChangedTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotConnectTrigger(::windows::core::IUnknown);
impl WiFiOnDemandHotspotConnectTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WiFiOnDemandHotspotConnectTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotConnectTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotConnectTrigger {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotConnectTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotConnectTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotConnectTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.WiFiOnDemandHotspotConnectTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotConnectTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotConnectTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotConnectTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotConnectTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotConnectTrigger";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotConnectTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for WiFiOnDemandHotspotConnectTrigger {}
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotConnectTrigger {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotConnectTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct WiFiOnDemandHotspotUpdateMetadataTrigger(::windows::core::IUnknown);
impl WiFiOnDemandHotspotUpdateMetadataTrigger {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WiFiOnDemandHotspotUpdateMetadataTrigger, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for WiFiOnDemandHotspotUpdateMetadataTrigger {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WiFiOnDemandHotspotUpdateMetadataTrigger {}
impl ::core::fmt::Debug for WiFiOnDemandHotspotUpdateMetadataTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WiFiOnDemandHotspotUpdateMetadataTrigger").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.WiFiOnDemandHotspotUpdateMetadataTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
impl ::core::clone::Clone for WiFiOnDemandHotspotUpdateMetadataTrigger {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WiFiOnDemandHotspotUpdateMetadataTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const IID: ::windows::core::GUID = <IBackgroundTrigger as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotUpdateMetadataTrigger";
}
::windows::imp::interface_hierarchy!(WiFiOnDemandHotspotUpdateMetadataTrigger, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IBackgroundTrigger> for WiFiOnDemandHotspotUpdateMetadataTrigger {}
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotUpdateMetadataTrigger {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotUpdateMetadataTrigger {}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithWakeupCapability: Self = Self(1i32);
    pub const AllowedWithoutWakeupCapability: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
}
impl ::core::marker::Copy for AlarmAccessStatus {}
impl ::core::clone::Clone for AlarmAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AlarmAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AlarmAccessStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AlarmAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlarmAccessStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AlarmAccessStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.AlarmAccessStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ApplicationTriggerResult {}
impl ::core::clone::Clone for ApplicationTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationTriggerResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ApplicationTriggerResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ApplicationTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationTriggerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ApplicationTriggerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.ApplicationTriggerResult;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundAccessRequestKind {}
impl ::core::clone::Clone for BackgroundAccessRequestKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundAccessRequestKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BackgroundAccessRequestKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BackgroundAccessRequestKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundAccessRequestKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundAccessRequestKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessRequestKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: Self = Self(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const AlwaysAllowed: Self = Self(4i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(5i32);
    pub const DeniedBySystemPolicy: Self = Self(6i32);
    pub const DeniedByUser: Self = Self(7i32);
}
impl ::core::marker::Copy for BackgroundAccessStatus {}
impl ::core::clone::Clone for BackgroundAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BackgroundAccessStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BackgroundAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundAccessStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundAccessStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: Self = Self(0i32);
    pub const Terminating: Self = Self(1i32);
    pub const LoggingOff: Self = Self(2i32);
    pub const ServicingUpdate: Self = Self(3i32);
    pub const IdleTask: Self = Self(4i32);
    pub const Uninstall: Self = Self(5i32);
    pub const ConditionLoss: Self = Self(6i32);
    pub const SystemPolicy: Self = Self(7i32);
    pub const QuietHoursEntered: Self = Self(8i32);
    pub const ExecutionTimeExceeded: Self = Self(9i32);
    pub const ResourceRevocation: Self = Self(10i32);
    pub const EnergySaver: Self = Self(11i32);
}
impl ::core::marker::Copy for BackgroundTaskCancellationReason {}
impl ::core::clone::Clone for BackgroundTaskCancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTaskCancellationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BackgroundTaskCancellationReason {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BackgroundTaskCancellationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskCancellationReason").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskCancellationReason {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskCancellationReason;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTaskThrottleCounter {}
impl ::core::clone::Clone for BackgroundTaskThrottleCounter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundTaskThrottleCounter {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BackgroundTaskThrottleCounter {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BackgroundTaskThrottleCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskThrottleCounter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundTaskThrottleCounter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskThrottleCounter;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundWorkCostValue {}
impl ::core::clone::Clone for BackgroundWorkCostValue {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BackgroundWorkCostValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for BackgroundWorkCostValue {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for BackgroundWorkCostValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundWorkCostValue").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for BackgroundWorkCostValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundWorkCostValue;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for CustomSystemEventTriggerRecurrence {}
impl ::core::clone::Clone for CustomSystemEventTriggerRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CustomSystemEventTriggerRecurrence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CustomSystemEventTriggerRecurrence {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CustomSystemEventTriggerRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSystemEventTriggerRecurrence").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CustomSystemEventTriggerRecurrence {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.CustomSystemEventTriggerRecurrence;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const LowBattery: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceTriggerResult {}
impl ::core::clone::Clone for DeviceTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceTriggerResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DeviceTriggerResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DeviceTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceTriggerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DeviceTriggerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.DeviceTriggerResult;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: Self = Self(0i32);
}
impl ::core::marker::Copy for LocationTriggerType {}
impl ::core::clone::Clone for LocationTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LocationTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for LocationTriggerType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for LocationTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationTriggerType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for LocationTriggerType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.LocationTriggerType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaProcessingTriggerResult {}
impl ::core::clone::Clone for MediaProcessingTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaProcessingTriggerResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MediaProcessingTriggerResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MediaProcessingTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProcessingTriggerResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for MediaProcessingTriggerResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.MediaProcessingTriggerResult;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: Self = Self(0i32);
    pub const UserPresent: Self = Self(1i32);
    pub const UserNotPresent: Self = Self(2i32);
    pub const InternetAvailable: Self = Self(3i32);
    pub const InternetNotAvailable: Self = Self(4i32);
    pub const SessionConnected: Self = Self(5i32);
    pub const SessionDisconnected: Self = Self(6i32);
    pub const FreeNetworkAvailable: Self = Self(7i32);
    pub const BackgroundWorkCostNotHigh: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemConditionType {}
impl ::core::clone::Clone for SystemConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemConditionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SystemConditionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SystemConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemConditionType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SystemConditionType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemConditionType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: Self = Self(0i32);
    pub const SmsReceived: Self = Self(1i32);
    pub const UserPresent: Self = Self(2i32);
    pub const UserAway: Self = Self(3i32);
    pub const NetworkStateChange: Self = Self(4i32);
    pub const ControlChannelReset: Self = Self(5i32);
    pub const InternetAvailable: Self = Self(6i32);
    pub const SessionConnected: Self = Self(7i32);
    pub const ServicingComplete: Self = Self(8i32);
    pub const LockScreenApplicationAdded: Self = Self(9i32);
    pub const LockScreenApplicationRemoved: Self = Self(10i32);
    pub const TimeZoneChange: Self = Self(11i32);
    pub const OnlineIdConnectedStateChange: Self = Self(12i32);
    pub const BackgroundWorkCostChange: Self = Self(13i32);
    pub const PowerStateChange: Self = Self(14i32);
    pub const DefaultSignInAccountChange: Self = Self(15i32);
}
impl ::core::marker::Copy for SystemTriggerType {}
impl ::core::clone::Clone for SystemTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SystemTriggerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SystemTriggerType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SystemTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemTriggerType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SystemTriggerType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemTriggerType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskCanceledEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskCanceledEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCanceledEventHandlerBox::<F> { vtable: &BackgroundTaskCanceledEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, reason: BackgroundTaskCancellationReason) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.try_into_param()?.abi(), reason).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCanceledEventHandlerBox<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskCanceledEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskCanceledEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCanceledEventHandler_Vtbl = BackgroundTaskCanceledEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<BackgroundTaskCanceledEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, reason: BackgroundTaskCancellationReason) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), reason).into()
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCanceledEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCanceledEventHandler {}
impl ::core::fmt::Debug for BackgroundTaskCanceledEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskCanceledEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskCanceledEventHandler {
    type Vtable = BackgroundTaskCanceledEventHandler_Vtbl;
}
impl ::core::clone::Clone for BackgroundTaskCanceledEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskCanceledEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6c4bac0_51f8_4c57_ac3f_156dd1680c4f);
}
impl ::windows::core::RuntimeType for BackgroundTaskCanceledEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{a6c4bac0-51f8-4c57-ac3f-156dd1680c4f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCanceledEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, reason: BackgroundTaskCancellationReason) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskCompletedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCompletedEventHandlerBox::<F> { vtable: &BackgroundTaskCompletedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &BackgroundTaskRegistration, args: &BackgroundTaskCompletedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCompletedEventHandlerBox<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskCompletedEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCompletedEventHandler_Vtbl = BackgroundTaskCompletedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<BackgroundTaskCompletedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskCompletedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskCompletedEventHandler {}
impl ::core::fmt::Debug for BackgroundTaskCompletedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskCompletedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskCompletedEventHandler {
    type Vtable = BackgroundTaskCompletedEventHandler_Vtbl;
}
impl ::core::clone::Clone for BackgroundTaskCompletedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskCompletedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b38e929_a086_46a7_a678_439135822bcf);
}
impl ::windows::core::RuntimeType for BackgroundTaskCompletedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{5b38e929-a086-46a7-a678-439135822bcf}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCompletedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
#[repr(transparent)]
pub struct BackgroundTaskProgressEventHandler(pub ::windows::core::IUnknown);
impl BackgroundTaskProgressEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskProgressEventHandlerBox::<F> { vtable: &BackgroundTaskProgressEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, sender: &BackgroundTaskRegistration, args: &BackgroundTaskProgressEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sender), ::core::mem::transmute_copy(args)).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskProgressEventHandlerBox<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskProgressEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskProgressEventHandlerBox<F> {
    const VTABLE: BackgroundTaskProgressEventHandler_Vtbl = BackgroundTaskProgressEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<BackgroundTaskProgressEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&sender), ::windows::core::from_raw_borrowed(&args)).into()
    }
}
impl ::core::cmp::PartialEq for BackgroundTaskProgressEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BackgroundTaskProgressEventHandler {}
impl ::core::fmt::Debug for BackgroundTaskProgressEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskProgressEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for BackgroundTaskProgressEventHandler {
    type Vtable = BackgroundTaskProgressEventHandler_Vtbl;
}
impl ::core::clone::Clone for BackgroundTaskProgressEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for BackgroundTaskProgressEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46e0683c_8a88_4c99_804c_76897f6277a6);
}
impl ::windows::core::RuntimeType for BackgroundTaskProgressEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{46e0683c-8a88-4c99-804c-76897f6277a6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskProgressEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
