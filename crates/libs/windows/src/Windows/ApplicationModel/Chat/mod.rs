#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatCapabilities(::windows::core::IUnknown);
impl ChatCapabilities {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsOnline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsChatCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsFileTransferCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsGeoLocationPushCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsIntegratedMessagingCapable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatCapabilities;{3aff77bc-39c9-4dd1-ad2d-3964dd9d403f})");
}
unsafe impl ::windows::core::Interface for ChatCapabilities {
    type Vtable = IChatCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3aff77bc_39c9_4dd1_ad2d_3964dd9d403f);
}
impl ::windows::core::RuntimeName for ChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilities";
}
impl ::core::convert::From<ChatCapabilities> for ::windows::core::IUnknown {
    fn from(value: ChatCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ChatCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatCapabilities> for ::windows::core::IInspectable {
    fn from(value: ChatCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ChatCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatCapabilities {}
unsafe impl ::core::marker::Sync for ChatCapabilities {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
pub struct ChatCapabilitiesManager {}
impl ChatCapabilitiesManager {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(address: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(address: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), address.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCachedCapabilitiesForTransportAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(address: Param0, transportid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), address.into_param().abi(), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitiesFromNetworkForTransportAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(address: Param0, transportid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), address.into_param().abi(), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics<R, F: FnOnce(&IChatCapabilitiesManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IChatCapabilitiesManagerStatics2<R, F: FnOnce(&IChatCapabilitiesManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ChatCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatConversation(::windows::core::IUnknown);
impl ChatConversation {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn HasUnreadMessages(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsConversationMuted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsConversationMuted(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MostRecentMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetMessageReader(&self) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAllMessagesAsReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessagesAsReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn NotifyLocalParticipantComposing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, transportid: Param0, participantaddress: Param1, iscomposing: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), transportid.into_param().abi(), participantaddress.into_param().abi(), iscomposing).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn NotifyRemoteParticipantComposing<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, transportid: Param0, participantaddress: Param1, iscomposing: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), transportid.into_param().abi(), participantaddress.into_param().abi(), iscomposing).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoteParticipantComposingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteParticipantComposingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn CanModifyParticipants(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatConversation2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetCanModifyParticipants(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatConversation2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = &::windows::core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__: ChatItemKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatConversation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversation;{a58c080d-1a6f-46dc-8f3d-f5028660b6ee})");
}
unsafe impl ::windows::core::Interface for ChatConversation {
    type Vtable = IChatConversationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa58c080d_1a6f_46dc_8f3d_f5028660b6ee);
}
impl ::windows::core::RuntimeName for ChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversation";
}
impl ::core::convert::From<ChatConversation> for ::windows::core::IUnknown {
    fn from(value: ChatConversation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversation> for ::windows::core::IUnknown {
    fn from(value: &ChatConversation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatConversation> for ::windows::core::IInspectable {
    fn from(value: ChatConversation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversation> for ::windows::core::IInspectable {
    fn from(value: &ChatConversation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IChatItem> for ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, IChatItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IChatItem> for &ChatConversation {
    fn into_param(self) -> ::windows::core::Param<'a, IChatItem> {
        ::core::convert::TryInto::<IChatItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChatConversation {}
unsafe impl ::core::marker::Sync for ChatConversation {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatConversationReader(::windows::core::IUnknown);
impl ChatConversationReader {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatConversationReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationReader;{055136d2-de32-4a47-a93a-b3dc0833852b})");
}
unsafe impl ::windows::core::Interface for ChatConversationReader {
    type Vtable = IChatConversationReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055136d2_de32_4a47_a93a_b3dc0833852b);
}
impl ::windows::core::RuntimeName for ChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationReader";
}
impl ::core::convert::From<ChatConversationReader> for ::windows::core::IUnknown {
    fn from(value: ChatConversationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversationReader> for ::windows::core::IUnknown {
    fn from(value: &ChatConversationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatConversationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatConversationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatConversationReader> for ::windows::core::IInspectable {
    fn from(value: ChatConversationReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversationReader> for ::windows::core::IInspectable {
    fn from(value: &ChatConversationReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatConversationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatConversationReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatConversationReader {}
unsafe impl ::core::marker::Sync for ChatConversationReader {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatConversationThreadingInfo(::windows::core::IUnknown);
impl ChatConversationThreadingInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatConversationThreadingInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetContactId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Custom(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetCustom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ConversationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetConversationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Participants(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Kind(&self) -> ::windows::core::Result<ChatConversationThreadingKind> {
        let this = self;
        unsafe {
            let mut result__: ChatConversationThreadingKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingKind>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatConversationThreadingInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationThreadingInfo;{331c21dc-7a07-4422-a32c-24be7c6dab24})");
}
unsafe impl ::windows::core::Interface for ChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x331c21dc_7a07_4422_a32c_24be7c6dab24);
}
impl ::windows::core::RuntimeName for ChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
}
impl ::core::convert::From<ChatConversationThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: ChatConversationThreadingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversationThreadingInfo> for ::windows::core::IUnknown {
    fn from(value: &ChatConversationThreadingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatConversationThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatConversationThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatConversationThreadingInfo> for ::windows::core::IInspectable {
    fn from(value: ChatConversationThreadingInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatConversationThreadingInfo> for ::windows::core::IInspectable {
    fn from(value: &ChatConversationThreadingInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatConversationThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatConversationThreadingInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatConversationThreadingInfo {}
unsafe impl ::core::marker::Sync for ChatConversationThreadingInfo {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatConversationThreadingKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatConversationThreadingKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatConversationThreadingKind {}
unsafe impl ::windows::core::RuntimeType for ChatConversationThreadingKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatConversationThreadingKind;i4)");
}
impl ::windows::core::DefaultType for ChatConversationThreadingKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatItemKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatItemKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatItemKind {}
unsafe impl ::windows::core::RuntimeType for ChatItemKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatItemKind;i4)");
}
impl ::windows::core::DefaultType for ChatItemKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessage(::windows::core::IUnknown);
impl ChatMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = &::windows::core::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__: ChatItemKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attachments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ChatMessageAttachment>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Body(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetBody<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsForwardingDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsIncoming(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LocalTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientSendStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ChatMessageStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ChatMessageStatus>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageStatus>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetTransportId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn EstimatedDownloadSize(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsAutoReply(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsAutoReply(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsForwardingDisabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsReplyDisabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsRead(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsSeen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsSeen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsSimMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLocalTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MessageKind(&self) -> ::windows::core::Result<ChatMessageKind> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ChatMessageKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageKind>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MessageOperatorKind(&self) -> ::windows::core::Result<ChatMessageOperatorKind> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ChatMessageOperatorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageOperatorKind>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetNetworkTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsReceivedDuringQuietHours(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetStatus(&self, value: ChatMessageStatus) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShouldSuppressNotification(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetShouldSuppressNotification(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ThreadingInfo(&self) -> ::windows::core::Result<ChatConversationThreadingInfo> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingInfo>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetThreadingInfo<'a, Param0: ::windows::core::IntoParam<'a, ChatConversationThreadingInfo>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RecipientsDeliveryInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>> {
        let this = &::windows::core::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessage3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SyncId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessage4>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetSyncId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessage4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessage;{4b39052a-1142-5089-76da-f2db3d17cd05})");
}
unsafe impl ::windows::core::Interface for ChatMessage {
    type Vtable = IChatMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b39052a_1142_5089_76da_f2db3d17cd05);
}
impl ::windows::core::RuntimeName for ChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessage";
}
impl ::core::convert::From<ChatMessage> for ::windows::core::IUnknown {
    fn from(value: ChatMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessage> for ::windows::core::IUnknown {
    fn from(value: &ChatMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessage> for ::windows::core::IInspectable {
    fn from(value: ChatMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessage> for ::windows::core::IInspectable {
    fn from(value: &ChatMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IChatItem> for ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IChatItem> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IChatItem> for &ChatMessage {
    fn into_param(self) -> ::windows::core::Param<'a, IChatItem> {
        ::core::convert::TryInto::<IChatItem>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ChatMessage {}
unsafe impl ::core::marker::Sync for ChatMessage {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageAttachment(::windows::core::IUnknown);
impl ChatMessageAttachment {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DataStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetDataStreamReference<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetGroupId(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MimeType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetMimeType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransferProgress(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetTransferProgress(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn OriginalFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetOriginalFileName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateChatMessageAttachment<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(mimetype: Param0, datastreamreference: Param1) -> ::windows::core::Result<ChatMessageAttachment> {
        Self::IChatMessageAttachmentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), mimetype.into_param().abi(), datastreamreference.into_param().abi(), &mut result__).from_abi::<ChatMessageAttachment>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageAttachmentFactory<R, F: FnOnce(&IChatMessageAttachmentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageAttachment, IChatMessageAttachmentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::RuntimeType for ChatMessageAttachment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageAttachment;{c7c4fd74-bf63-58eb-508c-8b863ff16b67})");
}
unsafe impl ::windows::core::Interface for ChatMessageAttachment {
    type Vtable = IChatMessageAttachmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7c4fd74_bf63_58eb_508c_8b863ff16b67);
}
impl ::windows::core::RuntimeName for ChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageAttachment";
}
impl ::core::convert::From<ChatMessageAttachment> for ::windows::core::IUnknown {
    fn from(value: ChatMessageAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageAttachment> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageAttachment> for ::windows::core::IInspectable {
    fn from(value: ChatMessageAttachment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageAttachment> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageAttachment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageAttachment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageAttachment {}
unsafe impl ::core::marker::Sync for ChatMessageAttachment {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
pub struct ChatMessageBlocking {}
impl ChatMessageBlocking {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageAsBlockedAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(localchatmessageid: Param0, blocked: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageBlockingStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), blocked, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageBlockingStatic<R, F: FnOnce(&IChatMessageBlockingStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageBlocking, IChatMessageBlockingStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ChatMessageBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageBlocking";
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageChange(::windows::core::IUnknown);
impl ChatMessageChange {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ChangeType(&self) -> ::windows::core::Result<ChatMessageChangeType> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeType>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Message(&self) -> ::windows::core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessage>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageChange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChange;{1c18c355-421e-54b8-6d38-6b3a6c82fccc})");
}
unsafe impl ::windows::core::Interface for ChatMessageChange {
    type Vtable = IChatMessageChangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c18c355_421e_54b8_6d38_6b3a6c82fccc);
}
impl ::windows::core::RuntimeName for ChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChange";
}
impl ::core::convert::From<ChatMessageChange> for ::windows::core::IUnknown {
    fn from(value: ChatMessageChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChange> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageChange> for ::windows::core::IInspectable {
    fn from(value: ChatMessageChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChange> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageChange {}
unsafe impl ::core::marker::Sync for ChatMessageChange {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageChangeReader(::windows::core::IUnknown);
impl ChatMessageChangeReader {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn AcceptChanges(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn AcceptChangesThrough<'a, Param0: ::windows::core::IntoParam<'a, ChatMessageChange>>(&self, lastchangetoacknowledge: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), lastchangetoacknowledge.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeReader;{14267020-28ce-5f26-7b05-9a5c7cce87ca})");
}
unsafe impl ::windows::core::Interface for ChatMessageChangeReader {
    type Vtable = IChatMessageChangeReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14267020_28ce_5f26_7b05_9a5c7cce87ca);
}
impl ::windows::core::RuntimeName for ChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeReader";
}
impl ::core::convert::From<ChatMessageChangeReader> for ::windows::core::IUnknown {
    fn from(value: ChatMessageChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangeReader> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageChangeReader> for ::windows::core::IInspectable {
    fn from(value: ChatMessageChangeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangeReader> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageChangeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageChangeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageChangeReader {}
unsafe impl ::core::marker::Sync for ChatMessageChangeReader {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageChangeTracker(::windows::core::IUnknown);
impl ChatMessageChangeTracker {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetChangeReader(&self) -> ::windows::core::Result<ChatMessageChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Reset(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeTracker {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeTracker;{60b7f066-70a0-5224-508c-242ef7c1d06f})");
}
unsafe impl ::windows::core::Interface for ChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTrackerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60b7f066_70a0_5224_508c_242ef7c1d06f);
}
impl ::windows::core::RuntimeName for ChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
}
impl ::core::convert::From<ChatMessageChangeTracker> for ::windows::core::IUnknown {
    fn from(value: ChatMessageChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangeTracker> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageChangeTracker> for ::windows::core::IInspectable {
    fn from(value: ChatMessageChangeTracker) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangeTracker> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageChangeTracker) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageChangeTracker {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageChangeTracker {}
unsafe impl ::core::marker::Sync for ChatMessageChangeTracker {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageChangeType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageChangeType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageChangeType {}
unsafe impl ::windows::core::RuntimeType for ChatMessageChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageChangeType;i4)");
}
impl ::windows::core::DefaultType for ChatMessageChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageChangedDeferral(::windows::core::IUnknown);
impl ChatMessageChangedDeferral {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatMessageChangedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedDeferral;{fbc6b30c-788c-4dcc-ace7-6282382968cf})");
}
unsafe impl ::windows::core::Interface for ChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc6b30c_788c_4dcc_ace7_6282382968cf);
}
impl ::windows::core::RuntimeName for ChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
}
impl ::core::convert::From<ChatMessageChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: ChatMessageChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangedDeferral> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: ChatMessageChangedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangedDeferral> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageChangedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageChangedDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageChangedDeferral {}
unsafe impl ::core::marker::Sync for ChatMessageChangedDeferral {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageChangedEventArgs(::windows::core::IUnknown);
impl ChatMessageChangedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<ChatMessageChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangedDeferral>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs;{b6b73e2d-691c-4edf-8660-6eb9896892e3})");
}
unsafe impl ::windows::core::Interface for ChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b73e2d_691c_4edf_8660_6eb9896892e3);
}
impl ::windows::core::RuntimeName for ChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
}
impl ::core::convert::From<ChatMessageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ChatMessageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ChatMessageChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageChangedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageKind {}
unsafe impl ::windows::core::RuntimeType for ChatMessageKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageKind;i4)");
}
impl ::windows::core::DefaultType for ChatMessageKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
pub struct ChatMessageManager {}
impl ChatMessageManager {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterTransportAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(transportid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessageTransport>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetTransportsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessageStore>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowComposeSmsMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatMessage>>(message: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShowSmsSettings() -> ::windows::core::Result<()> {
        Self::IChatMessageManagerStatic(|this| unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSyncManagerAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>> {
        Self::IChatMessageManagerStatics3(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatSyncManager>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IChatMessageManager2Statics<R, F: FnOnce(&IChatMessageManager2Statics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManager2Statics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatic<R, F: FnOnce(&IChatMessageManagerStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManagerStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IChatMessageManagerStatics3<R, F: FnOnce(&IChatMessageManagerStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatMessageManager, IChatMessageManagerStatics3> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ChatMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageManager";
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageNotificationTriggerDetails(::windows::core::IUnknown);
impl ChatMessageNotificationTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ChatMessage(&self) -> ::windows::core::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessage>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShouldDisplayToast(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShouldUpdateDetailText(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShouldUpdateBadge(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ShouldUpdateActionCenter(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageNotificationTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails;{fd344dfb-3063-4e17-8586-c6c08262e6c0})");
}
unsafe impl ::windows::core::Interface for ChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd344dfb_3063_4e17_8586_c6c08262e6c0);
}
impl ::windows::core::RuntimeName for ChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
}
impl ::core::convert::From<ChatMessageNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ChatMessageNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageNotificationTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ChatMessageNotificationTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageNotificationTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageNotificationTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageNotificationTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for ChatMessageNotificationTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageOperatorKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageOperatorKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageOperatorKind {}
unsafe impl ::windows::core::RuntimeType for ChatMessageOperatorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageOperatorKind;i4)");
}
impl ::windows::core::DefaultType for ChatMessageOperatorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageReader(::windows::core::IUnknown);
impl ChatMessageReader {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = &::windows::core::Interface::cast::<IChatMessageReader2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageReader;{b6ea78ce-4489-56f9-76aa-e204682514cf})");
}
unsafe impl ::windows::core::Interface for ChatMessageReader {
    type Vtable = IChatMessageReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6ea78ce_4489_56f9_76aa_e204682514cf);
}
impl ::windows::core::RuntimeName for ChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageReader";
}
impl ::core::convert::From<ChatMessageReader> for ::windows::core::IUnknown {
    fn from(value: ChatMessageReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageReader> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageReader> for ::windows::core::IInspectable {
    fn from(value: ChatMessageReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageReader> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageReader {}
unsafe impl ::core::marker::Sync for ChatMessageReader {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageStatus {}
unsafe impl ::windows::core::RuntimeType for ChatMessageStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageStatus;i4)");
}
impl ::windows::core::DefaultType for ChatMessageStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageStore(::windows::core::IUnknown);
impl ChatMessageStore {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<ChatMessageChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeTracker>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), localmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DownloadMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetMessageReader1(&self) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageReader2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, recenttimelimit: Param0) -> ::windows::core::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), recenttimelimit.into_param().abi(), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkMessageReadAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RetrySendMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ValidateMessage<'a, Param0: ::windows::core::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::core::Result<ChatMessageValidationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<ChatMessageValidationResult>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ForwardMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, localchatmessageid: Param0, addresses: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), addresses.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, conversationid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), conversationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetConversationForTransportsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, conversationid: Param0, transportids: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), conversationid.into_param().abi(), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConversationFromThreadingInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatConversationThreadingInfo>>(&self, threadinginfo: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), threadinginfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetConversationReader(&self) -> ::windows::core::Result<ChatConversationReader> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConversationForTransportsReader<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, transportids: Param0) -> ::windows::core::Result<ChatConversationReader> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<ChatConversationReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageByRemoteIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, transportid: Param0, remoteid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), transportid.into_param().abi(), remoteid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetUnseenCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetUnseenCountForTransportsReaderAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, transportids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MarkAsSeenAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn MarkAsSeenForTransportsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, transportids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetSearchReader<'a, Param0: ::windows::core::IntoParam<'a, ChatQueryOptions>>(&self, value: Param0) -> ::windows::core::Result<ChatSearchReader> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<ChatSearchReader>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelDownloadMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelSendMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetMessageBySyncIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, syncid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::core::Interface::cast::<IChatMessageStore3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), syncid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStore;{31f2fd01-ccf6-580b-4976-0a07dd5d3b47})");
}
unsafe impl ::windows::core::Interface for ChatMessageStore {
    type Vtable = IChatMessageStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31f2fd01_ccf6_580b_4976_0a07dd5d3b47);
}
impl ::windows::core::RuntimeName for ChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStore";
}
impl ::core::convert::From<ChatMessageStore> for ::windows::core::IUnknown {
    fn from(value: ChatMessageStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageStore> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageStore> for ::windows::core::IInspectable {
    fn from(value: ChatMessageStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageStore> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageStore {}
unsafe impl ::core::marker::Sync for ChatMessageStore {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageStoreChangedEventArgs(::windows::core::IUnknown);
impl ChatMessageStoreChangedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Kind(&self) -> ::windows::core::Result<ChatStoreChangedEventKind> {
        let this = self;
        unsafe {
            let mut result__: ChatStoreChangedEventKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatStoreChangedEventKind>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageStoreChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs;{65c66fac-fe8c-46d4-9119-57b8410311d5})");
}
unsafe impl ::windows::core::Interface for ChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c66fac_fe8c_46d4_9119_57b8410311d5);
}
impl ::windows::core::RuntimeName for ChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
}
impl ::core::convert::From<ChatMessageStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ChatMessageStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ChatMessageStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for ChatMessageStoreChangedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageTransport(::windows::core::IUnknown);
impl ChatMessageTransport {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsAppSetAsNotificationProvider(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsNotificationProviderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Configuration(&self) -> ::windows::core::Result<ChatMessageTransportConfiguration> {
        let this = &::windows::core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageTransportConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportKind(&self) -> ::windows::core::Result<ChatMessageTransportKind> {
        let this = &::windows::core::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__: ChatMessageTransportKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageTransportKind>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransport;{63a9dbf8-e6b3-5c9a-5f85-d47925b9bd18})");
}
unsafe impl ::windows::core::Interface for ChatMessageTransport {
    type Vtable = IChatMessageTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a9dbf8_e6b3_5c9a_5f85_d47925b9bd18);
}
impl ::windows::core::RuntimeName for ChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransport";
}
impl ::core::convert::From<ChatMessageTransport> for ::windows::core::IUnknown {
    fn from(value: ChatMessageTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageTransport> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageTransport> for ::windows::core::IInspectable {
    fn from(value: ChatMessageTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageTransport> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageTransport {}
unsafe impl ::core::marker::Sync for ChatMessageTransport {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageTransportConfiguration(::windows::core::IUnknown);
impl ChatMessageTransportConfiguration {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxRecipientCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Media_MediaProperties'*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn SupportedVideoFormat(&self) -> ::windows::core::Result<super::super::Media::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageTransportConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration;{879ff725-1a08-4aca-a075-3355126312e6})");
}
unsafe impl ::windows::core::Interface for ChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x879ff725_1a08_4aca_a075_3355126312e6);
}
impl ::windows::core::RuntimeName for ChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
}
impl ::core::convert::From<ChatMessageTransportConfiguration> for ::windows::core::IUnknown {
    fn from(value: ChatMessageTransportConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageTransportConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageTransportConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageTransportConfiguration> for ::windows::core::IInspectable {
    fn from(value: ChatMessageTransportConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageTransportConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageTransportConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageTransportConfiguration {}
unsafe impl ::core::marker::Sync for ChatMessageTransportConfiguration {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageTransportKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageTransportKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageTransportKind {}
unsafe impl ::windows::core::RuntimeType for ChatMessageTransportKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageTransportKind;i4)");
}
impl ::windows::core::DefaultType for ChatMessageTransportKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatMessageValidationResult(::windows::core::IUnknown);
impl ChatMessageValidationResult {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxPartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PartCount(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemainingCharacterCountInPart(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageValidationStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageValidationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageValidationStatus>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatMessageValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageValidationResult;{25e93a03-28ec-5889-569b-7e486b126f18})");
}
unsafe impl ::windows::core::Interface for ChatMessageValidationResult {
    type Vtable = IChatMessageValidationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25e93a03_28ec_5889_569b_7e486b126f18);
}
impl ::windows::core::RuntimeName for ChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageValidationResult";
}
impl ::core::convert::From<ChatMessageValidationResult> for ::windows::core::IUnknown {
    fn from(value: ChatMessageValidationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageValidationResult> for ::windows::core::IUnknown {
    fn from(value: &ChatMessageValidationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatMessageValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatMessageValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatMessageValidationResult> for ::windows::core::IInspectable {
    fn from(value: ChatMessageValidationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatMessageValidationResult> for ::windows::core::IInspectable {
    fn from(value: &ChatMessageValidationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatMessageValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatMessageValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatMessageValidationResult {}
unsafe impl ::core::marker::Sync for ChatMessageValidationResult {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatMessageValidationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatMessageValidationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatMessageValidationStatus {}
unsafe impl ::windows::core::RuntimeType for ChatMessageValidationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageValidationStatus;i4)");
}
impl ::windows::core::DefaultType for ChatMessageValidationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatQueryOptions(::windows::core::IUnknown);
impl ChatQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatQueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SearchString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetSearchString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatQueryOptions;{2fd364a6-bf36-42f7-b7e7-923c0aabfe16})");
}
unsafe impl ::windows::core::Interface for ChatQueryOptions {
    type Vtable = IChatQueryOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fd364a6_bf36_42f7_b7e7_923c0aabfe16);
}
impl ::windows::core::RuntimeName for ChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatQueryOptions";
}
impl ::core::convert::From<ChatQueryOptions> for ::windows::core::IUnknown {
    fn from(value: ChatQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatQueryOptions> for ::windows::core::IUnknown {
    fn from(value: &ChatQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatQueryOptions> for ::windows::core::IInspectable {
    fn from(value: ChatQueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatQueryOptions> for ::windows::core::IInspectable {
    fn from(value: &ChatQueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatQueryOptions {}
unsafe impl ::core::marker::Sync for ChatQueryOptions {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatRecipientDeliveryInfo(::windows::core::IUnknown);
impl ChatRecipientDeliveryInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ChatRecipientDeliveryInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetTransportAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeliveryTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDeliveryTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetReadTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportErrorCodeCategory(&self) -> ::windows::core::Result<ChatTransportErrorCodeCategory> {
        let this = self;
        unsafe {
            let mut result__: ChatTransportErrorCodeCategory = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatTransportErrorCodeCategory>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportInterpretedErrorCode(&self) -> ::windows::core::Result<ChatTransportInterpretedErrorCode> {
        let this = self;
        unsafe {
            let mut result__: ChatTransportInterpretedErrorCode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatTransportInterpretedErrorCode>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportErrorCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsErrorPermanent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Status(&self) -> ::windows::core::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageStatus>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatRecipientDeliveryInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo;{ffc7b2a2-283c-4c0a-8a0e-8c33bdbf0545})");
}
unsafe impl ::windows::core::Interface for ChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc7b2a2_283c_4c0a_8a0e_8c33bdbf0545);
}
impl ::windows::core::RuntimeName for ChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
}
impl ::core::convert::From<ChatRecipientDeliveryInfo> for ::windows::core::IUnknown {
    fn from(value: ChatRecipientDeliveryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatRecipientDeliveryInfo> for ::windows::core::IUnknown {
    fn from(value: &ChatRecipientDeliveryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatRecipientDeliveryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatRecipientDeliveryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatRecipientDeliveryInfo> for ::windows::core::IInspectable {
    fn from(value: ChatRecipientDeliveryInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatRecipientDeliveryInfo> for ::windows::core::IInspectable {
    fn from(value: &ChatRecipientDeliveryInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatRecipientDeliveryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatRecipientDeliveryInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatRecipientDeliveryInfo {}
unsafe impl ::core::marker::Sync for ChatRecipientDeliveryInfo {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatRestoreHistorySpan {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatRestoreHistorySpan {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatRestoreHistorySpan {}
unsafe impl ::windows::core::RuntimeType for ChatRestoreHistorySpan {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatRestoreHistorySpan;i4)");
}
impl ::windows::core::DefaultType for ChatRestoreHistorySpan {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatSearchReader(::windows::core::IUnknown);
impl ChatSearchReader {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatSearchReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSearchReader;{4665fe49-9020-4752-980d-39612325f589})");
}
unsafe impl ::windows::core::Interface for ChatSearchReader {
    type Vtable = IChatSearchReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4665fe49_9020_4752_980d_39612325f589);
}
impl ::windows::core::RuntimeName for ChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSearchReader";
}
impl ::core::convert::From<ChatSearchReader> for ::windows::core::IUnknown {
    fn from(value: ChatSearchReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSearchReader> for ::windows::core::IUnknown {
    fn from(value: &ChatSearchReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatSearchReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatSearchReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatSearchReader> for ::windows::core::IInspectable {
    fn from(value: ChatSearchReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSearchReader> for ::windows::core::IInspectable {
    fn from(value: &ChatSearchReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatSearchReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatSearchReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatSearchReader {}
unsafe impl ::core::marker::Sync for ChatSearchReader {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatStoreChangedEventKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatStoreChangedEventKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatStoreChangedEventKind {}
unsafe impl ::windows::core::RuntimeType for ChatStoreChangedEventKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatStoreChangedEventKind;i4)");
}
impl ::windows::core::DefaultType for ChatStoreChangedEventKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatSyncConfiguration(::windows::core::IUnknown);
impl ChatSyncConfiguration {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn RestoreHistorySpan(&self) -> ::windows::core::Result<ChatRestoreHistorySpan> {
        let this = self;
        unsafe {
            let mut result__: ChatRestoreHistorySpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatRestoreHistorySpan>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
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
unsafe impl ::windows::core::RuntimeType for ChatSyncConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncConfiguration;{09f869b2-69f4-4aff-82b6-06992ff402d2})");
}
unsafe impl ::windows::core::Interface for ChatSyncConfiguration {
    type Vtable = IChatSyncConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09f869b2_69f4_4aff_82b6_06992ff402d2);
}
impl ::windows::core::RuntimeName for ChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncConfiguration";
}
impl ::core::convert::From<ChatSyncConfiguration> for ::windows::core::IUnknown {
    fn from(value: ChatSyncConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSyncConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ChatSyncConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatSyncConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatSyncConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatSyncConfiguration> for ::windows::core::IInspectable {
    fn from(value: ChatSyncConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSyncConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ChatSyncConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatSyncConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatSyncConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatSyncConfiguration {}
unsafe impl ::core::marker::Sync for ChatSyncConfiguration {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct ChatSyncManager(::windows::core::IUnknown);
impl ChatSyncManager {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Configuration(&self) -> ::windows::core::Result<ChatSyncConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatSyncConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Security_Credentials'*"]
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    pub fn AssociateAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(&self, webaccount: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UnassociateAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Security_Credentials'*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn IsAccountAssociated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(&self, webaccount: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn StartSync(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatSyncConfiguration>>(&self, configuration: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
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
unsafe impl ::windows::core::RuntimeType for ChatSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncManager;{7ba52c63-2650-486f-b4b4-6bd9d3d63c84})");
}
unsafe impl ::windows::core::Interface for ChatSyncManager {
    type Vtable = IChatSyncManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba52c63_2650_486f_b4b4_6bd9d3d63c84);
}
impl ::windows::core::RuntimeName for ChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncManager";
}
impl ::core::convert::From<ChatSyncManager> for ::windows::core::IUnknown {
    fn from(value: ChatSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSyncManager> for ::windows::core::IUnknown {
    fn from(value: &ChatSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ChatSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ChatSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ChatSyncManager> for ::windows::core::IInspectable {
    fn from(value: ChatSyncManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ChatSyncManager> for ::windows::core::IInspectable {
    fn from(value: &ChatSyncManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ChatSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &ChatSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ChatSyncManager {}
unsafe impl ::core::marker::Sync for ChatSyncManager {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatTransportErrorCodeCategory {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatTransportErrorCodeCategory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatTransportErrorCodeCategory {}
unsafe impl ::windows::core::RuntimeType for ChatTransportErrorCodeCategory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory;i4)");
}
impl ::windows::core::DefaultType for ChatTransportErrorCodeCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for ChatTransportInterpretedErrorCode {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ChatTransportInterpretedErrorCode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChatTransportInterpretedErrorCode {}
unsafe impl ::windows::core::RuntimeType for ChatTransportInterpretedErrorCode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode;i4)");
}
impl ::windows::core::DefaultType for ChatTransportInterpretedErrorCode {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatCapabilities {
    type Vtable = IChatCapabilitiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3aff77bc_39c9_4dd1_ad2d_3964dd9d403f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatCapabilitiesManagerStatics {
    type Vtable = IChatCapabilitiesManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb57a2f30_7041_458e_b0cf_7c0d9fea333a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatCapabilitiesManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatCapabilitiesManagerStatics2 {
    type Vtable = IChatCapabilitiesManagerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe30d4274_d5c1_4ac9_9ffc_40e69184fec8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, address: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatConversation {
    type Vtable = IChatConversationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa58c080d_1a6f_46dc_8f3d_f5028660b6ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, participantaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iscomposing: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, participantaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iscomposing: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversation2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatConversation2 {
    type Vtable = IChatConversation2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a030cd1_983a_47aa_9a90_ee48ee997b59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatConversationReader {
    type Vtable = IChatConversationReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x055136d2_de32_4a47_a93a_b3dc0833852b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatConversationThreadingInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x331c21dc_7a07_4422_a32c_24be7c6dab24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationThreadingInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatConversationThreadingKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatConversationThreadingKind) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct IChatItem(::windows::core::IUnknown);
impl IChatItem {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ItemKind(&self) -> ::windows::core::Result<ChatItemKind> {
        let this = self;
        unsafe {
            let mut result__: ChatItemKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
        }
    }
}
impl ::core::convert::From<IChatItem> for ::windows::core::IInspectable {
    fn from(value: IChatItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChatItem> for ::windows::core::IInspectable {
    fn from(value: &IChatItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IChatItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IChatItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IChatItem> for ::windows::core::IUnknown {
    fn from(value: IChatItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChatItem> for ::windows::core::IUnknown {
    fn from(value: &IChatItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IChatItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IChatItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::RuntimeType for IChatItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8751d000-ceb1-4243-b803-15d45a1dd428}");
}
unsafe impl ::windows::core::Interface for IChatItem {
    type Vtable = IChatItemVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8751d000_ceb1_4243_b803_15d45a1dd428);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatItemVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessage {
    type Vtable = IChatMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b39052a_1142_5089_76da_f2db3d17cd05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessage2 {
    type Vtable = IChatMessage2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86668332_543f_49f5_ac71_6c2afc6565fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageOperatorKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageOperatorKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatMessageStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessage3 {
    type Vtable = IChatMessage3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74eb2fb0_3ba7_459f_8e0b_e8af0febd9ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessage4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessage4 {
    type Vtable = IChatMessage4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d144b0f_d2bf_460c_aa68_6d3f8483c9bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageAttachment {
    type Vtable = IChatMessageAttachmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7c4fd74_bf63_58eb_508c_8b863ff16b67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachment2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageAttachment2 {
    type Vtable = IChatMessageAttachment2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ed99270_7dd1_4a87_a8ce_acdd87d80dc8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageAttachmentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageAttachmentFactory {
    type Vtable = IChatMessageAttachmentFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x205852a2_a356_5b71_6ca9_66c985b7d0d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachmentFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datastreamreference: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageBlockingStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageBlockingStatic {
    type Vtable = IChatMessageBlockingStaticVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6b9a380_cdea_11e4_8830_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageBlockingStaticVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, blocked: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageChange {
    type Vtable = IChatMessageChangeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c18c355_421e_54b8_6d38_6b3a6c82fccc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageChangeType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageChangeReader {
    type Vtable = IChatMessageChangeReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14267020_28ce_5f26_7b05_9a5c7cce87ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoacknowledge: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangeTracker(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTrackerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60b7f066_70a0_5224_508c_242ef7c1d06f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeTrackerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferralVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc6b30c_788c_4dcc_ace7_6282382968cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedDeferralVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b73e2d_691c_4edf_8660_6eb9896892e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManager2Statics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageManager2Statics {
    type Vtable = IChatMessageManager2StaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d45390f_9f4f_4e35_964e_1b9ca61ac044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManager2StaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageManagerStatic {
    type Vtable = IChatMessageManagerStaticVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf15c60f7_d5e8_5e92_556d_e03b60253104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStaticVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageManagerStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageManagerStatics3 {
    type Vtable = IChatMessageManagerStatics3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x208b830d_6755_48cc_9ab3_fd03c463fc92);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatics3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd344dfb_3063_4e17_8586_c6c08262e6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageNotificationTriggerDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageNotificationTriggerDetails2 {
    type Vtable = IChatMessageNotificationTriggerDetails2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bb522e0_aa07_4fd1_9471_77934fb75ee6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageReader {
    type Vtable = IChatMessageReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6ea78ce_4489_56f9_76aa_e204682514cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageReader2 {
    type Vtable = IChatMessageReader2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89643683_64bb_470d_9df4_0de8be1a05bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageStore {
    type Vtable = IChatMessageStoreVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31f2fd01_ccf6_580b_4976_0a07dd5d3b47);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStoreVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageStore2 {
    type Vtable = IChatMessageStore2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4dc4ee_3ad4_491b_b311_abdf9bb22768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, addresses: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversationid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadinginfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remoteid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, chatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localchatmessageid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStore3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageStore3 {
    type Vtable = IChatMessageStore3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9adbbb09_4345_4ec1_8b74_b7338243719c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageStoreChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65c66fac_fe8c_46d4_9119_57b8410311d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStoreChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatStoreChangedEventKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageTransport {
    type Vtable = IChatMessageTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63a9dbf8_e6b3_5c9a_5f85_d47925b9bd18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransport2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageTransport2 {
    type Vtable = IChatMessageTransport2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x90a75622_d84a_4c22_a94d_544444edc8a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageTransportKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageTransportConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x879ff725_1a08_4aca_a075_3355126312e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransportConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatMessageValidationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatMessageValidationResult {
    type Vtable = IChatMessageValidationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25e93a03_28ec_5889_569b_7e486b126f18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageValidationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageValidationStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatQueryOptions {
    type Vtable = IChatQueryOptionsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fd364a6_bf36_42f7_b7e7_923c0aabfe16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatQueryOptionsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatRecipientDeliveryInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffc7b2a2_283c_4c0a_8a0e_8c33bdbf0545);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatRecipientDeliveryInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportErrorCodeCategory) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatMessageStatus) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSearchReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatSearchReader {
    type Vtable = IChatSearchReaderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4665fe49_9020_4752_980d_39612325f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSearchReaderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatSyncConfiguration {
    type Vtable = IChatSyncConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09f869b2_69f4_4aff_82b6_06992ff402d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ChatRestoreHistorySpan) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ChatRestoreHistorySpan) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IChatSyncManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IChatSyncManager {
    type Vtable = IChatSyncManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ba52c63_2650_486f_b4b4_6bd9d3d63c84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessage {
    type Vtable = IRcsEndUserMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7cda5eb_cbd7_4f3b_8526_b506dec35c53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAction(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92378737_9b42_46d3_9d5e_3c1b2dae7cb8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageActionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d45ae01_3f89_41ea_9702_9e9ed411aa98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b97742d_351f_4692_b41e_1b035dc18986);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerDetailsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsEndUserMessageManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3054ae5a_4d1f_4b59_9433_126c734e86a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsManagerStatics {
    type Vtable = IRcsManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7d270ac5_0abd_4f31_9b99_a59e71a7b731);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transportid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, conversation: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsManagerStatics2 {
    type Vtable = IRcsManagerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd49ad18_ad8a_42aa_8eeb_a798a8808959);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsServiceKindSupportedChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf47ea244_e783_4866_b3a7_4e5ccf023070);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsServiceKindSupportedChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RcsServiceKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsTransport {
    type Vtable = IRcsTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea34759_f37c_4319_8546_ec84d21d30ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransportVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRcsTransportConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRcsTransportConfiguration {
    type Vtable = IRcsTransportConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fccb102_2472_4bb9_9988_c1211c83e8a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransportConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteParticipantComposingChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec045a7_cfc9_45c9_9876_449f2bc180f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteParticipantComposingChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsEndUserMessage(::windows::core::IUnknown);
impl RcsEndUserMessage {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsPinRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Actions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseAsync<'a, Param0: ::windows::core::IntoParam<'a, RcsEndUserMessageAction>>(&self, action: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), action.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendResponseWithPinAsync<'a, Param0: ::windows::core::IntoParam<'a, RcsEndUserMessageAction>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, action: Param0, pin: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), action.into_param().abi(), pin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessage;{d7cda5eb-cbd7-4f3b-8526-b506dec35c53})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessage {
    type Vtable = IRcsEndUserMessageVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7cda5eb_cbd7_4f3b_8526_b506dec35c53);
}
impl ::windows::core::RuntimeName for RcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessage";
}
impl ::core::convert::From<RcsEndUserMessage> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessage> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessage> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessage> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessage {}
unsafe impl ::core::marker::Sync for RcsEndUserMessage {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAction(::windows::core::IUnknown);
impl RcsEndUserMessageAction {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAction;{92378737-9b42-46d3-9d5e-3c1b2dae7cb8})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageActionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92378737_9b42_46d3_9d5e_3c1b2dae7cb8);
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
}
impl ::core::convert::From<RcsEndUserMessageAction> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAction> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessageAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessageAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessageAction> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessageAction) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAction> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessageAction) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessageAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessageAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessageAction {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAction {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableEventArgs(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsMessageAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Message(&self) -> ::windows::core::Result<RcsEndUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RcsEndUserMessage>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs;{2d45ae01-3f89-41ea-9702-9e9ed411aa98})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d45ae01_3f89_41ea_9702_9e9ed411aa98);
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
}
impl ::core::convert::From<RcsEndUserMessageAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessageAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessageAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessageAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessageAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessageAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessageAvailableEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessageAvailableEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessageAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessageAvailableEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableEventArgs {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTriggerDetails(::windows::core::IUnknown);
impl RcsEndUserMessageAvailableTriggerDetails {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails;{5b97742d-351f-4692-b41e-1b035dc18986})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetailsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b97742d_351f_4692_b41e_1b035dc18986);
}
impl ::windows::core::RuntimeName for RcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
}
impl ::core::convert::From<RcsEndUserMessageAvailableTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessageAvailableTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessageAvailableTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessageAvailableTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessageAvailableTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessageAvailableTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessageAvailableTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageAvailableTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessageAvailableTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessageAvailableTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessageAvailableTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessageAvailableTriggerDetails {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageAvailableTriggerDetails {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsEndUserMessageManager(::windows::core::IUnknown);
impl RcsEndUserMessageManager {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageAvailableChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageAvailableChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::RuntimeType for RcsEndUserMessageManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageManager;{3054ae5a-4d1f-4b59-9433-126c734e86a6})");
}
unsafe impl ::windows::core::Interface for RcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3054ae5a_4d1f_4b59_9433_126c734e86a6);
}
impl ::windows::core::RuntimeName for RcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
}
impl ::core::convert::From<RcsEndUserMessageManager> for ::windows::core::IUnknown {
    fn from(value: RcsEndUserMessageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageManager> for ::windows::core::IUnknown {
    fn from(value: &RcsEndUserMessageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsEndUserMessageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsEndUserMessageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsEndUserMessageManager> for ::windows::core::IInspectable {
    fn from(value: RcsEndUserMessageManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsEndUserMessageManager> for ::windows::core::IInspectable {
    fn from(value: &RcsEndUserMessageManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsEndUserMessageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsEndUserMessageManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsEndUserMessageManager {}
unsafe impl ::core::marker::Sync for RcsEndUserMessageManager {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
pub struct RcsManager {}
impl RcsManager {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn GetEndUserMessageManager() -> ::windows::core::Result<RcsEndUserMessageManager> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RcsEndUserMessageManager>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetTransportsAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTransportAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(transportid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RcsTransport>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RcsTransport>>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn LeaveConversationAsync<'a, Param0: ::windows::core::IntoParam<'a, ChatConversation>>(conversation: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), conversation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TransportListChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRcsManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTransportListChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IRcsManagerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics<R, F: FnOnce(&IRcsManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RcsManager, IRcsManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IRcsManagerStatics2<R, F: FnOnce(&IRcsManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RcsManager, IRcsManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for RcsManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsManager";
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for RcsServiceKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RcsServiceKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RcsServiceKind {}
unsafe impl ::windows::core::RuntimeType for RcsServiceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.RcsServiceKind;i4)");
}
impl ::windows::core::DefaultType for RcsServiceKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsServiceKindSupportedChangedEventArgs(::windows::core::IUnknown);
impl RcsServiceKindSupportedChangedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ServiceKind(&self) -> ::windows::core::Result<RcsServiceKind> {
        let this = self;
        unsafe {
            let mut result__: RcsServiceKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RcsServiceKind>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs;{f47ea244-e783-4866-b3a7-4e5ccf023070})");
}
unsafe impl ::windows::core::Interface for RcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf47ea244_e783_4866_b3a7_4e5ccf023070);
}
impl ::windows::core::RuntimeName for RcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
}
impl ::core::convert::From<RcsServiceKindSupportedChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RcsServiceKindSupportedChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsServiceKindSupportedChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RcsServiceKindSupportedChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsServiceKindSupportedChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsServiceKindSupportedChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsServiceKindSupportedChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RcsServiceKindSupportedChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsServiceKindSupportedChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RcsServiceKindSupportedChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsServiceKindSupportedChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsServiceKindSupportedChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsServiceKindSupportedChangedEventArgs {}
unsafe impl ::core::marker::Sync for RcsServiceKindSupportedChangedEventArgs {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsTransport(::windows::core::IUnknown);
impl RcsTransport {
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn Configuration(&self) -> ::windows::core::Result<RcsTransportConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RcsTransportConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), servicekind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), servicekind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceKindSupportedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceKindSupportedChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::RuntimeType for RcsTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransport;{fea34759-f37c-4319-8546-ec84d21d30ff})");
}
unsafe impl ::windows::core::Interface for RcsTransport {
    type Vtable = IRcsTransportVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea34759_f37c_4319_8546_ec84d21d30ff);
}
impl ::windows::core::RuntimeName for RcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransport";
}
impl ::core::convert::From<RcsTransport> for ::windows::core::IUnknown {
    fn from(value: RcsTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsTransport> for ::windows::core::IUnknown {
    fn from(value: &RcsTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsTransport> for ::windows::core::IInspectable {
    fn from(value: RcsTransport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsTransport> for ::windows::core::IInspectable {
    fn from(value: &RcsTransport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsTransport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsTransport {}
unsafe impl ::core::marker::Sync for RcsTransport {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RcsTransportConfiguration(::windows::core::IUnknown);
impl RcsTransportConfiguration {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxAttachmentCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxRecipientCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn MaxFileSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn WarningFileSizeInKilobytes(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
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
unsafe impl ::windows::core::RuntimeType for RcsTransportConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransportConfiguration;{1fccb102-2472-4bb9-9988-c1211c83e8a9})");
}
unsafe impl ::windows::core::Interface for RcsTransportConfiguration {
    type Vtable = IRcsTransportConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fccb102_2472_4bb9_9988_c1211c83e8a9);
}
impl ::windows::core::RuntimeName for RcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransportConfiguration";
}
impl ::core::convert::From<RcsTransportConfiguration> for ::windows::core::IUnknown {
    fn from(value: RcsTransportConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsTransportConfiguration> for ::windows::core::IUnknown {
    fn from(value: &RcsTransportConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RcsTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RcsTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RcsTransportConfiguration> for ::windows::core::IInspectable {
    fn from(value: RcsTransportConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RcsTransportConfiguration> for ::windows::core::IInspectable {
    fn from(value: &RcsTransportConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RcsTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RcsTransportConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RcsTransportConfiguration {}
unsafe impl ::core::marker::Sync for RcsTransportConfiguration {}
#[doc = "*Required features: 'ApplicationModel_Chat'*"]
#[repr(transparent)]
pub struct RemoteParticipantComposingChangedEventArgs(::windows::core::IUnknown);
impl RemoteParticipantComposingChangedEventArgs {
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn TransportId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn ParticipantAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'ApplicationModel_Chat'*"]
    pub fn IsComposing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::RuntimeType for RemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs;{1ec045a7-cfc9-45c9-9876-449f2bc180f5})");
}
unsafe impl ::windows::core::Interface for RemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ec045a7_cfc9_45c9_9876_449f2bc180f5);
}
impl ::windows::core::RuntimeName for RemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
}
impl ::core::convert::From<RemoteParticipantComposingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: RemoteParticipantComposingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteParticipantComposingChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &RemoteParticipantComposingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteParticipantComposingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RemoteParticipantComposingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RemoteParticipantComposingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: RemoteParticipantComposingChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteParticipantComposingChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &RemoteParticipantComposingChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteParticipantComposingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RemoteParticipantComposingChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RemoteParticipantComposingChangedEventArgs {}
unsafe impl ::core::marker::Sync for RemoteParticipantComposingChangedEventArgs {}
