#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailDataProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailDataProviderConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b9c9dc7_37b2_4bf0_ae30_7b644a1c96e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MailboxSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MailboxSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMailboxSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMailboxSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadMessageRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadMessageRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadAttachmentRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadAttachmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDownloadAttachmentRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDownloadAttachmentRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub EmptyFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EmptyFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEmptyFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEmptyFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub MoveFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMoveFolderRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMoveFolderRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateMeetingResponseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateMeetingResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateMeetingResponseRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ForwardMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveForwardMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveForwardMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProposeNewTimeForMeetingRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProposeNewTimeForMeetingRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub GetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGetAutoReplySettingsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGetAutoReplySettingsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ResolveRecipientsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResolveRecipientsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResolveRecipientsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResolveRecipientsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ValidateCertificatesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateCertificatesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveValidateCertificatesRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveValidateCertificatesRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServerSearchReadBatchRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServerSearchReadBatchRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServerSearchReadBatchRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailDataProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f3e4e50_341e_45f3_bba0_84a005e1319a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxCreateFolderRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x184d3775_c921_4c39_a309_e16c9f22b04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: super::EmailMailboxCreateFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxCreateFolderRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e4c02c_241c_4ea9_a68f_ff20bc5afc85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDeleteFolderRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9469e88a_a931_4779_923d_09a3ea292e29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: super::EmailMailboxDeleteFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDeleteFolderRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4d32d06_2332_4678_8378_28b579336846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadAttachmentRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b1dbbb4_750c_48e1_bce4_8d589684ffbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailAttachmentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadAttachmentRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccddc46d_ffa8_4877_9f9d_fed7bcaf4104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadMessageRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x497b4187_5b4d_4b23_816c_f3842beb753e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadMessageRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470409ad_d0a0_4a5b_bb2a_37621039c53e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxEmptyFolderRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe4b03ab_f86d_46d9_b4ce_bc8a6d9e9268);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: super::EmailMailboxEmptyFolderStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxEmptyFolderRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7183f484_985a_4ac0_b33f_ee0e2627a3c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxForwardMeetingRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616d6af1_70d4_4832_b869_b80542ae9be8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ForwardHeaderType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMessageBodyKind) -> ::windows::core::HRESULT,
    pub ForwardHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxForwardMeetingRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bd8f33a_2974_4759_a5a5_58f44d3c0275);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxGetAutoReplySettingsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b380789_1e88_4e01_84cc_1386ad9a2c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RequestedFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoreplysettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd79f55c2_fd45_4004_8a91_9bacf38b7022);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxMoveFolderRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ba2856_4a95_4068_91cc_67cc7acf454f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NewParentFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NewFolderName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxMoveFolderRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxMoveFolderRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38623020_14ba_4c88_8698_7239e3c8aaa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5aeff152_9799_4f9f_a399_ff07f3eef04e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NewStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub NewDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewDuration: usize,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb480b98_33ad_4a67_8251_0f9c249b6a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxResolveRecipientsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa4cf70_7b39_4c9b_811e_41eaf43a332d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionresults: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxResolveRecipientsRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260f9e02_b2cf_40f8_8c28_e3ed43b1e89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxServerSearchReadBatchRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x090eebdf_5a96_41d3_8ad8_34912f9aa60e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuggestedBatchSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, batchstatus: super::EmailBatchStatus, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14101b4e_ed9e_45d1_ad7a_cc9b7f643ae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxSetAutoReplySettingsRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a422d0_a88e_4e54_8dc7_c243186b774e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AutoReplySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09da11ad_d7ca_4087_ac86_53fa67f76246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e10e8e4_7e67_405a_b673_dc60c91090fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439a031a_8fcc_4ae5_b9b5_d434e0a65aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxUpdateMeetingResponseRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b99ac93_b2cf_4888_ba4f_306b6b66df30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EmailMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMeetingResponseType) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SendUpdate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6898d761_56c9_4f17_be31_66fda94ba159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxValidateCertificatesRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94d3931_e11a_4f97_b81a_187a70a8f41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EmailMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub Certificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    Certificates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, validationstatuses: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IEmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IEmailMailboxValidateCertificatesRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2583bf17_02ff_49fe_a73c_03f37566c691);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailDataProviderConnection(::windows::core::IUnknown);
