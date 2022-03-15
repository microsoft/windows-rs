#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b7913ba_0c5e_528a_7458_86a46cbddc45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub Submitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, submittedeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitted: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43e97342_1750_59c9_61fb_383748a20362);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRequiresUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0aac4ed_fd4b_5df5_4bb6_8d0d159ebe3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SourceAppDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub JobTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowConfiguration2 {
    type Vtable = IPrintWorkflowConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde350a50_a6d4_5be2_8b9a_09d3d39ea780);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AbortPrintFlow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: PrintWorkflowJobAbortReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc79b63d0_f8ec_4ceb_953a_c8876157dd33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setupeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetupRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetupRequested: usize,
    #[cfg(feature = "Foundation")]
    pub XpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdataavailableeventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    XpsDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveXpsDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveXpsDataAvailable: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbe38247_9c1b_4dd3_9b2b_c80468d941b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetUserPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetUserPrintTicketAsync: usize,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4bd5e6d_034e_5e00_a616_f961a033dcc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobActivatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobBackgroundSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5ec6ad8_20c9_5d51_8507_2734b46f96c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobBackgroundSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub JobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobStarting: usize,
    #[cfg(feature = "Foundation")]
    pub PdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlModificationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlModificationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlModificationRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobNotificationEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ae16fba_5398_5eba_b472_978650186a9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobNotificationEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobStartingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d99ba8_31ad_5e09_b0d7_601b97f161ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobStartingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub SetSkipSystemRendering: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff296129_60e2_51db_ba8c_e2ccddb516b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrintWorkflowJobSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowJobUISession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00c8736b_7637_5687_a302_0f664d2aac65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowJobUISession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowSessionStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePdlDataAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePdlDataAvailable: usize,
    #[cfg(feature = "Foundation")]
    pub JobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    JobNotification: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveJobNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveJobNotification: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc36c8a6a_8a2a_419a_b3c3_2090e6bfab2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowObjectModelSourceFileContentFactory {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93b1b903_f013_56d6_b708_99ac2ccb12ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelTargetPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d96bc74_9b54_4ca1_ad3a_979c3d44ddac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelTargetPackage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlConverter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40604b62_0ae4_51f1_818f_731dc0b005ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlConverter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub ConvertPdlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticket: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams")))]
    ConvertPdlAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4ad6b50_1547_5991_a0ef_e2ee20211518);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlDataAvailableEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a339a61_2e13_5edd_a707_ceec61d7333b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PrinterJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SourceContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UILauncher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateJobOnPrinter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub CreateJobOnPrinterWithAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: ::windows::core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    CreateJobOnPrinterWithAttributes: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateJobOnPrinterWithAttributesBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows::core::RawPtr, targetcontenttype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateJobOnPrinterWithAttributesBuffer: usize,
    pub GetPdlConverter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversiontype: PrintWorkflowPdlConversionType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlSourceContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92f7fc41_32b8_56ab_845e_b1e68b3aedd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlSourceContent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetContentFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetContentFileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPdlTargetStream(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa742dfe5_1ee3_52a9_9f9f_2e2043180fd1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPdlTargetStream_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
    pub CompleteStreamSubmission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowPrinterJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12009f94_0d14_5443_bc09_250311ce570b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowPrinterJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub JobId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    pub GetJobStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintWorkflowPrinterJobStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetJobPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetJobPrintTicket: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub GetJobAttributesAsBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    GetJobAttributesAsBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub GetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributenames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    GetJobAttributes: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub SetJobAttributesFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributesbuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Storage_Streams")))]
    SetJobAttributesFromBuffer: usize,
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub SetJobAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobattributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Printers", feature = "Foundation_Collections")))]
    SetJobAttributes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSourceContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a28c641_ceb1_4533_bb73_fbe63eefdb18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSourceContent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub GetJobPrintTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket")))]
    GetJobPrintTicketAsync: usize,
    pub GetSourceSpoolDataAsStreamContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSourceSpoolDataAsXpsObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSpoolStreamContent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72e55ece_e406_4b74_84e1_3ff3fdcdaf70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSpoolStreamContent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetInputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowStreamTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb23bba84_8565_488b_9839_1c9e7c7aa916);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowStreamTarget_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetOutputStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3add0a41_3794_5569_5c87_40e8ff720f83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub GetTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, jobprintticket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    GetTarget: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowSubmittedOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e4e6216_3be1_5f0f_5c81_a5a2bd4eab0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowSubmittedOperation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: PrintWorkflowSubmittedStatus) -> ::windows::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub XpsContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29da276c_0a73_5aed_4f3d_970d3251f057);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTarget_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TargetAsStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub TargetAsXpsObjectModelPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5739d868_9d86_4052_b0cb_f310becd59bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUIActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc8a844d_09eb_5746_72a6_8dc8b5edbe9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUIActivatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PrintWorkflowSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowUILauncher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64e9e22f_14cc_5828_96fb_39163fb6c378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowUILauncher_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsUILaunchEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LaunchAndCompleteUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LaunchAndCompleteUIAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d11c331_54d1_434e_be0e_82c5fa58e5b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsDataAvailableEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Operation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSession(::windows::core::IUnknown);
