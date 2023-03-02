#[cfg(feature = "ApplicationModel_Email_DataProvider")]
pub mod DataProvider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailAttachment {
    type Vtable = IEmailAttachment_Vtbl;
}
impl ::core::clone::Clone for IEmailAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailAttachment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf353caf9_57c8_4adb_b992_60fceb584f54);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailAttachment2 {
    type Vtable = IEmailAttachment2_Vtbl;
}
impl ::core::clone::Clone for IEmailAttachment2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailAttachment2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225f1070_b0ff_4571_9d54_a706c48d55c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachment2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailAttachmentDownloadState) -> ::windows::core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailAttachmentDownloadState) -> ::windows::core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub IsFromBaseMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsInline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachmentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailAttachmentFactory {
    type Vtable = IEmailAttachmentFactory_Vtbl;
}
impl ::core::clone::Clone for IEmailAttachmentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailAttachmentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x796eac46_ed56_4979_8708_abb8bc854b7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachmentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailAttachmentFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailAttachmentFactory2 {
    type Vtable = IEmailAttachmentFactory2_Vtbl;
}
impl ::core::clone::Clone for IEmailAttachmentFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailAttachmentFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23259435_51f9_427d_adcd_241023c8cfb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailAttachmentFactory2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::HSTRING>, data: *mut ::core::ffi::c_void, mimetype: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailConversation {
    type Vtable = IEmailConversation_Vtbl;
}
impl ::core::clone::Clone for IEmailConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailConversation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda18c248_a0bc_4349_902d_90f66389f51b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows::core::HRESULT,
    pub HasAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows::core::HRESULT,
    pub LastEmailResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows::core::HRESULT,
    pub MessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MostRecentMessageTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MostRecentMessageTime: usize,
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LatestSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UnreadMessageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMessagesWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMessagesWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversationBatch(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailConversationBatch {
    type Vtable = IEmailConversationBatch_Vtbl;
}
impl ::core::clone::Clone for IEmailConversationBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailConversationBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8c1ab81_01c5_432a_9df1_fe85d98a279a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversationBatch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Conversations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Conversations: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailConversationReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailConversationReader {
    type Vtable = IEmailConversationReader_Vtbl;
}
impl ::core::clone::Clone for IEmailConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailConversationReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4630f82_2875_44c8_9b8c_85beb3a3c653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailConversationReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailFolder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailFolder {
    type Vtable = IEmailFolder_Vtbl;
}
impl ::core::clone::Clone for IEmailFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailFolder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa24f7771_996c_4864_b1ba_ed1240e57d11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailFolder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ParentFolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailSpecialFolderKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindChildFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindChildFoldersAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageCountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageCountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveWithNewNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newparentfolder: *mut ::core::ffi::c_void, newfoldername: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailIrmInfo {
    type Vtable = IEmailIrmInfo_Vtbl;
}
impl ::core::clone::Clone for IEmailIrmInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailIrmInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90f52193_b1a0_4ebd_a6b6_ddca55606e0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanExtractData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanExtractData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanModifyRecipientsOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanPrintData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanPrintData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanRemoveIrmOnResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CanReplyAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanReplyAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExpirationDate: usize,
    #[cfg(feature = "Foundation")]
    pub SetExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetExpirationDate: usize,
    pub IsIrmOriginator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsIrmOriginator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsProgramaticAccessAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Template: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTemplate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailIrmInfoFactory {
    type Vtable = IEmailIrmInfoFactory_Vtbl;
}
impl ::core::clone::Clone for IEmailIrmInfoFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailIrmInfoFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x314bb18c_e3e6_4d7b_be8d_91a96311b01b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmInfoFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, expiration: super::super::Foundation::DateTime, irmtemplate: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmTemplate(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailIrmTemplate {
    type Vtable = IEmailIrmTemplate_Vtbl;
}
impl ::core::clone::Clone for IEmailIrmTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailIrmTemplate {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf327758d_546d_4bea_a963_54a38b2cc016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmTemplate_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailIrmTemplateFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailIrmTemplateFactory {
    type Vtable = IEmailIrmTemplateFactory_Vtbl;
}
impl ::core::clone::Clone for IEmailIrmTemplateFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailIrmTemplateFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da31876_8738_4418_b9cb_471b936fe71e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailIrmTemplateFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, description: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailItemCounts(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailItemCounts {
    type Vtable = IEmailItemCounts_Vtbl;
}
impl ::core::clone::Clone for IEmailItemCounts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailItemCounts {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bd13321_fec8_4bab_83ba_0baf3c1f6cbd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailItemCounts_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Flagged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Important: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Total: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Unread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailbox {
    type Vtable = IEmailMailbox_Vtbl;
}
impl ::core::clone::Clone for IEmailMailbox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailbox {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8790649_cf5b_411b_80b1_4a6a1484ce25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOwnedByCurrentApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDataEncryptedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailAddressAliases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailAddressAliases: usize,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub Policies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSpecialFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, foldertype: EmailSpecialFolderKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSpecialFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderAsSeenAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, isread: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ChangeMessageFlagStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, flagstate: EmailFlagState, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ChangeMessageFlagStateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, newparentfolderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, newparentfolderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryMoveFolderWithNewNameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, newparentfolderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, newfoldername: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryMoveFolderWithNewNameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkFolderSyncEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, issyncenabled: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkFolderSyncEnabledAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveDraftAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveDraftAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadAttachmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attachmentid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadAttachmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateResponseMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, messageid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, responsetype: EmailMessageResponseKind, subject: ::std::mem::MaybeUninit<::windows::core::HSTRING>, responseheadertype: EmailMessageBodyKind, responseheader: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateResponseMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, response: EmailMeetingResponseType, subject: ::std::mem::MaybeUninit<::windows::core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows::core::HSTRING>, sendupdate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, recipients: *mut ::core::ffi::c_void, subject: ::std::mem::MaybeUninit<::windows::core::HSTRING>, forwardheadertype: EmailMessageBodyKind, forwardheader: ::std::mem::MaybeUninit<::windows::core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::std::mem::MaybeUninit<::windows::core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MailboxChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMailboxChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMailboxChanged: usize,
    #[cfg(feature = "Foundation")]
    pub SmartSendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, smartsend: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SmartSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, autoreplysettings: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetAutoReplySettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryGetAutoReplySettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedformat: EmailMailboxAutoReplyMessageResponseKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryGetAutoReplySettingsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailbox2 {
    type Vtable = IEmailMailbox2_Vtbl;
}
impl ::core::clone::Clone for IEmailMailbox2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailbox2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14f8e404_6ca2_4ab2_9241_79cd7bf46346);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LinkedMailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NetworkAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailbox3 {
    type Vtable = IEmailMailbox3_Vtbl;
}
impl ::core::clone::Clone for IEmailMailbox3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailbox3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3da5897b_458b_408a_8e37_ac8b05d8af56);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveRecipientsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recipients: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveRecipientsAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub ValidateCertificatesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificates: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    ValidateCertificatesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEmptyFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEmptyFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDeleteFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folderid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailbox4 {
    type Vtable = IEmailMailbox4_Vtbl;
}
impl ::core::clone::Clone for IEmailMailbox4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailbox4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d1f301b_f222_48a7_b7b6_716356cd26a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailbox5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailbox5 {
    type Vtable = IEmailMailbox5_Vtbl;
}
impl ::core::clone::Clone for IEmailMailbox5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailbox5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x39967087_0092_49be_bd0e_5d4dc9d96d90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailbox5_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxAction {
    type Vtable = IEmailMailboxAction_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxAction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac9889fa_21fa_4927_9210_d410582fdf3e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxActionKind) -> ::windows::core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAutoReply(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxAutoReply {
    type Vtable = IEmailMailboxAutoReply_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxAutoReply {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxAutoReply {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe223254c_8ab4_485b_b31f_04d15476bd59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAutoReply_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxAutoReplySettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxAutoReplySettings {
    type Vtable = IEmailMailboxAutoReplySettings_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxAutoReplySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxAutoReplySettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa87a9fa8_0ac6_4b77_ba77_a6b99e9a27b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxAutoReplySettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT,
    pub SetResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub EndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndTime: usize,
    pub InternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub KnownExternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnknownExternalReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxCapabilities {
    type Vtable = IEmailMailboxCapabilities_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeedec3a6_89db_4305_82c4_439e0a33da11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanServerSearchFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanServerSearchMailbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanSmartSend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxCapabilities2 {
    type Vtable = IEmailMailboxCapabilities2_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxCapabilities2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxCapabilities2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69723ee4_2f21_4cbc_88ab_2e7602a4806b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanResolveRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanValidateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanEmptyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanCreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanMoveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCapabilities3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxCapabilities3 {
    type Vtable = IEmailMailboxCapabilities3_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxCapabilities3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxCapabilities3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf690e944_56f2_45aa_872c_0ce9f3db0b5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCapabilities3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanGetAndSetExternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanGetAndSetInternalAutoReplies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanServerSearchFolders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanServerSearchMailbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanSmartSend: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanResolveRecipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanValidateCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanEmptyFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanCreateFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanDeleteFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetCanMoveFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxChange {
    type Vtable = IEmailMailboxChange_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxChange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x61edf54b_11ef_400c_adde_8cde65c85e66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChange_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxChangeType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MailboxActions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MailboxActions: usize,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxChangeReader {
    type Vtable = IEmailMailboxChangeReader_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxChangeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdbd0ebb_c53d_4331_97be_be75a2146a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangeReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxChangeTracker {
    type Vtable = IEmailMailboxChangeTracker_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxChangeTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ae48638_5166_42b7_8882_fd21c92bdd4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangeTracker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxChangedDeferral {
    type Vtable = IEmailMailboxChangedDeferral_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxChangedDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x779a74c1_97c5_4b54_b30d_306232623e6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxChangedEventArgs {
    type Vtable = IEmailMailboxChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cfd5f6e_01d4_4e4a_a44c_b22dd42ec207);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxCreateFolderResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxCreateFolderResult {
    type Vtable = IEmailMailboxCreateFolderResult_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxCreateFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxCreateFolderResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb228557f_2885_4998_b595_8a2d374ce950);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxCreateFolderStatus) -> ::windows::core::HRESULT,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxPolicies {
    type Vtable = IEmailMailboxPolicies_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxPolicies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxPolicies {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f3345c5_1c3b_4dc7_b410_6373783e545d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::HRESULT,
    pub AllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub RequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequiredSmimeSigningAlgorithm: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxPolicies2 {
    type Vtable = IEmailMailboxPolicies2_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxPolicies2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxPolicies2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbab58afb_a14b_497c_a8e2_55eac29cc4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MustSignSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxPolicies3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxPolicies3 {
    type Vtable = IEmailMailboxPolicies3_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxPolicies3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxPolicies3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdd4a01f_4867_414a_81a2_803919c44191);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxPolicies3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetAllowedSmimeEncryptionAlgorithmNegotiation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::HRESULT,
    pub SetAllowSmimeSoftCertificates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeEncryptionAlgorithm: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequiredSmimeSigningAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequiredSmimeSigningAlgorithm: usize,
    pub SetMustEncryptSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetMustSignSmimeMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManager {
    type Vtable = IEmailMailboxSyncManager_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxSyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x517ac55a_3591_4b5d_85bc_c71dde862263);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMailboxSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMailboxSyncManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManager2 {
    type Vtable = IEmailMailboxSyncManager2_Vtbl;
}
impl ::core::clone::Clone for IEmailMailboxSyncManager2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMailboxSyncManager2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8dc97e_95c1_4f89_81b7_e6aecb6695fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManager2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMailboxSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailManagerForUser {
    type Vtable = IEmailManagerForUser_Vtbl;
}
impl ::core::clone::Clone for IEmailManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailManagerForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf773de9f_3ca5_4b0f_90c1_156e40174ce5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailManagerStatics {
    type Vtable = IEmailManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IEmailManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5128654_55c5_4890_a824_216c2618ce7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowComposeNewEmailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeNewEmailAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailManagerStatics2 {
    type Vtable = IEmailManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IEmailManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac052da3_b194_425d_b6d9_d0f04135eda2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accesstype: EmailStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailManagerStatics3 {
    type Vtable = IEmailManagerStatics3_Vtbl;
}
impl ::core::clone::Clone for IEmailManagerStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a722395_843e_4945_b3aa_349e07a362c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMeetingInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMeetingInfo {
    type Vtable = IEmailMeetingInfo_Vtbl;
}
impl ::core::clone::Clone for IEmailMeetingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMeetingInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31c03fa9_7933_415f_a275_d165ba07026b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMeetingInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AppointmentRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAppointmentRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppointmentOriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppointmentOriginalStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub IsAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProposedStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, proposedstarttime: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ProposedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetProposedDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, duration: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProposedDuration: usize,
    #[cfg(feature = "Foundation")]
    pub RecurrenceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecurrenceStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecurrenceStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecurrenceStartTime: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    Recurrence: usize,
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub SetRecurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Appointments"))]
    SetRecurrence: usize,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMeetingInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMeetingInfo2 {
    type Vtable = IEmailMeetingInfo2_Vtbl;
}
impl ::core::clone::Clone for IEmailMeetingInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMeetingInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e59386d_b0d9_4fe5_867c_e31ed2b588b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMeetingInfo2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsReportedOutOfDateByServer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessage {
    type Vtable = IEmailMessage_Vtbl;
}
impl ::core::clone::Clone for IEmailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c6d948d_80b5_48f8_b0b1_e04e430f44e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub To: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    To: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CC: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Bcc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bcc: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessage2 {
    type Vtable = IEmailMessage2_Vtbl;
}
impl ::core::clone::Clone for IEmailMessage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfdc8248b_9f1a_44db_bd3c_65c384770f86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MailboxId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FolderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AllowInternetImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowInternetImages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub DownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageDownloadState) -> ::windows::core::HRESULT,
    pub SetDownloadState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageDownloadState) -> ::windows::core::HRESULT,
    pub EstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetEstimatedDownloadSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub FlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailFlagState) -> ::windows::core::HRESULT,
    pub SetFlagState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailFlagState) -> ::windows::core::HRESULT,
    pub HasPartialBodies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Importance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailImportance) -> ::windows::core::HRESULT,
    pub SetImportance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailImportance) -> ::windows::core::HRESULT,
    pub InResponseToMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IrmInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIrmInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsDraftMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsServerSearchMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSmartSendable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetMessageClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NormalizedSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OriginalCodePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetOriginalCodePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LastResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageResponseKind) -> ::windows::core::HRESULT,
    pub SetLastResponseKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageResponseKind) -> ::windows::core::HRESULT,
    pub Sender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SentTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetSentTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSentTime: usize,
    pub MeetingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMeetingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetBodyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetBodyStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetBodyStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: EmailMessageBodyKind, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetBodyStream: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessage3 {
    type Vtable = IEmailMessage3_Vtbl;
}
impl ::core::clone::Clone for IEmailMessage3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1ea675c_e598_4d29_a018_fc7b7eece0a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SmimeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SmimeData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetSmimeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetSmimeData: usize,
    pub SmimeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailMessageSmimeKind) -> ::windows::core::HRESULT,
    pub SetSmimeKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailMessageSmimeKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessage4 {
    type Vtable = IEmailMessage4_Vtbl;
}
impl ::core::clone::Clone for IEmailMessage4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x317cf181_3e7f_4a05_8394_3e10336dd435);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessage4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReplyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReplyTo: usize,
    pub SentRepresenting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSentRepresenting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessageBatch(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessageBatch {
    type Vtable = IEmailMessageBatch_Vtbl;
}
impl ::core::clone::Clone for IEmailMessageBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessageBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x605cd08f_25d9_4f1b_9e51_0514c0149653);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessageBatch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Messages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Messages: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailBatchStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailMessageReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailMessageReader {
    type Vtable = IEmailMessageReader_Vtbl;
}
impl ::core::clone::Clone for IEmailMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailMessageReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f4abe9f_6213_4a85_a3b0_f92d1a839d19);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMessageReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailQueryOptions {
    type Vtable = IEmailQueryOptions_Vtbl;
}
impl ::core::clone::Clone for IEmailQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailQueryOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45504b9b_3e7f_4d52_b6dd_d6fd4e1fbd9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextSearch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SortDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortDirection) -> ::windows::core::HRESULT,
    pub SetSortDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySortDirection) -> ::windows::core::HRESULT,
    pub SortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySortProperty) -> ::windows::core::HRESULT,
    pub SetSortProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySortProperty) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQueryKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQueryKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FolderIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FolderIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailQueryOptionsFactory {
    type Vtable = IEmailQueryOptionsFactory_Vtbl;
}
impl ::core::clone::Clone for IEmailQueryOptionsFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailQueryOptionsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88f1a1b8_78ab_4ee8_b4e3_046d6e2fe5e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryOptionsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithTextAndFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::HSTRING>, fields: EmailQuerySearchFields, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailQueryTextSearch(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailQueryTextSearch {
    type Vtable = IEmailQueryTextSearch_Vtbl;
}
impl ::core::clone::Clone for IEmailQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailQueryTextSearch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fa0a288_3c5d_46a5_a6e2_31d6fd17e540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailQueryTextSearch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Fields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchFields) -> ::windows::core::HRESULT,
    pub SetFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySearchFields) -> ::windows::core::HRESULT,
    pub SearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailQuerySearchScope) -> ::windows::core::HRESULT,
    pub SetSearchScope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailQuerySearchScope) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipient(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailRecipient {
    type Vtable = IEmailRecipient_Vtbl;
}
impl ::core::clone::Clone for IEmailRecipient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailRecipient {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcae825b3_4478_4814_b900_c902b5e19b53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipient_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailRecipientFactory {
    type Vtable = IEmailRecipientFactory_Vtbl;
}
impl ::core::clone::Clone for IEmailRecipientFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailRecipientFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5500b84d_c79a_4ef8_b909_722e18e3935d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::std::mem::MaybeUninit<::windows::core::HSTRING>, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailRecipientResolutionResult {
    type Vtable = IEmailRecipientResolutionResult_Vtbl;
}
impl ::core::clone::Clone for IEmailRecipientResolutionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailRecipientResolutionResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x918338fa_8d8d_4573_80d1_07172a34b98d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientResolutionResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut EmailRecipientResolutionStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub PublicKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    PublicKeys: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailRecipientResolutionResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailRecipientResolutionResult2 {
    type Vtable = IEmailRecipientResolutionResult2_Vtbl;
}
impl ::core::clone::Clone for IEmailRecipientResolutionResult2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailRecipientResolutionResult2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e420bb6_ce5b_4bde_b9d4_e16da0b09fca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailRecipientResolutionResult2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: EmailRecipientResolutionStatus) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub SetPublicKeys: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))]
    SetPublicKeys: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailStore {
    type Vtable = IEmailStore_Vtbl;
}
impl ::core::clone::Clone for IEmailStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf803226e_9137_4f8b_a470_279ac3058eb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindMailboxesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindMailboxesAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversationReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMessageReaderWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMailboxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, accountaddress: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateMailboxInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, accountaddress: ::std::mem::MaybeUninit<::windows::core::HSTRING>, userdataaccountid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateMailboxInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEmailStoreNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEmailStoreNotificationTriggerDetails {
    type Vtable = IEmailStoreNotificationTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IEmailStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEmailStoreNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce17563c_46e6_43c9_96f7_facf7dd710cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailAttachment(::windows::core::IUnknown);
impl EmailAttachment {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailAttachment, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn FileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FileName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFileName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetData)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ContentLocation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetContentLocation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DownloadState(&self) -> ::windows::core::Result<EmailAttachmentDownloadState> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailAttachmentDownloadState>();
            (::windows::core::Interface::vtable(this).DownloadState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDownloadState(&self, value: EmailAttachmentDownloadState) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDownloadState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).EstimatedDownloadSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEstimatedDownloadSizeInBytes(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetEstimatedDownloadSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFromBaseMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsFromBaseMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsInline(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsInline)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsInline(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsInline)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MimeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMimeType)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create<P0>(filename: &::windows::core::HSTRING, data: P0) -> ::windows::core::Result<EmailAttachment>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IEmailAttachmentFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailAttachment>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(filename), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Create2<P0>(filename: &::windows::core::HSTRING, data: P0, mimetype: &::windows::core::HSTRING) -> ::windows::core::Result<EmailAttachment>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IEmailAttachmentFactory2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailAttachment>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(filename), data.try_into_param()?.abi(), ::core::mem::transmute_copy(mimetype), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailAttachmentFactory<R, F: FnOnce(&IEmailAttachmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailAttachment, IEmailAttachmentFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IEmailAttachmentFactory2<R, F: FnOnce(&IEmailAttachmentFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailAttachment, IEmailAttachmentFactory2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailAttachment {}
impl ::core::fmt::Debug for EmailAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAttachment").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailAttachment {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailAttachment;{f353caf9-57c8-4adb-b992-60fceb584f54})");
}
impl ::core::clone::Clone for EmailAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailAttachment {
    type Vtable = IEmailAttachment_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailAttachment {
    const IID: ::windows::core::GUID = <IEmailAttachment as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailAttachment";
}
::windows::imp::interface_hierarchy!(EmailAttachment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailAttachment {}
unsafe impl ::core::marker::Sync for EmailAttachment {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailConversation(::windows::core::IUnknown);
impl EmailConversation {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MailboxId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FlagState(&self) -> ::windows::core::Result<EmailFlagState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailFlagState>();
            (::windows::core::Interface::vtable(this).FlagState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasAttachment(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasAttachment)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Importance(&self) -> ::windows::core::Result<EmailImportance> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailImportance>();
            (::windows::core::Interface::vtable(this).Importance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastEmailResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageResponseKind>();
            (::windows::core::Interface::vtable(this).LastEmailResponseKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MessageCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MostRecentMessageId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MostRecentMessageTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).MostRecentMessageTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Preview)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LatestSender(&self) -> ::windows::core::Result<EmailRecipient> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipient>();
            (::windows::core::Interface::vtable(this).LatestSender)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Subject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnreadMessageCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).UnreadMessageCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMessagesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>();
            (::windows::core::Interface::vtable(this).FindMessagesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMessagesWithCountAsync(&self, count: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMessage>>>();
            (::windows::core::Interface::vtable(this).FindMessagesWithCountAsync)(::windows::core::Interface::as_raw(this), count, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailConversation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversation {}
impl ::core::fmt::Debug for EmailConversation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailConversation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversation;{da18c248-a0bc-4349-902d-90f66389f51b})");
}
impl ::core::clone::Clone for EmailConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailConversation {
    type Vtable = IEmailConversation_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailConversation {
    const IID: ::windows::core::GUID = <IEmailConversation as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversation";
}
::windows::imp::interface_hierarchy!(EmailConversation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailConversation {}
unsafe impl ::core::marker::Sync for EmailConversation {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailConversationBatch(::windows::core::IUnknown);
impl EmailConversationBatch {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Conversations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<EmailConversation>>();
            (::windows::core::Interface::vtable(this).Conversations)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<EmailBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailBatchStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailConversationBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversationBatch {}
impl ::core::fmt::Debug for EmailConversationBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversationBatch").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailConversationBatch {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversationBatch;{b8c1ab81-01c5-432a-9df1-fe85d98a279a})");
}
impl ::core::clone::Clone for EmailConversationBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailConversationBatch {
    type Vtable = IEmailConversationBatch_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailConversationBatch {
    const IID: ::windows::core::GUID = <IEmailConversationBatch as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailConversationBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversationBatch";
}
::windows::imp::interface_hierarchy!(EmailConversationBatch, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailConversationBatch {}
unsafe impl ::core::marker::Sync for EmailConversationBatch {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailConversationReader(::windows::core::IUnknown);
impl EmailConversationReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversationBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailConversationBatch>>();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailConversationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailConversationReader {}
impl ::core::fmt::Debug for EmailConversationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailConversationReader").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailConversationReader {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailConversationReader;{b4630f82-2875-44c8-9b8c-85beb3a3c653})");
}
impl ::core::clone::Clone for EmailConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailConversationReader {
    type Vtable = IEmailConversationReader_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailConversationReader {
    const IID: ::windows::core::GUID = <IEmailConversationReader as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailConversationReader";
}
::windows::imp::interface_hierarchy!(EmailConversationReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailConversationReader {}
unsafe impl ::core::marker::Sync for EmailConversationReader {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailFolder(::windows::core::IUnknown);
impl EmailFolder {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemoteId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MailboxId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ParentFolderId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSyncEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSyncEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<EmailSpecialFolderKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailSpecialFolderKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailFolder>>();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindChildFoldersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailFolder>>>();
            (::windows::core::Interface::vtable(this).FindChildFoldersAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMessage>>();
            (::windows::core::Interface::vtable(this).GetMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageCountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailItemCounts>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailItemCounts>>();
            (::windows::core::Interface::vtable(this).GetMessageCountsAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveAsync(&self, newparentfolder: &EmailFolder) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryMoveAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(newparentfolder), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveWithNewNameAsync(&self, newparentfolder: &EmailFolder, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryMoveWithNewNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(newparentfolder), ::core::mem::transmute_copy(newfoldername), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySaveAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync(&self, message: &EmailMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailFolder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailFolder {}
impl ::core::fmt::Debug for EmailFolder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFolder").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailFolder {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailFolder;{a24f7771-996c-4864-b1ba-ed1240e57d11})");
}
impl ::core::clone::Clone for EmailFolder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailFolder {
    type Vtable = IEmailFolder_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailFolder {
    const IID: ::windows::core::GUID = <IEmailFolder as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailFolder {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailFolder";
}
::windows::imp::interface_hierarchy!(EmailFolder, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailFolder {}
unsafe impl ::core::marker::Sync for EmailFolder {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailIrmInfo(::windows::core::IUnknown);
impl EmailIrmInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailIrmInfo, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CanEdit(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanEdit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanEdit(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanEdit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanExtractData(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanExtractData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanExtractData(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanExtractData)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanForward)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanForward(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanForward)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanModifyRecipientsOnResponse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanModifyRecipientsOnResponse)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanModifyRecipientsOnResponse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanModifyRecipientsOnResponse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanPrintData(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanPrintData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanPrintData(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanPrintData)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanRemoveIrmOnResponse(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanRemoveIrmOnResponse)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanRemoveIrmOnResponse(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanRemoveIrmOnResponse)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanReply(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanReply)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanReply(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanReply)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanReplyAll(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanReplyAll)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanReplyAll(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCanReplyAll)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).ExpirationDate)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetExpirationDate(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetExpirationDate)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsIrmOriginator(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsIrmOriginator)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsIrmOriginator(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsIrmOriginator)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsProgramaticAccessAllowed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsProgramaticAccessAllowed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsProgramaticAccessAllowed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsProgramaticAccessAllowed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Template(&self) -> ::windows::core::Result<EmailIrmTemplate> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailIrmTemplate>();
            (::windows::core::Interface::vtable(this).Template)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTemplate(&self, value: &EmailIrmTemplate) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTemplate)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(expiration: super::super::Foundation::DateTime, irmtemplate: &EmailIrmTemplate) -> ::windows::core::Result<EmailIrmInfo> {
        Self::IEmailIrmInfoFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailIrmInfo>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), expiration, ::core::mem::transmute_copy(irmtemplate), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailIrmInfoFactory<R, F: FnOnce(&IEmailIrmInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailIrmInfo, IEmailIrmInfoFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailIrmInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailIrmInfo {}
impl ::core::fmt::Debug for EmailIrmInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailIrmInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailIrmInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailIrmInfo;{90f52193-b1a0-4ebd-a6b6-ddca55606e0e})");
}
impl ::core::clone::Clone for EmailIrmInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailIrmInfo {
    type Vtable = IEmailIrmInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailIrmInfo {
    const IID: ::windows::core::GUID = <IEmailIrmInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailIrmInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailIrmInfo";
}
::windows::imp::interface_hierarchy!(EmailIrmInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailIrmInfo {}
unsafe impl ::core::marker::Sync for EmailIrmInfo {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailIrmTemplate(::windows::core::IUnknown);
impl EmailIrmTemplate {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailIrmTemplate, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(id: &::windows::core::HSTRING, name: &::windows::core::HSTRING, description: &::windows::core::HSTRING) -> ::windows::core::Result<EmailIrmTemplate> {
        Self::IEmailIrmTemplateFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailIrmTemplate>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(description), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailIrmTemplateFactory<R, F: FnOnce(&IEmailIrmTemplateFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailIrmTemplate, IEmailIrmTemplateFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailIrmTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailIrmTemplate {}
impl ::core::fmt::Debug for EmailIrmTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailIrmTemplate").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailIrmTemplate {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailIrmTemplate;{f327758d-546d-4bea-a963-54a38b2cc016})");
}
impl ::core::clone::Clone for EmailIrmTemplate {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailIrmTemplate {
    type Vtable = IEmailIrmTemplate_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailIrmTemplate {
    const IID: ::windows::core::GUID = <IEmailIrmTemplate as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailIrmTemplate {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailIrmTemplate";
}
::windows::imp::interface_hierarchy!(EmailIrmTemplate, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailIrmTemplate {}
unsafe impl ::core::marker::Sync for EmailIrmTemplate {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailItemCounts(::windows::core::IUnknown);
impl EmailItemCounts {
    pub fn Flagged(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Flagged)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Important(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Important)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Total(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Total)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Unread(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Unread)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailItemCounts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailItemCounts {}
impl ::core::fmt::Debug for EmailItemCounts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailItemCounts").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailItemCounts {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailItemCounts;{5bd13321-fec8-4bab-83ba-0baf3c1f6cbd})");
}
impl ::core::clone::Clone for EmailItemCounts {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailItemCounts {
    type Vtable = IEmailItemCounts_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailItemCounts {
    const IID: ::windows::core::GUID = <IEmailItemCounts as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailItemCounts {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailItemCounts";
}
::windows::imp::interface_hierarchy!(EmailItemCounts, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailItemCounts {}
unsafe impl ::core::marker::Sync for EmailItemCounts {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailbox(::windows::core::IUnknown);
impl EmailMailbox {
    pub fn Capabilities(&self) -> ::windows::core::Result<EmailMailboxCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxCapabilities>();
            (::windows::core::Interface::vtable(this).Capabilities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeTracker(&self) -> ::windows::core::Result<EmailMailboxChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxChangeTracker>();
            (::windows::core::Interface::vtable(this).ChangeTracker)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOwnedByCurrentApp(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsOwnedByCurrentApp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDataEncryptedUnderLock(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDataEncryptedUnderLock)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MailAddress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMailAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMailAddress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MailAddressAliases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).MailAddressAliases)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxOtherAppReadAccess>();
            (::windows::core::Interface::vtable(this).OtherAppReadAccess)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: EmailMailboxOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppReadAccess)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<EmailMailboxOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxOtherAppWriteAccess>();
            (::windows::core::Interface::vtable(this).OtherAppWriteAccess)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: EmailMailboxOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppWriteAccess)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Policies(&self) -> ::windows::core::Result<EmailMailboxPolicies> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxPolicies>();
            (::windows::core::Interface::vtable(this).Policies)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SourceDisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows::core::Result<EmailMailboxSyncManager> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxSyncManager>();
            (::windows::core::Interface::vtable(this).SyncManager)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UserDataAccountId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailConversation>>();
            (::windows::core::Interface::vtable(this).GetConversationAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailFolder>>();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMessage>>();
            (::windows::core::Interface::vtable(this).GetMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSpecialFolderAsync(&self, foldertype: EmailSpecialFolderKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailFolder>>();
            (::windows::core::Interface::vtable(this).GetSpecialFolderAsync)(::windows::core::Interface::as_raw(this), foldertype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageAsSeenAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).MarkMessageAsSeenAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkFolderAsSeenAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).MarkFolderAsSeenAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageReadAsync(&self, messageid: &::windows::core::HSTRING, isread: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).MarkMessageReadAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), isread, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ChangeMessageFlagStateAsync(&self, messageid: &::windows::core::HSTRING, flagstate: EmailFlagState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ChangeMessageFlagStateAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), flagstate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveMessageAsync(&self, messageid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryMoveMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), ::core::mem::transmute_copy(newparentfolderid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFolderAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryMoveFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), ::core::mem::transmute_copy(newparentfolderid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryMoveFolderWithNewNameAsync(&self, folderid: &::windows::core::HSTRING, newparentfolderid: &::windows::core::HSTRING, newfoldername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryMoveFolderWithNewNameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), ::core::mem::transmute_copy(newparentfolderid), ::core::mem::transmute_copy(newfoldername), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DeleteMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkFolderSyncEnabledAsync(&self, folderid: &::windows::core::HSTRING, issyncenabled: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).MarkFolderSyncEnabledAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), issyncenabled, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAsync(&self, message: &EmailMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SendMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveDraftAsync(&self, message: &EmailMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveDraftAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageAsync(&self, messageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DownloadMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadAttachmentAsync(&self, attachmentid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).DownloadAttachmentAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(attachmentid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateResponseMessageAsync(&self, messageid: &::windows::core::HSTRING, responsetype: EmailMessageResponseKind, subject: &::windows::core::HSTRING, responseheadertype: EmailMessageBodyKind, responseheader: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMessage>>();
            (::windows::core::Interface::vtable(this).CreateResponseMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(messageid), responsetype, ::core::mem::transmute_copy(subject), responseheadertype, ::core::mem::transmute_copy(responseheader), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateMeetingResponseAsync(&self, meeting: &EmailMessage, response: EmailMeetingResponseType, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING, sendupdate: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryUpdateMeetingResponseAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(meeting), response, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(comment), sendupdate, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryForwardMeetingAsync<P0>(&self, meeting: &EmailMessage, recipients: P0, subject: &::windows::core::HSTRING, forwardheadertype: EmailMessageBodyKind, forwardheader: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<EmailRecipient>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryForwardMeetingAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(meeting), recipients.try_into_param()?.abi(), ::core::mem::transmute_copy(subject), forwardheadertype, ::core::mem::transmute_copy(forwardheader), ::core::mem::transmute_copy(comment), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryProposeNewTimeForMeetingAsync(&self, meeting: &EmailMessage, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: &::windows::core::HSTRING, comment: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryProposeNewTimeForMeetingAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(meeting), newstarttime, newduration, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(comment), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MailboxChanged(&self, phandler: &super::super::Foundation::TypedEventHandler<EmailMailbox, EmailMailboxChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MailboxChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(phandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMailboxChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMailboxChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SmartSendMessageAsync(&self, message: &EmailMessage, smartsend: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SmartSendMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), smartsend, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetAutoReplySettingsAsync(&self, autoreplysettings: &EmailMailboxAutoReplySettings) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TrySetAutoReplySettingsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(autoreplysettings), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetAutoReplySettingsAsync(&self, requestedformat: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailboxAutoReplySettings>>();
            (::windows::core::Interface::vtable(this).TryGetAutoReplySettingsAsync)(::windows::core::Interface::as_raw(this), requestedformat, &mut result__).from_abi(result__)
        }
    }
    pub fn LinkedMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LinkedMailboxId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NetworkAccountId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NetworkId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NetworkId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveRecipientsAsync<P0>(&self, recipients: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailRecipientResolutionResult>>>();
            (::windows::core::Interface::vtable(this).ResolveRecipientsAsync)(::windows::core::Interface::as_raw(this), recipients.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn ValidateCertificatesAsync<P0>(&self, certificates: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailCertificateValidationStatus>>>();
            (::windows::core::Interface::vtable(this).ValidateCertificatesAsync)(::windows::core::Interface::as_raw(this), certificates.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryEmptyFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailboxEmptyFolderStatus>>();
            (::windows::core::Interface::vtable(this).TryEmptyFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateFolderAsync(&self, parentfolderid: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailboxCreateFolderResult>>();
            (::windows::core::Interface::vtable(this).TryCreateFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(parentfolderid), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteFolderAsync(&self, folderid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailboxDeleteFolderStatus>>();
            (::windows::core::Interface::vtable(this).TryDeleteFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(folderid), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).RegisterSyncManagerAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows::core::HSTRING) -> ::windows::core::Result<EmailMailboxChangeTracker> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailbox5>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxChangeTracker>();
            (::windows::core::Interface::vtable(this).GetChangeTracker)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailbox {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailbox {}
impl ::core::fmt::Debug for EmailMailbox {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailbox").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailbox {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailbox;{a8790649-cf5b-411b-80b1-4a6a1484ce25})");
}
impl ::core::clone::Clone for EmailMailbox {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailbox {
    type Vtable = IEmailMailbox_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailbox {
    const IID: ::windows::core::GUID = <IEmailMailbox as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailbox {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailbox";
}
::windows::imp::interface_hierarchy!(EmailMailbox, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailbox {}
unsafe impl ::core::marker::Sync for EmailMailbox {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxAction(::windows::core::IUnknown);
impl EmailMailboxAction {
    pub fn Kind(&self) -> ::windows::core::Result<EmailMailboxActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxActionKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).ChangeNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAction {}
impl ::core::fmt::Debug for EmailMailboxAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAction").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxAction {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAction;{ac9889fa-21fa-4927-9210-d410582fdf3e})");
}
impl ::core::clone::Clone for EmailMailboxAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxAction {
    type Vtable = IEmailMailboxAction_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxAction {
    const IID: ::windows::core::GUID = <IEmailMailboxAction as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxAction {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAction";
}
::windows::imp::interface_hierarchy!(EmailMailboxAction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxAction {}
unsafe impl ::core::marker::Sync for EmailMailboxAction {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxAutoReply(::windows::core::IUnknown);
impl EmailMailboxAutoReply {
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Response(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Response)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResponse(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResponse)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAutoReply {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAutoReply {}
impl ::core::fmt::Debug for EmailMailboxAutoReply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReply").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxAutoReply {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAutoReply;{e223254c-8ab4-485b-b31f-04d15476bd59})");
}
impl ::core::clone::Clone for EmailMailboxAutoReply {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxAutoReply {
    type Vtable = IEmailMailboxAutoReply_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxAutoReply {
    const IID: ::windows::core::GUID = <IEmailMailboxAutoReply as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxAutoReply {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAutoReply";
}
::windows::imp::interface_hierarchy!(EmailMailboxAutoReply, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxAutoReply {}
unsafe impl ::core::marker::Sync for EmailMailboxAutoReply {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxAutoReplySettings(::windows::core::IUnknown);
impl EmailMailboxAutoReplySettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailMailboxAutoReplySettings, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResponseKind(&self) -> ::windows::core::Result<EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxAutoReplyMessageResponseKind>();
            (::windows::core::Interface::vtable(this).ResponseKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetResponseKind(&self, value: EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetResponseKind)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).EndTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEndTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn InternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxAutoReply>();
            (::windows::core::Interface::vtable(this).InternalReply)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn KnownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxAutoReply>();
            (::windows::core::Interface::vtable(this).KnownExternalReply)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnknownExternalReply(&self) -> ::windows::core::Result<EmailMailboxAutoReply> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxAutoReply>();
            (::windows::core::Interface::vtable(this).UnknownExternalReply)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxAutoReplySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxAutoReplySettings {}
impl ::core::fmt::Debug for EmailMailboxAutoReplySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReplySettings").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxAutoReplySettings {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxAutoReplySettings;{a87a9fa8-0ac6-4b77-ba77-a6b99e9a27b8})");
}
impl ::core::clone::Clone for EmailMailboxAutoReplySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxAutoReplySettings {
    type Vtable = IEmailMailboxAutoReplySettings_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxAutoReplySettings {
    const IID: ::windows::core::GUID = <IEmailMailboxAutoReplySettings as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxAutoReplySettings {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxAutoReplySettings";
}
::windows::imp::interface_hierarchy!(EmailMailboxAutoReplySettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxAutoReplySettings {}
unsafe impl ::core::marker::Sync for EmailMailboxAutoReplySettings {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxCapabilities(::windows::core::IUnknown);
impl EmailMailboxCapabilities {
    pub fn CanForwardMeetings(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanForwardMeetings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanGetAndSetExternalAutoReplies(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanGetAndSetExternalAutoReplies)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanGetAndSetInternalAutoReplies(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanGetAndSetInternalAutoReplies)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanUpdateMeetingResponses(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanUpdateMeetingResponses)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanServerSearchFolders(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanServerSearchFolders)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanServerSearchMailbox(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanServerSearchMailbox)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanProposeNewTimeForMeetings)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanSmartSend(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanSmartSend)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanResolveRecipients(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanResolveRecipients)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanValidateCertificates(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanValidateCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanEmptyFolder(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanEmptyFolder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanCreateFolder(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanCreateFolder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanDeleteFolder(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanDeleteFolder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanMoveFolder(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanMoveFolder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanForwardMeetings)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanGetAndSetExternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanGetAndSetExternalAutoReplies)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanGetAndSetInternalAutoReplies(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanGetAndSetInternalAutoReplies)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanUpdateMeetingResponses)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanServerSearchFolders(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanServerSearchFolders)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanServerSearchMailbox(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanServerSearchMailbox)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanProposeNewTimeForMeetings)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanSmartSend(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanSmartSend)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanResolveRecipients(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanResolveRecipients)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanValidateCertificates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanValidateCertificates)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanEmptyFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanEmptyFolder)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanCreateFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanCreateFolder)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanDeleteFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanDeleteFolder)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetCanMoveFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxCapabilities3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanMoveFolder)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCapabilities {}
impl ::core::fmt::Debug for EmailMailboxCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxCapabilities;{eedec3a6-89db-4305-82c4-439e0a33da11})");
}
impl ::core::clone::Clone for EmailMailboxCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxCapabilities {
    type Vtable = IEmailMailboxCapabilities_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxCapabilities {
    const IID: ::windows::core::GUID = <IEmailMailboxCapabilities as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxCapabilities";
}
::windows::imp::interface_hierarchy!(EmailMailboxCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxCapabilities {}
unsafe impl ::core::marker::Sync for EmailMailboxCapabilities {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChange(::windows::core::IUnknown);
impl EmailMailboxChange {
    pub fn ChangeType(&self) -> ::windows::core::Result<EmailMailboxChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxChangeType>();
            (::windows::core::Interface::vtable(this).ChangeType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MailboxActions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailMailboxAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailMailboxAction>>();
            (::windows::core::Interface::vtable(this).MailboxActions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<EmailMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessage>();
            (::windows::core::Interface::vtable(this).Message)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows::core::Result<EmailFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailFolder>();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChange {}
impl ::core::fmt::Debug for EmailMailboxChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChange").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChange {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChange;{61edf54b-11ef-400c-adde-8cde65c85e66})");
}
impl ::core::clone::Clone for EmailMailboxChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxChange {
    type Vtable = IEmailMailboxChange_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxChange {
    const IID: ::windows::core::GUID = <IEmailMailboxChange as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxChange {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChange";
}
::windows::imp::interface_hierarchy!(EmailMailboxChange, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxChange {}
unsafe impl ::core::marker::Sync for EmailMailboxChange {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChangeReader(::windows::core::IUnknown);
impl EmailMailboxChangeReader {
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcceptChanges)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough(&self, lastchangetoacknowledge: &EmailMailboxChange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AcceptChangesThrough)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(lastchangetoacknowledge)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailboxChange>>>();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangeReader {}
impl ::core::fmt::Debug for EmailMailboxChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeReader").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChangeReader {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangeReader;{bdbd0ebb-c53d-4331-97be-be75a2146a75})");
}
impl ::core::clone::Clone for EmailMailboxChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxChangeReader {
    type Vtable = IEmailMailboxChangeReader_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxChangeReader {
    const IID: ::windows::core::GUID = <IEmailMailboxChangeReader as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangeReader";
}
::windows::imp::interface_hierarchy!(EmailMailboxChangeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxChangeReader {}
unsafe impl ::core::marker::Sync for EmailMailboxChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChangeTracker(::windows::core::IUnknown);
impl EmailMailboxChangeTracker {
    pub fn IsTracking(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsTracking)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Enable)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows::core::Result<EmailMailboxChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxChangeReader>();
            (::windows::core::Interface::vtable(this).GetChangeReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Reset)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangeTracker {}
impl ::core::fmt::Debug for EmailMailboxChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeTracker").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChangeTracker {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangeTracker;{7ae48638-5166-42b7-8882-fd21c92bdd4b})");
}
impl ::core::clone::Clone for EmailMailboxChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxChangeTracker {
    type Vtable = IEmailMailboxChangeTracker_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxChangeTracker {
    const IID: ::windows::core::GUID = <IEmailMailboxChangeTracker as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangeTracker";
}
::windows::imp::interface_hierarchy!(EmailMailboxChangeTracker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxChangeTracker {}
unsafe impl ::core::marker::Sync for EmailMailboxChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChangedDeferral(::windows::core::IUnknown);
impl EmailMailboxChangedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangedDeferral {}
impl ::core::fmt::Debug for EmailMailboxChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangedDeferral").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChangedDeferral {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangedDeferral;{779a74c1-97c5-4b54-b30d-306232623e6d})");
}
impl ::core::clone::Clone for EmailMailboxChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxChangedDeferral {
    type Vtable = IEmailMailboxChangedDeferral_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxChangedDeferral {
    const IID: ::windows::core::GUID = <IEmailMailboxChangedDeferral as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangedDeferral";
}
::windows::imp::interface_hierarchy!(EmailMailboxChangedDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxChangedDeferral {}
unsafe impl ::core::marker::Sync for EmailMailboxChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxChangedEventArgs(::windows::core::IUnknown);
impl EmailMailboxChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<EmailMailboxChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxChangedDeferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxChangedEventArgs {}
impl ::core::fmt::Debug for EmailMailboxChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxChangedEventArgs;{3cfd5f6e-01d4-4e4a-a44c-b22dd42ec207})");
}
impl ::core::clone::Clone for EmailMailboxChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxChangedEventArgs {
    type Vtable = IEmailMailboxChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxChangedEventArgs {
    const IID: ::windows::core::GUID = <IEmailMailboxChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxChangedEventArgs";
}
::windows::imp::interface_hierarchy!(EmailMailboxChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxChangedEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxCreateFolderResult(::windows::core::IUnknown);
impl EmailMailboxCreateFolderResult {
    pub fn Status(&self) -> ::windows::core::Result<EmailMailboxCreateFolderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxCreateFolderStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Folder(&self) -> ::windows::core::Result<EmailFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailFolder>();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxCreateFolderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxCreateFolderResult {}
impl ::core::fmt::Debug for EmailMailboxCreateFolderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxCreateFolderResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxCreateFolderResult;{b228557f-2885-4998-b595-8a2d374ce950})");
}
impl ::core::clone::Clone for EmailMailboxCreateFolderResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxCreateFolderResult {
    type Vtable = IEmailMailboxCreateFolderResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxCreateFolderResult {
    const IID: ::windows::core::GUID = <IEmailMailboxCreateFolderResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxCreateFolderResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxCreateFolderResult";
}
::windows::imp::interface_hierarchy!(EmailMailboxCreateFolderResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderResult {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderResult {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxPolicies(::windows::core::IUnknown);
impl EmailMailboxPolicies {
    pub fn AllowedSmimeEncryptionAlgorithmNegotiation(&self) -> ::windows::core::Result<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation>();
            (::windows::core::Interface::vtable(this).AllowedSmimeEncryptionAlgorithmNegotiation)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowSmimeSoftCertificates(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowSmimeSoftCertificates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequiredSmimeEncryptionAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>();
            (::windows::core::Interface::vtable(this).RequiredSmimeEncryptionAlgorithm)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequiredSmimeSigningAlgorithm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>();
            (::windows::core::Interface::vtable(this).RequiredSmimeSigningAlgorithm)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MustEncryptSmimeMessages(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).MustEncryptSmimeMessages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MustSignSmimeMessages(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).MustSignSmimeMessages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowedSmimeEncryptionAlgorithmNegotiation(&self, value: EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowedSmimeEncryptionAlgorithmNegotiation)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAllowSmimeSoftCertificates(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowSmimeSoftCertificates)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequiredSmimeEncryptionAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<EmailMailboxSmimeEncryptionAlgorithm>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRequiredSmimeEncryptionAlgorithm)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRequiredSmimeSigningAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<EmailMailboxSmimeSigningAlgorithm>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRequiredSmimeSigningAlgorithm)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn SetMustEncryptSmimeMessages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMustEncryptSmimeMessages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMustSignSmimeMessages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxPolicies3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMustSignSmimeMessages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxPolicies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxPolicies {}
impl ::core::fmt::Debug for EmailMailboxPolicies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxPolicies").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxPolicies {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxPolicies;{1f3345c5-1c3b-4dc7-b410-6373783e545d})");
}
impl ::core::clone::Clone for EmailMailboxPolicies {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxPolicies {
    type Vtable = IEmailMailboxPolicies_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxPolicies {
    const IID: ::windows::core::GUID = <IEmailMailboxPolicies as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxPolicies {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxPolicies";
}
::windows::imp::interface_hierarchy!(EmailMailboxPolicies, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxPolicies {}
unsafe impl ::core::marker::Sync for EmailMailboxPolicies {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMailboxSyncManager(::windows::core::IUnknown);
impl EmailMailboxSyncManager {
    pub fn Status(&self) -> ::windows::core::Result<EmailMailboxSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMailboxSyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).LastSuccessfulSyncTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).LastAttemptedSyncTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).SyncAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged(&self, handler: &super::super::Foundation::TypedEventHandler<EmailMailboxSyncManager, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SyncStatusChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSyncStatusChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetStatus(&self, value: EmailMailboxSyncStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLastSuccessfulSyncTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMailboxSyncManager2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLastAttemptedSyncTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMailboxSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMailboxSyncManager {}
impl ::core::fmt::Debug for EmailMailboxSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxSyncManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMailboxSyncManager;{517ac55a-3591-4b5d-85bc-c71dde862263})");
}
impl ::core::clone::Clone for EmailMailboxSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMailboxSyncManager {
    type Vtable = IEmailMailboxSyncManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMailboxSyncManager {
    const IID: ::windows::core::GUID = <IEmailMailboxSyncManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMailboxSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMailboxSyncManager";
}
::windows::imp::interface_hierarchy!(EmailMailboxSyncManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMailboxSyncManager {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
pub struct EmailManager;
impl EmailManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeNewEmailAsync(message: &EmailMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IEmailManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowComposeNewEmailAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>> {
        Self::IEmailManagerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailStore>>();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::windows::core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<EmailManagerForUser> {
        Self::IEmailManagerStatics3(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailManagerForUser>();
            (::windows::core::Interface::vtable(this).GetForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailManagerStatics<R, F: FnOnce(&IEmailManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailManager, IEmailManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IEmailManagerStatics2<R, F: FnOnce(&IEmailManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailManager, IEmailManagerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IEmailManagerStatics3<R, F: FnOnce(&IEmailManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailManager, IEmailManagerStatics3> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for EmailManager {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailManager";
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailManagerForUser(::windows::core::IUnknown);
impl EmailManagerForUser {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeNewEmailAsync(&self, message: &EmailMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).ShowComposeNewEmailAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(message), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: EmailStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailStore>>();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::windows::core::Interface::as_raw(this), accesstype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailManagerForUser {}
impl ::core::fmt::Debug for EmailManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailManagerForUser").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailManagerForUser {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailManagerForUser;{f773de9f-3ca5-4b0f-90c1-156e40174ce5})");
}
impl ::core::clone::Clone for EmailManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailManagerForUser {
    type Vtable = IEmailManagerForUser_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailManagerForUser {
    const IID: ::windows::core::GUID = <IEmailManagerForUser as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailManagerForUser";
}
::windows::imp::interface_hierarchy!(EmailManagerForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailManagerForUser {}
unsafe impl ::core::marker::Sync for EmailManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMeetingInfo(::windows::core::IUnknown);
impl EmailMeetingInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailMeetingInfo, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AllowNewTimeProposal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowNewTimeProposal)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowNewTimeProposal)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppointmentRoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AppointmentRoamingId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppointmentRoamingId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppointmentRoamingId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppointmentOriginalStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).AppointmentOriginalStartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppointmentOriginalStartTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppointmentOriginalStartTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::TimeSpan>();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDuration)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAllDay(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAllDay)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsAllDay(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAllDay)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsResponseRequested(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsResponseRequested)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsResponseRequested)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Location)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProposedStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).ProposedStartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProposedStartTime<P0>(&self, proposedstarttime: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProposedStartTime)(::windows::core::Interface::as_raw(this), proposedstarttime.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProposedDuration(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>();
            (::windows::core::Interface::vtable(this).ProposedDuration)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProposedDuration<P0>(&self, duration: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProposedDuration)(::windows::core::Interface::as_raw(this), duration.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecurrenceStartTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).RecurrenceStartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRecurrenceStartTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecurrenceStartTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn Recurrence(&self) -> ::windows::core::Result<super::Appointments::AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::Appointments::AppointmentRecurrence>();
            (::windows::core::Interface::vtable(this).Recurrence)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Appointments\"`*"]
    #[cfg(feature = "ApplicationModel_Appointments")]
    pub fn SetRecurrence(&self, value: &super::Appointments::AppointmentRecurrence) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecurrence)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn RemoteChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).RemoteChangeNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteChangeNumber)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::DateTime>();
            (::windows::core::Interface::vtable(this).StartTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStartTime)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsReportedOutOfDateByServer(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMeetingInfo2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsReportedOutOfDateByServer)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMeetingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMeetingInfo {}
impl ::core::fmt::Debug for EmailMeetingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMeetingInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMeetingInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMeetingInfo;{31c03fa9-7933-415f-a275-d165ba07026b})");
}
impl ::core::clone::Clone for EmailMeetingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMeetingInfo {
    type Vtable = IEmailMeetingInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMeetingInfo {
    const IID: ::windows::core::GUID = <IEmailMeetingInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMeetingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMeetingInfo";
}
::windows::imp::interface_hierarchy!(EmailMeetingInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMeetingInfo {}
unsafe impl ::core::marker::Sync for EmailMeetingInfo {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessage(::windows::core::IUnknown);
impl EmailMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailMessage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Subject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubject)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Body)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBody)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn To(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailRecipient>>();
            (::windows::core::Interface::vtable(this).To)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CC(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailRecipient>>();
            (::windows::core::Interface::vtable(this).CC)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bcc(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailRecipient>>();
            (::windows::core::Interface::vtable(this).Bcc)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailAttachment>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailAttachment>>();
            (::windows::core::Interface::vtable(this).Attachments)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RemoteId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn MailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MailboxId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ConversationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FolderId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AllowInternetImages(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).AllowInternetImages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAllowInternetImages(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowInternetImages)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChangeNumber(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u64>();
            (::windows::core::Interface::vtable(this).ChangeNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DownloadState(&self) -> ::windows::core::Result<EmailMessageDownloadState> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageDownloadState>();
            (::windows::core::Interface::vtable(this).DownloadState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDownloadState(&self, value: EmailMessageDownloadState) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDownloadState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EstimatedDownloadSizeInBytes(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).EstimatedDownloadSizeInBytes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEstimatedDownloadSizeInBytes(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetEstimatedDownloadSizeInBytes)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn FlagState(&self) -> ::windows::core::Result<EmailFlagState> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailFlagState>();
            (::windows::core::Interface::vtable(this).FlagState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFlagState(&self, value: EmailFlagState) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFlagState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn HasPartialBodies(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasPartialBodies)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Importance(&self) -> ::windows::core::Result<EmailImportance> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailImportance>();
            (::windows::core::Interface::vtable(this).Importance)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetImportance(&self, value: EmailImportance) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetImportance)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn InResponseToMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).InResponseToMessageId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IrmInfo(&self) -> ::windows::core::Result<EmailIrmInfo> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailIrmInfo>();
            (::windows::core::Interface::vtable(this).IrmInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIrmInfo(&self, value: &EmailIrmInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIrmInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsDraftMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDraftMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsRead)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsRead)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSeen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSeen)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsServerSearchMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsServerSearchMessage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSmartSendable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsSmartSendable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MessageClass(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).MessageClass)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessageClass(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMessageClass)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NormalizedSubject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NormalizedSubject)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OriginalCodePage(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).OriginalCodePage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOriginalCodePage(&self, value: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOriginalCodePage)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Preview(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Preview)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPreview(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPreview)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn LastResponseKind(&self) -> ::windows::core::Result<EmailMessageResponseKind> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageResponseKind>();
            (::windows::core::Interface::vtable(this).LastResponseKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLastResponseKind(&self, value: EmailMessageResponseKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLastResponseKind)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Sender(&self) -> ::windows::core::Result<EmailRecipient> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipient>();
            (::windows::core::Interface::vtable(this).Sender)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSender(&self, value: &EmailRecipient) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSender)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SentTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>();
            (::windows::core::Interface::vtable(this).SentTime)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSentTime<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSentTime)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn MeetingInfo(&self) -> ::windows::core::Result<EmailMeetingInfo> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMeetingInfo>();
            (::windows::core::Interface::vtable(this).MeetingInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMeetingInfo(&self, value: &EmailMeetingInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMeetingInfo)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetBodyStream(&self, r#type: EmailMessageBodyKind) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).GetBodyStream)(::windows::core::Interface::as_raw(this), r#type, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetBodyStream<P0>(&self, r#type: EmailMessageBodyKind, stream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBodyStream)(::windows::core::Interface::as_raw(this), r#type, stream.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SmimeData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamReference>();
            (::windows::core::Interface::vtable(this).SmimeData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSmimeData<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSmimeData)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn SmimeKind(&self) -> ::windows::core::Result<EmailMessageSmimeKind> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageSmimeKind>();
            (::windows::core::Interface::vtable(this).SmimeKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSmimeKind(&self, value: EmailMessageSmimeKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSmimeKind)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplyTo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<EmailRecipient>> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<EmailRecipient>>();
            (::windows::core::Interface::vtable(this).ReplyTo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SentRepresenting(&self) -> ::windows::core::Result<EmailRecipient> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage4>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipient>();
            (::windows::core::Interface::vtable(this).SentRepresenting)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSentRepresenting(&self, value: &EmailRecipient) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailMessage4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSentRepresenting)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessage {}
impl ::core::fmt::Debug for EmailMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessage;{6c6d948d-80b5-48f8-b0b1-e04e430f44e5})");
}
impl ::core::clone::Clone for EmailMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMessage {
    type Vtable = IEmailMessage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMessage {
    const IID: ::windows::core::GUID = <IEmailMessage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessage";
}
::windows::imp::interface_hierarchy!(EmailMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMessage {}
unsafe impl ::core::marker::Sync for EmailMessage {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageBatch(::windows::core::IUnknown);
impl EmailMessageBatch {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Messages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<EmailMessage>>();
            (::windows::core::Interface::vtable(this).Messages)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<EmailBatchStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailBatchStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMessageBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessageBatch {}
impl ::core::fmt::Debug for EmailMessageBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageBatch").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageBatch {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessageBatch;{605cd08f-25d9-4f1b-9e51-0514c0149653})");
}
impl ::core::clone::Clone for EmailMessageBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMessageBatch {
    type Vtable = IEmailMessageBatch_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMessageBatch {
    const IID: ::windows::core::GUID = <IEmailMessageBatch as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMessageBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessageBatch";
}
::windows::imp::interface_hierarchy!(EmailMessageBatch, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMessageBatch {}
unsafe impl ::core::marker::Sync for EmailMessageBatch {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailMessageReader(::windows::core::IUnknown);
impl EmailMessageReader {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessageBatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMessageBatch>>();
            (::windows::core::Interface::vtable(this).ReadBatchAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailMessageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailMessageReader {}
impl ::core::fmt::Debug for EmailMessageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageReader").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageReader {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailMessageReader;{2f4abe9f-6213-4a85-a3b0-f92d1a839d19})");
}
impl ::core::clone::Clone for EmailMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailMessageReader {
    type Vtable = IEmailMessageReader_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailMessageReader {
    const IID: ::windows::core::GUID = <IEmailMessageReader as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailMessageReader";
}
::windows::imp::interface_hierarchy!(EmailMessageReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailMessageReader {}
unsafe impl ::core::marker::Sync for EmailMessageReader {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQueryOptions(::windows::core::IUnknown);
impl EmailQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailQueryOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextSearch(&self) -> ::windows::core::Result<EmailQueryTextSearch> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQueryTextSearch>();
            (::windows::core::Interface::vtable(this).TextSearch)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SortDirection(&self) -> ::windows::core::Result<EmailQuerySortDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQuerySortDirection>();
            (::windows::core::Interface::vtable(this).SortDirection)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSortDirection(&self, value: EmailQuerySortDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSortDirection)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SortProperty(&self) -> ::windows::core::Result<EmailQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQuerySortProperty>();
            (::windows::core::Interface::vtable(this).SortProperty)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSortProperty(&self, value: EmailQuerySortProperty) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSortProperty)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<EmailQueryKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQueryKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: EmailQueryKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetKind)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FolderIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).FolderIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateWithText(text: &::windows::core::HSTRING) -> ::windows::core::Result<EmailQueryOptions> {
        Self::IEmailQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQueryOptions>();
            (::windows::core::Interface::vtable(this).CreateWithText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithTextAndFields(text: &::windows::core::HSTRING, fields: EmailQuerySearchFields) -> ::windows::core::Result<EmailQueryOptions> {
        Self::IEmailQueryOptionsFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQueryOptions>();
            (::windows::core::Interface::vtable(this).CreateWithTextAndFields)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text), fields, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailQueryOptionsFactory<R, F: FnOnce(&IEmailQueryOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailQueryOptions, IEmailQueryOptionsFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailQueryOptions {}
impl ::core::fmt::Debug for EmailQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQueryOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailQueryOptions;{45504b9b-3e7f-4d52-b6dd-d6fd4e1fbd9a})");
}
impl ::core::clone::Clone for EmailQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailQueryOptions {
    type Vtable = IEmailQueryOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailQueryOptions {
    const IID: ::windows::core::GUID = <IEmailQueryOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailQueryOptions";
}
::windows::imp::interface_hierarchy!(EmailQueryOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailQueryOptions {}
unsafe impl ::core::marker::Sync for EmailQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailQueryTextSearch(::windows::core::IUnknown);
impl EmailQueryTextSearch {
    pub fn Fields(&self) -> ::windows::core::Result<EmailQuerySearchFields> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQuerySearchFields>();
            (::windows::core::Interface::vtable(this).Fields)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFields(&self, value: EmailQuerySearchFields) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFields)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SearchScope(&self) -> ::windows::core::Result<EmailQuerySearchScope> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailQuerySearchScope>();
            (::windows::core::Interface::vtable(this).SearchScope)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSearchScope(&self, value: EmailQuerySearchScope) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSearchScope)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailQueryTextSearch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailQueryTextSearch {}
impl ::core::fmt::Debug for EmailQueryTextSearch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryTextSearch").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQueryTextSearch {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailQueryTextSearch;{9fa0a288-3c5d-46a5-a6e2-31d6fd17e540})");
}
impl ::core::clone::Clone for EmailQueryTextSearch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailQueryTextSearch {
    type Vtable = IEmailQueryTextSearch_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailQueryTextSearch {
    const IID: ::windows::core::GUID = <IEmailQueryTextSearch as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailQueryTextSearch";
}
::windows::imp::interface_hierarchy!(EmailQueryTextSearch, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailQueryTextSearch {}
unsafe impl ::core::marker::Sync for EmailQueryTextSearch {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailRecipient(::windows::core::IUnknown);
impl EmailRecipient {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailRecipient, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAddress)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(address: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient> {
        Self::IEmailRecipientFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipient>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(address), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithName(address: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<EmailRecipient> {
        Self::IEmailRecipientFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipient>();
            (::windows::core::Interface::vtable(this).CreateWithName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(address), ::core::mem::transmute_copy(name), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEmailRecipientFactory<R, F: FnOnce(&IEmailRecipientFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailRecipient, IEmailRecipientFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for EmailRecipient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailRecipient {}
impl ::core::fmt::Debug for EmailRecipient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipient").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailRecipient {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailRecipient;{cae825b3-4478-4814-b900-c902b5e19b53})");
}
impl ::core::clone::Clone for EmailRecipient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailRecipient {
    type Vtable = IEmailRecipient_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailRecipient {
    const IID: ::windows::core::GUID = <IEmailRecipient as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailRecipient {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailRecipient";
}
::windows::imp::interface_hierarchy!(EmailRecipient, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailRecipient {}
unsafe impl ::core::marker::Sync for EmailRecipient {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailRecipientResolutionResult(::windows::core::IUnknown);
impl EmailRecipientResolutionResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<EmailRecipientResolutionResult, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<EmailRecipientResolutionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailRecipientResolutionStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn PublicKeys(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>();
            (::windows::core::Interface::vtable(this).PublicKeys)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStatus(&self, value: EmailRecipientResolutionStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IEmailRecipientResolutionResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Security_Cryptography_Certificates\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    pub fn SetPublicKeys<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Security::Cryptography::Certificates::Certificate>>,
    {
        let this = &::windows::core::ComInterface::cast::<IEmailRecipientResolutionResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPublicKeys)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for EmailRecipientResolutionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailRecipientResolutionResult {}
impl ::core::fmt::Debug for EmailRecipientResolutionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipientResolutionResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailRecipientResolutionResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailRecipientResolutionResult;{918338fa-8d8d-4573-80d1-07172a34b98d})");
}
impl ::core::clone::Clone for EmailRecipientResolutionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailRecipientResolutionResult {
    type Vtable = IEmailRecipientResolutionResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailRecipientResolutionResult {
    const IID: ::windows::core::GUID = <IEmailRecipientResolutionResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailRecipientResolutionResult {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailRecipientResolutionResult";
}
::windows::imp::interface_hierarchy!(EmailRecipientResolutionResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailRecipientResolutionResult {}
unsafe impl ::core::marker::Sync for EmailRecipientResolutionResult {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailStore(::windows::core::IUnknown);
impl EmailStore {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindMailboxesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<EmailMailbox>>>();
            (::windows::core::Interface::vtable(this).FindMailboxesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetConversationReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailConversationReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailConversationReader>();
            (::windows::core::Interface::vtable(this).GetConversationReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReader)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetMessageReaderWithOptions(&self, options: &EmailQueryOptions) -> ::windows::core::Result<EmailMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<EmailMessageReader>();
            (::windows::core::Interface::vtable(this).GetMessageReaderWithOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(options), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMailboxAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailbox>>();
            (::windows::core::Interface::vtable(this).GetMailboxAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailConversation>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailConversation>>();
            (::windows::core::Interface::vtable(this).GetConversationAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailFolder>>();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMessage>>();
            (::windows::core::Interface::vtable(this).GetMessageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMailboxAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailbox>>();
            (::windows::core::Interface::vtable(this).CreateMailboxAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(accountname), ::core::mem::transmute_copy(accountaddress), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateMailboxInAccountAsync(&self, accountname: &::windows::core::HSTRING, accountaddress: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<EmailMailbox>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<EmailMailbox>>();
            (::windows::core::Interface::vtable(this).CreateMailboxInAccountAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(accountname), ::core::mem::transmute_copy(accountaddress), ::core::mem::transmute_copy(userdataaccountid), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for EmailStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStore {}
impl ::core::fmt::Debug for EmailStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStore").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailStore {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailStore;{f803226e-9137-4f8b-a470-279ac3058eb6})");
}
impl ::core::clone::Clone for EmailStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailStore {
    type Vtable = IEmailStore_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailStore {
    const IID: ::windows::core::GUID = <IEmailStore as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailStore {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailStore";
}
::windows::imp::interface_hierarchy!(EmailStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailStore {}
unsafe impl ::core::marker::Sync for EmailStore {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
pub struct EmailStoreNotificationTriggerDetails(::windows::core::IUnknown);
impl EmailStoreNotificationTriggerDetails {}
impl ::core::cmp::PartialEq for EmailStoreNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EmailStoreNotificationTriggerDetails {}
impl ::core::fmt::Debug for EmailStoreNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStoreNotificationTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.EmailStoreNotificationTriggerDetails;{ce17563c-46e6-43c9-96f7-facf7dd710cb})");
}
impl ::core::clone::Clone for EmailStoreNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for EmailStoreNotificationTriggerDetails {
    type Vtable = IEmailStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for EmailStoreNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IEmailStoreNotificationTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for EmailStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.EmailStoreNotificationTriggerDetails";
}
::windows::imp::interface_hierarchy!(EmailStoreNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for EmailStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for EmailStoreNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailAttachmentDownloadState(pub i32);
impl EmailAttachmentDownloadState {
    pub const NotDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailAttachmentDownloadState {}
impl ::core::clone::Clone for EmailAttachmentDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailAttachmentDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailAttachmentDownloadState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailAttachmentDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailAttachmentDownloadState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailAttachmentDownloadState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailAttachmentDownloadState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailBatchStatus(pub i32);
impl EmailBatchStatus {
    pub const Success: Self = Self(0i32);
    pub const ServerSearchSyncManagerError: Self = Self(1i32);
    pub const ServerSearchUnknownError: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailBatchStatus {}
impl ::core::clone::Clone for EmailBatchStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailBatchStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailBatchStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailBatchStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailBatchStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailBatchStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailBatchStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailCertificateValidationStatus(pub i32);
impl EmailCertificateValidationStatus {
    pub const Success: Self = Self(0i32);
    pub const NoMatch: Self = Self(1i32);
    pub const InvalidUsage: Self = Self(2i32);
    pub const InvalidCertificate: Self = Self(3i32);
    pub const Revoked: Self = Self(4i32);
    pub const ChainRevoked: Self = Self(5i32);
    pub const RevocationServerFailure: Self = Self(6i32);
    pub const Expired: Self = Self(7i32);
    pub const Untrusted: Self = Self(8i32);
    pub const ServerError: Self = Self(9i32);
    pub const UnknownFailure: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailCertificateValidationStatus {}
impl ::core::clone::Clone for EmailCertificateValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailCertificateValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailCertificateValidationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailCertificateValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailCertificateValidationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailCertificateValidationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailCertificateValidationStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailFlagState(pub i32);
impl EmailFlagState {
    pub const Unflagged: Self = Self(0i32);
    pub const Flagged: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
    pub const Cleared: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailFlagState {}
impl ::core::clone::Clone for EmailFlagState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailFlagState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailFlagState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailFlagState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailFlagState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailFlagState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailFlagState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailImportance(pub i32);
impl EmailImportance {
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailImportance {}
impl ::core::clone::Clone for EmailImportance {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailImportance {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailImportance {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailImportance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailImportance").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailImportance {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailImportance;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxActionKind(pub i32);
impl EmailMailboxActionKind {
    pub const MarkMessageAsSeen: Self = Self(0i32);
    pub const MarkMessageRead: Self = Self(1i32);
    pub const ChangeMessageFlagState: Self = Self(2i32);
    pub const MoveMessage: Self = Self(3i32);
    pub const SaveDraft: Self = Self(4i32);
    pub const SendMessage: Self = Self(5i32);
    pub const CreateResponseReplyMessage: Self = Self(6i32);
    pub const CreateResponseReplyAllMessage: Self = Self(7i32);
    pub const CreateResponseForwardMessage: Self = Self(8i32);
    pub const MoveFolder: Self = Self(9i32);
    pub const MarkFolderForSyncEnabled: Self = Self(10i32);
}
impl ::core::marker::Copy for EmailMailboxActionKind {}
impl ::core::clone::Clone for EmailMailboxActionKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxActionKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxActionKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxActionKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxActionKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxActionKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxActionKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation(pub i32);
impl EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    pub const None: Self = Self(0i32);
    pub const StrongAlgorithm: Self = Self(1i32);
    pub const AnyAlgorithm: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {}
impl ::core::clone::Clone for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxAllowedSmimeEncryptionAlgorithmNegotiation;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxAutoReplyMessageResponseKind(pub i32);
impl EmailMailboxAutoReplyMessageResponseKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxAutoReplyMessageResponseKind {}
impl ::core::clone::Clone for EmailMailboxAutoReplyMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxAutoReplyMessageResponseKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxAutoReplyMessageResponseKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxAutoReplyMessageResponseKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxAutoReplyMessageResponseKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxAutoReplyMessageResponseKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxAutoReplyMessageResponseKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxChangeType(pub i32);
impl EmailMailboxChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const FolderCreated: Self = Self(3i32);
    pub const FolderModified: Self = Self(4i32);
    pub const FolderDeleted: Self = Self(5i32);
    pub const ChangeTrackingLost: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxChangeType {}
impl ::core::clone::Clone for EmailMailboxChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxChangeType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxChangeType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxChangeType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxChangeType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxCreateFolderStatus(pub i32);
impl EmailMailboxCreateFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const NameCollision: Self = Self(5i32);
    pub const ServerRejected: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxCreateFolderStatus {}
impl ::core::clone::Clone for EmailMailboxCreateFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxCreateFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxCreateFolderStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxCreateFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxCreateFolderStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxCreateFolderStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxCreateFolderStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxDeleteFolderStatus(pub i32);
impl EmailMailboxDeleteFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxDeleteFolderStatus {}
impl ::core::clone::Clone for EmailMailboxDeleteFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxDeleteFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxDeleteFolderStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxDeleteFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxDeleteFolderStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxDeleteFolderStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxDeleteFolderStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxEmptyFolderStatus(pub i32);
impl EmailMailboxEmptyFolderStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const PermissionsError: Self = Self(2i32);
    pub const ServerError: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
    pub const CouldNotDeleteEverything: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxEmptyFolderStatus {}
impl ::core::clone::Clone for EmailMailboxEmptyFolderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxEmptyFolderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxEmptyFolderStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxEmptyFolderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxEmptyFolderStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxEmptyFolderStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxEmptyFolderStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxOtherAppReadAccess(pub i32);
impl EmailMailboxOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppReadAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxOtherAppReadAccess {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxOtherAppReadAccess {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxOtherAppReadAccess;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxOtherAppWriteAccess(pub i32);
impl EmailMailboxOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMailboxOtherAppWriteAccess {}
impl ::core::clone::Clone for EmailMailboxOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxOtherAppWriteAccess {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxOtherAppWriteAccess {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxOtherAppWriteAccess;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxSmimeEncryptionAlgorithm(pub i32);
impl EmailMailboxSmimeEncryptionAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const TripleDes: Self = Self(1i32);
    pub const Des: Self = Self(2i32);
    pub const RC2128Bit: Self = Self(3i32);
    pub const RC264Bit: Self = Self(4i32);
    pub const RC240Bit: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeEncryptionAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeEncryptionAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSmimeEncryptionAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxSmimeEncryptionAlgorithm {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxSmimeEncryptionAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSmimeEncryptionAlgorithm").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxSmimeEncryptionAlgorithm {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSmimeEncryptionAlgorithm;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxSmimeSigningAlgorithm(pub i32);
impl EmailMailboxSmimeSigningAlgorithm {
    pub const Any: Self = Self(0i32);
    pub const Sha1: Self = Self(1i32);
    pub const MD5: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMailboxSmimeSigningAlgorithm {}
impl ::core::clone::Clone for EmailMailboxSmimeSigningAlgorithm {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSmimeSigningAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxSmimeSigningAlgorithm {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxSmimeSigningAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSmimeSigningAlgorithm").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxSmimeSigningAlgorithm {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSmimeSigningAlgorithm;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMailboxSyncStatus(pub i32);
impl EmailMailboxSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailMailboxSyncStatus {}
impl ::core::clone::Clone for EmailMailboxSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMailboxSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMailboxSyncStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMailboxSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMailboxSyncStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMailboxSyncStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMailboxSyncStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMeetingResponseType(pub i32);
impl EmailMeetingResponseType {
    pub const Accept: Self = Self(0i32);
    pub const Decline: Self = Self(1i32);
    pub const Tentative: Self = Self(2i32);
}
impl ::core::marker::Copy for EmailMeetingResponseType {}
impl ::core::clone::Clone for EmailMeetingResponseType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMeetingResponseType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMeetingResponseType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMeetingResponseType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMeetingResponseType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMeetingResponseType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMeetingResponseType;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMessageBodyKind(pub i32);
impl EmailMessageBodyKind {
    pub const Html: Self = Self(0i32);
    pub const PlainText: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailMessageBodyKind {}
impl ::core::clone::Clone for EmailMessageBodyKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageBodyKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMessageBodyKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMessageBodyKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageBodyKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageBodyKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageBodyKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMessageDownloadState(pub i32);
impl EmailMessageDownloadState {
    pub const PartiallyDownloaded: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const Downloaded: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageDownloadState {}
impl ::core::clone::Clone for EmailMessageDownloadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageDownloadState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMessageDownloadState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMessageDownloadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageDownloadState").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageDownloadState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageDownloadState;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMessageResponseKind(pub i32);
impl EmailMessageResponseKind {
    pub const None: Self = Self(0i32);
    pub const Reply: Self = Self(1i32);
    pub const ReplyAll: Self = Self(2i32);
    pub const Forward: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageResponseKind {}
impl ::core::clone::Clone for EmailMessageResponseKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageResponseKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMessageResponseKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMessageResponseKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageResponseKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageResponseKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageResponseKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailMessageSmimeKind(pub i32);
impl EmailMessageSmimeKind {
    pub const None: Self = Self(0i32);
    pub const ClearSigned: Self = Self(1i32);
    pub const OpaqueSigned: Self = Self(2i32);
    pub const Encrypted: Self = Self(3i32);
}
impl ::core::marker::Copy for EmailMessageSmimeKind {}
impl ::core::clone::Clone for EmailMessageSmimeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailMessageSmimeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailMessageSmimeKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailMessageSmimeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailMessageSmimeKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailMessageSmimeKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailMessageSmimeKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailQueryKind(pub i32);
impl EmailQueryKind {
    pub const All: Self = Self(0i32);
    pub const Important: Self = Self(1i32);
    pub const Flagged: Self = Self(2i32);
    pub const Unread: Self = Self(3i32);
    pub const Read: Self = Self(4i32);
    pub const Unseen: Self = Self(5i32);
}
impl ::core::marker::Copy for EmailQueryKind {}
impl ::core::clone::Clone for EmailQueryKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQueryKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailQueryKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailQueryKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQueryKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQueryKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQueryKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailQuerySearchFields(pub u32);
impl EmailQuerySearchFields {
    pub const None: Self = Self(0u32);
    pub const Subject: Self = Self(1u32);
    pub const Sender: Self = Self(2u32);
    pub const Preview: Self = Self(4u32);
    pub const Recipients: Self = Self(8u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for EmailQuerySearchFields {}
impl ::core::clone::Clone for EmailQuerySearchFields {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySearchFields {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailQuerySearchFields {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailQuerySearchFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySearchFields").field(&self.0).finish()
    }
}
impl EmailQuerySearchFields {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for EmailQuerySearchFields {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for EmailQuerySearchFields {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for EmailQuerySearchFields {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for EmailQuerySearchFields {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for EmailQuerySearchFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for EmailQuerySearchFields {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySearchFields;u4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailQuerySearchScope(pub i32);
impl EmailQuerySearchScope {
    pub const Local: Self = Self(0i32);
    pub const Server: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySearchScope {}
impl ::core::clone::Clone for EmailQuerySearchScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySearchScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailQuerySearchScope {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailQuerySearchScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySearchScope").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQuerySearchScope {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySearchScope;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailQuerySortDirection(pub i32);
impl EmailQuerySortDirection {
    pub const Descending: Self = Self(0i32);
    pub const Ascending: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailQuerySortDirection {}
impl ::core::clone::Clone for EmailQuerySortDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySortDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailQuerySortDirection {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailQuerySortDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySortDirection").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQuerySortDirection {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySortDirection;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailQuerySortProperty(pub i32);
impl EmailQuerySortProperty {
    pub const Date: Self = Self(0i32);
}
impl ::core::marker::Copy for EmailQuerySortProperty {}
impl ::core::clone::Clone for EmailQuerySortProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailQuerySortProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailQuerySortProperty {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailQuerySortProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailQuerySortProperty").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailQuerySortProperty {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailQuerySortProperty;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailRecipientResolutionStatus(pub i32);
impl EmailRecipientResolutionStatus {
    pub const Success: Self = Self(0i32);
    pub const RecipientNotFound: Self = Self(1i32);
    pub const AmbiguousRecipient: Self = Self(2i32);
    pub const NoCertificate: Self = Self(3i32);
    pub const CertificateRequestLimitReached: Self = Self(4i32);
    pub const CannotResolveDistributionList: Self = Self(5i32);
    pub const ServerError: Self = Self(6i32);
    pub const UnknownFailure: Self = Self(7i32);
}
impl ::core::marker::Copy for EmailRecipientResolutionStatus {}
impl ::core::clone::Clone for EmailRecipientResolutionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailRecipientResolutionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailRecipientResolutionStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailRecipientResolutionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailRecipientResolutionStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailRecipientResolutionStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailRecipientResolutionStatus;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailSpecialFolderKind(pub i32);
impl EmailSpecialFolderKind {
    pub const None: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const Inbox: Self = Self(2i32);
    pub const Outbox: Self = Self(3i32);
    pub const Drafts: Self = Self(4i32);
    pub const DeletedItems: Self = Self(5i32);
    pub const Sent: Self = Self(6i32);
}
impl ::core::marker::Copy for EmailSpecialFolderKind {}
impl ::core::clone::Clone for EmailSpecialFolderKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailSpecialFolderKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailSpecialFolderKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailSpecialFolderKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailSpecialFolderKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailSpecialFolderKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailSpecialFolderKind;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_Email\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EmailStoreAccessType(pub i32);
impl EmailStoreAccessType {
    pub const AppMailboxesReadWrite: Self = Self(0i32);
    pub const AllMailboxesLimitedReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for EmailStoreAccessType {}
impl ::core::clone::Clone for EmailStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EmailStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for EmailStoreAccessType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for EmailStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EmailStoreAccessType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for EmailStoreAccessType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Email.EmailStoreAccessType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