impl EmailDataProviderConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MailboxSyncRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MailboxSyncRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMailboxSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMailboxSyncRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadMessageRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadMessageRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDownloadMessageRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadAttachmentRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadAttachmentRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDownloadAttachmentRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDownloadAttachmentRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFolderRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCreateFolderRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteFolderRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteFolderRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDeleteFolderRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EmptyFolderRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmptyFolderRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEmptyFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveEmptyFolderRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveFolderRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveFolderRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMoveFolderRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMoveFolderRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateMeetingResponseRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateMeetingResponseRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateMeetingResponseRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUpdateMeetingResponseRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ForwardMeetingRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForwardMeetingRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveForwardMeetingRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveForwardMeetingRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProposeNewTimeForMeetingRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProposeNewTimeForMeetingRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProposeNewTimeForMeetingRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveProposeNewTimeForMeetingRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAutoReplySettingsRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetAutoReplySettingsRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSetAutoReplySettingsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSetAutoReplySettingsRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAutoReplySettingsRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAutoReplySettingsRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGetAutoReplySettingsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGetAutoReplySettingsRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ResolveRecipientsRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResolveRecipientsRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResolveRecipientsRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveResolveRecipientsRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateCertificatesRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateCertificatesRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveValidateCertificatesRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveValidateCertificatesRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServerSearchReadBatchRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServerSearchReadBatchRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerSearchReadBatchRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveServerSearchReadBatchRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for EmailDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailDataProviderConnection {}
impl ::core::fmt::Debug for EmailDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection;{3b9c9dc7-37b2-4bf0-ae30-7b644a1c96e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailDataProviderConnection {
    const IID: ::windows::core::GUID = <IEmailDataProviderConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
}
::windows::core::interface_hierarchy!(EmailDataProviderConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailDataProviderConnection {}
unsafe impl ::core::marker::Sync for EmailDataProviderConnection {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailDataProviderTriggerDetails(::windows::core::IUnknown);
impl EmailDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows::core::Result<EmailDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailDataProviderTriggerDetails {}
impl ::core::fmt::Debug for EmailDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails;{8f3e4e50-341e-45f3-bba0-84a005e1319a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = <IEmailDataProviderTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
}
::windows::core::interface_hierarchy!(EmailDataProviderTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for EmailDataProviderTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequest(::windows::core::IUnknown);
impl EmailMailboxCreateFolderRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self, folder: &super::EmailFolder) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(folder), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), status, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxCreateFolderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCreateFolderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCreateFolderRequest {}
impl ::core::fmt::Debug for EmailMailboxCreateFolderRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxCreateFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest;{184d3775-c921-4c39-a309-e16c9f22b04b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxCreateFolderRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxCreateFolderRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxCreateFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxCreateFolderRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxCreateFolderRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxCreateFolderRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxCreateFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxCreateFolderRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCreateFolderRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCreateFolderRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxCreateFolderRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxCreateFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs;{03e4c02c-241c-4ea9-a68f-ff20bc5afc85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxCreateFolderRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxCreateFolderRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxCreateFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxCreateFolderRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequest(::windows::core::IUnknown);
impl EmailMailboxDeleteFolderRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), status, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDeleteFolderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDeleteFolderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDeleteFolderRequest {}
impl ::core::fmt::Debug for EmailMailboxDeleteFolderRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDeleteFolderRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDeleteFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest;{9469e88a-a931-4779-923d-09a3ea292e29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDeleteFolderRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxDeleteFolderRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDeleteFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxDeleteFolderRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDeleteFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDeleteFolderRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxDeleteFolderRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDeleteFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDeleteFolderRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDeleteFolderRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDeleteFolderRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxDeleteFolderRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDeleteFolderRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDeleteFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs;{b4d32d06-2332-4678-8378-28b579336846})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDeleteFolderRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxDeleteFolderRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDeleteFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxDeleteFolderRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDeleteFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDeleteFolderRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequest(::windows::core::IUnknown);
impl EmailMailboxDownloadAttachmentRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailAttachmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailAttachmentId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDownloadAttachmentRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDownloadAttachmentRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDownloadAttachmentRequest {}
impl ::core::fmt::Debug for EmailMailboxDownloadAttachmentRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDownloadAttachmentRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadAttachmentRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest;{0b1dbbb4-750c-48e1-bce4-8d589684ffbc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadAttachmentRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxDownloadAttachmentRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadAttachmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxDownloadAttachmentRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDownloadAttachmentRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadAttachmentRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxDownloadAttachmentRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadAttachmentRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDownloadAttachmentRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDownloadAttachmentRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadAttachmentRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs;{ccddc46d-ffa8-4877-9f9d-fed7bcaf4104})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadAttachmentRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxDownloadAttachmentRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadAttachmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxDownloadAttachmentRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDownloadAttachmentRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadAttachmentRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequest(::windows::core::IUnknown);
impl EmailMailboxDownloadMessageRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDownloadMessageRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDownloadMessageRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDownloadMessageRequest {}
impl ::core::fmt::Debug for EmailMailboxDownloadMessageRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDownloadMessageRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadMessageRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest;{497b4187-5b4d-4b23-816c-f3842beb753e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadMessageRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxDownloadMessageRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadMessageRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxDownloadMessageRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDownloadMessageRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadMessageRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxDownloadMessageRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadMessageRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxDownloadMessageRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxDownloadMessageRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxDownloadMessageRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxDownloadMessageRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDownloadMessageRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadMessageRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs;{470409ad-d0a0-4a5b-bb2a-37621039c53e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadMessageRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxDownloadMessageRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadMessageRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxDownloadMessageRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxDownloadMessageRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadMessageRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequest(::windows::core::IUnknown);
impl EmailMailboxEmptyFolderRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), status, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxEmptyFolderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxEmptyFolderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxEmptyFolderRequest {}
impl ::core::fmt::Debug for EmailMailboxEmptyFolderRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxEmptyFolderRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxEmptyFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest;{fe4b03ab-f86d-46d9-b4ce-bc8a6d9e9268})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxEmptyFolderRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxEmptyFolderRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxEmptyFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxEmptyFolderRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxEmptyFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxEmptyFolderRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxEmptyFolderRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxEmptyFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxEmptyFolderRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxEmptyFolderRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxEmptyFolderRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxEmptyFolderRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxEmptyFolderRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxEmptyFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs;{7183f484-985a-4ac0-b33f-ee0e2627a3c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxEmptyFolderRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxEmptyFolderRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxEmptyFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxEmptyFolderRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxEmptyFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxEmptyFolderRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequest(::windows::core::IUnknown);
impl EmailMailboxForwardMeetingRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Recipients)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ForwardHeaderType(&self) -> ::windows::core::Result<super::EmailMessageBodyKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForwardHeaderType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForwardHeader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxForwardMeetingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxForwardMeetingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxForwardMeetingRequest {}
impl ::core::fmt::Debug for EmailMailboxForwardMeetingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxForwardMeetingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxForwardMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest;{616d6af1-70d4-4832-b869-b80542ae9be8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxForwardMeetingRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxForwardMeetingRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxForwardMeetingRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxForwardMeetingRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxForwardMeetingRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxForwardMeetingRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxForwardMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxForwardMeetingRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxForwardMeetingRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxForwardMeetingRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxForwardMeetingRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxForwardMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs;{2bd8f33a-2974-4759-a5a5-58f44d3c0275})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxForwardMeetingRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxForwardMeetingRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxForwardMeetingRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxForwardMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxForwardMeetingRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequest(::windows::core::IUnknown);
impl EmailMailboxGetAutoReplySettingsRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RequestedFormat(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestedFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self, autoreplysettings: &super::EmailMailboxAutoReplySettings) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(autoreplysettings), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxGetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxGetAutoReplySettingsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxGetAutoReplySettingsRequest {}
impl ::core::fmt::Debug for EmailMailboxGetAutoReplySettingsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxGetAutoReplySettingsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxGetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest;{9b380789-1e88-4e01-84cc-1386ad9a2c2f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxGetAutoReplySettingsRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxGetAutoReplySettingsRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxGetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxGetAutoReplySettingsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxGetAutoReplySettingsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxGetAutoReplySettingsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxGetAutoReplySettingsRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxGetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxGetAutoReplySettingsRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs;{d79f55c2-fd45-4004-8a91-9bacf38b7022})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxGetAutoReplySettingsRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxGetAutoReplySettingsRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequest(::windows::core::IUnknown);
impl EmailMailboxMoveFolderRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewParentFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NewFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewFolderName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxMoveFolderRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxMoveFolderRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxMoveFolderRequest {}
impl ::core::fmt::Debug for EmailMailboxMoveFolderRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxMoveFolderRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxMoveFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest;{10ba2856-4a95-4068-91cc-67cc7acf454f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxMoveFolderRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxMoveFolderRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxMoveFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxMoveFolderRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxMoveFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxMoveFolderRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxMoveFolderRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxMoveFolderRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxMoveFolderRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxMoveFolderRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxMoveFolderRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxMoveFolderRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxMoveFolderRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxMoveFolderRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxMoveFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs;{38623020-14ba-4c88-8698-7239e3c8aaa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxMoveFolderRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxMoveFolderRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxMoveFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxMoveFolderRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxMoveFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxMoveFolderRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(::windows::core::IUnknown);
impl EmailMailboxProposeNewTimeForMeetingRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewStartTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewDuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxProposeNewTimeForMeetingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxProposeNewTimeForMeetingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxProposeNewTimeForMeetingRequest {}
impl ::core::fmt::Debug for EmailMailboxProposeNewTimeForMeetingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxProposeNewTimeForMeetingRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest;{5aeff152-9799-4f9f-a399-ff07f3eef04e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxProposeNewTimeForMeetingRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxProposeNewTimeForMeetingRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxProposeNewTimeForMeetingRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxProposeNewTimeForMeetingRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxProposeNewTimeForMeetingRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs;{fb480b98-33ad-4a67-8251-0f9c249b6a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxProposeNewTimeForMeetingRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxProposeNewTimeForMeetingRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequest(::windows::core::IUnknown);
impl EmailMailboxResolveRecipientsRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Recipients)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompletedAsync<P0, E0>(&self, resolutionresults: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), resolutionresults.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxResolveRecipientsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxResolveRecipientsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxResolveRecipientsRequest {}
impl ::core::fmt::Debug for EmailMailboxResolveRecipientsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxResolveRecipientsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxResolveRecipientsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest;{efa4cf70-7b39-4c9b-811e-41eaf43a332d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxResolveRecipientsRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxResolveRecipientsRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxResolveRecipientsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxResolveRecipientsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxResolveRecipientsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxResolveRecipientsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxResolveRecipientsRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxResolveRecipientsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxResolveRecipientsRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxResolveRecipientsRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxResolveRecipientsRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxResolveRecipientsRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxResolveRecipientsRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxResolveRecipientsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs;{260f9e02-b2cf-40f8-8c28-e3ed43b1e89a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxResolveRecipientsRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxResolveRecipientsRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxResolveRecipientsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxResolveRecipientsRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxResolveRecipientsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxResolveRecipientsRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequest(::windows::core::IUnknown);
impl EmailMailboxServerSearchReadBatchRequest {
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailFolderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<super::EmailQueryOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Options)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SuggestedBatchSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync(&self, message: &super::EmailMessage) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), batchstatus, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxServerSearchReadBatchRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxServerSearchReadBatchRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxServerSearchReadBatchRequest {}
impl ::core::fmt::Debug for EmailMailboxServerSearchReadBatchRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxServerSearchReadBatchRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest;{090eebdf-5a96-41d3-8ad8-34912f9aa60e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxServerSearchReadBatchRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxServerSearchReadBatchRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxServerSearchReadBatchRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxServerSearchReadBatchRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxServerSearchReadBatchRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxServerSearchReadBatchRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxServerSearchReadBatchRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxServerSearchReadBatchRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs;{14101b4e-ed9e-45d1-ad7a-cc9b7f643ae2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxServerSearchReadBatchRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxServerSearchReadBatchRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxServerSearchReadBatchRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequest(::windows::core::IUnknown);
impl EmailMailboxSetAutoReplySettingsRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn AutoReplySettings(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AutoReplySettings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxSetAutoReplySettingsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSetAutoReplySettingsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSetAutoReplySettingsRequest {}
impl ::core::fmt::Debug for EmailMailboxSetAutoReplySettingsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSetAutoReplySettingsRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest;{75a422d0-a88e-4e54-8dc7-c243186b774e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxSetAutoReplySettingsRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxSetAutoReplySettingsRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxSetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxSetAutoReplySettingsRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxSetAutoReplySettingsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxSetAutoReplySettingsRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxSetAutoReplySettingsRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxSetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSetAutoReplySettingsRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs;{09da11ad-d7ca-4087-ac86-53fa67f76246})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxSetAutoReplySettingsRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxSetAutoReplySettingsRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequest(::windows::core::IUnknown);
impl EmailMailboxSyncManagerSyncRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSyncManagerSyncRequest {}
impl ::core::fmt::Debug for EmailMailboxSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest;{4e10e8e4-7e67-405a-b673-dc60c91090fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxSyncManagerSyncRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxSyncManagerSyncRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManagerSyncRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs;{439a031a-8fcc-4ae5-b9b5-d434e0a65aa8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxSyncManagerSyncRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxSyncManagerSyncRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManagerSyncRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequest(::windows::core::IUnknown);
impl EmailMailboxUpdateMeetingResponseRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Response(&self) -> ::windows::core::Result<super::EmailMeetingResponseType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Response)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Comment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SendUpdate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendUpdate)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxUpdateMeetingResponseRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxUpdateMeetingResponseRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxUpdateMeetingResponseRequest {}
impl ::core::fmt::Debug for EmailMailboxUpdateMeetingResponseRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxUpdateMeetingResponseRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxUpdateMeetingResponseRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest;{5b99ac93-b2cf-4888-ba4f-306b6b66df30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxUpdateMeetingResponseRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxUpdateMeetingResponseRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxUpdateMeetingResponseRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxUpdateMeetingResponseRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxUpdateMeetingResponseRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxUpdateMeetingResponseRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxUpdateMeetingResponseRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs;{6898d761-56c9-4f17-be31-66fda94ba159})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxUpdateMeetingResponseRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxUpdateMeetingResponseRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequest(::windows::core::IUnknown);
impl EmailMailboxValidateCertificatesRequest {
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EmailMailboxId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Certificates)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReportCompletedAsync<P0, E0>(&self, validationstatuses: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), validationstatuses.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxValidateCertificatesRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxValidateCertificatesRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxValidateCertificatesRequest {}
impl ::core::fmt::Debug for EmailMailboxValidateCertificatesRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxValidateCertificatesRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxValidateCertificatesRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest;{a94d3931-e11a-4f97-b81a-187a70a8f41a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxValidateCertificatesRequest {
    const IID: ::windows::core::GUID = <IEmailMailboxValidateCertificatesRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxValidateCertificatesRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
}
::windows::core::interface_hierarchy!(EmailMailboxValidateCertificatesRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxValidateCertificatesRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxValidateCertificatesRequest {}
#[doc = "*Required features: `\"ApplicationModel_Email_DataProvider\"`*"]
#[repr(transparent)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(::windows::core::IUnknown);
impl EmailMailboxValidateCertificatesRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxValidateCertificatesRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for EmailMailboxValidateCertificatesRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EmailMailboxValidateCertificatesRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxValidateCertificatesRequestEventArgs {}
impl ::core::fmt::Debug for EmailMailboxValidateCertificatesRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxValidateCertificatesRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxValidateCertificatesRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs;{2583bf17-02ff-49fe-a73c-03f37566c691})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for EmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for EmailMailboxValidateCertificatesRequestEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxValidateCertificatesRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxValidateCertificatesRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
}
::windows::core::interface_hierarchy!(EmailMailboxValidateCertificatesRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxValidateCertificatesRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxValidateCertificatesRequestEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
