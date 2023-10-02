#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivitySensorTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivitySensorTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0dd4342_e37b_4823_a5fe_6b31dfefdeb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SubscribedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SubscribedActivities: usize,
    pub ReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub SupportedActivities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Sensors", feature = "Foundation_Collections")))]
    SupportedActivities: usize,
    pub MinimumReportInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IActivitySensorTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IActivitySensorTriggerFactory {
    type Vtable = IActivitySensorTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IActivitySensorTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa72691c3_3837_44f7_831b_0132cc872bc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivitySensorTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportintervalinmilliseconds: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAlarmApplicationManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAlarmApplicationManagerStatics {
    type Vtable = IAlarmApplicationManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAlarmApplicationManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca03fa3b_cce6_4de2_b09b_9628bd33bbbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAlarmApplicationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AlarmAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppBroadcastTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74d4f496_8d37_44ec_9481_2a0b9854eb48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppBroadcastTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastTriggerFactory {
    type Vtable = IAppBroadcastTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x280b9f44_22f4_4618_a02e_e7e411eb7238);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateAppBroadcastTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerkey: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppBroadcastTriggerProviderInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppBroadcastTriggerProviderInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf219352d_9de8_4420_9ce2_5eff8f17376b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastTriggerProviderInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLogoResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LogoResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetVideoKeyFrameInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetVideoKeyFrameInterval: usize,
    #[cfg(feature = "Foundation")]
    pub VideoKeyFrameInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VideoKeyFrameInterval: usize,
    pub SetMaxVideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxVideoBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxVideoWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub MaxVideoHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IApplicationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationTrigger {
    type Vtable = IApplicationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b468630_9574_492c_9e93_1a3ae6335fe9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IApplicationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IApplicationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97dc6ab2_2219_4a9e_9c5e_41d047f76e82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Arguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Arguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64d4040c_c201_42ad_aa2a_e21ba3425b6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundCondition(::windows_core::IUnknown);
impl IBackgroundCondition {}
::windows_core::imp::interface_hierarchy!(IBackgroundCondition, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundCondition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{ae48a1ee-8951-400a-8302-9c9c9a2a3a3b}");
}
unsafe impl ::windows_core::Interface for IBackgroundCondition {
    type Vtable = IBackgroundCondition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundCondition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae48a1ee_8951_400a_8302_9c9c9a2a3a3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCondition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundExecutionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundExecutionManagerStatics {
    type Vtable = IBackgroundExecutionManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundExecutionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe826ea58_66a9_4d41_83d4_b4c18c87b846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForApplicationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForApplicationAsync: usize,
    pub RemoveAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAccessForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetAccessStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows_core::HRESULT,
    pub GetAccessStatusForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundExecutionManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundExecutionManagerStatics2 {
    type Vtable = IBackgroundExecutionManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundExecutionManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x469b24ef_9bbb_4e18_999a_fd6512931be9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundExecutionManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundExecutionManagerStatics3 {
    type Vtable = IBackgroundExecutionManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundExecutionManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98a5d3f6_5a25_5b6c_9192_d77a43dfedc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundExecutionManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessKindForModernStandbyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedaccess: BackgroundAccessRequestKind, reason: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessKindForModernStandbyAsync: usize,
    pub GetAccessStatusForModernStandby: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundAccessStatus) -> ::windows_core::HRESULT,
    pub GetAccessStatusForModernStandbyForApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut BackgroundAccessStatus) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTask(::windows_core::IUnknown);
impl IBackgroundTask {
    pub fn Run<P0>(&self, taskinstance: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Run)(::windows_core::Interface::as_raw(this), taskinstance.try_into_param()?.abi()).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTask, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundTask {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{7d13d534-fd12-43ce-8c22-ea1ff13c06df}");
}
unsafe impl ::windows_core::Interface for IBackgroundTask {
    type Vtable = IBackgroundTask_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTask {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d13d534_fd12_43ce_8c22_ea1ff13c06df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTask_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Run: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskinstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskBuilder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0351550e_3e64_4572_a93a_84075a37c917);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTaskEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskEntryPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trigger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddCondition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, condition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskBuilder2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskBuilder2 {
    type Vtable = IBackgroundTaskBuilder2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskBuilder2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ae7cfb1_104f_406d_8db6_844a570f42bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetCancelOnConditionLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CancelOnConditionLoss: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskBuilder3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskBuilder3 {
    type Vtable = IBackgroundTaskBuilder3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskBuilder3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28c74f4a_8ba9_4c09_a24f_19683e2c924c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsNetworkRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsNetworkRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskBuilder4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskBuilder4 {
    type Vtable = IBackgroundTaskBuilder4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskBuilder4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4755e522_cba2_4e35_bd16_a6da7f1c19aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskBuilder5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskBuilder5 {
    type Vtable = IBackgroundTaskBuilder5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskBuilder5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x077103f6_99f5_4af4_bcad_4731d0330d43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskBuilder5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTaskEntryPointClsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, taskentrypoint: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x565d25cf_f209_48f4_9967_2b184f7bfbf0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CheckResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93cc156d_af27_4dd3_846e_24ee40cadd25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskInstance(::windows_core::IUnknown);
impl IBackgroundTaskInstance {
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows_core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCanceledEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), cancelhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuspendedCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<BackgroundTaskDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundTaskInstance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{865bda7a-21d8-4573-8f32-928a1b0641f6}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskInstance {
    type Vtable = IBackgroundTaskInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskInstance {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x865bda7a_21d8_4573_8f32_928a1b0641f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub TriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cancelhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
    pub SuspendedCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskInstance2(::windows_core::IUnknown);
impl IBackgroundTaskInstance2 {
    pub fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThrottleCount)(::windows_core::Interface::as_raw(this), counter, &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows_core::Result<BackgroundTaskRegistration> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCanceledEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), cancelhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuspendedCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<BackgroundTaskDeferral> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTaskInstance> for IBackgroundTaskInstance2 {}
impl ::windows_core::RuntimeType for IBackgroundTaskInstance2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{4f7d0176-0c76-4fb4-896d-5de1864122f6}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskInstance2 {
    type Vtable = IBackgroundTaskInstance2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskInstance2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4f7d0176_0c76_4fb4_896d_5de1864122f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetThrottleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskInstance4(::windows_core::IUnknown);
impl IBackgroundTaskInstance4 {
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows_core::Result<BackgroundTaskRegistration> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetProgress(&self, value: u32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProgress)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerDetails)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<P0>(&self, cancelhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCanceledEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), cancelhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn SuspendedCount(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuspendedCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<BackgroundTaskDeferral> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskInstance4, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTaskInstance> for IBackgroundTaskInstance4 {}
impl ::windows_core::RuntimeType for IBackgroundTaskInstance4 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{7f29f23c-aa04-4b08-97b0-06d874cdabf5}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskInstance4 {
    type Vtable = IBackgroundTaskInstance4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskInstance4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f29f23c_aa04_4b08_97b0_06d874cdabf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskInstance4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskProgressEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskProgressEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb1468ac_8332_4d0a_9532_03eae684da31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskProgressEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistration(::windows_core::IUnknown);
impl IBackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskProgressEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProgress)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCompletedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundTaskRegistration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10654cc2_a26e_43bf_8c12_1fb40dbfbfa0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Progress: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProgress: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistration2(::windows_core::IUnknown);
impl IBackgroundTaskRegistration2 {
    pub fn Trigger(&self) -> ::windows_core::Result<IBackgroundTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskProgressEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProgress)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCompletedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTaskRegistration> for IBackgroundTaskRegistration2 {}
impl ::windows_core::RuntimeType for IBackgroundTaskRegistration2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{6138c703-bb86-4112-afc3-7f939b166e3b}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistration2 {
    type Vtable = IBackgroundTaskRegistration2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6138c703_bb86_4112_afc3_7f939b166e3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistration3(::windows_core::IUnknown);
impl IBackgroundTaskRegistration3 {
    pub fn TaskGroup(&self) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskGroup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskProgressEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProgress)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCompletedEventHandler>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), canceltask).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IBackgroundTaskRegistration3, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTaskRegistration> for IBackgroundTaskRegistration3 {}
impl ::windows_core::RuntimeType for IBackgroundTaskRegistration3 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fe338195-9423-4d8b-830d-b1dd2c7badd5}");
}
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistration3 {
    type Vtable = IBackgroundTaskRegistration3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistration3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe338195_9423_4d8b_830d_b1dd2c7badd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistration3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistrationGroup(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistrationGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2ab1919a_871b_4167_8a76_055cd67b5b23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroup_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistrationGroupFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistrationGroupFactory {
    type Vtable = IBackgroundTaskRegistrationGroupFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistrationGroupFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d92b69_44cf_4631_9740_03c7d8741bc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationGroupFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistrationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistrationStatics {
    type Vtable = IBackgroundTaskRegistrationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistrationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4c542f69_b000_42ba_a093_6a563c65e3f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTasks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTaskRegistrationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundTaskRegistrationStatics2 {
    type Vtable = IBackgroundTaskRegistrationStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTaskRegistrationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x174b671e_b20d_4fa9_ad9a_e93ad6c71e01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTaskRegistrationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AllTaskGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllTaskGroups: usize,
    pub GetTaskGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundTrigger(::windows_core::IUnknown);
impl IBackgroundTrigger {}
::windows_core::imp::interface_hierarchy!(IBackgroundTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IBackgroundTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{84b3a058-6027-4b87-9790-bdf3f757dbd7}");
}
unsafe impl ::windows_core::Interface for IBackgroundTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84b3a058_6027_4b87_9790_bdf3f757dbd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBackgroundWorkCostStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBackgroundWorkCostStatics {
    type Vtable = IBackgroundWorkCostStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBackgroundWorkCostStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc740a662_c310_4b82_b3e3_3bcfb9e4c77d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundWorkCostStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CurrentBackgroundWorkCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BackgroundWorkCostValue) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBluetoothLEAdvertisementPublisherTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab3e2612_25d3_48ae_8724_d81877ae6129);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub Advertisement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    Advertisement: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementPublisherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBluetoothLEAdvertisementPublisherTrigger2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaa28d064_38f4_597d_b597_4e55588c6503);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredTransmitPowerLevelInDBm: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredTransmitPowerLevelInDBm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredTransmitPowerLevelInDBm: usize,
    pub UseExtendedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetUseExtendedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsAnonymous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeTransmitPowerLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBluetoothLEAdvertisementWatcherTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1aab1819_bce1_48eb_a827_59fb7cee52a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MinSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSamplingInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSamplingInterval: usize,
    #[cfg(feature = "Foundation")]
    pub MinOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MinOutOfRangeTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxOutOfRangeTimeout: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth")]
    pub SetSignalStrengthFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    SetSignalStrengthFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub AdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    AdvertisementFilter: usize,
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub SetAdvertisementFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Advertisement"))]
    SetAdvertisementFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBluetoothLEAdvertisementWatcherTrigger2 {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBluetoothLEAdvertisementWatcherTrigger2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x39b56799_eb39_5ab6_9932_aa9e4549604d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICachedFileUpdaterTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICachedFileUpdaterTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe21caeeb_32f2_4d31_b553_b9e01bde37e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICachedFileUpdaterTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICachedFileUpdaterTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71838c13_1314_47b4_9597_dc7e248c17cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Storage::Provider::CachedFileTarget) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateTarget: usize,
    #[cfg(feature = "Storage_Provider")]
    pub UpdateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Provider"))]
    UpdateRequest: usize,
    pub CanRequestUserInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IChatMessageNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IChatMessageNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x513b43bf_1d40_5c5d_78f5_c923fee3739e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IChatMessageReceivedNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IChatMessageReceivedNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ea3760e_baf5_4077_88e9_060cf6f0c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReceivedNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICommunicationBlockingAppSetAsActiveTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb91f28a_16a5_486d_974c_7835a8477be2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContactStoreNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContactStoreNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc833419b_4705_4571_9a16_06b997bf9c96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentPrefetchTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentPrefetchTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x710627ee_04fa_440b_80c0_173202199e5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub WaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    WaitInterval: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IContentPrefetchTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContentPrefetchTriggerFactory {
    type Vtable = IContentPrefetchTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IContentPrefetchTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2643eda_8a03_409e_b8c4_88814c28ccb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentPrefetchTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, waitinterval: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSystemEventTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSystemEventTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3596798_cf6b_4ef4_a0ca_29cf4a278c87);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CustomSystemEventTriggerRecurrence) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICustomSystemEventTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICustomSystemEventTriggerFactory {
    type Vtable = ICustomSystemEventTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICustomSystemEventTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6bcb16c5_f2dc_41b2_9efd_b96bdcd13ced);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomSystemEventTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, recurrence: CustomSystemEventTriggerRecurrence, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceConnectionChangeTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceConnectionChangeTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90875e64_3cdd_4efb_ab1c_5b3b6a60ce34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub MaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMaintainConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceConnectionChangeTriggerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceConnectionChangeTriggerStatics {
    type Vtable = IDeviceConnectionChangeTriggerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceConnectionChangeTriggerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc3ea246a_4efd_4498_aa60_a4e4e3b17ab9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceConnectionChangeTriggerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceManufacturerNotificationTrigger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IDeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IDeviceManufacturerNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81278ab5_41ab_16da_86c2_7f7bf0912f5b);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub TriggerQualifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TriggerQualifier: usize,
    #[cfg(feature = "deprecated")]
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    OneShot: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceManufacturerNotificationTriggerFactory(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IDeviceManufacturerNotificationTriggerFactory {
    type Vtable = IDeviceManufacturerNotificationTriggerFactory_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IDeviceManufacturerNotificationTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7955de75_25bb_4153_a1a2_3029fcabb652);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceManufacturerNotificationTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerqualifier: ::std::mem::MaybeUninit<::windows_core::HSTRING>, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceServicingTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceServicingTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ab217ad_6e34_49d3_9e6f_17f1b6dfa881);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceServicingTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, expectedduration: super::super::Foundation::TimeSpan, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceUseTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceUseTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0da68011_334f_4d57_b6ec_6dca64b412e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceUseTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncSimple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncSimple: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDeviceWatcherTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDeviceWatcherTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4617fdd_8573_4260_befc_5bec89cb693d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceWatcherTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IEmailStoreNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IEmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IEmailStoreNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x986d06da_47eb_4268_a4f2_f3f77188388a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattCharacteristicNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattCharacteristicNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe25f8fc8_0696_474f_a732_f292b0cebc5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Characteristic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Characteristic: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattCharacteristicNotificationTrigger2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattCharacteristicNotificationTrigger2 {
    type Vtable = IGattCharacteristicNotificationTrigger2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattCharacteristicNotificationTrigger2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9322a2c4_ae0e_42f2_b28c_f51372e69245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTrigger2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub EventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    EventTriggeringMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattCharacteristicNotificationTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattCharacteristicNotificationTriggerFactory {
    type Vtable = IGattCharacteristicNotificationTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattCharacteristicNotificationTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57ba1995_b143_4575_9f6b_fd59d93ace1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattCharacteristicNotificationTriggerFactory2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattCharacteristicNotificationTriggerFactory2 {
    type Vtable = IGattCharacteristicNotificationTriggerFactory2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattCharacteristicNotificationTriggerFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5998e91f_8a53_4e9f_a32c_23cd33664cee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattCharacteristicNotificationTriggerFactory2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub CreateWithEventTriggeringMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characteristic: *mut ::core::ffi::c_void, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile")))]
    CreateWithEventTriggeringMode: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattServiceProviderTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattServiceProviderTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddc6a3e9_1557_4bd8_8542_468aa0c696f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TriggerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    Service: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub SetAdvertisingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    SetAdvertisingParameters: usize,
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub AdvertisingParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    AdvertisingParameters: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattServiceProviderTriggerResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattServiceProviderTriggerResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3c4691b1_b198_4e84_bad4_cf4ad299ed3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Trigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Bluetooth")]
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Bluetooth::BluetoothError) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth"))]
    Error: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGattServiceProviderTriggerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGattServiceProviderTriggerStatics {
    type Vtable = IGattServiceProviderTriggerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGattServiceProviderTriggerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb413a36a_e294_4591_a5a6_64891a828153);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGattServiceProviderTriggerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, serviceuuid: ::windows_core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGeovisitTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGeovisitTrigger {
    type Vtable = IGeovisitTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGeovisitTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4818edaa_04e1_4127_9a4c_19351b8a80a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeovisitTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Geolocation")]
    pub MonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    MonitoringScope: usize,
    #[cfg(feature = "Devices_Geolocation")]
    pub SetMonitoringScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Geolocation"))]
    SetMonitoringScope: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocationTrigger {
    type Vtable = ILocationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47666a1c_6877_481e_8026_ff7e14a811a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LocationTriggerType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILocationTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILocationTriggerFactory {
    type Vtable = ILocationTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILocationTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1106bb07_ff69_4e09_aa8b_1384ea475e98);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: LocationTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMaintenanceTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMaintenanceTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68184c83_fc22_4ce5_841a_7239a9810047);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMaintenanceTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMaintenanceTriggerFactory {
    type Vtable = IMaintenanceTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMaintenanceTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b3ddb2e_97dd_4629_88b0_b06cf9482ae5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMaintenanceTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMediaProcessingTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMediaProcessingTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a95be65_8a52_4b30_9011_cf38040ea8b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProcessingTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RequestAsyncWithArguments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequestAsyncWithArguments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorHotspotAuthenticationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe756c791_3001_4de5_83c7_de61d88831d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorHotspotAuthenticationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorNotificationTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorNotificationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x90089cc6_63cd_480c_95d1_6e6aef801e4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkOperatorNotificationTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkOperatorNotificationTriggerFactory {
    type Vtable = INetworkOperatorNotificationTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkOperatorNotificationTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a223e00_27d7_4353_adb9_9265aaea579d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkOperatorNotificationTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneTrigger {
    type Vtable = IPhoneTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8dcfe99b_d4c5_49f1_b7d3_82e87a0e9dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::Calls::Background::PhoneTriggerType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    TriggerType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPhoneTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPhoneTriggerFactory {
    type Vtable = IPhoneTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPhoneTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa0d93cda_5fc1_48fb_a546_32262040157b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhoneTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Calls_Background"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPushNotificationTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPushNotificationTriggerFactory {
    type Vtable = IPushNotificationTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPushNotificationTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6dd8ed1b_458e_4fc2_bc2e_d5664f77ed19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPushNotificationTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRcsEndUserMessageAvailableTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRcsEndUserMessageAvailableTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x986d0d6a_b2f6_467f_a978_a44091c11a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRfcommConnectionTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRfcommConnectionTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8c4cae2_0b53_4464_9394_fd875654de64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRfcommConnectionTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub InboundConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    InboundConnection: usize,
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub OutboundConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Bluetooth_Background"))]
    OutboundConnection: usize,
    pub AllowMultipleConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowMultipleConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Networking_Sockets")]
    pub ProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    ProtectionLevel: usize,
    #[cfg(feature = "Networking_Sockets")]
    pub SetProtectionLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_Sockets"))]
    SetProtectionLevel: usize,
    #[cfg(feature = "Networking")]
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    RemoteHostName: usize,
    #[cfg(feature = "Networking")]
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking"))]
    SetRemoteHostName: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for ISecondaryAuthenticationFactorAuthenticationTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf237f327_5181_4f24_96a7_700a4e5fac62);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISensorDataThresholdTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISensorDataThresholdTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5bc0f372_d48b_4b7f_abec_15f9bacc12e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISensorDataThresholdTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISensorDataThresholdTriggerFactory {
    type Vtable = ISensorDataThresholdTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISensorDataThresholdTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x921fe675_7df0_4da3_97b3_e544ee857fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataThresholdTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sensors")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threshold: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISmartCardTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTrigger {
    type Vtable = ISmartCardTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISmartCardTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf53bc5ac_84ca_4972_8ce9_e58f97b37a50);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    TriggerType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISmartCardTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmartCardTriggerFactory {
    type Vtable = ISmartCardTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISmartCardTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63bf54c3_89c1_4e00_a9d3_97c629269dad);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmartCardTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_SmartCards")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_SmartCards"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISmsMessageReceivedTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISmsMessageReceivedTriggerFactory {
    type Vtable = ISmsMessageReceivedTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISmsMessageReceivedTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea3ad8c8_6ba4_4ab2_8d21_bc6b09c77564);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Sms")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filterrules: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Sms"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISocketActivityTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISocketActivityTrigger {
    type Vtable = ISocketActivityTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISocketActivityTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9bbf810_9dde_4f8a_83e3_b0e0e7a50d70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISocketActivityTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsWakeFromLowPowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryChangeTrackerTriggerFactory {
    type Vtable = IStorageLibraryChangeTrackerTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorageLibraryChangeTrackerTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1eb0ffd0_5a85_499e_a888_824607124f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tracker: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorageLibraryContentChangedTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorageLibraryContentChangedTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1637e0a7_829c_45bc_929b_a1e7ea78d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorageLibraryContentChangedTriggerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageLibraryContentChangedTriggerStatics {
    type Vtable = IStorageLibraryContentChangedTriggerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorageLibraryContentChangedTriggerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f9f1b39_5f90_4e12_914e_a7d8e0bbfb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibrary: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Create: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub CreateFromLibraries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storagelibraries: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    CreateFromLibraries: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemCondition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemCondition {
    type Vtable = ISystemCondition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemCondition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc15fb476_89c5_420b_abd3_fb3030472128);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemCondition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConditionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemConditionType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemConditionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemConditionFactory {
    type Vtable = ISystemConditionFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemConditionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd269d1f1_05a7_49ae_87d7_16b2b8b9a553);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemConditionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conditiontype: SystemConditionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemTrigger {
    type Vtable = ISystemTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d80c776_3748_4463_8d7e_276dc139ac1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TriggerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemTriggerType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ISystemTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISystemTriggerFactory {
    type Vtable = ISystemTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ISystemTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe80423d4_8791_4579_8126_87ec8aaa407a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggertype: SystemTriggerType, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITimeTrigger(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITimeTrigger {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x656e5556_0b2a_4377_ba70_3b45a935547f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTrigger_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FreshnessTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OneShot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ITimeTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimeTriggerFactory {
    type Vtable = ITimeTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ITimeTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x38c682fe_9b54_45e6_b2f3_269b87a6f734);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, freshnesstime: u32, oneshot: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IToastNotificationActionTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationActionTriggerFactory {
    type Vtable = IToastNotificationActionTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToastNotificationActionTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb09dfc27_6480_4349_8125_97b3efaa0a3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationActionTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IToastNotificationHistoryChangedTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IToastNotificationHistoryChangedTriggerFactory {
    type Vtable = IToastNotificationHistoryChangedTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IToastNotificationHistoryChangedTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x81c6faad_8797_4785_81b4_b0cccb73d1d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationHistoryChangedTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applicationid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUserNotificationChangedTriggerFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUserNotificationChangedTriggerFactory {
    type Vtable = IUserNotificationChangedTriggerFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUserNotificationChangedTriggerFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcad4436c_69ab_4e18_a48a_5ed2ac435957);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserNotificationChangedTriggerFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Notifications")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationkinds: super::super::UI::Notifications::NotificationKinds, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))]
    Create: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ActivitySensorTrigger(::windows_core::IUnknown);
impl ActivitySensorTrigger {
    #[doc = "Required features: `\"Devices_Sensors\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SubscribedActivities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubscribedActivities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Sensors\"`, `\"Foundation_Collections\"`"]
    #[cfg(all(feature = "Devices_Sensors", feature = "Foundation_Collections"))]
    pub fn SupportedActivities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedActivities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinimumReportInterval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinimumReportInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(reportintervalinmilliseconds: u32) -> ::windows_core::Result<ActivitySensorTrigger> {
        Self::IActivitySensorTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), reportintervalinmilliseconds, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IActivitySensorTriggerFactory<R, F: FnOnce(&IActivitySensorTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ActivitySensorTrigger, IActivitySensorTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ActivitySensorTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ActivitySensorTrigger;{d0dd4342-e37b-4823-a5fe-6b31dfefdeb0})");
}
unsafe impl ::windows_core::Interface for ActivitySensorTrigger {
    type Vtable = IActivitySensorTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ActivitySensorTrigger {
    const IID: ::windows_core::GUID = <IActivitySensorTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ActivitySensorTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ActivitySensorTrigger";
}
::windows_core::imp::interface_hierarchy!(ActivitySensorTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ActivitySensorTrigger {}
unsafe impl ::core::marker::Send for ActivitySensorTrigger {}
unsafe impl ::core::marker::Sync for ActivitySensorTrigger {}
pub struct AlarmApplicationManager;
impl AlarmApplicationManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatus() -> ::windows_core::Result<AlarmAccessStatus> {
        Self::IAlarmApplicationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAlarmApplicationManagerStatics<R, F: FnOnce(&IAlarmApplicationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AlarmApplicationManager, IAlarmApplicationManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AlarmApplicationManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AlarmApplicationManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppBroadcastTrigger(::windows_core::IUnknown);
impl AppBroadcastTrigger {
    pub fn SetProviderInfo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppBroadcastTriggerProviderInfo>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProviderInfo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn ProviderInfo(&self) -> ::windows_core::Result<AppBroadcastTriggerProviderInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProviderInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAppBroadcastTrigger(providerkey: &::windows_core::HSTRING) -> ::windows_core::Result<AppBroadcastTrigger> {
        Self::IAppBroadcastTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAppBroadcastTrigger)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(providerkey), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastTriggerFactory<R, F: FnOnce(&IAppBroadcastTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppBroadcastTrigger, IAppBroadcastTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppBroadcastTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTrigger;{74d4f496-8d37-44ec-9481-2a0b9854eb48})");
}
unsafe impl ::windows_core::Interface for AppBroadcastTrigger {
    type Vtable = IAppBroadcastTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastTrigger {
    const IID: ::windows_core::GUID = <IAppBroadcastTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTrigger";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for AppBroadcastTrigger {}
unsafe impl ::core::marker::Send for AppBroadcastTrigger {}
unsafe impl ::core::marker::Sync for AppBroadcastTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppBroadcastTriggerProviderInfo(::windows_core::IUnknown);
impl AppBroadcastTriggerProviderInfo {
    pub fn SetDisplayNameResource(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayNameResource)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayNameResource(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayNameResource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLogoResource(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLogoResource)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LogoResource(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LogoResource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetVideoKeyFrameInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoKeyFrameInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn VideoKeyFrameInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoKeyFrameInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxVideoBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoBitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxVideoBitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxVideoWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxVideoWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxVideoHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxVideoHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxVideoHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxVideoHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppBroadcastTriggerProviderInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo;{f219352d-9de8-4420-9ce2-5eff8f17376b})");
}
unsafe impl ::windows_core::Interface for AppBroadcastTriggerProviderInfo {
    type Vtable = IAppBroadcastTriggerProviderInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppBroadcastTriggerProviderInfo {
    const IID: ::windows_core::GUID = <IAppBroadcastTriggerProviderInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppBroadcastTriggerProviderInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppBroadcastTriggerProviderInfo";
}
::windows_core::imp::interface_hierarchy!(AppBroadcastTriggerProviderInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastTriggerProviderInfo {}
unsafe impl ::core::marker::Sync for AppBroadcastTriggerProviderInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ApplicationTrigger(::windows_core::IUnknown);
impl ApplicationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ApplicationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments<P0>(&self, arguments: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncWithArguments)(::windows_core::Interface::as_raw(this), arguments.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ApplicationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTrigger;{0b468630-9574-492c-9e93-1a3ae6335fe9})");
}
unsafe impl ::windows_core::Interface for ApplicationTrigger {
    type Vtable = IApplicationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ApplicationTrigger {
    const IID: ::windows_core::GUID = <IApplicationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ApplicationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTrigger";
}
::windows_core::imp::interface_hierarchy!(ApplicationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ApplicationTrigger {}
unsafe impl ::core::marker::Send for ApplicationTrigger {}
unsafe impl ::core::marker::Sync for ApplicationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ApplicationTriggerDetails(::windows_core::IUnknown);
impl ApplicationTriggerDetails {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Arguments(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Arguments)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ApplicationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ApplicationTriggerDetails;{97dc6ab2-2219-4a9e-9c5e-41d047f76e82})");
}
unsafe impl ::windows_core::Interface for ApplicationTriggerDetails {
    type Vtable = IApplicationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ApplicationTriggerDetails {
    const IID: ::windows_core::GUID = <IApplicationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ApplicationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ApplicationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(ApplicationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ApplicationTriggerDetails {}
unsafe impl ::core::marker::Sync for ApplicationTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreNotificationTrigger(::windows_core::IUnknown);
impl AppointmentStoreNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentStoreNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger;{64d4040c-c201-42ad-aa2a-e21ba3425b6d})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreNotificationTrigger {
    type Vtable = IAppointmentStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreNotificationTrigger {
    const IID: ::windows_core::GUID = <IAppointmentStoreNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.AppointmentStoreNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for AppointmentStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTrigger {}
pub struct BackgroundExecutionManager;
impl BackgroundExecutionManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForApplicationAsync(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForApplicationAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    pub fn RemoveAccess() -> ::windows_core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAccess)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn RemoveAccessForApplication(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveAccessForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid)).ok() })
    }
    pub fn GetAccessStatus() -> ::windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForApplication(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatusForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindAsync(requestedaccess: BackgroundAccessRequestKind, reason: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessKindAsync)(::windows_core::Interface::as_raw(this), requestedaccess, ::core::mem::transmute_copy(reason), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessKindForModernStandbyAsync(requestedaccess: BackgroundAccessRequestKind, reason: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessKindForModernStandbyAsync)(::windows_core::Interface::as_raw(this), requestedaccess, ::core::mem::transmute_copy(reason), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForModernStandby() -> ::windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatusForModernStandby)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetAccessStatusForModernStandbyForApplication(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundAccessStatus> {
        Self::IBackgroundExecutionManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAccessStatusForModernStandbyForApplication)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics<R, F: FnOnce(&IBackgroundExecutionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics2<R, F: FnOnce(&IBackgroundExecutionManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundExecutionManagerStatics3<R, F: FnOnce(&IBackgroundExecutionManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundExecutionManager, IBackgroundExecutionManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for BackgroundExecutionManager {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundExecutionManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskBuilder(::windows_core::IUnknown);
impl BackgroundTaskBuilder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundTaskBuilder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTaskEntryPoint(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTaskEntryPoint)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TaskEntryPoint(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskEntryPoint)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTrigger<P0>(&self, trigger: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBackgroundTrigger>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTrigger)(::windows_core::Interface::as_raw(this), trigger.try_into_param()?.abi()).ok() }
    }
    pub fn AddCondition<P0>(&self, condition: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBackgroundCondition>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddCondition)(::windows_core::Interface::as_raw(this), condition.try_into_param()?.abi()).ok() }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Register(&self) -> ::windows_core::Result<BackgroundTaskRegistration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Register)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetCancelOnConditionLoss)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelOnConditionLoss(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CancelOnConditionLoss)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsNetworkRequested(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsNetworkRequested)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsNetworkRequested(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsNetworkRequested)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskGroup(&self) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskGroup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTaskGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskRegistrationGroup>,
    {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTaskGroup)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn SetTaskEntryPointClsid(&self, taskentrypoint: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskBuilder5>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTaskEntryPointClsid)(::windows_core::Interface::as_raw(this), taskentrypoint).ok() }
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskBuilder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskBuilder;{0351550e-3e64-4572-a93a-84075a37c917})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskBuilder {
    type Vtable = IBackgroundTaskBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskBuilder {
    const IID: ::windows_core::GUID = <IBackgroundTaskBuilder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskBuilder {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskBuilder";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskBuilder, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskCompletedEventArgs(::windows_core::IUnknown);
impl BackgroundTaskCompletedEventArgs {
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CheckResult(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CheckResult)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs;{565d25cf-f209-48f4-9967-2b184f7bfbf0})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskCompletedEventArgs {
    type Vtable = IBackgroundTaskCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskCompletedEventArgs {
    const IID: ::windows_core::GUID = <IBackgroundTaskCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskCompletedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskDeferral(::windows_core::IUnknown);
impl BackgroundTaskDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskDeferral;{93cc156d-af27-4dd3-846e-24ee40cadd25})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskDeferral {
    type Vtable = IBackgroundTaskDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskDeferral {
    const IID: ::windows_core::GUID = <IBackgroundTaskDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskDeferral";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskDeferral {}
unsafe impl ::core::marker::Sync for BackgroundTaskDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskProgressEventArgs(::windows_core::IUnknown);
impl BackgroundTaskProgressEventArgs {
    pub fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstanceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Progress(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskProgressEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs;{fb1468ac-8332-4d0a-9532-03eae684da31})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskProgressEventArgs {
    type Vtable = IBackgroundTaskProgressEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskProgressEventArgs {
    const IID: ::windows_core::GUID = <IBackgroundTaskProgressEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskProgressEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskProgressEventArgs";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskProgressEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskProgressEventArgs {}
unsafe impl ::core::marker::Sync for BackgroundTaskProgressEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskRegistration(::windows_core::IUnknown);
impl BackgroundTaskRegistration {
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Progress<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskProgressEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Progress)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProgress(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProgress)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Completed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskCompletedEventHandler>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Completed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleted)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Unregister(&self, canceltask: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Unregister)(::windows_core::Interface::as_raw(this), canceltask).ok() }
    }
    pub fn Trigger(&self) -> ::windows_core::Result<IBackgroundTrigger> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TaskGroup(&self) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        let this = &::windows_core::ComInterface::cast::<IBackgroundTaskRegistration3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskGroup)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks() -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, IBackgroundTaskRegistration>> {
        Self::IBackgroundTaskRegistrationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllTasks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTaskGroups() -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, BackgroundTaskRegistrationGroup>> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllTaskGroups)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetTaskGroup(groupid: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTaskGroup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(groupid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics<R, F: FnOnce(&IBackgroundTaskRegistrationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationStatics2<R, F: FnOnce(&IBackgroundTaskRegistrationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundTaskRegistration, IBackgroundTaskRegistrationStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskRegistration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistration;{10654cc2-a26e-43bf-8c12-1fb40dbfbfa0})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskRegistration {
    type Vtable = IBackgroundTaskRegistration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskRegistration {
    const IID: ::windows_core::GUID = <IBackgroundTaskRegistration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistration";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskRegistration, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTaskRegistration> for BackgroundTaskRegistration {}
impl ::windows_core::CanTryInto<IBackgroundTaskRegistration2> for BackgroundTaskRegistration {}
impl ::windows_core::CanTryInto<IBackgroundTaskRegistration3> for BackgroundTaskRegistration {}
unsafe impl ::core::marker::Send for BackgroundTaskRegistration {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskRegistrationGroup(::windows_core::IUnknown);
impl BackgroundTaskRegistrationGroup {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BackgroundActivated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBackgroundActivated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllTasks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, BackgroundTaskRegistration>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllTasks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(id: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithName(id: &::windows_core::HSTRING, name: &::windows_core::HSTRING) -> ::windows_core::Result<BackgroundTaskRegistrationGroup> {
        Self::IBackgroundTaskRegistrationGroupFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundTaskRegistrationGroupFactory<R, F: FnOnce(&IBackgroundTaskRegistrationGroupFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundTaskRegistrationGroup, IBackgroundTaskRegistrationGroupFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskRegistrationGroup {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup;{2ab1919a-871b-4167-8a76-055cd67b5b23})");
}
unsafe impl ::windows_core::Interface for BackgroundTaskRegistrationGroup {
    type Vtable = IBackgroundTaskRegistrationGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskRegistrationGroup {
    const IID: ::windows_core::GUID = <IBackgroundTaskRegistrationGroup as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BackgroundTaskRegistrationGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundTaskRegistrationGroup";
}
::windows_core::imp::interface_hierarchy!(BackgroundTaskRegistrationGroup, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BackgroundTaskRegistrationGroup {}
unsafe impl ::core::marker::Sync for BackgroundTaskRegistrationGroup {}
pub struct BackgroundWorkCost;
impl BackgroundWorkCost {
    pub fn CurrentBackgroundWorkCost() -> ::windows_core::Result<BackgroundWorkCostValue> {
        Self::IBackgroundWorkCostStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CurrentBackgroundWorkCost)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBackgroundWorkCostStatics<R, F: FnOnce(&IBackgroundWorkCostStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BackgroundWorkCost, IBackgroundWorkCostStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for BackgroundWorkCost {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BackgroundWorkCost";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BluetoothLEAdvertisementPublisherTrigger(::windows_core::IUnknown);
impl BluetoothLEAdvertisementPublisherTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BluetoothLEAdvertisementPublisherTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Advertisement\"`"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn Advertisement(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Advertisement)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i16>> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreferredTransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredTransmitPowerLevelInDBm<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i16>>,
    {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetPreferredTransmitPowerLevelInDBm)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn UseExtendedFormat(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UseExtendedFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUseExtendedFormat(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetUseExtendedFormat)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAnonymous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsAnonymous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAnonymous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsAnonymous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IncludeTransmitPowerLevel(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IncludeTransmitPowerLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementPublisherTrigger2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIncludeTransmitPowerLevel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for BluetoothLEAdvertisementPublisherTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger;{ab3e2612-25d3-48ae-8724-d81877ae6129})");
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementPublisherTrigger {
    type Vtable = IBluetoothLEAdvertisementPublisherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BluetoothLEAdvertisementPublisherTrigger {
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementPublisherTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementPublisherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementPublisherTrigger";
}
::windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementPublisherTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementPublisherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementPublisherTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BluetoothLEAdvertisementWatcherTrigger(::windows_core::IUnknown);
impl BluetoothLEAdvertisementWatcherTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BluetoothLEAdvertisementWatcherTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MinSamplingInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinSamplingInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSamplingInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSamplingInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MinOutOfRangeTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MinOutOfRangeTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MaxOutOfRangeTimeout(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxOutOfRangeTimeout)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth\"`"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SignalStrengthFilter(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignalStrengthFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth\"`"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn SetSignalStrengthFilter<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSignalStrengthFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Advertisement\"`"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn AdvertisementFilter(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisementFilter)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Advertisement\"`"]
    #[cfg(feature = "Devices_Bluetooth_Advertisement")]
    pub fn SetAdvertisementFilter<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAdvertisementFilter)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AllowExtendedAdvertisements(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowExtendedAdvertisements)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IBluetoothLEAdvertisementWatcherTrigger2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowExtendedAdvertisements)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for BluetoothLEAdvertisementWatcherTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger;{1aab1819-bce1-48eb-a827-59fb7cee52a6})");
}
unsafe impl ::windows_core::Interface for BluetoothLEAdvertisementWatcherTrigger {
    type Vtable = IBluetoothLEAdvertisementWatcherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BluetoothLEAdvertisementWatcherTrigger {
    const IID: ::windows_core::GUID = <IBluetoothLEAdvertisementWatcherTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BluetoothLEAdvertisementWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.BluetoothLEAdvertisementWatcherTrigger";
}
::windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementWatcherTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::core::marker::Send for BluetoothLEAdvertisementWatcherTrigger {}
unsafe impl ::core::marker::Sync for BluetoothLEAdvertisementWatcherTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CachedFileUpdaterTrigger(::windows_core::IUnknown);
impl CachedFileUpdaterTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CachedFileUpdaterTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CachedFileUpdaterTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTrigger;{e21caeeb-32f2-4d31-b553-b9e01bde37e0})");
}
unsafe impl ::windows_core::Interface for CachedFileUpdaterTrigger {
    type Vtable = ICachedFileUpdaterTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CachedFileUpdaterTrigger {
    const IID: ::windows_core::GUID = <ICachedFileUpdaterTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CachedFileUpdaterTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTrigger";
}
::windows_core::imp::interface_hierarchy!(CachedFileUpdaterTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for CachedFileUpdaterTrigger {}
unsafe impl ::core::marker::Send for CachedFileUpdaterTrigger {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CachedFileUpdaterTriggerDetails(::windows_core::IUnknown);
impl CachedFileUpdaterTriggerDetails {
    #[doc = "Required features: `\"Storage_Provider\"`"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateTarget(&self) -> ::windows_core::Result<super::super::Storage::Provider::CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateTarget)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Provider\"`"]
    #[cfg(feature = "Storage_Provider")]
    pub fn UpdateRequest(&self) -> ::windows_core::Result<super::super::Storage::Provider::FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateRequest)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanRequestUserInput(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRequestUserInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CachedFileUpdaterTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails;{71838c13-1314-47b4-9597-dc7e248c17cc})");
}
unsafe impl ::windows_core::Interface for CachedFileUpdaterTriggerDetails {
    type Vtable = ICachedFileUpdaterTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CachedFileUpdaterTriggerDetails {
    const IID: ::windows_core::GUID = <ICachedFileUpdaterTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CachedFileUpdaterTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CachedFileUpdaterTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(CachedFileUpdaterTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CachedFileUpdaterTriggerDetails {}
unsafe impl ::core::marker::Sync for CachedFileUpdaterTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ChatMessageNotificationTrigger(::windows_core::IUnknown);
impl ChatMessageNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ChatMessageNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageNotificationTrigger;{513b43bf-1d40-5c5d-78f5-c923fee3739e})");
}
unsafe impl ::windows_core::Interface for ChatMessageNotificationTrigger {
    type Vtable = IChatMessageNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageNotificationTrigger {
    const IID: ::windows_core::GUID = <IChatMessageNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(ChatMessageNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ChatMessageNotificationTrigger {}
unsafe impl ::core::marker::Send for ChatMessageNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ChatMessageReceivedNotificationTrigger(::windows_core::IUnknown);
impl ChatMessageReceivedNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChatMessageReceivedNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ChatMessageReceivedNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger;{3ea3760e-baf5-4077-88e9-060cf6f0c6d5})");
}
unsafe impl ::windows_core::Interface for ChatMessageReceivedNotificationTrigger {
    type Vtable = IChatMessageReceivedNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChatMessageReceivedNotificationTrigger {
    const IID: ::windows_core::GUID = <IChatMessageReceivedNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChatMessageReceivedNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ChatMessageReceivedNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(ChatMessageReceivedNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::core::marker::Send for ChatMessageReceivedNotificationTrigger {}
unsafe impl ::core::marker::Sync for ChatMessageReceivedNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(::windows_core::IUnknown);
impl CommunicationBlockingAppSetAsActiveTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CommunicationBlockingAppSetAsActiveTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CommunicationBlockingAppSetAsActiveTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger;{fb91f28a-16a5-486d-974c-7835a8477be2})");
}
unsafe impl ::windows_core::Interface for CommunicationBlockingAppSetAsActiveTrigger {
    type Vtable = ICommunicationBlockingAppSetAsActiveTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CommunicationBlockingAppSetAsActiveTrigger {
    const IID: ::windows_core::GUID = <ICommunicationBlockingAppSetAsActiveTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CommunicationBlockingAppSetAsActiveTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CommunicationBlockingAppSetAsActiveTrigger";
}
::windows_core::imp::interface_hierarchy!(CommunicationBlockingAppSetAsActiveTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::core::marker::Send for CommunicationBlockingAppSetAsActiveTrigger {}
unsafe impl ::core::marker::Sync for CommunicationBlockingAppSetAsActiveTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContactStoreNotificationTrigger(::windows_core::IUnknown);
impl ContactStoreNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactStoreNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ContactStoreNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContactStoreNotificationTrigger;{c833419b-4705-4571-9a16-06b997bf9c96})");
}
unsafe impl ::windows_core::Interface for ContactStoreNotificationTrigger {
    type Vtable = IContactStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContactStoreNotificationTrigger {
    const IID: ::windows_core::GUID = <IContactStoreNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContactStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContactStoreNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(ContactStoreNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ContactStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for ContactStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for ContactStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContentPrefetchTrigger(::windows_core::IUnknown);
impl ContentPrefetchTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContentPrefetchTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn WaitInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WaitInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Create(waitinterval: super::super::Foundation::TimeSpan) -> ::windows_core::Result<ContentPrefetchTrigger> {
        Self::IContentPrefetchTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), waitinterval, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentPrefetchTriggerFactory<R, F: FnOnce(&IContentPrefetchTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContentPrefetchTrigger, IContentPrefetchTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ContentPrefetchTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ContentPrefetchTrigger;{710627ee-04fa-440b-80c0-173202199e5d})");
}
unsafe impl ::windows_core::Interface for ContentPrefetchTrigger {
    type Vtable = IContentPrefetchTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContentPrefetchTrigger {
    const IID: ::windows_core::GUID = <IContentPrefetchTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContentPrefetchTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ContentPrefetchTrigger";
}
::windows_core::imp::interface_hierarchy!(ContentPrefetchTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ContentPrefetchTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConversationalAgentTrigger(::windows_core::IUnknown);
impl ConversationalAgentTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ConversationalAgentTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ConversationalAgentTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ConversationalAgentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for ConversationalAgentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConversationalAgentTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConversationalAgentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ConversationalAgentTrigger";
}
::windows_core::imp::interface_hierarchy!(ConversationalAgentTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ConversationalAgentTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CustomSystemEventTrigger(::windows_core::IUnknown);
impl CustomSystemEventTrigger {
    pub fn TriggerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Recurrence(&self) -> ::windows_core::Result<CustomSystemEventTriggerRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Recurrence)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggerid: &::windows_core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows_core::Result<CustomSystemEventTrigger> {
        Self::ICustomSystemEventTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerid), recurrence, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICustomSystemEventTriggerFactory<R, F: FnOnce(&ICustomSystemEventTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CustomSystemEventTrigger, ICustomSystemEventTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CustomSystemEventTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.CustomSystemEventTrigger;{f3596798-cf6b-4ef4-a0ca-29cf4a278c87})");
}
unsafe impl ::windows_core::Interface for CustomSystemEventTrigger {
    type Vtable = ICustomSystemEventTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CustomSystemEventTrigger {
    const IID: ::windows_core::GUID = <ICustomSystemEventTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CustomSystemEventTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.CustomSystemEventTrigger";
}
::windows_core::imp::interface_hierarchy!(CustomSystemEventTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for CustomSystemEventTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceConnectionChangeTrigger(::windows_core::IUnknown);
impl DeviceConnectionChangeTrigger {
    pub fn DeviceId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeviceId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanMaintainConnection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanMaintainConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaintainConnection(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaintainConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaintainConnection(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMaintainConnection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>> {
        Self::IDeviceConnectionChangeTriggerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromIdAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeviceConnectionChangeTriggerStatics<R, F: FnOnce(&IDeviceConnectionChangeTriggerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceConnectionChangeTrigger, IDeviceConnectionChangeTriggerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for DeviceConnectionChangeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger;{90875e64-3cdd-4efb-ab1c-5b3b6a60ce34})");
}
unsafe impl ::windows_core::Interface for DeviceConnectionChangeTrigger {
    type Vtable = IDeviceConnectionChangeTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceConnectionChangeTrigger {
    const IID: ::windows_core::GUID = <IDeviceConnectionChangeTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceConnectionChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceConnectionChangeTrigger";
}
::windows_core::imp::interface_hierarchy!(DeviceConnectionChangeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for DeviceConnectionChangeTrigger {}
unsafe impl ::core::marker::Send for DeviceConnectionChangeTrigger {}
unsafe impl ::core::marker::Sync for DeviceConnectionChangeTrigger {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceManufacturerNotificationTrigger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl DeviceManufacturerNotificationTrigger {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn TriggerQualifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerQualifier)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn OneShot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OneShot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Create(triggerqualifier: &::windows_core::HSTRING, oneshot: bool) -> ::windows_core::Result<DeviceManufacturerNotificationTrigger> {
        Self::IDeviceManufacturerNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerqualifier), oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IDeviceManufacturerNotificationTriggerFactory<R, F: FnOnce(&IDeviceManufacturerNotificationTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceManufacturerNotificationTrigger, IDeviceManufacturerNotificationTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for DeviceManufacturerNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger;{81278ab5-41ab-16da-86c2-7f7bf0912f5b})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for DeviceManufacturerNotificationTrigger {
    type Vtable = IDeviceManufacturerNotificationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for DeviceManufacturerNotificationTrigger {
    const IID: ::windows_core::GUID = <IDeviceManufacturerNotificationTrigger as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for DeviceManufacturerNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceManufacturerNotificationTrigger";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(DeviceManufacturerNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IBackgroundTrigger> for DeviceManufacturerNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceServicingTrigger(::windows_core::IUnknown);
impl DeviceServicingTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceServicingTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple(&self, deviceid: &::windows_core::HSTRING, expectedduration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncSimple)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), expectedduration, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments(&self, deviceid: &::windows_core::HSTRING, expectedduration: super::super::Foundation::TimeSpan, arguments: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncWithArguments)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), expectedduration, ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DeviceServicingTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceServicingTrigger;{1ab217ad-6e34-49d3-9e6f-17f1b6dfa881})");
}
unsafe impl ::windows_core::Interface for DeviceServicingTrigger {
    type Vtable = IDeviceServicingTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceServicingTrigger {
    const IID: ::windows_core::GUID = <IDeviceServicingTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceServicingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceServicingTrigger";
}
::windows_core::imp::interface_hierarchy!(DeviceServicingTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for DeviceServicingTrigger {}
unsafe impl ::core::marker::Send for DeviceServicingTrigger {}
unsafe impl ::core::marker::Sync for DeviceServicingTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceUseTrigger(::windows_core::IUnknown);
impl DeviceUseTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DeviceUseTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncSimple(&self, deviceid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncSimple)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsyncWithArguments(&self, deviceid: &::windows_core::HSTRING, arguments: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncWithArguments)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(deviceid), ::core::mem::transmute_copy(arguments), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DeviceUseTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceUseTrigger;{0da68011-334f-4d57-b6ec-6dca64b412e4})");
}
unsafe impl ::windows_core::Interface for DeviceUseTrigger {
    type Vtable = IDeviceUseTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceUseTrigger {
    const IID: ::windows_core::GUID = <IDeviceUseTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceUseTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceUseTrigger";
}
::windows_core::imp::interface_hierarchy!(DeviceUseTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for DeviceUseTrigger {}
unsafe impl ::core::marker::Send for DeviceUseTrigger {}
unsafe impl ::core::marker::Sync for DeviceUseTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DeviceWatcherTrigger(::windows_core::IUnknown);
impl DeviceWatcherTrigger {}
impl ::windows_core::RuntimeType for DeviceWatcherTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.DeviceWatcherTrigger;{a4617fdd-8573-4260-befc-5bec89cb693d})");
}
unsafe impl ::windows_core::Interface for DeviceWatcherTrigger {
    type Vtable = IDeviceWatcherTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DeviceWatcherTrigger {
    const IID: ::windows_core::GUID = <IDeviceWatcherTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DeviceWatcherTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.DeviceWatcherTrigger";
}
::windows_core::imp::interface_hierarchy!(DeviceWatcherTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for DeviceWatcherTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct EmailStoreNotificationTrigger(::windows_core::IUnknown);
impl EmailStoreNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<EmailStoreNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for EmailStoreNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.EmailStoreNotificationTrigger;{986d06da-47eb-4268-a4f2-f3f77188388a})");
}
unsafe impl ::windows_core::Interface for EmailStoreNotificationTrigger {
    type Vtable = IEmailStoreNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for EmailStoreNotificationTrigger {
    const IID: ::windows_core::GUID = <IEmailStoreNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for EmailStoreNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.EmailStoreNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(EmailStoreNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for EmailStoreNotificationTrigger {}
unsafe impl ::core::marker::Send for EmailStoreNotificationTrigger {}
unsafe impl ::core::marker::Sync for EmailStoreNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GattCharacteristicNotificationTrigger(::windows_core::IUnknown);
impl GattCharacteristicNotificationTrigger {
    #[doc = "Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Characteristic(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Characteristic)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Background\"`"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn EventTriggeringMode(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode> {
        let this = &::windows_core::ComInterface::cast::<IGattCharacteristicNotificationTrigger2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EventTriggeringMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Create<P0>(characteristic: P0) -> ::windows_core::Result<GattCharacteristicNotificationTrigger>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>,
    {
        Self::IGattCharacteristicNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), characteristic.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Background\"`, `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(all(feature = "Devices_Bluetooth_Background", feature = "Devices_Bluetooth_GenericAttributeProfile"))]
    pub fn CreateWithEventTriggeringMode<P0>(characteristic: P0, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows_core::Result<GattCharacteristicNotificationTrigger>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>,
    {
        Self::IGattCharacteristicNotificationTriggerFactory2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithEventTriggeringMode)(::windows_core::Interface::as_raw(this), characteristic.into_param().abi(), eventtriggeringmode, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IGattCharacteristicNotificationTriggerFactory2<R, F: FnOnce(&IGattCharacteristicNotificationTriggerFactory2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GattCharacteristicNotificationTrigger, IGattCharacteristicNotificationTriggerFactory2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GattCharacteristicNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger;{e25f8fc8-0696-474f-a732-f292b0cebc5d})");
}
unsafe impl ::windows_core::Interface for GattCharacteristicNotificationTrigger {
    type Vtable = IGattCharacteristicNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GattCharacteristicNotificationTrigger {
    const IID: ::windows_core::GUID = <IGattCharacteristicNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GattCharacteristicNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattCharacteristicNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(GattCharacteristicNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for GattCharacteristicNotificationTrigger {}
unsafe impl ::core::marker::Send for GattCharacteristicNotificationTrigger {}
unsafe impl ::core::marker::Sync for GattCharacteristicNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GattServiceProviderTrigger(::windows_core::IUnknown);
impl GattServiceProviderTrigger {
    pub fn TriggerId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn Service(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Service)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn SetAdvertisingParameters<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAdvertisingParameters)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_GenericAttributeProfile\"`"]
    #[cfg(feature = "Devices_Bluetooth_GenericAttributeProfile")]
    pub fn AdvertisingParameters(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdvertisingParameters)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(triggerid: &::windows_core::HSTRING, serviceuuid: ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>> {
        Self::IGattServiceProviderTriggerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(triggerid), serviceuuid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGattServiceProviderTriggerStatics<R, F: FnOnce(&IGattServiceProviderTriggerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GattServiceProviderTrigger, IGattServiceProviderTriggerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for GattServiceProviderTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTrigger;{ddc6a3e9-1557-4bd8-8542-468aa0c696f6})");
}
unsafe impl ::windows_core::Interface for GattServiceProviderTrigger {
    type Vtable = IGattServiceProviderTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GattServiceProviderTrigger {
    const IID: ::windows_core::GUID = <IGattServiceProviderTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GattServiceProviderTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTrigger";
}
::windows_core::imp::interface_hierarchy!(GattServiceProviderTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for GattServiceProviderTrigger {}
unsafe impl ::core::marker::Send for GattServiceProviderTrigger {}
unsafe impl ::core::marker::Sync for GattServiceProviderTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GattServiceProviderTriggerResult(::windows_core::IUnknown);
impl GattServiceProviderTriggerResult {
    pub fn Trigger(&self) -> ::windows_core::Result<GattServiceProviderTrigger> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Trigger)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth\"`"]
    #[cfg(feature = "Devices_Bluetooth")]
    pub fn Error(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::BluetoothError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for GattServiceProviderTriggerResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GattServiceProviderTriggerResult;{3c4691b1-b198-4e84-bad4-cf4ad299ed3a})");
}
unsafe impl ::windows_core::Interface for GattServiceProviderTriggerResult {
    type Vtable = IGattServiceProviderTriggerResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GattServiceProviderTriggerResult {
    const IID: ::windows_core::GUID = <IGattServiceProviderTriggerResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GattServiceProviderTriggerResult {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GattServiceProviderTriggerResult";
}
::windows_core::imp::interface_hierarchy!(GattServiceProviderTriggerResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for GattServiceProviderTriggerResult {}
unsafe impl ::core::marker::Sync for GattServiceProviderTriggerResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct GeovisitTrigger(::windows_core::IUnknown);
impl GeovisitTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<GeovisitTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn MonitoringScope(&self) -> ::windows_core::Result<super::super::Devices::Geolocation::VisitMonitoringScope> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonitoringScope)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Geolocation\"`"]
    #[cfg(feature = "Devices_Geolocation")]
    pub fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMonitoringScope)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::windows_core::RuntimeType for GeovisitTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.GeovisitTrigger;{4818edaa-04e1-4127-9a4c-19351b8a80a4})");
}
unsafe impl ::windows_core::Interface for GeovisitTrigger {
    type Vtable = IGeovisitTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for GeovisitTrigger {
    const IID: ::windows_core::GUID = <IGeovisitTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for GeovisitTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.GeovisitTrigger";
}
::windows_core::imp::interface_hierarchy!(GeovisitTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for GeovisitTrigger {}
unsafe impl ::core::marker::Send for GeovisitTrigger {}
unsafe impl ::core::marker::Sync for GeovisitTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LocationTrigger(::windows_core::IUnknown);
impl LocationTrigger {
    pub fn TriggerType(&self) -> ::windows_core::Result<LocationTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggertype: LocationTriggerType) -> ::windows_core::Result<LocationTrigger> {
        Self::ILocationTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), triggertype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILocationTriggerFactory<R, F: FnOnce(&ILocationTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LocationTrigger, ILocationTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for LocationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.LocationTrigger;{47666a1c-6877-481e-8026-ff7e14a811a0})");
}
unsafe impl ::windows_core::Interface for LocationTrigger {
    type Vtable = ILocationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LocationTrigger {
    const IID: ::windows_core::GUID = <ILocationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LocationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.LocationTrigger";
}
::windows_core::imp::interface_hierarchy!(LocationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for LocationTrigger {}
unsafe impl ::core::marker::Send for LocationTrigger {}
unsafe impl ::core::marker::Sync for LocationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MaintenanceTrigger(::windows_core::IUnknown);
impl MaintenanceTrigger {
    pub fn FreshnessTime(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FreshnessTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OneShot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows_core::Result<MaintenanceTrigger> {
        Self::IMaintenanceTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMaintenanceTriggerFactory<R, F: FnOnce(&IMaintenanceTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MaintenanceTrigger, IMaintenanceTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MaintenanceTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MaintenanceTrigger;{68184c83-fc22-4ce5-841a-7239a9810047})");
}
unsafe impl ::windows_core::Interface for MaintenanceTrigger {
    type Vtable = IMaintenanceTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MaintenanceTrigger {
    const IID: ::windows_core::GUID = <IMaintenanceTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MaintenanceTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MaintenanceTrigger";
}
::windows_core::imp::interface_hierarchy!(MaintenanceTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MaintenanceTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaProcessingTrigger(::windows_core::IUnknown);
impl MediaProcessingTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaProcessingTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequestAsyncWithArguments<P0>(&self, arguments: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::ValueSet>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAsyncWithArguments)(::windows_core::Interface::as_raw(this), arguments.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaProcessingTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MediaProcessingTrigger;{9a95be65-8a52-4b30-9011-cf38040ea8b0})");
}
unsafe impl ::windows_core::Interface for MediaProcessingTrigger {
    type Vtable = IMediaProcessingTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaProcessingTrigger {
    const IID: ::windows_core::GUID = <IMediaProcessingTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaProcessingTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MediaProcessingTrigger";
}
::windows_core::imp::interface_hierarchy!(MediaProcessingTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MediaProcessingTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(::windows_core::IUnknown);
impl MobileBroadbandDeviceServiceNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandDeviceServiceNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandDeviceServiceNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for MobileBroadbandDeviceServiceNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandDeviceServiceNotificationTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandDeviceServiceNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandDeviceServiceNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandDeviceServiceNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandDeviceServiceNotificationTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandDeviceServiceNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPcoDataChangeTrigger(::windows_core::IUnknown);
impl MobileBroadbandPcoDataChangeTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandPcoDataChangeTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPcoDataChangeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for MobileBroadbandPcoDataChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPcoDataChangeTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPcoDataChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPcoDataChangeTrigger";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPcoDataChangeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandPcoDataChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPcoDataChangeTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandPinLockStateChangeTrigger(::windows_core::IUnknown);
impl MobileBroadbandPinLockStateChangeTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandPinLockStateChangeTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandPinLockStateChangeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for MobileBroadbandPinLockStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandPinLockStateChangeTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandPinLockStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandPinLockStateChangeTrigger";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandPinLockStateChangeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandPinLockStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandPinLockStateChangeTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandRadioStateChangeTrigger(::windows_core::IUnknown);
impl MobileBroadbandRadioStateChangeTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandRadioStateChangeTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandRadioStateChangeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for MobileBroadbandRadioStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandRadioStateChangeTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandRadioStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRadioStateChangeTrigger";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandRadioStateChangeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandRadioStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRadioStateChangeTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(::windows_core::IUnknown);
impl MobileBroadbandRegistrationStateChangeTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MobileBroadbandRegistrationStateChangeTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MobileBroadbandRegistrationStateChangeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for MobileBroadbandRegistrationStateChangeTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MobileBroadbandRegistrationStateChangeTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MobileBroadbandRegistrationStateChangeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.MobileBroadbandRegistrationStateChangeTrigger";
}
::windows_core::imp::interface_hierarchy!(MobileBroadbandRegistrationStateChangeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::core::marker::Send for MobileBroadbandRegistrationStateChangeTrigger {}
unsafe impl ::core::marker::Sync for MobileBroadbandRegistrationStateChangeTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorDataUsageTrigger(::windows_core::IUnknown);
impl NetworkOperatorDataUsageTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorDataUsageTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorDataUsageTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for NetworkOperatorDataUsageTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorDataUsageTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorDataUsageTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorDataUsageTrigger";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorDataUsageTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for NetworkOperatorDataUsageTrigger {}
unsafe impl ::core::marker::Send for NetworkOperatorDataUsageTrigger {}
unsafe impl ::core::marker::Sync for NetworkOperatorDataUsageTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(::windows_core::IUnknown);
impl NetworkOperatorHotspotAuthenticationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorHotspotAuthenticationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorHotspotAuthenticationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger;{e756c791-3001-4de5-83c7-de61d88831d0})");
}
unsafe impl ::windows_core::Interface for NetworkOperatorHotspotAuthenticationTrigger {
    type Vtable = INetworkOperatorHotspotAuthenticationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorHotspotAuthenticationTrigger {
    const IID: ::windows_core::GUID = <INetworkOperatorHotspotAuthenticationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorHotspotAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorHotspotAuthenticationTrigger";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorHotspotAuthenticationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for NetworkOperatorHotspotAuthenticationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkOperatorNotificationTrigger(::windows_core::IUnknown);
impl NetworkOperatorNotificationTrigger {
    pub fn NetworkAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NetworkAccountId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<NetworkOperatorNotificationTrigger> {
        Self::INetworkOperatorNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkOperatorNotificationTriggerFactory<R, F: FnOnce(&INetworkOperatorNotificationTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkOperatorNotificationTrigger, INetworkOperatorNotificationTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for NetworkOperatorNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger;{90089cc6-63cd-480c-95d1-6e6aef801e4a})");
}
unsafe impl ::windows_core::Interface for NetworkOperatorNotificationTrigger {
    type Vtable = INetworkOperatorNotificationTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkOperatorNotificationTrigger {
    const IID: ::windows_core::GUID = <INetworkOperatorNotificationTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkOperatorNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.NetworkOperatorNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(NetworkOperatorNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for NetworkOperatorNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PaymentAppCanMakePaymentTrigger(::windows_core::IUnknown);
impl PaymentAppCanMakePaymentTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PaymentAppCanMakePaymentTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PaymentAppCanMakePaymentTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for PaymentAppCanMakePaymentTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PaymentAppCanMakePaymentTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PaymentAppCanMakePaymentTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PaymentAppCanMakePaymentTrigger";
}
::windows_core::imp::interface_hierarchy!(PaymentAppCanMakePaymentTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::core::marker::Send for PaymentAppCanMakePaymentTrigger {}
unsafe impl ::core::marker::Sync for PaymentAppCanMakePaymentTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PhoneTrigger(::windows_core::IUnknown);
impl PhoneTrigger {
    pub fn OneShot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OneShot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Calls_Background\"`"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn TriggerType(&self) -> ::windows_core::Result<super::Calls::Background::PhoneTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_Calls_Background\"`"]
    #[cfg(feature = "ApplicationModel_Calls_Background")]
    pub fn Create(r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows_core::Result<PhoneTrigger> {
        Self::IPhoneTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), r#type, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhoneTriggerFactory<R, F: FnOnce(&IPhoneTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PhoneTrigger, IPhoneTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PhoneTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PhoneTrigger;{8dcfe99b-d4c5-49f1-b7d3-82e87a0e9dde})");
}
unsafe impl ::windows_core::Interface for PhoneTrigger {
    type Vtable = IPhoneTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PhoneTrigger {
    const IID: ::windows_core::GUID = <IPhoneTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PhoneTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PhoneTrigger";
}
::windows_core::imp::interface_hierarchy!(PhoneTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for PhoneTrigger {}
unsafe impl ::core::marker::Send for PhoneTrigger {}
unsafe impl ::core::marker::Sync for PhoneTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct PushNotificationTrigger(::windows_core::IUnknown);
impl PushNotificationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<PushNotificationTrigger> {
        Self::IPushNotificationTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPushNotificationTriggerFactory<R, F: FnOnce(&IPushNotificationTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PushNotificationTrigger, IPushNotificationTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for PushNotificationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.PushNotificationTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for PushNotificationTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PushNotificationTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PushNotificationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.PushNotificationTrigger";
}
::windows_core::imp::interface_hierarchy!(PushNotificationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for PushNotificationTrigger {}
unsafe impl ::core::marker::Send for PushNotificationTrigger {}
unsafe impl ::core::marker::Sync for PushNotificationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RcsEndUserMessageAvailableTrigger(::windows_core::IUnknown);
impl RcsEndUserMessageAvailableTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RcsEndUserMessageAvailableTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RcsEndUserMessageAvailableTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger;{986d0d6a-b2f6-467f-a978-a44091c11a66})");
}
unsafe impl ::windows_core::Interface for RcsEndUserMessageAvailableTrigger {
    type Vtable = IRcsEndUserMessageAvailableTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RcsEndUserMessageAvailableTrigger {
    const IID: ::windows_core::GUID = <IRcsEndUserMessageAvailableTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RcsEndUserMessageAvailableTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RcsEndUserMessageAvailableTrigger";
}
::windows_core::imp::interface_hierarchy!(RcsEndUserMessageAvailableTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTrigger {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RfcommConnectionTrigger(::windows_core::IUnknown);
impl RfcommConnectionTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RfcommConnectionTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Background\"`"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn InboundConnection(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InboundConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Bluetooth_Background\"`"]
    #[cfg(feature = "Devices_Bluetooth_Background")]
    pub fn OutboundConnection(&self) -> ::windows_core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutboundConnection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowMultipleConnections(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllowMultipleConnections)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowMultipleConnections(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAllowMultipleConnections)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Networking_Sockets\"`"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn ProtectionLevel(&self) -> ::windows_core::Result<super::super::Networking::Sockets::SocketProtectionLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionLevel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Networking_Sockets\"`"]
    #[cfg(feature = "Networking_Sockets")]
    pub fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetProtectionLevel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "Required features: `\"Networking\"`"]
    #[cfg(feature = "Networking")]
    pub fn RemoteHostName(&self) -> ::windows_core::Result<super::super::Networking::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemoteHostName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Networking\"`"]
    #[cfg(feature = "Networking")]
    pub fn SetRemoteHostName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRemoteHostName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::windows_core::RuntimeType for RfcommConnectionTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.RfcommConnectionTrigger;{e8c4cae2-0b53-4464-9394-fd875654de64})");
}
unsafe impl ::windows_core::Interface for RfcommConnectionTrigger {
    type Vtable = IRfcommConnectionTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RfcommConnectionTrigger {
    const IID: ::windows_core::GUID = <IRfcommConnectionTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RfcommConnectionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.RfcommConnectionTrigger";
}
::windows_core::imp::interface_hierarchy!(RfcommConnectionTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for RfcommConnectionTrigger {}
unsafe impl ::core::marker::Send for RfcommConnectionTrigger {}
unsafe impl ::core::marker::Sync for RfcommConnectionTrigger {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl SecondaryAuthenticationFactorAuthenticationTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SecondaryAuthenticationFactorAuthenticationTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for SecondaryAuthenticationFactorAuthenticationTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger;{f237f327-5181-4f24-96a7-700a4e5fac62})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for SecondaryAuthenticationFactorAuthenticationTrigger {
    type Vtable = ISecondaryAuthenticationFactorAuthenticationTrigger_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for SecondaryAuthenticationFactorAuthenticationTrigger {
    const IID: ::windows_core::GUID = <ISecondaryAuthenticationFactorAuthenticationTrigger as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for SecondaryAuthenticationFactorAuthenticationTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SecondaryAuthenticationFactorAuthenticationTrigger";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(SecondaryAuthenticationFactorAuthenticationTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SecondaryAuthenticationFactorAuthenticationTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SensorDataThresholdTrigger(::windows_core::IUnknown);
impl SensorDataThresholdTrigger {
    #[doc = "Required features: `\"Devices_Sensors\"`"]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Create<P0>(threshold: P0) -> ::windows_core::Result<SensorDataThresholdTrigger>
    where
        P0: ::windows_core::TryIntoParam<super::super::Devices::Sensors::ISensorDataThreshold>,
    {
        Self::ISensorDataThresholdTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), threshold.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISensorDataThresholdTriggerFactory<R, F: FnOnce(&ISensorDataThresholdTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SensorDataThresholdTrigger, ISensorDataThresholdTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SensorDataThresholdTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SensorDataThresholdTrigger;{5bc0f372-d48b-4b7f-abec-15f9bacc12e2})");
}
unsafe impl ::windows_core::Interface for SensorDataThresholdTrigger {
    type Vtable = ISensorDataThresholdTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SensorDataThresholdTrigger {
    const IID: ::windows_core::GUID = <ISensorDataThresholdTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SensorDataThresholdTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SensorDataThresholdTrigger";
}
::windows_core::imp::interface_hierarchy!(SensorDataThresholdTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SensorDataThresholdTrigger {}
unsafe impl ::core::marker::Send for SensorDataThresholdTrigger {}
unsafe impl ::core::marker::Sync for SensorDataThresholdTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SmartCardTrigger(::windows_core::IUnknown);
impl SmartCardTrigger {
    #[doc = "Required features: `\"Devices_SmartCards\"`"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn TriggerType(&self) -> ::windows_core::Result<super::super::Devices::SmartCards::SmartCardTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_SmartCards\"`"]
    #[cfg(feature = "Devices_SmartCards")]
    pub fn Create(triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows_core::Result<SmartCardTrigger> {
        Self::ISmartCardTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), triggertype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmartCardTriggerFactory<R, F: FnOnce(&ISmartCardTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmartCardTrigger, ISmartCardTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SmartCardTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmartCardTrigger;{f53bc5ac-84ca-4972-8ce9-e58f97b37a50})");
}
unsafe impl ::windows_core::Interface for SmartCardTrigger {
    type Vtable = ISmartCardTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmartCardTrigger {
    const IID: ::windows_core::GUID = <ISmartCardTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmartCardTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmartCardTrigger";
}
::windows_core::imp::interface_hierarchy!(SmartCardTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SmartCardTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SmsMessageReceivedTrigger(::windows_core::IUnknown);
impl SmsMessageReceivedTrigger {
    #[doc = "Required features: `\"Devices_Sms\"`"]
    #[cfg(feature = "Devices_Sms")]
    pub fn Create<P0>(filterrules: P0) -> ::windows_core::Result<SmsMessageReceivedTrigger>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Sms::SmsFilterRules>,
    {
        Self::ISmsMessageReceivedTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), filterrules.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISmsMessageReceivedTriggerFactory<R, F: FnOnce(&ISmsMessageReceivedTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SmsMessageReceivedTrigger, ISmsMessageReceivedTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SmsMessageReceivedTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SmsMessageReceivedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for SmsMessageReceivedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SmsMessageReceivedTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SmsMessageReceivedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SmsMessageReceivedTrigger";
}
::windows_core::imp::interface_hierarchy!(SmsMessageReceivedTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SmsMessageReceivedTrigger {}
unsafe impl ::core::marker::Send for SmsMessageReceivedTrigger {}
unsafe impl ::core::marker::Sync for SmsMessageReceivedTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SocketActivityTrigger(::windows_core::IUnknown);
impl SocketActivityTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SocketActivityTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsWakeFromLowPowerSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ISocketActivityTrigger>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsWakeFromLowPowerSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for SocketActivityTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SocketActivityTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for SocketActivityTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SocketActivityTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SocketActivityTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SocketActivityTrigger";
}
::windows_core::imp::interface_hierarchy!(SocketActivityTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SocketActivityTrigger {}
unsafe impl ::core::marker::Send for SocketActivityTrigger {}
unsafe impl ::core::marker::Sync for SocketActivityTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StorageLibraryChangeTrackerTrigger(::windows_core::IUnknown);
impl StorageLibraryChangeTrackerTrigger {
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn Create<P0>(tracker: P0) -> ::windows_core::Result<StorageLibraryChangeTrackerTrigger>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageLibraryChangeTracker>,
    {
        Self::IStorageLibraryChangeTrackerTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), tracker.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryChangeTrackerTriggerFactory<R, F: FnOnce(&IStorageLibraryChangeTrackerTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StorageLibraryChangeTrackerTrigger, IStorageLibraryChangeTrackerTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for StorageLibraryChangeTrackerTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for StorageLibraryChangeTrackerTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageLibraryChangeTrackerTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryChangeTrackerTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryChangeTrackerTrigger";
}
::windows_core::imp::interface_hierarchy!(StorageLibraryChangeTrackerTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::core::marker::Send for StorageLibraryChangeTrackerTrigger {}
unsafe impl ::core::marker::Sync for StorageLibraryChangeTrackerTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct StorageLibraryContentChangedTrigger(::windows_core::IUnknown);
impl StorageLibraryContentChangedTrigger {
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn Create<P0>(storagelibrary: P0) -> ::windows_core::Result<StorageLibraryContentChangedTrigger>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageLibrary>,
    {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), storagelibrary.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn CreateFromLibraries<P0>(storagelibraries: P0) -> ::windows_core::Result<StorageLibraryContentChangedTrigger>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>,
    {
        Self::IStorageLibraryContentChangedTriggerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromLibraries)(::windows_core::Interface::as_raw(this), storagelibraries.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageLibraryContentChangedTriggerStatics<R, F: FnOnce(&IStorageLibraryContentChangedTriggerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StorageLibraryContentChangedTrigger, IStorageLibraryContentChangedTriggerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for StorageLibraryContentChangedTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger;{1637e0a7-829c-45bc-929b-a1e7ea78d89b})");
}
unsafe impl ::windows_core::Interface for StorageLibraryContentChangedTrigger {
    type Vtable = IStorageLibraryContentChangedTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageLibraryContentChangedTrigger {
    const IID: ::windows_core::GUID = <IStorageLibraryContentChangedTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageLibraryContentChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.StorageLibraryContentChangedTrigger";
}
::windows_core::imp::interface_hierarchy!(StorageLibraryContentChangedTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for StorageLibraryContentChangedTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SystemCondition(::windows_core::IUnknown);
impl SystemCondition {
    pub fn ConditionType(&self) -> ::windows_core::Result<SystemConditionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConditionType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(conditiontype: SystemConditionType) -> ::windows_core::Result<SystemCondition> {
        Self::ISystemConditionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), conditiontype, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemConditionFactory<R, F: FnOnce(&ISystemConditionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemCondition, ISystemConditionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SystemCondition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemCondition;{c15fb476-89c5-420b-abd3-fb3030472128})");
}
unsafe impl ::windows_core::Interface for SystemCondition {
    type Vtable = ISystemCondition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemCondition {
    const IID: ::windows_core::GUID = <ISystemCondition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemCondition";
}
::windows_core::imp::interface_hierarchy!(SystemCondition, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundCondition> for SystemCondition {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct SystemTrigger(::windows_core::IUnknown);
impl SystemTrigger {
    pub fn OneShot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OneShot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TriggerType(&self) -> ::windows_core::Result<SystemTriggerType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TriggerType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(triggertype: SystemTriggerType, oneshot: bool) -> ::windows_core::Result<SystemTrigger> {
        Self::ISystemTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), triggertype, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISystemTriggerFactory<R, F: FnOnce(&ISystemTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SystemTrigger, ISystemTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for SystemTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.SystemTrigger;{1d80c776-3748-4463-8d7e-276dc139ac1c})");
}
unsafe impl ::windows_core::Interface for SystemTrigger {
    type Vtable = ISystemTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SystemTrigger {
    const IID: ::windows_core::GUID = <ISystemTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SystemTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.SystemTrigger";
}
::windows_core::imp::interface_hierarchy!(SystemTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for SystemTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TetheringEntitlementCheckTrigger(::windows_core::IUnknown);
impl TetheringEntitlementCheckTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TetheringEntitlementCheckTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TetheringEntitlementCheckTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for TetheringEntitlementCheckTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TetheringEntitlementCheckTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TetheringEntitlementCheckTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TetheringEntitlementCheckTrigger";
}
::windows_core::imp::interface_hierarchy!(TetheringEntitlementCheckTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for TetheringEntitlementCheckTrigger {}
unsafe impl ::core::marker::Send for TetheringEntitlementCheckTrigger {}
unsafe impl ::core::marker::Sync for TetheringEntitlementCheckTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimeTrigger(::windows_core::IUnknown);
impl TimeTrigger {
    pub fn FreshnessTime(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FreshnessTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OneShot(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OneShot)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(freshnesstime: u32, oneshot: bool) -> ::windows_core::Result<TimeTrigger> {
        Self::ITimeTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), freshnesstime, oneshot, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimeTriggerFactory<R, F: FnOnce(&ITimeTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimeTrigger, ITimeTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TimeTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.TimeTrigger;{656e5556-0b2a-4377-ba70-3b45a935547f})");
}
unsafe impl ::windows_core::Interface for TimeTrigger {
    type Vtable = ITimeTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimeTrigger {
    const IID: ::windows_core::GUID = <ITimeTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimeTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.TimeTrigger";
}
::windows_core::imp::interface_hierarchy!(TimeTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for TimeTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ToastNotificationActionTrigger(::windows_core::IUnknown);
impl ToastNotificationActionTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ToastNotificationActionTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<ToastNotificationActionTrigger> {
        Self::IToastNotificationActionTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationActionTriggerFactory<R, F: FnOnce(&IToastNotificationActionTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ToastNotificationActionTrigger, IToastNotificationActionTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ToastNotificationActionTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationActionTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for ToastNotificationActionTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToastNotificationActionTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationActionTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationActionTrigger";
}
::windows_core::imp::interface_hierarchy!(ToastNotificationActionTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ToastNotificationActionTrigger {}
unsafe impl ::core::marker::Send for ToastNotificationActionTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationActionTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ToastNotificationHistoryChangedTrigger(::windows_core::IUnknown);
impl ToastNotificationHistoryChangedTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Create(applicationid: &::windows_core::HSTRING) -> ::windows_core::Result<ToastNotificationHistoryChangedTrigger> {
        Self::IToastNotificationHistoryChangedTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(applicationid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IToastNotificationHistoryChangedTriggerFactory<R, F: FnOnce(&IToastNotificationHistoryChangedTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ToastNotificationHistoryChangedTrigger, IToastNotificationHistoryChangedTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ToastNotificationHistoryChangedTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for ToastNotificationHistoryChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ToastNotificationHistoryChangedTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ToastNotificationHistoryChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.ToastNotificationHistoryChangedTrigger";
}
::windows_core::imp::interface_hierarchy!(ToastNotificationHistoryChangedTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::core::marker::Send for ToastNotificationHistoryChangedTrigger {}
unsafe impl ::core::marker::Sync for ToastNotificationHistoryChangedTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserNotificationChangedTrigger(::windows_core::IUnknown);
impl UserNotificationChangedTrigger {
    #[doc = "Required features: `\"UI_Notifications\"`"]
    #[cfg(feature = "UI_Notifications")]
    pub fn Create(notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows_core::Result<UserNotificationChangedTrigger> {
        Self::IUserNotificationChangedTriggerFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), notificationkinds, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserNotificationChangedTriggerFactory<R, F: FnOnce(&IUserNotificationChangedTriggerFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<UserNotificationChangedTrigger, IUserNotificationChangedTriggerFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for UserNotificationChangedTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.UserNotificationChangedTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for UserNotificationChangedTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UserNotificationChangedTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UserNotificationChangedTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.UserNotificationChangedTrigger";
}
::windows_core::imp::interface_hierarchy!(UserNotificationChangedTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for UserNotificationChangedTrigger {}
unsafe impl ::core::marker::Send for UserNotificationChangedTrigger {}
unsafe impl ::core::marker::Sync for UserNotificationChangedTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WiFiOnDemandHotspotConnectTrigger(::windows_core::IUnknown);
impl WiFiOnDemandHotspotConnectTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiOnDemandHotspotConnectTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WiFiOnDemandHotspotConnectTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.WiFiOnDemandHotspotConnectTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for WiFiOnDemandHotspotConnectTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiOnDemandHotspotConnectTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiOnDemandHotspotConnectTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotConnectTrigger";
}
::windows_core::imp::interface_hierarchy!(WiFiOnDemandHotspotConnectTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for WiFiOnDemandHotspotConnectTrigger {}
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotConnectTrigger {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotConnectTrigger {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WiFiOnDemandHotspotUpdateMetadataTrigger(::windows_core::IUnknown);
impl WiFiOnDemandHotspotUpdateMetadataTrigger {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WiFiOnDemandHotspotUpdateMetadataTrigger, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Background.WiFiOnDemandHotspotUpdateMetadataTrigger;{84b3a058-6027-4b87-9790-bdf3f757dbd7})");
}
unsafe impl ::windows_core::Interface for WiFiOnDemandHotspotUpdateMetadataTrigger {
    type Vtable = IBackgroundTrigger_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const IID: ::windows_core::GUID = <IBackgroundTrigger as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WiFiOnDemandHotspotUpdateMetadataTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.WiFiOnDemandHotspotUpdateMetadataTrigger";
}
::windows_core::imp::interface_hierarchy!(WiFiOnDemandHotspotUpdateMetadataTrigger, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBackgroundTrigger> for WiFiOnDemandHotspotUpdateMetadataTrigger {}
unsafe impl ::core::marker::Send for WiFiOnDemandHotspotUpdateMetadataTrigger {}
unsafe impl ::core::marker::Sync for WiFiOnDemandHotspotUpdateMetadataTrigger {}
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
impl ::windows_core::TypeKind for AlarmAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AlarmAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlarmAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AlarmAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.AlarmAccessStatus;i4)");
}
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
impl ::windows_core::TypeKind for ApplicationTriggerResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ApplicationTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationTriggerResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ApplicationTriggerResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.ApplicationTriggerResult;i4)");
}
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
impl ::windows_core::TypeKind for BackgroundAccessRequestKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BackgroundAccessRequestKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundAccessRequestKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackgroundAccessRequestKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessRequestKind;i4)");
}
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
impl ::windows_core::TypeKind for BackgroundAccessStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BackgroundAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundAccessStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackgroundAccessStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundAccessStatus;i4)");
}
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
impl ::windows_core::TypeKind for BackgroundTaskCancellationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BackgroundTaskCancellationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskCancellationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskCancellationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskCancellationReason;i4)");
}
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
impl ::windows_core::TypeKind for BackgroundTaskThrottleCounter {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BackgroundTaskThrottleCounter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundTaskThrottleCounter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackgroundTaskThrottleCounter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundTaskThrottleCounter;i4)");
}
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
impl ::windows_core::TypeKind for BackgroundWorkCostValue {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BackgroundWorkCostValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundWorkCostValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BackgroundWorkCostValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.BackgroundWorkCostValue;i4)");
}
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
impl ::windows_core::TypeKind for CustomSystemEventTriggerRecurrence {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CustomSystemEventTriggerRecurrence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CustomSystemEventTriggerRecurrence").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CustomSystemEventTriggerRecurrence {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.CustomSystemEventTriggerRecurrence;i4)");
}
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
impl ::windows_core::TypeKind for DeviceTriggerResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DeviceTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceTriggerResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DeviceTriggerResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.DeviceTriggerResult;i4)");
}
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
impl ::windows_core::TypeKind for LocationTriggerType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LocationTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LocationTriggerType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LocationTriggerType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.LocationTriggerType;i4)");
}
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
impl ::windows_core::TypeKind for MediaProcessingTriggerResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaProcessingTriggerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaProcessingTriggerResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaProcessingTriggerResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.MediaProcessingTriggerResult;i4)");
}
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
impl ::windows_core::TypeKind for SystemConditionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemConditionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemConditionType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemConditionType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemConditionType;i4)");
}
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
impl ::windows_core::TypeKind for SystemTriggerType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SystemTriggerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SystemTriggerType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SystemTriggerType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Background.SystemTriggerType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskCanceledEventHandler(pub ::windows_core::IUnknown);
impl BackgroundTaskCanceledEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCanceledEventHandlerBox::<F> { vtable: &BackgroundTaskCanceledEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0, reason: BackgroundTaskCancellationReason) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IBackgroundTaskInstance>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.try_into_param()?.abi(), reason).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCanceledEventHandlerBox<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskCanceledEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&IBackgroundTaskInstance>, BackgroundTaskCancellationReason) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskCanceledEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCanceledEventHandler_Vtbl = BackgroundTaskCanceledEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskCanceledEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, reason: BackgroundTaskCancellationReason) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), reason).into()
    }
}
unsafe impl ::windows_core::Interface for BackgroundTaskCanceledEventHandler {
    type Vtable = BackgroundTaskCanceledEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskCanceledEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6c4bac0_51f8_4c57_ac3f_156dd1680c4f);
}
impl ::windows_core::RuntimeType for BackgroundTaskCanceledEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{a6c4bac0-51f8-4c57-ac3f-156dd1680c4f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCanceledEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, reason: BackgroundTaskCancellationReason) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskCompletedEventHandler(pub ::windows_core::IUnknown);
impl BackgroundTaskCompletedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskCompletedEventHandlerBox::<F> { vtable: &BackgroundTaskCompletedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskRegistration>,
        P1: ::windows_core::IntoParam<BackgroundTaskCompletedEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskCompletedEventHandlerBox<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskCompletedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskCompletedEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskCompletedEventHandlerBox<F> {
    const VTABLE: BackgroundTaskCompletedEventHandler_Vtbl = BackgroundTaskCompletedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskCompletedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into()
    }
}
unsafe impl ::windows_core::Interface for BackgroundTaskCompletedEventHandler {
    type Vtable = BackgroundTaskCompletedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskCompletedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b38e929_a086_46a7_a678_439135822bcf);
}
impl ::windows_core::RuntimeType for BackgroundTaskCompletedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5b38e929-a086-46a7-a678-439135822bcf}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskCompletedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BackgroundTaskProgressEventHandler(pub ::windows_core::IUnknown);
impl BackgroundTaskProgressEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundTaskProgressEventHandlerBox::<F> { vtable: &BackgroundTaskProgressEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<BackgroundTaskRegistration>,
        P1: ::windows_core::IntoParam<BackgroundTaskProgressEventArgs>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct BackgroundTaskProgressEventHandlerBox<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundTaskProgressEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&BackgroundTaskRegistration>, ::core::option::Option<&BackgroundTaskProgressEventArgs>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> BackgroundTaskProgressEventHandlerBox<F> {
    const VTABLE: BackgroundTaskProgressEventHandler_Vtbl = BackgroundTaskProgressEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <BackgroundTaskProgressEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into()
    }
}
unsafe impl ::windows_core::Interface for BackgroundTaskProgressEventHandler {
    type Vtable = BackgroundTaskProgressEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BackgroundTaskProgressEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x46e0683c_8a88_4c99_804c_76897f6277a6);
}
impl ::windows_core::RuntimeType for BackgroundTaskProgressEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{46e0683c-8a88-4c99-804c-76897f6277a6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundTaskProgressEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
