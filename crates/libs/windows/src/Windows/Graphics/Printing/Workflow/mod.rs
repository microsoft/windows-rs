#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowBackgroundSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5b7913ba_0c5e_528a_7458_86a46cbddc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Submitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, submittedeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitted: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43e97342_1750_59c9_61fb_383748a20362);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRequiresUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0aac4ed_fd4b_5df5_4bb6_8d0d159ebe3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SourceAppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowConfiguration2 {
    type Vtable = IPrintWorkflowConfiguration2_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowConfiguration2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowConfiguration2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde350a50_a6d4_5be2_8b9a_09d3d39ea780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AbortPrintFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: PrintWorkflowJobAbortReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowForegroundSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc79b63d0_f8ec_4ceb_953a_c8876157dd33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub XpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdataavailableeventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XpsDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXpsDataAvailable: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowForegroundSetupRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbbe38247_9c1b_4dd3_9b2b_c80468d941b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4bd5e6d_034e_5e00_a616_f961a033dcc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobBackgroundSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobBackgroundSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc5ec6ad8_20c9_5d51_8507_2734b46f96c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobBackgroundSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub JobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub PdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlModificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlModificationRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobNotificationEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobNotificationEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0ae16fba_5398_5eba_b472_978650186a9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobNotificationEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobStartingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe3d99ba8_31ad_5e09_b0d7_601b97f161ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub SetSkipSystemRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff296129_60e2_51db_ba8c_e2ccddb516b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobUISession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowJobUISession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00c8736b_7637_5687_a302_0f664d2aac65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobUISession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub JobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobNotification: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobNotification: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowObjectModelSourceFileContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc36c8a6a_8a2a_419a_b3c3_2090e6bfab2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelSourceFileContentFactory {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowObjectModelSourceFileContentFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x93b1b903_f013_56d6_b708_99ac2ccb12ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelTargetPackage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowObjectModelTargetPackage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelTargetPackage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlConverter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x40604b62_0ae4_51f1_818f_731dc0b005ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlConverter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticket: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlConverter2 {
    type Vtable = IPrintWorkflowPdlConverter2_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlConverter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlConverter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x854ceec1_7837_5b93_b7af_57a6998c2f71);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlConverter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticket: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, hostbasedprocessingoperations: PdlConversionHostBasedProcessingOperations, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlDataAvailableEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd4ad6b50_1547_5991_a0ef_e2ee20211518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlModificationRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a339a61_2e13_5edd_a707_ceec61d7333b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateJobOnPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: *mut ::core::ffi::c_void, targetcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: *mut ::core::ffi::c_void, targetcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
    pub GetPdlConverter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversiontype: PrintWorkflowPdlConversionType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlModificationRequestedEventArgs2 {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlModificationRequestedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlModificationRequestedEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8d692147_6c62_5e31_a0e7_d49f92c111c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: *mut ::core::ffi::c_void, targetcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, operationattributes: *mut ::core::ffi::c_void, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: *mut ::core::ffi::c_void, targetcontenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, operationattributesbuffer: *mut ::core::ffi::c_void, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlSourceContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlSourceContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92f7fc41_32b8_56ab_845e_b1e68b3aedd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlSourceContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetContentFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetContentFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlTargetStream(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPdlTargetStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa742dfe5_1ee3_52a9_9f9f_2e2043180fd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlTargetStream_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPrinterJob(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowPrinterJob {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12009f94_0d14_5443_bc09_250311ce570b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPrinterJob_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub JobId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowPrinterJobStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetJobAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetJobAttributesAsBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub GetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    GetJobAttributes: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub SetJobAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Storage_Streams")))]
    SetJobAttributesFromBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub SetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    SetJobAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSourceContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowSourceContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a28c641_ceb1_4533_bb73_fbe63eefdb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSourceContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetJobPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetJobPrintTicketAsync: usize,
    pub GetSourceSpoolDataAsStreamContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSourceSpoolDataAsXpsObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSpoolStreamContent(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowSpoolStreamContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72e55ece_e406_4b74_84e1_3ff3fdcdaf70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSpoolStreamContent_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowStreamTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowStreamTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb23bba84_8565_488b_9839_1c9e7c7aa916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowStreamTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowSubmittedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3add0a41_3794_5569_5c87_40e8ff720f83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobprintticket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetTarget: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedOperation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowSubmittedOperation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e4e6216_3be1_5f0f_5c81_a5a2bd4eab0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedOperation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows_core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29da276c_0a73_5aed_4f3d_970d3251f057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TargetAsStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5739d868_9d86_4052_b0cb_f310becd59bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUIActivatedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowUIActivatedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc8a844d_09eb_5746_72a6_8dc8b5edbe9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUIActivatedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUILauncher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowUILauncher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64e9e22f_14cc_5828_96fb_39163fb6c378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUILauncher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsUILaunchEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAndCompleteUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAndCompleteUIAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPrintWorkflowXpsDataAvailableEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d11c331_54d1_434e_be0e_82c5fa58e5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSession(::windows_core::IUnknown);
impl PrintWorkflowBackgroundSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetupRequested<P0>(&self, setupeventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetupRequested)(::windows_core::Interface::as_raw(this), setupeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetupRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetupRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Submitted<P0>(&self, submittedeventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Submitted)(::windows_core::Interface::as_raw(this), submittedeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubmitted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSubmitted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowBackgroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowBackgroundSession {}
impl ::core::fmt::Debug for PrintWorkflowBackgroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowBackgroundSession").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowBackgroundSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession;{5b7913ba-0c5e-528a-7458-86a46cbddc45})");
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowBackgroundSession {
    const IID: ::windows_core::GUID = <IPrintWorkflowBackgroundSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowBackgroundSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowBackgroundSetupRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRequiresUI(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRequiresUI)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowBackgroundSetupRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowBackgroundSetupRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs;{43e97342-1750-59c9-61fb-383748a20362})");
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowBackgroundSetupRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowBackgroundSetupRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSetupRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowConfiguration(::windows_core::IUnknown);
impl PrintWorkflowConfiguration {
    pub fn SourceAppDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceAppDisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn JobTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JobTitle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SessionId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IPrintWorkflowConfiguration2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AbortPrintFlow)(::windows_core::Interface::as_raw(this), reason).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowConfiguration {}
impl ::core::fmt::Debug for PrintWorkflowConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowConfiguration").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration;{d0aac4ed-fd4b-5df5-4bb6-8d0d159ebe3f})");
}
impl ::core::clone::Clone for PrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowConfiguration {
    const IID: ::windows_core::GUID = <IPrintWorkflowConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowConfiguration {}
unsafe impl ::core::marker::Sync for PrintWorkflowConfiguration {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowForegroundSession(::windows_core::IUnknown);
impl PrintWorkflowForegroundSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetupRequested<P0>(&self, setupeventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetupRequested)(::windows_core::Interface::as_raw(this), setupeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetupRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSetupRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn XpsDataAvailable<P0>(&self, xpsdataavailableeventhandler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XpsDataAvailable)(::windows_core::Interface::as_raw(this), xpsdataavailableeventhandler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXpsDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveXpsDataAvailable)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowForegroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowForegroundSession {}
impl ::core::fmt::Debug for PrintWorkflowForegroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowForegroundSession").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowForegroundSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession;{c79b63d0-f8ec-4ceb-953a-c8876157dd33})");
}
impl ::core::clone::Clone for PrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowForegroundSession {
    const IID: ::windows_core::GUID = <IPrintWorkflowForegroundSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowForegroundSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowForegroundSetupRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetUserPrintTicketAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowForegroundSetupRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowForegroundSetupRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowForegroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs;{bbe38247-9c1b-4dd3-9b2b-c80468d941b3})");
}
impl ::core::clone::Clone for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowForegroundSetupRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowForegroundSetupRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowForegroundSetupRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSetupRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobActivatedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows_core::Result<PrintWorkflowJobUISession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Session)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobActivatedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs;{d4bd5e6d-034e-5e00-a616-f961a033dcc8})");
}
impl ::core::clone::Clone for PrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintWorkflowJobActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowJobActivatedEventArgs {}
unsafe impl ::core::marker::Send for PrintWorkflowJobActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobActivatedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobBackgroundSession(::windows_core::IUnknown);
impl PrintWorkflowJobBackgroundSession {
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JobStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JobStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveJobStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveJobStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PdlModificationRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PdlModificationRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePdlModificationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePdlModificationRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobBackgroundSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobBackgroundSession {}
impl ::core::fmt::Debug for PrintWorkflowJobBackgroundSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobBackgroundSession").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobBackgroundSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession;{c5ec6ad8-20c9-5d51-8507-2734b46f96c5})");
}
impl ::core::clone::Clone for PrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobBackgroundSession {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobBackgroundSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobBackgroundSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowJobBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobBackgroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobNotificationEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobNotificationEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobNotificationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobNotificationEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobNotificationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobNotificationEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobNotificationEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs;{0ae16fba-5398-5eba-b472-978650186a9a})");
}
impl ::core::clone::Clone for PrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobNotificationEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobNotificationEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobNotificationEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowJobNotificationEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobNotificationEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobStartingEventArgs(::windows_core::IUnknown);
impl PrintWorkflowJobStartingEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Printer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSkipSystemRendering(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSkipSystemRendering)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobStartingEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowJobStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobStartingEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs;{e3d99ba8-31ad-5e09-b0d7-601b97f161ad})");
}
impl ::core::clone::Clone for PrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobStartingEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobStartingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowJobStartingEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobStartingEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobTriggerDetails(::windows_core::IUnknown);
impl PrintWorkflowJobTriggerDetails {
    pub fn PrintWorkflowJobSession(&self) -> ::windows_core::Result<PrintWorkflowJobBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowJobSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobTriggerDetails {}
impl ::core::fmt::Debug for PrintWorkflowJobTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails;{ff296129-60e2-51db-ba8c-e2ccddb516b9})");
}
impl ::core::clone::Clone for PrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobTriggerDetails {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowJobTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobTriggerDetails {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobUISession(::windows_core::IUnknown);
impl PrintWorkflowJobUISession {
    pub fn Status(&self) -> ::windows_core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PdlDataAvailable<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PdlDataAvailable)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePdlDataAvailable(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePdlDataAvailable)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JobNotification<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JobNotification)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveJobNotification(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveJobNotification)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowJobUISession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowJobUISession {}
impl ::core::fmt::Debug for PrintWorkflowJobUISession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobUISession").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobUISession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession;{00c8736b-7637-5687-a302-0f664d2aac65})");
}
impl ::core::clone::Clone for PrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowJobUISession {
    const IID: ::windows_core::GUID = <IPrintWorkflowJobUISession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowJobUISession, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowJobUISession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobUISession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowObjectModelSourceFileContent(::windows_core::IUnknown);
impl PrintWorkflowObjectModelSourceFileContent {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<P0>(xpsstream: P0) -> ::windows_core::Result<PrintWorkflowObjectModelSourceFileContent>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
    {
        Self::IPrintWorkflowObjectModelSourceFileContentFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), xpsstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintWorkflowObjectModelSourceFileContentFactory<R, F: FnOnce(&IPrintWorkflowObjectModelSourceFileContentFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<PrintWorkflowObjectModelSourceFileContent, IPrintWorkflowObjectModelSourceFileContentFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowObjectModelSourceFileContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowObjectModelSourceFileContent {}
impl ::core::fmt::Debug for PrintWorkflowObjectModelSourceFileContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowObjectModelSourceFileContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowObjectModelSourceFileContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent;{c36c8a6a-8a2a-419a-b3c3-2090e6bfab2f})");
}
impl ::core::clone::Clone for PrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowObjectModelSourceFileContent {
    const IID: ::windows_core::GUID = <IPrintWorkflowObjectModelSourceFileContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowObjectModelSourceFileContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowObjectModelSourceFileContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelSourceFileContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelSourceFileContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowObjectModelTargetPackage(::windows_core::IUnknown);
impl PrintWorkflowObjectModelTargetPackage {}
impl ::core::cmp::PartialEq for PrintWorkflowObjectModelTargetPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowObjectModelTargetPackage {}
impl ::core::fmt::Debug for PrintWorkflowObjectModelTargetPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowObjectModelTargetPackage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowObjectModelTargetPackage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage;{7d96bc74-9b54-4ca1-ad3a-979c3d44ddac})");
}
impl ::core::clone::Clone for PrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowObjectModelTargetPackage {
    const IID: ::windows_core::GUID = <IPrintWorkflowObjectModelTargetPackage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowObjectModelTargetPackage {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowObjectModelTargetPackage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelTargetPackage {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelTargetPackage {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlConverter(::windows_core::IUnknown);
impl PrintWorkflowPdlConverter {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub fn ConvertPdlAsync<P0, P1, P2>(&self, printticket: P0, inputstream: P1, outputstream: P2) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::PrintTicket::WorkflowPrintTicket>,
        P1: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
        P2: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertPdlAsync)(::windows_core::Interface::as_raw(this), printticket.into_param().abi(), inputstream.try_into_param()?.abi(), outputstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub fn ConvertPdlAsync2<P0, P1, P2>(&self, printticket: P0, inputstream: P1, outputstream: P2, hostbasedprocessingoperations: PdlConversionHostBasedProcessingOperations) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::PrintTicket::WorkflowPrintTicket>,
        P1: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
        P2: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IOutputStream>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintWorkflowPdlConverter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertPdlAsync)(::windows_core::Interface::as_raw(this), printticket.into_param().abi(), inputstream.try_into_param()?.abi(), outputstream.try_into_param()?.abi(), hostbasedprocessingoperations, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlConverter {}
impl ::core::fmt::Debug for PrintWorkflowPdlConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConverter").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlConverter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter;{40604b62-0ae4-51f1-818f-731dc0b005ab})");
}
impl ::core::clone::Clone for PrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPdlConverter {
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlConverter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPdlConverter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPdlConverter {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlConverter {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(::windows_core::IUnknown);
impl PrintWorkflowPdlDataAvailableEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceContent(&self) -> ::windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceContent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlDataAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlDataAvailableEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowPdlDataAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlDataAvailableEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlDataAvailableEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs;{d4ad6b50-1547-5991-a0ef-e2ee20211518})");
}
impl ::core::clone::Clone for PrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPdlDataAvailableEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlDataAvailableEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPdlDataAvailableEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPdlDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlDataAvailableEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowPdlModificationRequestedEventArgs {
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrinterJob(&self) -> ::windows_core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrinterJob)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceContent(&self) -> ::windows_core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceContent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UILauncher(&self) -> ::windows_core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UILauncher)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateJobOnPrinter(&self, targetcontenttype: &::windows_core::HSTRING) -> ::windows_core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinter)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(targetcontenttype), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn CreateJobOnPrinterWithAttributes<P0>(&self, jobattributes: P0, targetcontenttype: &::windows_core::HSTRING) -> ::windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(::windows_core::Interface::as_raw(this), jobattributes.try_into_param()?.abi(), ::core::mem::transmute_copy(targetcontenttype), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateJobOnPrinterWithAttributesBuffer<P0>(&self, jobattributesbuffer: P0, targetcontenttype: &::windows_core::HSTRING) -> ::windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(::windows_core::Interface::as_raw(this), jobattributesbuffer.try_into_param()?.abi(), ::core::mem::transmute_copy(targetcontenttype), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows_core::Result<PrintWorkflowPdlConverter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPdlConverter)(::windows_core::Interface::as_raw(this), conversiontype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn CreateJobOnPrinterWithAttributes2<P0, P1>(&self, jobattributes: P0, targetcontenttype: &::windows_core::HSTRING, operationattributes: P1, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy) -> ::windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintWorkflowPdlModificationRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(::windows_core::Interface::as_raw(this), jobattributes.try_into_param()?.abi(), ::core::mem::transmute_copy(targetcontenttype), operationattributes.try_into_param()?.abi(), jobattributesmergepolicy, operationattributesmergepolicy, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateJobOnPrinterWithAttributesBuffer2<P0, P1>(&self, jobattributesbuffer: P0, targetcontenttype: &::windows_core::HSTRING, operationattributesbuffer: P1, jobattributesmergepolicy: PrintWorkflowAttributesMergePolicy, operationattributesmergepolicy: PrintWorkflowAttributesMergePolicy) -> ::windows_core::Result<PrintWorkflowPdlTargetStream>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
        P1: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<IPrintWorkflowPdlModificationRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(::windows_core::Interface::as_raw(this), jobattributesbuffer.try_into_param()?.abi(), ::core::mem::transmute_copy(targetcontenttype), operationattributesbuffer.try_into_param()?.abi(), jobattributesmergepolicy, operationattributesmergepolicy, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlModificationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlModificationRequestedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowPdlModificationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlModificationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlModificationRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs;{1a339a61-2e13-5edd-a707-ceec61d7333b})");
}
impl ::core::clone::Clone for PrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPdlModificationRequestedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlModificationRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPdlModificationRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPdlModificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlModificationRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlSourceContent(::windows_core::IUnknown);
impl PrintWorkflowPdlSourceContent {
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetContentFileAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetContentFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlSourceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlSourceContent {}
impl ::core::fmt::Debug for PrintWorkflowPdlSourceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlSourceContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlSourceContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent;{92f7fc41-32b8-56ab-845e-b1e68b3aedd5})");
}
impl ::core::clone::Clone for PrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPdlSourceContent {
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlSourceContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPdlSourceContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPdlSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlSourceContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlTargetStream(::windows_core::IUnknown);
impl PrintWorkflowPdlTargetStream {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CompleteStreamSubmission)(::windows_core::Interface::as_raw(this), status).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPdlTargetStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPdlTargetStream {}
impl ::core::fmt::Debug for PrintWorkflowPdlTargetStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlTargetStream").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlTargetStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream;{a742dfe5-1ee3-52a9-9f9f-2e2043180fd1})");
}
impl ::core::clone::Clone for PrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPdlTargetStream {
    const IID: ::windows_core::GUID = <IPrintWorkflowPdlTargetStream as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPdlTargetStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPdlTargetStream {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlTargetStream {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPrinterJob(::windows_core::IUnknown);
impl PrintWorkflowPrinterJob {
    pub fn JobId(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JobId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows_core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Printer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetJobStatus(&self) -> ::windows_core::Result<PrintWorkflowPrinterJobStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetJobStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicket(&self) -> ::windows_core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetJobPrintTicket)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetJobAttributesAsBuffer<P0>(&self, attributenames: P0) -> ::windows_core::Result<super::super::super::Storage::Streams::IBuffer>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetJobAttributesAsBuffer)(::windows_core::Interface::as_raw(this), attributenames.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn GetJobAttributes<P0>(&self, attributenames: P0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetJobAttributes)(::windows_core::Interface::as_raw(this), attributenames.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub fn SetJobAttributesFromBuffer<P0>(&self, jobattributesbuffer: P0) -> ::windows_core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetJobAttributesFromBuffer)(::windows_core::Interface::as_raw(this), jobattributesbuffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn SetJobAttributes<P0>(&self, jobattributes: P0) -> ::windows_core::Result<super::super::super::Devices::Printers::IppSetAttributesResult>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetJobAttributes)(::windows_core::Interface::as_raw(this), jobattributes.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowPrinterJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowPrinterJob {}
impl ::core::fmt::Debug for PrintWorkflowPrinterJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJob").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPrinterJob {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob;{12009f94-0d14-5443-bc09-250311ce570b})");
}
impl ::core::clone::Clone for PrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowPrinterJob {
    const IID: ::windows_core::GUID = <IPrintWorkflowPrinterJob as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowPrinterJob, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowPrinterJob {}
unsafe impl ::core::marker::Sync for PrintWorkflowPrinterJob {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSourceContent(::windows_core::IUnknown);
impl PrintWorkflowSourceContent {
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetJobPrintTicketAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetJobPrintTicketAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSourceSpoolDataAsStreamContent(&self) -> ::windows_core::Result<PrintWorkflowSpoolStreamContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSourceSpoolDataAsStreamContent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSourceSpoolDataAsXpsObjectModel(&self) -> ::windows_core::Result<PrintWorkflowObjectModelSourceFileContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSourceSpoolDataAsXpsObjectModel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSourceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSourceContent {}
impl ::core::fmt::Debug for PrintWorkflowSourceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSourceContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSourceContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent;{1a28c641-ceb1-4533-bb73-fbe63eefdb18})");
}
impl ::core::clone::Clone for PrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowSourceContent {
    const IID: ::windows_core::GUID = <IPrintWorkflowSourceContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowSourceContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSourceContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSpoolStreamContent(::windows_core::IUnknown);
impl PrintWorkflowSpoolStreamContent {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSpoolStreamContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSpoolStreamContent {}
impl ::core::fmt::Debug for PrintWorkflowSpoolStreamContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSpoolStreamContent").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSpoolStreamContent {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent;{72e55ece-e406-4b74-84e1-3ff3fdcdaf70})");
}
impl ::core::clone::Clone for PrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowSpoolStreamContent {
    const IID: ::windows_core::GUID = <IPrintWorkflowSpoolStreamContent as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowSpoolStreamContent, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowSpoolStreamContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSpoolStreamContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowStreamTarget(::windows_core::IUnknown);
impl PrintWorkflowStreamTarget {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows_core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowStreamTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowStreamTarget {}
impl ::core::fmt::Debug for PrintWorkflowStreamTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowStreamTarget").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowStreamTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget;{b23bba84-8565-488b-9839-1c9e7c7aa916})");
}
impl ::core::clone::Clone for PrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowStreamTarget {
    const IID: ::windows_core::GUID = <IPrintWorkflowStreamTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowStreamTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowStreamTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowStreamTarget {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSubmittedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowSubmittedEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetTarget<P0>(&self, jobprintticket: P0) -> ::windows_core::Result<PrintWorkflowTarget>
    where
        P0: ::windows_core::IntoParam<super::PrintTicket::WorkflowPrintTicket>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTarget)(::windows_core::Interface::as_raw(this), jobprintticket.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSubmittedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSubmittedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowSubmittedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSubmittedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs;{3add0a41-3794-5569-5c87-40e8ff720f83})");
}
impl ::core::clone::Clone for PrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowSubmittedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowSubmittedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowSubmittedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSubmittedOperation(::windows_core::IUnknown);
impl PrintWorkflowSubmittedOperation {
    pub fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this), status).ok() }
    }
    pub fn Configuration(&self) -> ::windows_core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Configuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XpsContent(&self) -> ::windows_core::Result<PrintWorkflowSourceContent> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).XpsContent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowSubmittedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowSubmittedOperation {}
impl ::core::fmt::Debug for PrintWorkflowSubmittedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedOperation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSubmittedOperation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation;{2e4e6216-3be1-5f0f-5c81-a5a2bd4eab0e})");
}
impl ::core::clone::Clone for PrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowSubmittedOperation {
    const IID: ::windows_core::GUID = <IPrintWorkflowSubmittedOperation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowSubmittedOperation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedOperation {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedOperation {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowTarget(::windows_core::IUnknown);
impl PrintWorkflowTarget {
    pub fn TargetAsStream(&self) -> ::windows_core::Result<PrintWorkflowStreamTarget> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetAsStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TargetAsXpsObjectModelPackage(&self) -> ::windows_core::Result<PrintWorkflowObjectModelTargetPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetAsXpsObjectModelPackage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowTarget {}
impl ::core::fmt::Debug for PrintWorkflowTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowTarget").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTarget;{29da276c-0a73-5aed-4f3d-970d3251f057})");
}
impl ::core::clone::Clone for PrintWorkflowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowTarget {
    const IID: ::windows_core::GUID = <IPrintWorkflowTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTarget";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowTarget {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowTriggerDetails(::windows_core::IUnknown);
impl PrintWorkflowTriggerDetails {
    pub fn PrintWorkflowSession(&self) -> ::windows_core::Result<PrintWorkflowBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowTriggerDetails {}
impl ::core::fmt::Debug for PrintWorkflowTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails;{5739d868-9d86-4052-b0cb-f310becd59bb})");
}
impl ::core::clone::Clone for PrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowTriggerDetails {
    const IID: ::windows_core::GUID = <IPrintWorkflowTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowTriggerDetails {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowUIActivatedEventArgs(::windows_core::IUnknown);
impl PrintWorkflowUIActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PreviousExecutionState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows_core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SplashScreen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows_core::Result<super::super::super::System::User> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).User)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PrintWorkflowSession(&self) -> ::windows_core::Result<PrintWorkflowForegroundSession> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrintWorkflowSession)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowUIActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowUIActivatedEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowUIActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUIActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowUIActivatedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs;{bc8a844d-09eb-5746-72a6-8dc8b5edbe9b})");
}
impl ::core::clone::Clone for PrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowUIActivatedEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowUIActivatedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowUIActivatedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintWorkflowUIActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows_core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowUIActivatedEventArgs {}
unsafe impl ::core::marker::Send for PrintWorkflowUIActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowUIActivatedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowUILauncher(::windows_core::IUnknown);
impl PrintWorkflowUILauncher {
    pub fn IsUILaunchEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUILaunchEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAndCompleteUIAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LaunchAndCompleteUIAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowUILauncher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowUILauncher {}
impl ::core::fmt::Debug for PrintWorkflowUILauncher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUILauncher").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowUILauncher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher;{64e9e22f-14cc-5828-96fb-39163fb6c378})");
}
impl ::core::clone::Clone for PrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowUILauncher {
    const IID: ::windows_core::GUID = <IPrintWorkflowUILauncher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowUILauncher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowUILauncher {}
unsafe impl ::core::marker::Sync for PrintWorkflowUILauncher {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(::windows_core::IUnknown);
impl PrintWorkflowXpsDataAvailableEventArgs {
    pub fn Operation(&self) -> ::windows_core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Operation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintWorkflowXpsDataAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintWorkflowXpsDataAvailableEventArgs {}
impl ::core::fmt::Debug for PrintWorkflowXpsDataAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowXpsDataAvailableEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowXpsDataAvailableEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs;{4d11c331-54d1-434e-be0e-82c5fa58e5b2})");
}
impl ::core::clone::Clone for PrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PrintWorkflowXpsDataAvailableEventArgs {
    const IID: ::windows_core::GUID = <IPrintWorkflowXpsDataAvailableEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs";
}
::windows_core::imp::interface_hierarchy!(PrintWorkflowXpsDataAvailableEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PrintWorkflowXpsDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowXpsDataAvailableEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PdlConversionHostBasedProcessingOperations(pub u32);
impl PdlConversionHostBasedProcessingOperations {
    pub const None: Self = Self(0u32);
    pub const PageRotation: Self = Self(1u32);
    pub const PageOrdering: Self = Self(2u32);
    pub const Copies: Self = Self(4u32);
    pub const BlankPageInsertion: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PdlConversionHostBasedProcessingOperations {}
impl ::core::clone::Clone for PdlConversionHostBasedProcessingOperations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PdlConversionHostBasedProcessingOperations {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PdlConversionHostBasedProcessingOperations {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PdlConversionHostBasedProcessingOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PdlConversionHostBasedProcessingOperations").field(&self.0).finish()
    }
}
impl PdlConversionHostBasedProcessingOperations {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PdlConversionHostBasedProcessingOperations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PdlConversionHostBasedProcessingOperations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PdlConversionHostBasedProcessingOperations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for PdlConversionHostBasedProcessingOperations {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PdlConversionHostBasedProcessingOperations;u4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowAttributesMergePolicy(pub i32);
impl PrintWorkflowAttributesMergePolicy {
    pub const MergePreferPrintTicketOnConflict: Self = Self(0i32);
    pub const MergePreferPsaOnConflict: Self = Self(1i32);
    pub const DoNotMergeWithPrintTicket: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowAttributesMergePolicy {}
impl ::core::clone::Clone for PrintWorkflowAttributesMergePolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowAttributesMergePolicy {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowAttributesMergePolicy {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowAttributesMergePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowAttributesMergePolicy").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowAttributesMergePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowAttributesMergePolicy;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowJobAbortReason(pub i32);
impl PrintWorkflowJobAbortReason {
    pub const JobFailed: Self = Self(0i32);
    pub const UserCanceled: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintWorkflowJobAbortReason {}
impl ::core::clone::Clone for PrintWorkflowJobAbortReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowJobAbortReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowJobAbortReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowJobAbortReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobAbortReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowJobAbortReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowJobAbortReason;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowPdlConversionType(pub i32);
impl PrintWorkflowPdlConversionType {
    pub const XpsToPdf: Self = Self(0i32);
    pub const XpsToPwgr: Self = Self(1i32);
    pub const XpsToPclm: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowPdlConversionType {}
impl ::core::clone::Clone for PrintWorkflowPdlConversionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowPdlConversionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowPdlConversionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowPdlConversionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConversionType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPdlConversionType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConversionType;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowPrinterJobStatus(pub i32);
impl PrintWorkflowPrinterJobStatus {
    pub const Error: Self = Self(0i32);
    pub const Aborted: Self = Self(1i32);
    pub const InProgress: Self = Self(2i32);
    pub const Completed: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowPrinterJobStatus {}
impl ::core::clone::Clone for PrintWorkflowPrinterJobStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowPrinterJobStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowPrinterJobStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowPrinterJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJobStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowPrinterJobStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJobStatus;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowSessionStatus(pub i32);
impl PrintWorkflowSessionStatus {
    pub const Started: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Aborted: Self = Self(2i32);
    pub const Closed: Self = Self(3i32);
    pub const PdlDataAvailableForModification: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintWorkflowSessionStatus {}
impl ::core::clone::Clone for PrintWorkflowSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowSessionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowSessionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSessionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSessionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSessionStatus;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowSubmittedStatus(pub i32);
impl PrintWorkflowSubmittedStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintWorkflowSubmittedStatus {}
impl ::core::clone::Clone for PrintWorkflowSubmittedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowSubmittedStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowSubmittedStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowSubmittedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowSubmittedStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedStatus;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintWorkflowUICompletionStatus(pub i32);
impl PrintWorkflowUICompletionStatus {
    pub const Completed: Self = Self(0i32);
    pub const LaunchFailed: Self = Self(1i32);
    pub const JobFailed: Self = Self(2i32);
    pub const UserCanceled: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintWorkflowUICompletionStatus {}
impl ::core::clone::Clone for PrintWorkflowUICompletionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintWorkflowUICompletionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PrintWorkflowUICompletionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PrintWorkflowUICompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUICompletionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PrintWorkflowUICompletionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowUICompletionStatus;i4)");
}
