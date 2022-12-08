#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatCapabilities {
    type Vtable = IChatCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3aff77bc_39c9_4dd1_ad2d_3964dd9d403f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsOnline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsChatCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsFileTransferCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGeoLocationPushCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsIntegratedMessagingCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatCapabilitiesManagerStatics {
    type Vtable = IChatCapabilitiesManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatCapabilitiesManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb57a2f30_7041_458e_b0cf_7c0d9fea333a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatCapabilitiesManagerStatics2 {
    type Vtable = IChatCapabilitiesManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatCapabilitiesManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30d4274_d5c1_4ac9_9ffc_40e69184fec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetCachedCapabilitiesForTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCachedCapabilitiesForTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCapabilitiesFromNetworkForTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCapabilitiesFromNetworkForTransportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatConversation {
    type Vtable = IChatConversation_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatConversation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa58c080d_1a6f_46dc_8f3d_f5028660b6ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasUnreadMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsConversationMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsConversationMuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub MostRecentMessageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    pub GetMessageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MarkAllMessagesAsReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAllMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessagesAsReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessagesAsReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    pub NotifyLocalParticipantComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, participantaddress: *mut ::core::ffi::c_void, iscomposing: bool) -> ::windows::core::HRESULT,
    pub NotifyRemoteParticipantComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, participantaddress: *mut ::core::ffi::c_void, iscomposing: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoteParticipantComposingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRemoteParticipantComposingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRemoteParticipantComposingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatConversation2 {
    type Vtable = IChatConversation2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatConversation2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a030cd1_983a_47aa_9a90_ee48ee997b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanModifyParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanModifyParticipants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatConversationReader {
    type Vtable = IChatConversationReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatConversationReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055136d2_de32_4a47_a93a_b3dc0833852b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationThreadingInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatConversationThreadingInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x331c21dc_7a07_4422_a32c_24be7c6dab24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationThreadingInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetContactId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Custom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetConversationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Participants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Participants: usize,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatConversationThreadingKind) -> ::windows::core::HRESULT,
    pub SetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatConversationThreadingKind) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct IChatItem(::windows::core::IUnknown);
impl IChatItem {
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IChatItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IChatItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChatItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChatItem {}
impl ::core::fmt::Debug for IChatItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChatItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IChatItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8751d000-ceb1-4243-b803-15d45a1dd428}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IChatItem {
    type Vtable = IChatItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8751d000_ceb1_4243_b803_15d45a1dd428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ItemKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessage {
    type Vtable = IChatMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b39052a_1142_5089_76da_f2db3d17cd05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Attachments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Attachments: usize,
    pub Body: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBody: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub From: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsForwardingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LocalTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LocalTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkTimestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Recipients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Recipients: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientSendStatuses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientSendStatuses: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessage2 {
    type Vtable = IChatMessage2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86668332_543f_49f5_ac71_6c2afc6565fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EstimatedDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SetEstimatedDownloadSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub SetFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsAutoReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAutoReply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetIsForwardingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsReplyDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsIncoming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetIsRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSeen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsSimMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLocalTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLocalTimestamp: usize,
    pub MessageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageKind) -> ::windows::core::HRESULT,
    pub SetMessageKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageKind) -> ::windows::core::HRESULT,
    pub MessageOperatorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageOperatorKind) -> ::windows::core::HRESULT,
    pub SetMessageOperatorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageOperatorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetNetworkTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetNetworkTimestamp: usize,
    pub IsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsReceivedDuringQuietHours: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageStatus) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShouldSuppressNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShouldSuppressNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThreadingInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RecipientsDeliveryInfos: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RecipientsDeliveryInfos: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessage3 {
    type Vtable = IChatMessage3_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessage3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74eb2fb0_3ba7_459f_8e0b_e8af0febd9ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessage4 {
    type Vtable = IChatMessage4_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessage4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d144b0f_d2bf_460c_aa68_6d3f8483c9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage4_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SyncId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSyncId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageAttachment {
    type Vtable = IChatMessageAttachment_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageAttachment {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7c4fd74_bf63_58eb_508c_8b863ff16b67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub DataStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DataStreamReference: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetDataStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetDataStreamReference: usize,
    pub GroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageAttachment2 {
    type Vtable = IChatMessageAttachment2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageAttachment2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ed99270_7dd1_4a87_a8ce_acdd87d80dc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    pub TransferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetTransferProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub OriginalFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOriginalFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachmentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageAttachmentFactory {
    type Vtable = IChatMessageAttachmentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageAttachmentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205852a2_a356_5b71_6ca9_66c985b7d0d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachmentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateChatMessageAttachment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: *mut ::core::ffi::c_void, datastreamreference: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateChatMessageAttachment: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageBlockingStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageBlockingStatic {
    type Vtable = IChatMessageBlockingStatic_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageBlockingStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6b9a380_cdea_11e4_8830_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageBlockingStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MarkMessageAsBlockedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, blocked: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageAsBlockedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageChange {
    type Vtable = IChatMessageChange_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageChange {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c18c355_421e_54b8_6d38_6b3a6c82fccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChange_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageChangeType) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageChangeReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14267020_28ce_5f26_7b05_9a5c7cce87ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeReader_Vtbl {
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
pub struct IChatMessageChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageChangeTracker {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60b7f066_70a0_5224_508c_242ef7c1d06f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeTracker_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageChangedDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc6b30c_788c_4dcc_ace7_6282382968cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b73e2d_691c_4edf_8660_6eb9896892e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManager2Statics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageManager2Statics {
    type Vtable = IChatMessageManager2Statics_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageManager2Statics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d45390f_9f4f_4e35_964e_1b9ca61ac044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManager2Statics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageManagerStatic {
    type Vtable = IChatMessageManagerStatic_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageManagerStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf15c60f7_d5e8_5e92_556d_e03b60253104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowComposeSmsMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowComposeSmsMessageAsync: usize,
    pub ShowSmsSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageManagerStatics3 {
    type Vtable = IChatMessageManagerStatics3_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageManagerStatics3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x208b830d_6755_48cc_9ab3_fd03c463fc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatics3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageNotificationTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd344dfb_3063_4e17_8586_c6c08262e6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChatMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageNotificationTriggerDetails2 {
    type Vtable = IChatMessageNotificationTriggerDetails2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageNotificationTriggerDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bb522e0_aa07_4fd1_9471_77934fb75ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ShouldDisplayToast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShouldUpdateDetailText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShouldUpdateBadge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShouldUpdateActionCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageReader {
    type Vtable = IChatMessageReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6ea78ce_4489_56f9_76aa_e204682514cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageReader2 {
    type Vtable = IChatMessageReader2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89643683_64bb_470d_9df4_0de8be1a05bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageStore {
    type Vtable = IChatMessageStore_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageStore {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31f2fd01_ccf6_580b_4976_0a07dd5d3b47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageAsync: usize,
    pub GetMessageReader1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetMessageReader2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageReader2: usize,
    #[cfg(feature = "Foundation")]
    pub MarkMessageReadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkMessageReadAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetrySendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetrySendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendMessageAsync: usize,
    pub ValidateMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MessageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageStore2 {
    type Vtable = IChatMessageStore2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageStore2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4dc4ee_3ad4_491b_b311_abdf9bb22768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ForwardMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, addresses: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ForwardMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetConversationFromThreadingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadinginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConversationFromThreadingInfoAsync: usize,
    pub GetConversationReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConversationForTransportsReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConversationForTransportsReader: usize,
    #[cfg(feature = "Foundation")]
    pub GetMessageByRemoteIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, remoteid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageByRemoteIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetUnseenCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetUnseenCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetUnseenCountForTransportsReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetUnseenCountForTransportsReaderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub MarkAsSeenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MarkAsSeenAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MarkAsSeenForTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MarkAsSeenForTransportsAsync: usize,
    pub GetSearchReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelDownloadMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelDownloadMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelSendMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelSendMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore3(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageStore3 {
    type Vtable = IChatMessageStore3_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageStore3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9adbbb09_4345_4ec1_8b74_b7338243719c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetMessageBySyncIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetMessageBySyncIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStoreChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageStoreChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c66fac_fe8c_46d4_9119_57b8410311d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStoreChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatStoreChangedEventKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageTransport {
    type Vtable = IChatMessageTransport_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageTransport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a9dbf8_e6b3_5c9a_5f85_d47925b9bd18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAppSetAsNotificationProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsNotificationProviderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsNotificationProviderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageTransport2 {
    type Vtable = IChatMessageTransport2_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageTransport2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a75622_d84a_4c22_a94d_544444edc8a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransportKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageTransportKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransportConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageTransportConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x879ff725_1a08_4aca_a075_3355126312e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransportConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub SupportedVideoFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    SupportedVideoFormat: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageValidationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatMessageValidationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25e93a03_28ec_5889_569b_7e486b126f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageValidationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MaxPartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxPartCount: usize,
    #[cfg(feature = "Foundation")]
    pub PartCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PartCount: usize,
    #[cfg(feature = "Foundation")]
    pub RemainingCharacterCountInPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemainingCharacterCountInPart: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageValidationStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatQueryOptions {
    type Vtable = IChatQueryOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatQueryOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fd364a6_bf36_42f7_b7e7_923c0aabfe16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatQueryOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SearchString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSearchString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatRecipientDeliveryInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatRecipientDeliveryInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc7b2a2_283c_4c0a_8a0e_8c33bdbf0545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatRecipientDeliveryInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDeliveryTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDeliveryTime: usize,
    #[cfg(feature = "Foundation")]
    pub ReadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReadTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReadTime: usize,
    pub TransportErrorCodeCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportErrorCodeCategory) -> ::windows::core::HRESULT,
    pub TransportInterpretedErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows::core::HRESULT,
    pub TransportErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsErrorPermanent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSearchReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatSearchReader {
    type Vtable = IChatSearchReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatSearchReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4665fe49_9020_4752_980d_39612325f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSearchReader_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchWithCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchWithCountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatSyncConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09f869b2_69f4_4aff_82b6_06992ff402d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RestoreHistorySpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatRestoreHistorySpan) -> ::windows::core::HRESULT,
    pub SetRestoreHistorySpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatRestoreHistorySpan) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IChatSyncManager {
    type Vtable = IChatSyncManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IChatSyncManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba52c63_2650_486f_b4b4_6bd9d3d63c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub AssociateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))]
    AssociateAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UnassociateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UnassociateAccountAsync: usize,
    #[cfg(feature = "Security_Credentials")]
    pub IsAccountAssociated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IsAccountAssociated: usize,
    pub StartSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsEndUserMessage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7cda5eb_cbd7_4f3b_8526_b506dec35c53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPinRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Actions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Actions: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendResponseWithPinAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendResponseWithPinAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAction(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92378737_9b42_46d3_9d5e_3c1b2dae7cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d45ae01_3f89_41ea_9702_9e9ed411aa98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsMessageAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b97742d_351f_4692_b41e_1b035dc18986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsEndUserMessageManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3054ae5a_4d1f_4b59_9433_126c734e86a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub MessageAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageAvailableChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsManagerStatics {
    type Vtable = IRcsManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d270ac5_0abd_4f31_9b99_a59e71a7b731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetEndUserMessageManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetTransportsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetTransportsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetTransportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTransportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub LeaveConversationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeaveConversationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsManagerStatics2 {
    type Vtable = IRcsManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd49ad18_ad8a_42aa_8eeb_a798a8808959);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TransportListChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TransportListChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTransportListChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTransportListChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsServiceKindSupportedChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsServiceKindSupportedChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf47ea244_e783_4866_b3a7_4e5ccf023070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsServiceKindSupportedChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ServiceKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RcsServiceKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsTransport {
    type Vtable = IRcsTransport_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsTransport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea34759_f37c_4319_8546_ec84d21d30ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TransportFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsStoreAndForwardEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsServiceKindSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceKindSupportedChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveServiceKindSupportedChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveServiceKindSupportedChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransportConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for IRcsTransportConfiguration {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fccb102_2472_4bb9_9988_c1211c83e8a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransportConfiguration_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxAttachmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxGroupMessageSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxRecipientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub MaxFileSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub WarningFileSizeInKilobytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteParticipantComposingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IRemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IRemoteParticipantComposingChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec045a7_cfc9_45c9_9876_449f2bc180f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteParticipantComposingChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TransportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParticipantAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsComposing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatCapabilities(::windows::core::IUnknown);
impl ChatCapabilities {
    pub fn IsOnline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOnline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsChatCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsChatCapable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsFileTransferCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsFileTransferCapable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsGeoLocationPushCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsGeoLocationPushCapable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsIntegratedMessagingCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIntegratedMessagingCapable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatCapabilities {}
impl ::core::fmt::Debug for ChatCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatCapabilities;{3aff77bc-39c9-4dd1-ad2d-3964dd9d403f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatCapabilities {
    type Vtable = IChatCapabilities_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatCapabilities {
    const IID: ::windows::core::GUID = <IChatCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilities";
}
::windows::core::interface_hierarchy!(ChatCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatCapabilities {}
unsafe impl ::core::marker::Sync for ChatCapabilities {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatCapabilitiesManager;
impl ChatCapabilitiesManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesAsync(address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCachedCapabilitiesAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(address), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkAsync(address: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapabilitiesFromNetworkAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(address), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesForTransportAsync(address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCachedCapabilitiesForTransportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(address), ::core::mem::transmute_copy(transportid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkForTransportAsync(address: &::windows::core::HSTRING, transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCapabilitiesFromNetworkForTransportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(address), ::core::mem::transmute_copy(transportid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics<R, F: FnOnce(&IChatCapabilitiesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics2<R, F: FnOnce(&IChatCapabilitiesManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ChatCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversation(::windows::core::IUnknown);
impl ChatConversation {
    pub fn HasUnreadMessages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasUnreadMessages)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubject)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsConversationMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsConversationMuted)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsConversationMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsConversationMuted)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MostRecentMessageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Participants)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThreadingInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetMessageReader(&self) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAllMessagesAsReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkAllMessagesAsReadAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessagesAsReadAsync(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkMessagesAsReadAsync)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn NotifyLocalParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyLocalParticipantComposing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    pub fn NotifyRemoteParticipantComposing(&self, transportid: &::windows::core::HSTRING, participantaddress: &::windows::core::HSTRING, iscomposing: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NotifyRemoteParticipantComposing)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(participantaddress), iscomposing).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteParticipantComposingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteParticipantComposingChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteParticipantComposingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveRemoteParticipantComposingChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CanModifyParticipants(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatConversation2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanModifyParticipants)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCanModifyParticipants(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatConversation2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetCanModifyParticipants)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = &::windows::core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatConversation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatConversation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversation {}
impl ::core::fmt::Debug for ChatConversation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatConversation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversation;{a58c080d-1a6f-46dc-8f3d-f5028660b6ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatConversation {
    type Vtable = IChatConversation_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatConversation {
    const IID: ::windows::core::GUID = <IChatConversation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversation";
}
::windows::core::interface_hierarchy!(ChatConversation, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ChatConversation> for IChatItem {
    type Error = ::windows::core::Error;
    fn try_from(value: ChatConversation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChatConversation> for IChatItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatConversation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ChatConversation> for ::windows::core::InParam<IChatItem> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatConversation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ChatConversation {}
unsafe impl ::core::marker::Sync for ChatConversation {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversationReader(::windows::core::IUnknown);
impl ChatConversationReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchWithCountAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatConversationReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatConversationReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationReader {}
impl ::core::fmt::Debug for ChatConversationReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatConversationReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationReader;{055136d2-de32-4a47-a93a-b3dc0833852b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatConversationReader {
    type Vtable = IChatConversationReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatConversationReader {
    const IID: ::windows::core::GUID = <IChatConversationReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationReader";
}
::windows::core::interface_hierarchy!(ChatConversationReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatConversationReader {}
unsafe impl ::core::marker::Sync for ChatConversationReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(::windows::core::IUnknown);
impl ChatConversationThreadingInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatConversationThreadingInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContactId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetContactId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetContactId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Custom(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Custom)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCustom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCustom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConversationId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetConversationId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetConversationId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Participants)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ChatConversationThreadingKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ChatConversationThreadingInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatConversationThreadingInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationThreadingInfo {}
impl ::core::fmt::Debug for ChatConversationThreadingInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatConversationThreadingInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationThreadingInfo;{331c21dc-7a07-4422-a32c-24be7c6dab24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatConversationThreadingInfo {
    const IID: ::windows::core::GUID = <IChatConversationThreadingInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
}
::windows::core::interface_hierarchy!(ChatConversationThreadingInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatConversationThreadingInfo {}
unsafe impl ::core::marker::Sync for ChatConversationThreadingInfo {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessage(::windows::core::IUnknown);
impl ChatMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessage, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = &::windows::core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Attachments)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Body)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBody(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBody)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).From)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsForwardingDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsForwardingDisabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsIncoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsIncoming)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LocalTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalTimestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NetworkTimestamp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Recipients)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientSendStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ChatMessageStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecipientSendStatuses)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Subject)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportFriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTransportId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransportId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn EstimatedDownloadSize(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EstimatedDownloadSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetEstimatedDownloadSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetFrom)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsAutoReply(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAutoReply)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsAutoReply(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsAutoReply)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetIsForwardingDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsForwardingDisabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsReplyDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReplyDisabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsIncoming)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsRead)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsSeen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSeen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsSeen)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsSimMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSimMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLocalTimestamp(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalTimestamp)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MessageKind(&self) -> ::windows::core::Result<ChatMessageKind> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMessageKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MessageOperatorKind(&self) -> ::windows::core::Result<ChatMessageOperatorKind> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageOperatorKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetMessageOperatorKind)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNetworkTimestamp(&self, value: super::super::Foundation::DateTime) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetNetworkTimestamp)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsReceivedDuringQuietHours(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReceivedDuringQuietHours)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsReceivedDuringQuietHours)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetRemoteId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetRemoteId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SetStatus(&self, value: ChatMessageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetStatus)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSubject)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ShouldSuppressNotification(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldSuppressNotification)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetShouldSuppressNotification(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetShouldSuppressNotification)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ThreadingInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetThreadingInfo(&self, value: &ChatConversationThreadingInfo) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetThreadingInfo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientsDeliveryInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RecipientsDeliveryInfos)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessage3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemoteId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SyncId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessage4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSyncId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage4>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSyncId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ChatMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessage {}
impl ::core::fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessage;{4b39052a-1142-5089-76da-f2db3d17cd05})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessage {
    type Vtable = IChatMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessage {
    const IID: ::windows::core::GUID = <IChatMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessage";
}
::windows::core::interface_hierarchy!(ChatMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<ChatMessage> for IChatItem {
    type Error = ::windows::core::Error;
    fn try_from(value: ChatMessage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ChatMessage> for IChatItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatMessage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ChatMessage> for ::windows::core::InParam<IChatItem> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ChatMessage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for ChatMessage {}
unsafe impl ::core::marker::Sync for ChatMessage {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageAttachment(::windows::core::IUnknown);
impl ChatMessageAttachment {
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DataStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DataStreamReference)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDataStreamReference<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDataStreamReference)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GroupId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGroupId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGroupId)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MimeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetMimeType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetMimeType)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Thumbnail)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetThumbnail)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn TransferProgress(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransferProgress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTransferProgress(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransferProgress)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn OriginalFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OriginalFileName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOriginalFileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetOriginalFileName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateChatMessageAttachment<P0, E0>(mimetype: &::windows::core::HSTRING, datastreamreference: P0) -> ::windows::core::Result<ChatMessageAttachment>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamReference>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IChatMessageAttachmentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateChatMessageAttachment)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(mimetype), datastreamreference.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageAttachmentFactory<R, F: FnOnce(&IChatMessageAttachmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessageAttachment, IChatMessageAttachmentFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ChatMessageAttachment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageAttachment {}
impl ::core::fmt::Debug for ChatMessageAttachment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageAttachment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageAttachment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageAttachment;{c7c4fd74-bf63-58eb-508c-8b863ff16b67})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageAttachment {
    type Vtable = IChatMessageAttachment_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageAttachment {
    const IID: ::windows::core::GUID = <IChatMessageAttachment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageAttachment";
}
::windows::core::interface_hierarchy!(ChatMessageAttachment, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageAttachment {}
unsafe impl ::core::marker::Sync for ChatMessageAttachment {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatMessageBlocking;
impl ChatMessageBlocking {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageAsBlockedAsync(localchatmessageid: &::windows::core::HSTRING, blocked: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageBlockingStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkMessageAsBlockedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), blocked, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageBlockingStatic<R, F: FnOnce(&IChatMessageBlockingStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessageBlocking, IChatMessageBlockingStatic> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ChatMessageBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageBlocking";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChange(::windows::core::IUnknown);
impl ChatMessageChange {
    pub fn ChangeType(&self) -> ::windows::core::Result<ChatMessageChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChange {}
impl ::core::fmt::Debug for ChatMessageChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChange;{1c18c355-421e-54b8-6d38-6b3a6c82fccc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageChange {
    type Vtable = IChatMessageChange_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageChange {
    const IID: ::windows::core::GUID = <IChatMessageChange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChange";
}
::windows::core::interface_hierarchy!(ChatMessageChange, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChange {}
unsafe impl ::core::marker::Sync for ChatMessageChange {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangeReader(::windows::core::IUnknown);
impl ChatMessageChangeReader {
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcceptChanges)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn AcceptChangesThrough(&self, lastchangetoacknowledge: &ChatMessageChange) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).AcceptChangesThrough)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(lastchangetoacknowledge)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageChangeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeReader {}
impl ::core::fmt::Debug for ChatMessageChangeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeReader;{14267020-28ce-5f26-7b05-9a5c7cce87ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageChangeReader {
    const IID: ::windows::core::GUID = <IChatMessageChangeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeReader";
}
::windows::core::interface_hierarchy!(ChatMessageChangeReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangeReader {}
unsafe impl ::core::marker::Sync for ChatMessageChangeReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangeTracker(::windows::core::IUnknown);
impl ChatMessageChangeTracker {
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Enable)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GetChangeReader(&self) -> ::windows::core::Result<ChatMessageChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChangeReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Reset)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ChatMessageChangeTracker {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeTracker {}
impl ::core::fmt::Debug for ChatMessageChangeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeTracker").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeTracker;{60b7f066-70a0-5224-508c-242ef7c1d06f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageChangeTracker {
    const IID: ::windows::core::GUID = <IChatMessageChangeTracker as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
}
::windows::core::interface_hierarchy!(ChatMessageChangeTracker, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangeTracker {}
unsafe impl ::core::marker::Sync for ChatMessageChangeTracker {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(::windows::core::IUnknown);
impl ChatMessageChangedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ChatMessageChangedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedDeferral {}
impl ::core::fmt::Debug for ChatMessageChangedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedDeferral;{fbc6b30c-788c-4dcc-ace7-6282382968cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageChangedDeferral {
    const IID: ::windows::core::GUID = <IChatMessageChangedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
}
::windows::core::interface_hierarchy!(ChatMessageChangedDeferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangedDeferral {}
unsafe impl ::core::marker::Sync for ChatMessageChangedDeferral {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(::windows::core::IUnknown);
impl ChatMessageChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows::core::Result<ChatMessageChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs;{b6b73e2d-691c-4edf-8660-6eb9896892e3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageChangedEventArgs {
    const IID: ::windows::core::GUID = <IChatMessageChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
}
::windows::core::interface_hierarchy!(ChatMessageChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct ChatMessageManager;
impl ChatMessageManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterTransportAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RegisterTransportAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync(transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTransportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transportid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTransportsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestStoreAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeSmsMessageAsync(message: &ChatMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowComposeSmsMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(message), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShowSmsSettings() -> ::windows::core::Result<()> {
        Self::IChatMessageManagerStatic(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowSmsSettings)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSyncManagerAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>> {
        Self::IChatMessageManagerStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSyncManagerAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageManager2Statics<R, F: FnOnce(&IChatMessageManager2Statics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManager2Statics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatic<R, F: FnOnce(&IChatMessageManagerStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManagerStatic> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatics3<R, F: FnOnce(&IChatMessageManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManagerStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ChatMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(::windows::core::IUnknown);
impl ChatMessageNotificationTriggerDetails {
    pub fn ChatMessage(&self) -> ::windows::core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChatMessage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShouldDisplayToast(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldDisplayToast)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShouldUpdateDetailText(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldUpdateDetailText)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShouldUpdateBadge(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldUpdateBadge)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ShouldUpdateActionCenter(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShouldUpdateActionCenter)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageNotificationTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageNotificationTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageNotificationTriggerDetails {}
impl ::core::fmt::Debug for ChatMessageNotificationTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageNotificationTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails;{fd344dfb-3063-4e17-8586-c6c08262e6c0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageNotificationTriggerDetails {
    const IID: ::windows::core::GUID = <IChatMessageNotificationTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
}
::windows::core::interface_hierarchy!(ChatMessageNotificationTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageReader(::windows::core::IUnknown);
impl ChatMessageReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = &::windows::core::Interface::cast::<IChatMessageReader2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchWithCountAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageReader {}
impl ::core::fmt::Debug for ChatMessageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageReader;{b6ea78ce-4489-56f9-76aa-e204682514cf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageReader {
    type Vtable = IChatMessageReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageReader {
    const IID: ::windows::core::GUID = <IChatMessageReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageReader";
}
::windows::core::interface_hierarchy!(ChatMessageReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageReader {}
unsafe impl ::core::marker::Sync for ChatMessageReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageStore(::windows::core::IUnknown);
impl ChatMessageStore {
    pub fn ChangeTracker(&self) -> ::windows::core::Result<ChatMessageChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ChangeTracker)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteMessageAsync(&self, localmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DownloadMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetMessageReader1(&self) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageReader1)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageReader2(&self, recenttimelimit: super::super::Foundation::TimeSpan) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageReader2)(::windows::core::Vtable::as_raw(this), recenttimelimit, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageReadAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkMessageReadAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrySendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RetrySendMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAsync(&self, chatmessage: &ChatMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(chatmessage), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ValidateMessage(&self, chatmessage: &ChatMessage) -> ::windows::core::Result<ChatMessageValidationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ValidateMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(chatmessage), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageChanged(&self, value: &super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageChanged(&self, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageChanged)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ForwardMessageAsync<P0, E0>(&self, localchatmessageid: &::windows::core::HSTRING, addresses: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForwardMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), addresses.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync(&self, conversationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConversationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(conversationid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsAsync<P0, E0>(&self, conversationid: &::windows::core::HSTRING, transportids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConversationForTransportsAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(conversationid), transportids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationFromThreadingInfoAsync(&self, threadinginfo: &ChatConversationThreadingInfo) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConversationFromThreadingInfoAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(threadinginfo), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetConversationReader(&self) -> ::windows::core::Result<ChatConversationReader> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConversationReader)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsReader<P0, E0>(&self, transportids: P0) -> ::windows::core::Result<ChatConversationReader>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetConversationForTransportsReader)(::windows::core::Vtable::as_raw(this), transportids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageByRemoteIdAsync(&self, transportid: &::windows::core::HSTRING, remoteid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageByRemoteIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transportid), ::core::mem::transmute_copy(remoteid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUnseenCountAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnseenCountForTransportsReaderAsync<P0, E0>(&self, transportids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUnseenCountForTransportsReaderAsync)(::windows::core::Vtable::as_raw(this), transportids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkAsSeenAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MarkAsSeenForTransportsAsync<P0, E0>(&self, transportids: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MarkAsSeenForTransportsAsync)(::windows::core::Vtable::as_raw(this), transportids.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSearchReader(&self, value: &ChatQueryOptions) -> ::windows::core::Result<ChatSearchReader> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSearchReader)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync(&self, chatmessage: &ChatMessage) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SaveMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(chatmessage), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelDownloadMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCancelDownloadMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelSendMessageAsync(&self, localchatmessageid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryCancelSendMessageAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(localchatmessageid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged(&self, handler: &super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StoreChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStoreChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageBySyncIdAsync(&self, syncid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMessageBySyncIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(syncid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStore {}
impl ::core::fmt::Debug for ChatMessageStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStore;{31f2fd01-ccf6-580b-4976-0a07dd5d3b47})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageStore {
    type Vtable = IChatMessageStore_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageStore {
    const IID: ::windows::core::GUID = <IChatMessageStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStore";
}
::windows::core::interface_hierarchy!(ChatMessageStore, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageStore {}
unsafe impl ::core::marker::Sync for ChatMessageStore {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(::windows::core::IUnknown);
impl ChatMessageStoreChangedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<ChatStoreChangedEventKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStoreChangedEventArgs {}
impl ::core::fmt::Debug for ChatMessageStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStoreChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageStoreChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs;{65c66fac-fe8c-46d4-9119-57b8410311d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageStoreChangedEventArgs {
    const IID: ::windows::core::GUID = <IChatMessageStoreChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
}
::windows::core::interface_hierarchy!(ChatMessageStoreChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageStoreChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageTransport(::windows::core::IUnknown);
impl ChatMessageTransport {
    pub fn IsAppSetAsNotificationProvider(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAppSetAsNotificationProvider)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportFriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsNotificationProviderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetAsNotificationProviderAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<ChatMessageTransportConfiguration> {
        let this = &::windows::core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Configuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportKind(&self) -> ::windows::core::Result<ChatMessageTransportKind> {
        let this = &::windows::core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransport {}
impl ::core::fmt::Debug for ChatMessageTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransport;{63a9dbf8-e6b3-5c9a-5f85-d47925b9bd18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageTransport {
    type Vtable = IChatMessageTransport_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageTransport {
    const IID: ::windows::core::GUID = <IChatMessageTransport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransport";
}
::windows::core::interface_hierarchy!(ChatMessageTransport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageTransport {}
unsafe impl ::core::marker::Sync for ChatMessageTransport {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(::windows::core::IUnknown);
impl ChatMessageTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAttachmentCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxMessageSizeInKilobytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxRecipientCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SupportedVideoFormat(&self) -> ::windows::core::Result<super::super::Media::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SupportedVideoFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransportConfiguration {}
impl ::core::fmt::Debug for ChatMessageTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageTransportConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration;{879ff725-1a08-4aca-a075-3355126312e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageTransportConfiguration {
    const IID: ::windows::core::GUID = <IChatMessageTransportConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
}
::windows::core::interface_hierarchy!(ChatMessageTransportConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageTransportConfiguration {}
unsafe impl ::core::marker::Sync for ChatMessageTransportConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatMessageValidationResult(::windows::core::IUnknown);
impl ChatMessageValidationResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxPartCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PartCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingCharacterCountInPart(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RemainingCharacterCountInPart)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageValidationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatMessageValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatMessageValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageValidationResult {}
impl ::core::fmt::Debug for ChatMessageValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageValidationResult;{25e93a03-28ec-5889-569b-7e486b126f18})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatMessageValidationResult {
    const IID: ::windows::core::GUID = <IChatMessageValidationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageValidationResult";
}
::windows::core::interface_hierarchy!(ChatMessageValidationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatMessageValidationResult {}
unsafe impl ::core::marker::Sync for ChatMessageValidationResult {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatQueryOptions(::windows::core::IUnknown);
impl ChatQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatQueryOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SearchString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSearchString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSearchString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for ChatQueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatQueryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatQueryOptions {}
impl ::core::fmt::Debug for ChatQueryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatQueryOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatQueryOptions;{2fd364a6-bf36-42f7-b7e7-923c0aabfe16})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatQueryOptions {
    type Vtable = IChatQueryOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatQueryOptions {
    const IID: ::windows::core::GUID = <IChatQueryOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatQueryOptions";
}
::windows::core::interface_hierarchy!(ChatQueryOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatQueryOptions {}
unsafe impl ::core::marker::Sync for ChatQueryOptions {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(::windows::core::IUnknown);
impl ChatRecipientDeliveryInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ChatRecipientDeliveryInfo, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TransportAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetTransportAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetTransportAddress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeliveryTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDeliveryTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDeliveryTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReadTime<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetReadTime)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn TransportErrorCodeCategory(&self) -> ::windows::core::Result<ChatTransportErrorCodeCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportErrorCodeCategory)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportInterpretedErrorCode(&self) -> ::windows::core::Result<ChatTransportInterpretedErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportInterpretedErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportErrorCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsErrorPermanent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsErrorPermanent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatRecipientDeliveryInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatRecipientDeliveryInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatRecipientDeliveryInfo {}
impl ::core::fmt::Debug for ChatRecipientDeliveryInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRecipientDeliveryInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatRecipientDeliveryInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo;{ffc7b2a2-283c-4c0a-8a0e-8c33bdbf0545})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatRecipientDeliveryInfo {
    const IID: ::windows::core::GUID = <IChatRecipientDeliveryInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
}
::windows::core::interface_hierarchy!(ChatRecipientDeliveryInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatRecipientDeliveryInfo {}
unsafe impl ::core::marker::Sync for ChatRecipientDeliveryInfo {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSearchReader(::windows::core::IUnknown);
impl ChatSearchReader {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadBatchWithCountAsync)(::windows::core::Vtable::as_raw(this), count, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatSearchReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatSearchReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSearchReader {}
impl ::core::fmt::Debug for ChatSearchReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSearchReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatSearchReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSearchReader;{4665fe49-9020-4752-980d-39612325f589})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatSearchReader {
    type Vtable = IChatSearchReader_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatSearchReader {
    const IID: ::windows::core::GUID = <IChatSearchReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSearchReader";
}
::windows::core::interface_hierarchy!(ChatSearchReader, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatSearchReader {}
unsafe impl ::core::marker::Sync for ChatSearchReader {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSyncConfiguration(::windows::core::IUnknown);
impl ChatSyncConfiguration {
    pub fn IsSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsSyncEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsSyncEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn RestoreHistorySpan(&self) -> ::windows::core::Result<ChatRestoreHistorySpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RestoreHistorySpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRestoreHistorySpan)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for ChatSyncConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatSyncConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncConfiguration {}
impl ::core::fmt::Debug for ChatSyncConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatSyncConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncConfiguration;{09f869b2-69f4-4aff-82b6-06992ff402d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatSyncConfiguration {
    const IID: ::windows::core::GUID = <IChatSyncConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncConfiguration";
}
::windows::core::interface_hierarchy!(ChatSyncConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatSyncConfiguration {}
unsafe impl ::core::marker::Sync for ChatSyncConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct ChatSyncManager(::windows::core::IUnknown);
impl ChatSyncManager {
    pub fn Configuration(&self) -> ::windows::core::Result<ChatSyncConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Configuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Security_Credentials\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn AssociateAccountAsync(&self, webaccount: &super::super::Security::Credentials::WebAccount) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AssociateAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(webaccount), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnassociateAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnassociateAccountAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn IsAccountAssociated(&self, webaccount: &super::super::Security::Credentials::WebAccount) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAccountAssociated)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(webaccount), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn StartSync(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).StartSync)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetConfigurationAsync(&self, configuration: &ChatSyncConfiguration) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetConfigurationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(configuration), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for ChatSyncManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ChatSyncManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatSyncManager {}
impl ::core::fmt::Debug for ChatSyncManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatSyncManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncManager;{7ba52c63-2650-486f-b4b4-6bd9d3d63c84})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ChatSyncManager {
    type Vtable = IChatSyncManager_Vtbl;
}
unsafe impl ::windows::core::Interface for ChatSyncManager {
    const IID: ::windows::core::GUID = <IChatSyncManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncManager";
}
::windows::core::interface_hierarchy!(ChatSyncManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for ChatSyncManager {}
unsafe impl ::core::marker::Sync for ChatSyncManager {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessage(::windows::core::IUnknown);
impl RcsEndUserMessage {
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPinRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPinRequired)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Actions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseAsync(&self, action: &RcsEndUserMessageAction) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendResponseAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(action), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseWithPinAsync(&self, action: &RcsEndUserMessageAction, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendResponseWithPinAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(action), ::core::mem::transmute_copy(pin), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsEndUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessage {}
impl ::core::fmt::Debug for RcsEndUserMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessage;{d7cda5eb-cbd7-4f3b-8526-b506dec35c53})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsEndUserMessage {
    const IID: ::windows::core::GUID = <IRcsEndUserMessage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessage";
}
::windows::core::interface_hierarchy!(RcsEndUserMessage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessage {}
unsafe impl ::core::marker::Sync for RcsEndUserMessage {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAction(::windows::core::IUnknown);
impl RcsEndUserMessageAction {
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Label)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsEndUserMessageAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAction {}
impl ::core::fmt::Debug for RcsEndUserMessageAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAction;{92378737-9b42-46d3-9d5e-3c1b2dae7cb8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAction {
    const IID: ::windows::core::GUID = <IRcsEndUserMessageAction as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
}
::windows::core::interface_hierarchy!(RcsEndUserMessageAction, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAction {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAction {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableEventArgs(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableEventArgs {
    pub fn IsMessageAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsMessageAvailable)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows::core::Result<RcsEndUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Message)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableEventArgs {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs;{2d45ae01-3f89-41ea-9702-9e9ed411aa98})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableEventArgs {
    const IID: ::windows::core::GUID = <IRcsEndUserMessageAvailableEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
}
::windows::core::interface_hierarchy!(RcsEndUserMessageAvailableEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableEventArgs {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTriggerDetails(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableTriggerDetails {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Title)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageAvailableTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageAvailableTriggerDetails {}
impl ::core::fmt::Debug for RcsEndUserMessageAvailableTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageAvailableTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails;{5b97742d-351f-4692-b41e-1b035dc18986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableTriggerDetails {
    const IID: ::windows::core::GUID = <IRcsEndUserMessageAvailableTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
}
::windows::core::interface_hierarchy!(RcsEndUserMessageAvailableTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTriggerDetails {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsEndUserMessageManager(::windows::core::IUnknown);
impl RcsEndUserMessageManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageAvailableChanged(&self, handler: &super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MessageAvailableChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageAvailableChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMessageAvailableChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for RcsEndUserMessageManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsEndUserMessageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsEndUserMessageManager {}
impl ::core::fmt::Debug for RcsEndUserMessageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsEndUserMessageManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageManager;{3054ae5a-4d1f-4b59-9433-126c734e86a6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageManager {
    const IID: ::windows::core::GUID = <IRcsEndUserMessageManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
}
::windows::core::interface_hierarchy!(RcsEndUserMessageManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsEndUserMessageManager {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageManager {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
pub struct RcsManager;
impl RcsManager {
    pub fn GetEndUserMessageManager() -> ::windows::core::Result<RcsEndUserMessageManager> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEndUserMessageManager)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTransportsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTransportsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync(transportid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTransportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(transportid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveConversationAsync(conversation: &ChatConversation) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LeaveConversationAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(conversation), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TransportListChanged(handler: &super::super::Foundation::EventHandler<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRcsManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportListChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransportListChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IRcsManagerStatics2(|this| unsafe { (::windows::core::Vtable::vtable(this).RemoveTransportListChanged)(::windows::core::Vtable::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics<R, F: FnOnce(&IRcsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RcsManager, IRcsManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics2<R, F: FnOnce(&IRcsManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RcsManager, IRcsManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for RcsManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsManager";
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(::windows::core::IUnknown);
impl RcsServiceKindSupportedChangedEventArgs {
    pub fn ServiceKind(&self) -> ::windows::core::Result<RcsServiceKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsServiceKindSupportedChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsServiceKindSupportedChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsServiceKindSupportedChangedEventArgs {}
impl ::core::fmt::Debug for RcsServiceKindSupportedChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKindSupportedChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs;{f47ea244-e783-4866-b3a7-4e5ccf023070})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsServiceKindSupportedChangedEventArgs {
    const IID: ::windows::core::GUID = <IRcsServiceKindSupportedChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
}
::windows::core::interface_hierarchy!(RcsServiceKindSupportedChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsServiceKindSupportedChangedEventArgs {}
unsafe impl ::core::marker::Sync for RcsServiceKindSupportedChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsTransport(::windows::core::IUnknown);
impl RcsTransport {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ExtendedProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportFriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Configuration(&self) -> ::windows::core::Result<RcsTransportConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Configuration)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsStoreAndForwardEnabled)(::windows::core::Vtable::as_raw(this), servicekind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsServiceKindSupported)(::windows::core::Vtable::as_raw(this), servicekind, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceKindSupportedChanged(&self, handler: &super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ServiceKindSupportedChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceKindSupportedChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveServiceKindSupportedChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for RcsTransport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransport {}
impl ::core::fmt::Debug for RcsTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransport;{fea34759-f37c-4319-8546-ec84d21d30ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsTransport {
    type Vtable = IRcsTransport_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsTransport {
    const IID: ::windows::core::GUID = <IRcsTransport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransport";
}
::windows::core::interface_hierarchy!(RcsTransport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsTransport {}
unsafe impl ::core::marker::Sync for RcsTransport {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RcsTransportConfiguration(::windows::core::IUnknown);
impl RcsTransportConfiguration {
    pub fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxAttachmentCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxMessageSizeInKilobytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxGroupMessageSizeInKilobytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxRecipientCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxRecipientCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn MaxFileSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MaxFileSizeInKilobytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn WarningFileSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WarningFileSizeInKilobytes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RcsTransportConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RcsTransportConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsTransportConfiguration {}
impl ::core::fmt::Debug for RcsTransportConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsTransportConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsTransportConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransportConfiguration;{1fccb102-2472-4bb9-9988-c1211c83e8a9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_Vtbl;
}
unsafe impl ::windows::core::Interface for RcsTransportConfiguration {
    const IID: ::windows::core::GUID = <IRcsTransportConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransportConfiguration";
}
::windows::core::interface_hierarchy!(RcsTransportConfiguration, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RcsTransportConfiguration {}
unsafe impl ::core::marker::Sync for RcsTransportConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(::windows::core::IUnknown);
impl RemoteParticipantComposingChangedEventArgs {
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ParticipantAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParticipantAddress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsComposing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsComposing)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for RemoteParticipantComposingChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteParticipantComposingChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteParticipantComposingChangedEventArgs {}
impl ::core::fmt::Debug for RemoteParticipantComposingChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteParticipantComposingChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs;{1ec045a7-cfc9-45c9-9876-449f2bc180f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for RemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for RemoteParticipantComposingChangedEventArgs {
    const IID: ::windows::core::GUID = <IRemoteParticipantComposingChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
}
::windows::core::interface_hierarchy!(RemoteParticipantComposingChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for RemoteParticipantComposingChangedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteParticipantComposingChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: Self = Self(0i32);
    pub const ContactId: Self = Self(1i32);
    pub const ConversationId: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatConversationThreadingKind {}
impl ::core::clone::Clone for ChatConversationThreadingKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatConversationThreadingKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatConversationThreadingKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatConversationThreadingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatConversationThreadingKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatConversationThreadingKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatConversationThreadingKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: Self = Self(0i32);
    pub const Conversation: Self = Self(1i32);
}
impl ::core::marker::Copy for ChatItemKind {}
impl ::core::clone::Clone for ChatItemKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatItemKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatItemKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatItemKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatItemKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatItemKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: Self = Self(0i32);
    pub const MessageModified: Self = Self(1i32);
    pub const MessageDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageChangeType {}
impl ::core::clone::Clone for ChatMessageChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: Self = Self(0i32);
    pub const FileTransferRequest: Self = Self(1i32);
    pub const TransportCustom: Self = Self(2i32);
    pub const JoinedConversation: Self = Self(3i32);
    pub const LeftConversation: Self = Self(4i32);
    pub const OtherParticipantJoinedConversation: Self = Self(5i32);
    pub const OtherParticipantLeftConversation: Self = Self(6i32);
}
impl ::core::marker::Copy for ChatMessageKind {}
impl ::core::clone::Clone for ChatMessageKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: Self = Self(0i32);
    pub const Sms: Self = Self(1i32);
    pub const Mms: Self = Self(2i32);
    pub const Rcs: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageOperatorKind {}
impl ::core::clone::Clone for ChatMessageOperatorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageOperatorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageOperatorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageOperatorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageOperatorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageOperatorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageOperatorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: Self = Self(0i32);
    pub const Sending: Self = Self(1i32);
    pub const Sent: Self = Self(2i32);
    pub const SendRetryNeeded: Self = Self(3i32);
    pub const SendFailed: Self = Self(4i32);
    pub const Received: Self = Self(5i32);
    pub const ReceiveDownloadNeeded: Self = Self(6i32);
    pub const ReceiveDownloadFailed: Self = Self(7i32);
    pub const ReceiveDownloading: Self = Self(8i32);
    pub const Deleted: Self = Self(9i32);
    pub const Declined: Self = Self(10i32);
    pub const Cancelled: Self = Self(11i32);
    pub const Recalled: Self = Self(12i32);
    pub const ReceiveRetryNeeded: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageStatus {}
impl ::core::clone::Clone for ChatMessageStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: Self = Self(0i32);
    pub const Untriaged: Self = Self(1i32);
    pub const Blocked: Self = Self(2i32);
    pub const Custom: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatMessageTransportKind {}
impl ::core::clone::Clone for ChatMessageTransportKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageTransportKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageTransportKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageTransportKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageTransportKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageTransportKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageTransportKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: Self = Self(0i32);
    pub const NoRecipients: Self = Self(1i32);
    pub const InvalidData: Self = Self(2i32);
    pub const MessageTooLarge: Self = Self(3i32);
    pub const TooManyRecipients: Self = Self(4i32);
    pub const TransportInactive: Self = Self(5i32);
    pub const TransportNotFound: Self = Self(6i32);
    pub const TooManyAttachments: Self = Self(7i32);
    pub const InvalidRecipients: Self = Self(8i32);
    pub const InvalidBody: Self = Self(9i32);
    pub const InvalidOther: Self = Self(10i32);
    pub const ValidWithLargeMessage: Self = Self(11i32);
    pub const VoiceRoamingRestriction: Self = Self(12i32);
    pub const DataRoamingRestriction: Self = Self(13i32);
}
impl ::core::marker::Copy for ChatMessageValidationStatus {}
impl ::core::clone::Clone for ChatMessageValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatMessageValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatMessageValidationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatMessageValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatMessageValidationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatMessageValidationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageValidationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: Self = Self(0i32);
    pub const LastYear: Self = Self(1i32);
    pub const AnyTime: Self = Self(2i32);
}
impl ::core::marker::Copy for ChatRestoreHistorySpan {}
impl ::core::clone::Clone for ChatRestoreHistorySpan {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatRestoreHistorySpan {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatRestoreHistorySpan {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatRestoreHistorySpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatRestoreHistorySpan").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatRestoreHistorySpan {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatRestoreHistorySpan;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: Self = Self(0i32);
    pub const StoreModified: Self = Self(1i32);
    pub const MessageCreated: Self = Self(2i32);
    pub const MessageModified: Self = Self(3i32);
    pub const MessageDeleted: Self = Self(4i32);
    pub const ConversationModified: Self = Self(5i32);
    pub const ConversationDeleted: Self = Self(6i32);
    pub const ConversationTransportDeleted: Self = Self(7i32);
}
impl ::core::marker::Copy for ChatStoreChangedEventKind {}
impl ::core::clone::Clone for ChatStoreChangedEventKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatStoreChangedEventKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatStoreChangedEventKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatStoreChangedEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatStoreChangedEventKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatStoreChangedEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatStoreChangedEventKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: Self = Self(0i32);
    pub const Http: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
    pub const MmsServer: Self = Self(3i32);
}
impl ::core::marker::Copy for ChatTransportErrorCodeCategory {}
impl ::core::clone::Clone for ChatTransportErrorCodeCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatTransportErrorCodeCategory {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatTransportErrorCodeCategory {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatTransportErrorCodeCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportErrorCodeCategory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatTransportErrorCodeCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const InvalidRecipientAddress: Self = Self(2i32);
    pub const NetworkConnectivity: Self = Self(3i32);
    pub const ServiceDenied: Self = Self(4i32);
    pub const Timeout: Self = Self(5i32);
}
impl ::core::marker::Copy for ChatTransportInterpretedErrorCode {}
impl ::core::clone::Clone for ChatTransportInterpretedErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ChatTransportInterpretedErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ChatTransportInterpretedErrorCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ChatTransportInterpretedErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChatTransportInterpretedErrorCode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ChatTransportInterpretedErrorCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Chat\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: Self = Self(0i32);
    pub const GroupChat: Self = Self(1i32);
    pub const FileTransfer: Self = Self(2i32);
    pub const Capability: Self = Self(3i32);
}
impl ::core::marker::Copy for RcsServiceKind {}
impl ::core::clone::Clone for RcsServiceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RcsServiceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RcsServiceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for RcsServiceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RcsServiceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RcsServiceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.RcsServiceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