impl PrintWorkflowBackgroundSession {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetupRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>>>(&self, setupeventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetupRequested)(::core::mem::transmute_copy(this), setupeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetupRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSetupRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Submitted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>>>(&self, submittedeventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Submitted)(::core::mem::transmute_copy(this), submittedeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubmitted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSubmitted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: PrintWorkflowSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowBackgroundSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession;{5b7913ba-0c5e-528a-7458-86a46cbddc45})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowBackgroundSession {
    type Vtable = IPrintWorkflowBackgroundSession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowBackgroundSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSession";
}
impl ::core::convert::From<PrintWorkflowBackgroundSession> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSession> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowBackgroundSession> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSession> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowBackgroundSetupRequestedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowBackgroundSetupRequestedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUserPrintTicketAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SetRequiresUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRequiresUI)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs;{43e97342-1750-59c9-61fb-383748a20362})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowBackgroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowBackgroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowBackgroundSetupRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowBackgroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowBackgroundSetupRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowBackgroundSetupRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowBackgroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowBackgroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowBackgroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowBackgroundSetupRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowConfiguration(::windows::core::IUnknown);
impl PrintWorkflowConfiguration {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SourceAppDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceAppDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn JobTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobTitle)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn AbortPrintFlow(&self, reason: PrintWorkflowJobAbortReason) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintWorkflowConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AbortPrintFlow)(::core::mem::transmute_copy(this), reason).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration;{d0aac4ed-fd4b-5df5-4bb6-8d0d159ebe3f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowConfiguration {
    type Vtable = IPrintWorkflowConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowConfiguration {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowConfiguration";
}
impl ::core::convert::From<PrintWorkflowConfiguration> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowConfiguration> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowConfiguration> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowConfiguration> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowConfiguration {}
unsafe impl ::core::marker::Sync for PrintWorkflowConfiguration {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowForegroundSession(::windows::core::IUnknown);
impl PrintWorkflowForegroundSession {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetupRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>>>(&self, setupeventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetupRequested)(::core::mem::transmute_copy(this), setupeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetupRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSetupRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn XpsDataAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>>>(&self, xpsdataavailableeventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XpsDataAvailable)(::core::mem::transmute_copy(this), xpsdataavailableeventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveXpsDataAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveXpsDataAvailable)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: PrintWorkflowSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowForegroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowForegroundSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession;{c79b63d0-f8ec-4ceb-953a-c8876157dd33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowForegroundSession {
    type Vtable = IPrintWorkflowForegroundSession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowForegroundSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowForegroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSession";
}
impl ::core::convert::From<PrintWorkflowForegroundSession> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSession> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowForegroundSession> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowForegroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSession> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowForegroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowForegroundSetupRequestedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowForegroundSetupRequestedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetUserPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetUserPrintTicketAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowForegroundSetupRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs;{bbe38247-9c1b-4dd3-9b2b-c80468d941b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowForegroundSetupRequestedEventArgs {
    type Vtable = IPrintWorkflowForegroundSetupRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowForegroundSetupRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowForegroundSetupRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowForegroundSetupRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowForegroundSetupRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowForegroundSetupRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowForegroundSetupRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowForegroundSetupRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowForegroundSetupRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowJobAbortReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowJobAbortReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowJobAbortReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobAbortReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowJobAbortReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobActivatedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowJobActivatedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Session(&self) -> ::windows::core::Result<PrintWorkflowJobUISession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowJobUISession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs;{d4bd5e6d-034e-5e00-a616-f961a033dcc8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobActivatedEventArgs {
    type Vtable = IPrintWorkflowJobActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobActivatedEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintWorkflowJobActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintWorkflowJobActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintWorkflowJobActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintWorkflowJobActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for &PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintWorkflowJobActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintWorkflowJobActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintWorkflowJobActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintWorkflowJobActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for &PrintWorkflowJobActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobActivatedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobBackgroundSession(::windows::core::IUnknown);
impl PrintWorkflowJobBackgroundSession {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: PrintWorkflowSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JobStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowJobStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobStarting)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveJobStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveJobStarting)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PdlModificationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobBackgroundSession, PrintWorkflowPdlModificationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PdlModificationRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePdlModificationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePdlModificationRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobBackgroundSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobBackgroundSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession;{c5ec6ad8-20c9-5d51-8507-2734b46f96c5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobBackgroundSession {
    type Vtable = IPrintWorkflowJobBackgroundSession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobBackgroundSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobBackgroundSession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobBackgroundSession";
}
impl ::core::convert::From<PrintWorkflowJobBackgroundSession> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobBackgroundSession> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobBackgroundSession> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobBackgroundSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobBackgroundSession> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobBackgroundSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobBackgroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobBackgroundSession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobBackgroundSession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobNotificationEventArgs(::windows::core::IUnknown);
impl PrintWorkflowJobNotificationEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrinterJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobNotificationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobNotificationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs;{0ae16fba-5398-5eba-b472-978650186a9a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobNotificationEventArgs {
    type Vtable = IPrintWorkflowJobNotificationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobNotificationEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobNotificationEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobNotificationEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobNotificationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobNotificationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobNotificationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobNotificationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobNotificationEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobNotificationEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobNotificationEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobStartingEventArgs(::windows::core::IUnknown);
impl PrintWorkflowJobStartingEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Printer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SetSkipSystemRendering(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSkipSystemRendering)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobStartingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs;{e3d99ba8-31ad-5e09-b0d7-601b97f161ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobStartingEventArgs {
    type Vtable = IPrintWorkflowJobStartingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobStartingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobStartingEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobStartingEventArgs";
}
impl ::core::convert::From<PrintWorkflowJobStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobStartingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobStartingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobStartingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobStartingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobStartingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobStartingEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobStartingEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobTriggerDetails(::windows::core::IUnknown);
impl PrintWorkflowJobTriggerDetails {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrintWorkflowJobSession(&self) -> ::windows::core::Result<PrintWorkflowJobBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintWorkflowJobSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowJobBackgroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails;{ff296129-60e2-51db-ba8c-e2ccddb516b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobTriggerDetails {
    type Vtable = IPrintWorkflowJobTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobTriggerDetails";
}
impl ::core::convert::From<PrintWorkflowJobTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobTriggerDetails {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowJobUISession(::windows::core::IUnknown);
impl PrintWorkflowJobUISession {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PrintWorkflowSessionStatus> {
        let this = self;
        unsafe {
            let mut result__: PrintWorkflowSessionStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSessionStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PdlDataAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowPdlDataAvailableEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PdlDataAvailable)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePdlDataAvailable<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePdlDataAvailable)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn JobNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintWorkflowJobUISession, PrintWorkflowJobNotificationEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobNotification)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveJobNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveJobNotification)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowJobUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowJobUISession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession;{00c8736b-7637-5687-a302-0f664d2aac65})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowJobUISession {
    type Vtable = IPrintWorkflowJobUISession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowJobUISession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowJobUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowJobUISession";
}
impl ::core::convert::From<PrintWorkflowJobUISession> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowJobUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobUISession> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowJobUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowJobUISession> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowJobUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowJobUISession> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowJobUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowJobUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowJobUISession {}
unsafe impl ::core::marker::Sync for PrintWorkflowJobUISession {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowObjectModelSourceFileContent(::windows::core::IUnknown);
impl PrintWorkflowObjectModelSourceFileContent {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(xpsstream: Param0) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent> {
        Self::IPrintWorkflowObjectModelSourceFileContentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), xpsstream.into_param().abi(), &mut result__).from_abi::<PrintWorkflowObjectModelSourceFileContent>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintWorkflowObjectModelSourceFileContentFactory<R, F: FnOnce(&IPrintWorkflowObjectModelSourceFileContentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintWorkflowObjectModelSourceFileContent, IPrintWorkflowObjectModelSourceFileContentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PrintWorkflowObjectModelSourceFileContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowObjectModelSourceFileContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent;{c36c8a6a-8a2a-419a-b3c3-2090e6bfab2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowObjectModelSourceFileContent {
    type Vtable = IPrintWorkflowObjectModelSourceFileContent_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowObjectModelSourceFileContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowObjectModelSourceFileContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelSourceFileContent";
}
impl ::core::convert::From<PrintWorkflowObjectModelSourceFileContent> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowObjectModelSourceFileContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelSourceFileContent> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowObjectModelSourceFileContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowObjectModelSourceFileContent> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowObjectModelSourceFileContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelSourceFileContent> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowObjectModelSourceFileContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowObjectModelSourceFileContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelSourceFileContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelSourceFileContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowObjectModelTargetPackage(::windows::core::IUnknown);
impl PrintWorkflowObjectModelTargetPackage {}
impl ::core::clone::Clone for PrintWorkflowObjectModelTargetPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowObjectModelTargetPackage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage;{7d96bc74-9b54-4ca1-ad3a-979c3d44ddac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowObjectModelTargetPackage {
    type Vtable = IPrintWorkflowObjectModelTargetPackage_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowObjectModelTargetPackage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowObjectModelTargetPackage {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowObjectModelTargetPackage";
}
impl ::core::convert::From<PrintWorkflowObjectModelTargetPackage> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowObjectModelTargetPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelTargetPackage> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowObjectModelTargetPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowObjectModelTargetPackage> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowObjectModelTargetPackage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowObjectModelTargetPackage> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowObjectModelTargetPackage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowObjectModelTargetPackage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowObjectModelTargetPackage {}
unsafe impl ::core::marker::Sync for PrintWorkflowObjectModelTargetPackage {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowPdlConversionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowPdlConversionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPdlConversionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlConversionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConversionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlConverter(::windows::core::IUnknown);
impl PrintWorkflowPdlConverter {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket", feature = "Storage_Streams"))]
    pub fn ConvertPdlAsync<'a, Param0: ::windows::core::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>, Param1: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, printticket: Param0, inputstream: Param1, outputstream: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertPdlAsync)(::core::mem::transmute_copy(this), printticket.into_param().abi(), inputstream.into_param().abi(), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlConverter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter;{40604b62-0ae4-51f1-818f-731dc0b005ab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPdlConverter {
    type Vtable = IPrintWorkflowPdlConverter_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPdlConverter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPdlConverter {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlConverter";
}
impl ::core::convert::From<PrintWorkflowPdlConverter> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPdlConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlConverter> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPdlConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlConverter> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPdlConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlConverter> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPdlConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPdlConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlConverter {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlConverter {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlDataAvailableEventArgs(::windows::core::IUnknown);
impl PrintWorkflowPdlDataAvailableEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrinterJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceContent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPdlSourceContent>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlDataAvailableEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs;{d4ad6b50-1547-5991-a0ef-e2ee20211518})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPdlDataAvailableEventArgs {
    type Vtable = IPrintWorkflowPdlDataAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPdlDataAvailableEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPdlDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlDataAvailableEventArgs";
}
impl ::core::convert::From<PrintWorkflowPdlDataAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlDataAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlDataAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlDataAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPdlDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPdlDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlDataAvailableEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlModificationRequestedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowPdlModificationRequestedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrinterJob(&self) -> ::windows::core::Result<PrintWorkflowPrinterJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrinterJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPrinterJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn SourceContent(&self) -> ::windows::core::Result<PrintWorkflowPdlSourceContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceContent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPdlSourceContent>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn UILauncher(&self) -> ::windows::core::Result<PrintWorkflowUILauncher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UILauncher)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowUILauncher>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn CreateJobOnPrinter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, targetcontenttype: Param0) -> ::windows::core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJobOnPrinter)(::core::mem::transmute_copy(this), targetcontenttype.into_param().abi(), &mut result__).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn CreateJobOnPrinterWithAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, jobattributes: Param0, targetcontenttype: Param1) -> ::windows::core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJobOnPrinterWithAttributes)(::core::mem::transmute_copy(this), jobattributes.into_param().abi(), targetcontenttype.into_param().abi(), &mut result__).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateJobOnPrinterWithAttributesBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, jobattributesbuffer: Param0, targetcontenttype: Param1) -> ::windows::core::Result<PrintWorkflowPdlTargetStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJobOnPrinterWithAttributesBuffer)(::core::mem::transmute_copy(this), jobattributesbuffer.into_param().abi(), targetcontenttype.into_param().abi(), &mut result__).from_abi::<PrintWorkflowPdlTargetStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn GetPdlConverter(&self, conversiontype: PrintWorkflowPdlConversionType) -> ::windows::core::Result<PrintWorkflowPdlConverter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPdlConverter)(::core::mem::transmute_copy(this), conversiontype, &mut result__).from_abi::<PrintWorkflowPdlConverter>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlModificationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlModificationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs;{1a339a61-2e13-5edd-a707-ceec61d7333b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPdlModificationRequestedEventArgs {
    type Vtable = IPrintWorkflowPdlModificationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPdlModificationRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPdlModificationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlModificationRequestedEventArgs";
}
impl ::core::convert::From<PrintWorkflowPdlModificationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlModificationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlModificationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlModificationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPdlModificationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPdlModificationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlModificationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlModificationRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlSourceContent(::windows::core::IUnknown);
impl PrintWorkflowPdlSourceContent {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetContentFileAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetContentFileAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlSourceContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent;{92f7fc41-32b8-56ab-845e-b1e68b3aedd5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPdlSourceContent {
    type Vtable = IPrintWorkflowPdlSourceContent_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPdlSourceContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPdlSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlSourceContent";
}
impl ::core::convert::From<PrintWorkflowPdlSourceContent> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPdlSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlSourceContent> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPdlSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlSourceContent> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPdlSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlSourceContent> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPdlSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPdlSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlSourceContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPdlTargetStream(::windows::core::IUnknown);
impl PrintWorkflowPdlTargetStream {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOutputStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn CompleteStreamSubmission(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CompleteStreamSubmission)(::core::mem::transmute_copy(this), status).ok() }
    }
}
impl ::core::clone::Clone for PrintWorkflowPdlTargetStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPdlTargetStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream;{a742dfe5-1ee3-52a9-9f9f-2e2043180fd1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPdlTargetStream {
    type Vtable = IPrintWorkflowPdlTargetStream_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPdlTargetStream as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPdlTargetStream {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPdlTargetStream";
}
impl ::core::convert::From<PrintWorkflowPdlTargetStream> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPdlTargetStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlTargetStream> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPdlTargetStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPdlTargetStream> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPdlTargetStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPdlTargetStream> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPdlTargetStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPdlTargetStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPdlTargetStream {}
unsafe impl ::core::marker::Sync for PrintWorkflowPdlTargetStream {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowPrinterJob(::windows::core::IUnknown);
impl PrintWorkflowPrinterJob {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn JobId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Printer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn GetJobStatus(&self) -> ::windows::core::Result<PrintWorkflowPrinterJobStatus> {
        let this = self;
        unsafe {
            let mut result__: PrintWorkflowPrinterJobStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJobStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowPrinterJobStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetJobPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJobPrintTicket)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetJobAttributesAsBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, attributenames: Param0) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJobAttributesAsBuffer)(::core::mem::transmute_copy(this), attributenames.into_param().abi(), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn GetJobAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, attributenames: Param0) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJobAttributes)(::core::mem::transmute_copy(this), attributenames.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Storage_Streams"))]
    pub fn SetJobAttributesFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IBuffer>>(&self, jobattributesbuffer: Param0) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetJobAttributesFromBuffer)(::core::mem::transmute_copy(this), jobattributesbuffer.into_param().abi(), &mut result__).from_abi::<super::super::super::Devices::Printers::IppSetAttributesResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Devices_Printers\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Devices_Printers", feature = "Foundation_Collections"))]
    pub fn SetJobAttributes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, super::super::super::Devices::Printers::IppAttributeValue>>>>(&self, jobattributes: Param0) -> ::windows::core::Result<super::super::super::Devices::Printers::IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetJobAttributes)(::core::mem::transmute_copy(this), jobattributes.into_param().abi(), &mut result__).from_abi::<super::super::super::Devices::Printers::IppSetAttributesResult>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPrinterJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob;{12009f94-0d14-5443-bc09-250311ce570b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowPrinterJob {
    type Vtable = IPrintWorkflowPrinterJob_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowPrinterJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowPrinterJob {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJob";
}
impl ::core::convert::From<PrintWorkflowPrinterJob> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPrinterJob> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowPrinterJob> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowPrinterJob> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowPrinterJob {}
unsafe impl ::core::marker::Sync for PrintWorkflowPrinterJob {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowPrinterJobStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowPrinterJobStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowPrinterJobStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowPrinterJobStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowPrinterJobStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowSessionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSessionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSessionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSessionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSourceContent(::windows::core::IUnknown);
impl PrintWorkflowSourceContent {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_Printing_PrintTicket"))]
    pub fn GetJobPrintTicketAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetJobPrintTicketAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PrintTicket::WorkflowPrintTicket>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn GetSourceSpoolDataAsStreamContent(&self) -> ::windows::core::Result<PrintWorkflowSpoolStreamContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSourceSpoolDataAsStreamContent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSpoolStreamContent>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn GetSourceSpoolDataAsXpsObjectModel(&self) -> ::windows::core::Result<PrintWorkflowObjectModelSourceFileContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSourceSpoolDataAsXpsObjectModel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowObjectModelSourceFileContent>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSourceContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSourceContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent;{1a28c641-ceb1-4533-bb73-fbe63eefdb18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowSourceContent {
    type Vtable = IPrintWorkflowSourceContent_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowSourceContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowSourceContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSourceContent";
}
impl ::core::convert::From<PrintWorkflowSourceContent> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSourceContent> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSourceContent> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowSourceContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSourceContent> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowSourceContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowSourceContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSourceContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSourceContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSpoolStreamContent(::windows::core::IUnknown);
impl PrintWorkflowSpoolStreamContent {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetInputStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IInputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSpoolStreamContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSpoolStreamContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent;{72e55ece-e406-4b74-84e1-3ff3fdcdaf70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowSpoolStreamContent {
    type Vtable = IPrintWorkflowSpoolStreamContent_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowSpoolStreamContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowSpoolStreamContent {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSpoolStreamContent";
}
impl ::core::convert::From<PrintWorkflowSpoolStreamContent> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowSpoolStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSpoolStreamContent> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowSpoolStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSpoolStreamContent> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowSpoolStreamContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSpoolStreamContent> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowSpoolStreamContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowSpoolStreamContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSpoolStreamContent {}
unsafe impl ::core::marker::Sync for PrintWorkflowSpoolStreamContent {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowStreamTarget(::windows::core::IUnknown);
impl PrintWorkflowStreamTarget {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOutputStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowStreamTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowStreamTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget;{b23bba84-8565-488b-9839-1c9e7c7aa916})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowStreamTarget {
    type Vtable = IPrintWorkflowStreamTarget_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowStreamTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowStreamTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowStreamTarget";
}
impl ::core::convert::From<PrintWorkflowStreamTarget> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowStreamTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowStreamTarget> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowStreamTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowStreamTarget> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowStreamTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowStreamTarget> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowStreamTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowStreamTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowStreamTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowStreamTarget {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSubmittedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowSubmittedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSubmittedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn GetTarget<'a, Param0: ::windows::core::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>>(&self, jobprintticket: Param0) -> ::windows::core::Result<PrintWorkflowTarget> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetTarget)(::core::mem::transmute_copy(this), jobprintticket.into_param().abi(), &mut result__).from_abi::<PrintWorkflowTarget>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSubmittedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSubmittedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs;{3add0a41-3794-5569-5c87-40e8ff720f83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowSubmittedEventArgs {
    type Vtable = IPrintWorkflowSubmittedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowSubmittedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowSubmittedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedEventArgs";
}
impl ::core::convert::From<PrintWorkflowSubmittedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowSubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowSubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSubmittedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowSubmittedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowSubmittedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowSubmittedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowSubmittedOperation(::windows::core::IUnknown);
impl PrintWorkflowSubmittedOperation {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this), status).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<PrintWorkflowConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn XpsContent(&self) -> ::windows::core::Result<PrintWorkflowSourceContent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XpsContent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSourceContent>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowSubmittedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSubmittedOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation;{2e4e6216-3be1-5f0f-5c81-a5a2bd4eab0e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowSubmittedOperation {
    type Vtable = IPrintWorkflowSubmittedOperation_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowSubmittedOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowSubmittedOperation {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedOperation";
}
impl ::core::convert::From<PrintWorkflowSubmittedOperation> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowSubmittedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedOperation> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowSubmittedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowSubmittedOperation> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowSubmittedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowSubmittedOperation> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowSubmittedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowSubmittedOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowSubmittedOperation {}
unsafe impl ::core::marker::Sync for PrintWorkflowSubmittedOperation {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowSubmittedStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowSubmittedStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowSubmittedStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowSubmittedStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowSubmittedStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowTarget(::windows::core::IUnknown);
impl PrintWorkflowTarget {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn TargetAsStream(&self) -> ::windows::core::Result<PrintWorkflowStreamTarget> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetAsStream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowStreamTarget>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn TargetAsXpsObjectModelPackage(&self) -> ::windows::core::Result<PrintWorkflowObjectModelTargetPackage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetAsXpsObjectModelPackage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowObjectModelTargetPackage>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTarget;{29da276c-0a73-5aed-4f3d-970d3251f057})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowTarget {
    type Vtable = IPrintWorkflowTarget_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowTarget {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTarget";
}
impl ::core::convert::From<PrintWorkflowTarget> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTarget> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowTarget> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTarget> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowTarget {}
unsafe impl ::core::marker::Sync for PrintWorkflowTarget {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowTriggerDetails(::windows::core::IUnknown);
impl PrintWorkflowTriggerDetails {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowBackgroundSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintWorkflowSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowBackgroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails;{5739d868-9d86-4052-b0cb-f310becd59bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowTriggerDetails {
    type Vtable = IPrintWorkflowTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowTriggerDetails";
}
impl ::core::convert::From<PrintWorkflowTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintWorkflowTriggerDetails {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowUIActivatedEventArgs(::windows::core::IUnknown);
impl PrintWorkflowUIActivatedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn PrintWorkflowSession(&self) -> ::windows::core::Result<PrintWorkflowForegroundSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintWorkflowSession)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowForegroundSession>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowUIActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowUIActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs;{bc8a844d-09eb-5746-72a6-8dc8b5edbe9b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowUIActivatedEventArgs {
    type Vtable = IPrintWorkflowUIActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowUIActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowUIActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUIActivatedEventArgs";
}
impl ::core::convert::From<PrintWorkflowUIActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowUIActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUIActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowUIActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowUIActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowUIActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUIActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowUIActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintWorkflowUIActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintWorkflowUIActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintWorkflowUIActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintWorkflowUIActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for &PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintWorkflowUIActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintWorkflowUIActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintWorkflowUIActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintWorkflowUIActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for &PrintWorkflowUIActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowUIActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowUIActivatedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintWorkflowUICompletionStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintWorkflowUICompletionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintWorkflowUICompletionStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintWorkflowUICompletionStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.Workflow.PrintWorkflowUICompletionStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowUILauncher(::windows::core::IUnknown);
impl PrintWorkflowUILauncher {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn IsUILaunchEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUILaunchEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LaunchAndCompleteUIAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LaunchAndCompleteUIAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PrintWorkflowUICompletionStatus>>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowUILauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowUILauncher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher;{64e9e22f-14cc-5828-96fb-39163fb6c378})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowUILauncher {
    type Vtable = IPrintWorkflowUILauncher_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowUILauncher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowUILauncher {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowUILauncher";
}
impl ::core::convert::From<PrintWorkflowUILauncher> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowUILauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUILauncher> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowUILauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowUILauncher> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowUILauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowUILauncher> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowUILauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowUILauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowUILauncher {}
unsafe impl ::core::marker::Sync for PrintWorkflowUILauncher {}
#[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
#[repr(transparent)]
pub struct PrintWorkflowXpsDataAvailableEventArgs(::windows::core::IUnknown);
impl PrintWorkflowXpsDataAvailableEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`*"]
    pub fn Operation(&self) -> ::windows::core::Result<PrintWorkflowSubmittedOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintWorkflowSubmittedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_Workflow\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintWorkflowXpsDataAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintWorkflowXpsDataAvailableEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs;{4d11c331-54d1-434e-be0e-82c5fa58e5b2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintWorkflowXpsDataAvailableEventArgs {
    type Vtable = IPrintWorkflowXpsDataAvailableEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintWorkflowXpsDataAvailableEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintWorkflowXpsDataAvailableEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.Workflow.PrintWorkflowXpsDataAvailableEventArgs";
}
impl ::core::convert::From<PrintWorkflowXpsDataAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowXpsDataAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintWorkflowXpsDataAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintWorkflowXpsDataAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintWorkflowXpsDataAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintWorkflowXpsDataAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintWorkflowXpsDataAvailableEventArgs {}
unsafe impl ::core::marker::Sync for PrintWorkflowXpsDataAvailableEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
