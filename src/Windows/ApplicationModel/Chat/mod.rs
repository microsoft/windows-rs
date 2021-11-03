#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatCapabilities(::windows::runtime::IInspectable);
impl ChatCapabilities {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsOnline(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsChatCapable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsFileTransferCapable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsGeoLocationPushCapable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsIntegratedMessagingCapable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatCapabilities;{3aff77bc-39c9-4dd1-ad2d-3964dd9d403f})");
}
unsafe impl ::windows::runtime::Interface for ChatCapabilities {
    type Vtable = IChatCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(989820860, 14793, 19921, [173, 45, 57, 100, 221, 157, 64, 63]);
}
impl ::windows::runtime::RuntimeName for ChatCapabilities {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilities";
}
unsafe impl ::std::marker::Send for ChatCapabilities {}
unsafe impl ::std::marker::Sync for ChatCapabilities {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
pub struct ChatCapabilitiesManager {}
impl ChatCapabilitiesManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetCachedCapabilitiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(address: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), address.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetCapabilitiesFromNetworkAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(address: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), address.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetCachedCapabilitiesForTransportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(address: Param0, transportid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), address.into_param().abi(), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetCapabilitiesFromNetworkForTransportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(address: Param0, transportid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatCapabilities>> {
        Self::IChatCapabilitiesManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), address.into_param().abi(), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatCapabilities>>(result__)
        })
    }
    pub fn IChatCapabilitiesManagerStatics<R, F: FnOnce(&IChatCapabilitiesManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IChatCapabilitiesManagerStatics2<R, F: FnOnce(&IChatCapabilitiesManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatCapabilitiesManager, IChatCapabilitiesManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ChatCapabilitiesManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatConversation(::windows::runtime::IInspectable);
impl ChatConversation {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn HasUnreadMessages(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsConversationMuted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsConversationMuted(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MostRecentMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn Participants(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ThreadingInfo(&self) -> ::windows::runtime::Result<ChatConversationThreadingInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetMessageReader(&self) -> ::windows::runtime::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MarkAllMessagesAsReadAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MarkMessagesAsReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn NotifyLocalParticipantComposing<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, transportid: Param0, participantaddress: Param1, iscomposing: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), transportid.into_param().abi(), participantaddress.into_param().abi(), iscomposing).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn NotifyRemoteParticipantComposing<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, transportid: Param0, participantaddress: Param1, iscomposing: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), transportid.into_param().abi(), participantaddress.into_param().abi(), iscomposing).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoteParticipantComposingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatConversation, RemoteParticipantComposingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveRemoteParticipantComposingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ItemKind(&self) -> ::windows::runtime::Result<ChatItemKind> {
        let this = &::windows::runtime::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__: ChatItemKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn CanModifyParticipants(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatConversation2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetCanModifyParticipants(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatConversation2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatConversation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversation;{a58c080d-1a6f-46dc-8f3d-f5028660b6ee})");
}
unsafe impl ::windows::runtime::Interface for ChatConversation {
    type Vtable = IChatConversation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2777417741, 6767, 18140, [143, 61, 245, 2, 134, 96, 182, 238]);
}
impl ::windows::runtime::RuntimeName for ChatConversation {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversation";
}
impl ::std::convert::TryFrom<ChatConversation> for IChatItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ChatConversation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ChatConversation> for IChatItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ChatConversation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IChatItem> for ChatConversation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IChatItem> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IChatItem> for &ChatConversation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IChatItem> {
        ::std::convert::TryInto::<IChatItem>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ChatConversation {}
unsafe impl ::std::marker::Sync for ChatConversation {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatConversationReader(::windows::runtime::IInspectable);
impl ChatConversationReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatConversation>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatConversationReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationReader;{055136d2-de32-4a47-a93a-b3dc0833852b})");
}
unsafe impl ::windows::runtime::Interface for ChatConversationReader {
    type Vtable = IChatConversationReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89208530, 56882, 19015, [169, 58, 179, 220, 8, 51, 133, 43]);
}
impl ::windows::runtime::RuntimeName for ChatConversationReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationReader";
}
unsafe impl ::std::marker::Send for ChatConversationReader {}
unsafe impl ::std::marker::Sync for ChatConversationReader {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatConversationThreadingInfo(::windows::runtime::IInspectable);
impl ChatConversationThreadingInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatConversationThreadingInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ContactId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetContactId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Custom(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetCustom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ConversationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetConversationId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn Participants(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ChatConversationThreadingKind> {
        let this = self;
        unsafe {
            let mut result__: ChatConversationThreadingKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetKind(&self, value: ChatConversationThreadingKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatConversationThreadingInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatConversationThreadingInfo;{331c21dc-7a07-4422-a32c-24be7c6dab24})");
}
unsafe impl ::windows::runtime::Interface for ChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(857481692, 31239, 17442, [163, 44, 36, 190, 124, 109, 171, 36]);
}
impl ::windows::runtime::RuntimeName for ChatConversationThreadingInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
}
unsafe impl ::std::marker::Send for ChatConversationThreadingInfo {}
unsafe impl ::std::marker::Sync for ChatConversationThreadingInfo {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatConversationThreadingKind(pub i32);
impl ChatConversationThreadingKind {
    pub const Participants: ChatConversationThreadingKind = ChatConversationThreadingKind(0i32);
    pub const ContactId: ChatConversationThreadingKind = ChatConversationThreadingKind(1i32);
    pub const ConversationId: ChatConversationThreadingKind = ChatConversationThreadingKind(2i32);
    pub const Custom: ChatConversationThreadingKind = ChatConversationThreadingKind(3i32);
}
impl ::std::convert::From<i32> for ChatConversationThreadingKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatConversationThreadingKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatConversationThreadingKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatConversationThreadingKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatItemKind(pub i32);
impl ChatItemKind {
    pub const Message: ChatItemKind = ChatItemKind(0i32);
    pub const Conversation: ChatItemKind = ChatItemKind(1i32);
}
impl ::std::convert::From<i32> for ChatItemKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatItemKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatItemKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatItemKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessage(::windows::runtime::IInspectable);
impl ChatMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn Attachments(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ChatMessageAttachment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ChatMessageAttachment>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsForwardingDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsIncoming(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsRead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn LocalTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn NetworkTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn Recipients(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn RecipientSendStatuses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ChatMessageStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ChatMessageStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportFriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetTransportId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ItemKind(&self) -> ::windows::runtime::Result<ChatItemKind> {
        let this = &::windows::runtime::Interface::cast::<IChatItem>(self)?;
        unsafe {
            let mut result__: ChatItemKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn EstimatedDownloadSize(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetEstimatedDownloadSize(&self, value: u64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetFrom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsAutoReply(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsAutoReply(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsForwardingDisabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsReplyDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsIncoming(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsRead(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsSeen(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsSeen(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsSimMessage(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SetLocalTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MessageKind(&self) -> ::windows::runtime::Result<ChatMessageKind> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ChatMessageKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetMessageKind(&self, value: ChatMessageKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MessageOperatorKind(&self) -> ::windows::runtime::Result<ChatMessageOperatorKind> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ChatMessageOperatorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageOperatorKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetMessageOperatorKind(&self, value: ChatMessageOperatorKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SetNetworkTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsReceivedDuringQuietHours(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsReceivedDuringQuietHours(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetStatus(&self, value: ChatMessageStatus) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShouldSuppressNotification(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetShouldSuppressNotification(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ThreadingInfo(&self) -> ::windows::runtime::Result<ChatConversationThreadingInfo> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationThreadingInfo>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetThreadingInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ChatConversationThreadingInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn RecipientsDeliveryInfos(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ChatRecipientDeliveryInfo>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SyncId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage4>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetSyncId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessage4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessage;{4b39052a-1142-5089-76da-f2db3d17cd05})");
}
unsafe impl ::windows::runtime::Interface for ChatMessage {
    type Vtable = IChatMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1262028074, 4418, 20617, [118, 218, 242, 219, 61, 23, 205, 5]);
}
impl ::windows::runtime::RuntimeName for ChatMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessage";
}
impl ::std::convert::TryFrom<ChatMessage> for IChatItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ChatMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ChatMessage> for IChatItem {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ChatMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IChatItem> for ChatMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IChatItem> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IChatItem> for &ChatMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, IChatItem> {
        ::std::convert::TryInto::<IChatItem>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ChatMessage {}
unsafe impl ::std::marker::Sync for ChatMessage {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageAttachment(::windows::runtime::IInspectable);
impl ChatMessageAttachment {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Storage_Streams`*"]
    pub fn DataStreamReference(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Storage_Streams`*"]
    pub fn SetDataStreamReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetGroupId(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MimeType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetMimeType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Storage_Streams`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransferProgress(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetTransferProgress(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn OriginalFileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetOriginalFileName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageAttachment2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Storage_Streams`*"]
    pub fn CreateChatMessageAttachment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(mimetype: Param0, datastreamreference: Param1) -> ::windows::runtime::Result<ChatMessageAttachment> {
        Self::IChatMessageAttachmentFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), mimetype.into_param().abi(), datastreamreference.into_param().abi(), &mut result__).from_abi::<ChatMessageAttachment>(result__)
        })
    }
    pub fn IChatMessageAttachmentFactory<R, F: FnOnce(&IChatMessageAttachmentFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessageAttachment, IChatMessageAttachmentFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageAttachment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageAttachment;{c7c4fd74-bf63-58eb-508c-8b863ff16b67})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageAttachment {
    type Vtable = IChatMessageAttachment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3351575924, 48995, 22763, [80, 140, 139, 134, 63, 241, 107, 103]);
}
impl ::windows::runtime::RuntimeName for ChatMessageAttachment {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageAttachment";
}
unsafe impl ::std::marker::Send for ChatMessageAttachment {}
unsafe impl ::std::marker::Sync for ChatMessageAttachment {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
pub struct ChatMessageBlocking {}
impl ChatMessageBlocking {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MarkMessageAsBlockedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(localchatmessageid: Param0, blocked: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageBlockingStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), blocked, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IChatMessageBlockingStatic<R, F: FnOnce(&IChatMessageBlockingStatic) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessageBlocking, IChatMessageBlockingStatic> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ChatMessageBlocking {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageBlocking";
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageChange(::windows::runtime::IInspectable);
impl ChatMessageChange {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ChangeType(&self) -> ::windows::runtime::Result<ChatMessageChangeType> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageChangeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChange;{1c18c355-421e-54b8-6d38-6b3a6c82fccc})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageChange {
    type Vtable = IChatMessageChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(471384917, 16926, 21688, [109, 56, 107, 58, 108, 130, 252, 204]);
}
impl ::windows::runtime::RuntimeName for ChatMessageChange {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChange";
}
unsafe impl ::std::marker::Send for ChatMessageChange {}
unsafe impl ::std::marker::Sync for ChatMessageChange {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageChangeReader(::windows::runtime::IInspectable);
impl ChatMessageChangeReader {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn AcceptChanges(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn AcceptChangesThrough<'a, Param0: ::windows::runtime::IntoParam<'a, ChatMessageChange>>(&self, lastchangetoacknowledge: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), lastchangetoacknowledge.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageChange>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChangeReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeReader;{14267020-28ce-5f26-7b05-9a5c7cce87ca})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(338063392, 10446, 24358, [123, 5, 154, 92, 124, 206, 135, 202]);
}
impl ::windows::runtime::RuntimeName for ChatMessageChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeReader";
}
unsafe impl ::std::marker::Send for ChatMessageChangeReader {}
unsafe impl ::std::marker::Sync for ChatMessageChangeReader {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageChangeTracker(::windows::runtime::IInspectable);
impl ChatMessageChangeTracker {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Enable(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetChangeReader(&self) -> ::windows::runtime::Result<ChatMessageChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChangeTracker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangeTracker;{60b7f066-70a0-5224-508c-242ef7c1d06f})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1622667366, 28832, 21028, [80, 140, 36, 46, 247, 193, 208, 111]);
}
impl ::windows::runtime::RuntimeName for ChatMessageChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
}
unsafe impl ::std::marker::Send for ChatMessageChangeTracker {}
unsafe impl ::std::marker::Sync for ChatMessageChangeTracker {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageChangeType(pub i32);
impl ChatMessageChangeType {
    pub const MessageCreated: ChatMessageChangeType = ChatMessageChangeType(0i32);
    pub const MessageModified: ChatMessageChangeType = ChatMessageChangeType(1i32);
    pub const MessageDeleted: ChatMessageChangeType = ChatMessageChangeType(2i32);
    pub const ChangeTrackingLost: ChatMessageChangeType = ChatMessageChangeType(3i32);
}
impl ::std::convert::From<i32> for ChatMessageChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageChangeType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChangeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageChangeType;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageChangedDeferral(::windows::runtime::IInspectable);
impl ChatMessageChangedDeferral {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChangedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedDeferral;{fbc6b30c-788c-4dcc-ace7-6282382968cf})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224103180, 30860, 19916, [172, 231, 98, 130, 56, 41, 104, 207]);
}
impl ::windows::runtime::RuntimeName for ChatMessageChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
}
unsafe impl ::std::marker::Send for ChatMessageChangedDeferral {}
unsafe impl ::std::marker::Sync for ChatMessageChangedDeferral {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageChangedEventArgs(::windows::runtime::IInspectable);
impl ChatMessageChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<ChatMessageChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs;{b6b73e2d-691c-4edf-8660-6eb9896892e3})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3065462317, 26908, 20191, [134, 96, 110, 185, 137, 104, 146, 227]);
}
impl ::windows::runtime::RuntimeName for ChatMessageChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
}
unsafe impl ::std::marker::Send for ChatMessageChangedEventArgs {}
unsafe impl ::std::marker::Sync for ChatMessageChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageKind(pub i32);
impl ChatMessageKind {
    pub const Standard: ChatMessageKind = ChatMessageKind(0i32);
    pub const FileTransferRequest: ChatMessageKind = ChatMessageKind(1i32);
    pub const TransportCustom: ChatMessageKind = ChatMessageKind(2i32);
    pub const JoinedConversation: ChatMessageKind = ChatMessageKind(3i32);
    pub const LeftConversation: ChatMessageKind = ChatMessageKind(4i32);
    pub const OtherParticipantJoinedConversation: ChatMessageKind = ChatMessageKind(5i32);
    pub const OtherParticipantLeftConversation: ChatMessageKind = ChatMessageKind(6i32);
}
impl ::std::convert::From<i32> for ChatMessageKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
pub struct ChatMessageManager {}
impl ChatMessageManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RegisterTransportAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetTransportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(transportid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessageTransport>> {
        Self::IChatMessageManager2Statics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessageTransport>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetTransportsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessageTransport>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RequestStoreAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessageStore>> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessageStore>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn ShowComposeSmsMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatMessage>>(message: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IChatMessageManagerStatic(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShowSmsSettings() -> ::windows::runtime::Result<()> {
        Self::IChatMessageManagerStatic(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RequestSyncManagerAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatSyncManager>> {
        Self::IChatMessageManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatSyncManager>>(result__)
        })
    }
    pub fn IChatMessageManager2Statics<R, F: FnOnce(&IChatMessageManager2Statics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessageManager, IChatMessageManager2Statics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IChatMessageManagerStatic<R, F: FnOnce(&IChatMessageManagerStatic) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessageManager, IChatMessageManagerStatic> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IChatMessageManagerStatics3<R, F: FnOnce(&IChatMessageManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatMessageManager, IChatMessageManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ChatMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageManager";
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageNotificationTriggerDetails(::windows::runtime::IInspectable);
impl ChatMessageNotificationTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ChatMessage(&self) -> ::windows::runtime::Result<ChatMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessage>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShouldDisplayToast(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShouldUpdateDetailText(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShouldUpdateBadge(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ShouldUpdateActionCenter(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageNotificationTriggerDetails2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails;{fd344dfb-3063-4e17-8586-c6c08262e6c0})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4248063483, 12387, 19991, [133, 134, 198, 192, 130, 98, 230, 192]);
}
impl ::windows::runtime::RuntimeName for ChatMessageNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
}
unsafe impl ::std::marker::Send for ChatMessageNotificationTriggerDetails {}
unsafe impl ::std::marker::Sync for ChatMessageNotificationTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageOperatorKind(pub i32);
impl ChatMessageOperatorKind {
    pub const Unspecified: ChatMessageOperatorKind = ChatMessageOperatorKind(0i32);
    pub const Sms: ChatMessageOperatorKind = ChatMessageOperatorKind(1i32);
    pub const Mms: ChatMessageOperatorKind = ChatMessageOperatorKind(2i32);
    pub const Rcs: ChatMessageOperatorKind = ChatMessageOperatorKind(3i32);
}
impl ::std::convert::From<i32> for ChatMessageOperatorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageOperatorKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageOperatorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageOperatorKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageReader(::windows::runtime::IInspectable);
impl ChatMessageReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageReader2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ChatMessage>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageReader;{b6ea78ce-4489-56f9-76aa-e204682514cf})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageReader {
    type Vtable = IChatMessageReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3068819662, 17545, 22265, [118, 170, 226, 4, 104, 37, 20, 207]);
}
impl ::windows::runtime::RuntimeName for ChatMessageReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageReader";
}
unsafe impl ::std::marker::Send for ChatMessageReader {}
unsafe impl ::std::marker::Sync for ChatMessageReader {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageStatus(pub i32);
impl ChatMessageStatus {
    pub const Draft: ChatMessageStatus = ChatMessageStatus(0i32);
    pub const Sending: ChatMessageStatus = ChatMessageStatus(1i32);
    pub const Sent: ChatMessageStatus = ChatMessageStatus(2i32);
    pub const SendRetryNeeded: ChatMessageStatus = ChatMessageStatus(3i32);
    pub const SendFailed: ChatMessageStatus = ChatMessageStatus(4i32);
    pub const Received: ChatMessageStatus = ChatMessageStatus(5i32);
    pub const ReceiveDownloadNeeded: ChatMessageStatus = ChatMessageStatus(6i32);
    pub const ReceiveDownloadFailed: ChatMessageStatus = ChatMessageStatus(7i32);
    pub const ReceiveDownloading: ChatMessageStatus = ChatMessageStatus(8i32);
    pub const Deleted: ChatMessageStatus = ChatMessageStatus(9i32);
    pub const Declined: ChatMessageStatus = ChatMessageStatus(10i32);
    pub const Cancelled: ChatMessageStatus = ChatMessageStatus(11i32);
    pub const Recalled: ChatMessageStatus = ChatMessageStatus(12i32);
    pub const ReceiveRetryNeeded: ChatMessageStatus = ChatMessageStatus(13i32);
}
impl ::std::convert::From<i32> for ChatMessageStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageStatus;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageStore(::windows::runtime::IInspectable);
impl ChatMessageStore {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ChangeTracker(&self) -> ::windows::runtime::Result<ChatMessageChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageChangeTracker>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn DeleteMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), localmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn DownloadMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetMessageReader1(&self) -> ::windows::runtime::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetMessageReader2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, recenttimelimit: Param0) -> ::windows::runtime::Result<ChatMessageReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), recenttimelimit.into_param().abi(), &mut result__).from_abi::<ChatMessageReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MarkMessageReadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RetrySendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ValidateMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::runtime::Result<ChatMessageValidationResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<ChatMessageValidationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MessageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveMessageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ForwardMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, localchatmessageid: Param0, addresses: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), addresses.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetConversationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, conversationid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), conversationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetConversationForTransportsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, conversationid: Param0, transportids: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), conversationid.into_param().abi(), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetConversationFromThreadingInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatConversationThreadingInfo>>(&self, threadinginfo: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatConversation>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), threadinginfo.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatConversation>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetConversationReader(&self) -> ::windows::runtime::Result<ChatConversationReader> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatConversationReader>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn GetConversationForTransportsReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, transportids: Param0) -> ::windows::runtime::Result<ChatConversationReader> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<ChatConversationReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetMessageByRemoteIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, transportid: Param0, remoteid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), transportid.into_param().abi(), remoteid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetUnseenCountAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetUnseenCountForTransportsReaderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, transportids: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<i32>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MarkAsSeenAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn MarkAsSeenForTransportsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, transportids: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), transportids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetSearchReader<'a, Param0: ::windows::runtime::IntoParam<'a, ChatQueryOptions>>(&self, value: Param0) -> ::windows::runtime::Result<ChatSearchReader> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<ChatSearchReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SaveMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatMessage>>(&self, chatmessage: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), chatmessage.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn TryCancelDownloadMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn TryCancelSendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, localchatmessageid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), localchatmessageid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn StoreChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ChatMessageStore, ChatMessageStoreChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetMessageBySyncIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, syncid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ChatMessage>> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageStore3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), syncid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ChatMessage>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStore;{31f2fd01-ccf6-580b-4976-0a07dd5d3b47})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageStore {
    type Vtable = IChatMessageStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(838008065, 52470, 22539, [73, 118, 10, 7, 221, 93, 59, 71]);
}
impl ::windows::runtime::RuntimeName for ChatMessageStore {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStore";
}
unsafe impl ::std::marker::Send for ChatMessageStore {}
unsafe impl ::std::marker::Sync for ChatMessageStore {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageStoreChangedEventArgs(::windows::runtime::IInspectable);
impl ChatMessageStoreChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ChatStoreChangedEventKind> {
        let this = self;
        unsafe {
            let mut result__: ChatStoreChangedEventKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatStoreChangedEventKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageStoreChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs;{65c66fac-fe8c-46d4-9119-57b8410311d5})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1707503532, 65164, 18132, [145, 25, 87, 184, 65, 3, 17, 213]);
}
impl ::windows::runtime::RuntimeName for ChatMessageStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
}
unsafe impl ::std::marker::Send for ChatMessageStoreChangedEventArgs {}
unsafe impl ::std::marker::Sync for ChatMessageStoreChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageTransport(::windows::runtime::IInspectable);
impl ChatMessageTransport {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsAppSetAsNotificationProvider(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportFriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RequestSetAsNotificationProviderAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<ChatMessageTransportConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageTransportConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportKind(&self) -> ::windows::runtime::Result<ChatMessageTransportKind> {
        let this = &::windows::runtime::Interface::cast::<IChatMessageTransport2>(self)?;
        unsafe {
            let mut result__: ChatMessageTransportKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageTransportKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageTransport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransport;{63a9dbf8-e6b3-5c9a-5f85-d47925b9bd18})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageTransport {
    type Vtable = IChatMessageTransport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1672076280, 59059, 23706, [95, 133, 212, 121, 37, 185, 189, 24]);
}
impl ::windows::runtime::RuntimeName for ChatMessageTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransport";
}
unsafe impl ::std::marker::Send for ChatMessageTransport {}
unsafe impl ::std::marker::Sync for ChatMessageTransport {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageTransportConfiguration(::windows::runtime::IInspectable);
impl ChatMessageTransportConfiguration {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxAttachmentCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxRecipientCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Media_MediaProperties")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Media_MediaProperties`*"]
    pub fn SupportedVideoFormat(&self) -> ::windows::runtime::Result<super::super::Media::MediaProperties::MediaEncodingProfile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::MediaProperties::MediaEncodingProfile>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageTransportConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration;{879ff725-1a08-4aca-a075-3355126312e6})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275407653, 6664, 19146, [160, 117, 51, 85, 18, 99, 18, 230]);
}
impl ::windows::runtime::RuntimeName for ChatMessageTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
}
unsafe impl ::std::marker::Send for ChatMessageTransportConfiguration {}
unsafe impl ::std::marker::Sync for ChatMessageTransportConfiguration {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageTransportKind(pub i32);
impl ChatMessageTransportKind {
    pub const Text: ChatMessageTransportKind = ChatMessageTransportKind(0i32);
    pub const Untriaged: ChatMessageTransportKind = ChatMessageTransportKind(1i32);
    pub const Blocked: ChatMessageTransportKind = ChatMessageTransportKind(2i32);
    pub const Custom: ChatMessageTransportKind = ChatMessageTransportKind(3i32);
}
impl ::std::convert::From<i32> for ChatMessageTransportKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageTransportKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageTransportKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageTransportKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatMessageValidationResult(::windows::runtime::IInspectable);
impl ChatMessageValidationResult {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MaxPartCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn PartCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemainingCharacterCountInPart(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ChatMessageValidationStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageValidationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageValidationStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageValidationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatMessageValidationResult;{25e93a03-28ec-5889-569b-7e486b126f18})");
}
unsafe impl ::windows::runtime::Interface for ChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(636041731, 10476, 22665, [86, 155, 126, 72, 107, 18, 111, 24]);
}
impl ::windows::runtime::RuntimeName for ChatMessageValidationResult {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatMessageValidationResult";
}
unsafe impl ::std::marker::Send for ChatMessageValidationResult {}
unsafe impl ::std::marker::Sync for ChatMessageValidationResult {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatMessageValidationStatus(pub i32);
impl ChatMessageValidationStatus {
    pub const Valid: ChatMessageValidationStatus = ChatMessageValidationStatus(0i32);
    pub const NoRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(1i32);
    pub const InvalidData: ChatMessageValidationStatus = ChatMessageValidationStatus(2i32);
    pub const MessageTooLarge: ChatMessageValidationStatus = ChatMessageValidationStatus(3i32);
    pub const TooManyRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(4i32);
    pub const TransportInactive: ChatMessageValidationStatus = ChatMessageValidationStatus(5i32);
    pub const TransportNotFound: ChatMessageValidationStatus = ChatMessageValidationStatus(6i32);
    pub const TooManyAttachments: ChatMessageValidationStatus = ChatMessageValidationStatus(7i32);
    pub const InvalidRecipients: ChatMessageValidationStatus = ChatMessageValidationStatus(8i32);
    pub const InvalidBody: ChatMessageValidationStatus = ChatMessageValidationStatus(9i32);
    pub const InvalidOther: ChatMessageValidationStatus = ChatMessageValidationStatus(10i32);
    pub const ValidWithLargeMessage: ChatMessageValidationStatus = ChatMessageValidationStatus(11i32);
    pub const VoiceRoamingRestriction: ChatMessageValidationStatus = ChatMessageValidationStatus(12i32);
    pub const DataRoamingRestriction: ChatMessageValidationStatus = ChatMessageValidationStatus(13i32);
}
impl ::std::convert::From<i32> for ChatMessageValidationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatMessageValidationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatMessageValidationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatMessageValidationStatus;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatQueryOptions(::windows::runtime::IInspectable);
impl ChatQueryOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatQueryOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SearchString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetSearchString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatQueryOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatQueryOptions;{2fd364a6-bf36-42f7-b7e7-923c0aabfe16})");
}
unsafe impl ::windows::runtime::Interface for ChatQueryOptions {
    type Vtable = IChatQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(802383014, 48950, 17143, [183, 231, 146, 60, 10, 171, 254, 22]);
}
impl ::windows::runtime::RuntimeName for ChatQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatQueryOptions";
}
unsafe impl ::std::marker::Send for ChatQueryOptions {}
unsafe impl ::std::marker::Sync for ChatQueryOptions {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatRecipientDeliveryInfo(::windows::runtime::IInspectable);
impl ChatRecipientDeliveryInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ChatRecipientDeliveryInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetTransportAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn DeliveryTime(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SetDeliveryTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn ReadTime(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SetReadTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportErrorCodeCategory(&self) -> ::windows::runtime::Result<ChatTransportErrorCodeCategory> {
        let this = self;
        unsafe {
            let mut result__: ChatTransportErrorCodeCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatTransportErrorCodeCategory>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportInterpretedErrorCode(&self) -> ::windows::runtime::Result<ChatTransportInterpretedErrorCode> {
        let this = self;
        unsafe {
            let mut result__: ChatTransportInterpretedErrorCode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatTransportInterpretedErrorCode>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportErrorCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsErrorPermanent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ChatMessageStatus> {
        let this = self;
        unsafe {
            let mut result__: ChatMessageStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatMessageStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatRecipientDeliveryInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo;{ffc7b2a2-283c-4c0a-8a0e-8c33bdbf0545})");
}
unsafe impl ::windows::runtime::Interface for ChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4291277474, 10300, 19466, [138, 14, 140, 51, 189, 191, 5, 69]);
}
impl ::windows::runtime::RuntimeName for ChatRecipientDeliveryInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
}
unsafe impl ::std::marker::Send for ChatRecipientDeliveryInfo {}
unsafe impl ::std::marker::Sync for ChatRecipientDeliveryInfo {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatRestoreHistorySpan(pub i32);
impl ChatRestoreHistorySpan {
    pub const LastMonth: ChatRestoreHistorySpan = ChatRestoreHistorySpan(0i32);
    pub const LastYear: ChatRestoreHistorySpan = ChatRestoreHistorySpan(1i32);
    pub const AnyTime: ChatRestoreHistorySpan = ChatRestoreHistorySpan(2i32);
}
impl ::std::convert::From<i32> for ChatRestoreHistorySpan {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatRestoreHistorySpan {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatRestoreHistorySpan {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatRestoreHistorySpan;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatSearchReader(::windows::runtime::IInspectable);
impl ChatSearchReader {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchWithCountAsync(&self, count: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IChatItem>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatSearchReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSearchReader;{4665fe49-9020-4752-980d-39612325f589})");
}
unsafe impl ::windows::runtime::Interface for ChatSearchReader {
    type Vtable = IChatSearchReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1181089353, 36896, 18258, [152, 13, 57, 97, 35, 37, 245, 137]);
}
impl ::windows::runtime::RuntimeName for ChatSearchReader {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSearchReader";
}
unsafe impl ::std::marker::Send for ChatSearchReader {}
unsafe impl ::std::marker::Sync for ChatSearchReader {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatStoreChangedEventKind(pub i32);
impl ChatStoreChangedEventKind {
    pub const NotificationsMissed: ChatStoreChangedEventKind = ChatStoreChangedEventKind(0i32);
    pub const StoreModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(1i32);
    pub const MessageCreated: ChatStoreChangedEventKind = ChatStoreChangedEventKind(2i32);
    pub const MessageModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(3i32);
    pub const MessageDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(4i32);
    pub const ConversationModified: ChatStoreChangedEventKind = ChatStoreChangedEventKind(5i32);
    pub const ConversationDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(6i32);
    pub const ConversationTransportDeleted: ChatStoreChangedEventKind = ChatStoreChangedEventKind(7i32);
}
impl ::std::convert::From<i32> for ChatStoreChangedEventKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatStoreChangedEventKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatStoreChangedEventKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatStoreChangedEventKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatSyncConfiguration(::windows::runtime::IInspectable);
impl ChatSyncConfiguration {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsSyncEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetIsSyncEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn RestoreHistorySpan(&self) -> ::windows::runtime::Result<ChatRestoreHistorySpan> {
        let this = self;
        unsafe {
            let mut result__: ChatRestoreHistorySpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatRestoreHistorySpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn SetRestoreHistorySpan(&self, value: ChatRestoreHistorySpan) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatSyncConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncConfiguration;{09f869b2-69f4-4aff-82b6-06992ff402d2})");
}
unsafe impl ::windows::runtime::Interface for ChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(167274930, 27124, 19199, [130, 182, 6, 153, 47, 244, 2, 210]);
}
impl ::windows::runtime::RuntimeName for ChatSyncConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncConfiguration";
}
unsafe impl ::std::marker::Send for ChatSyncConfiguration {}
unsafe impl ::std::marker::Sync for ChatSyncConfiguration {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ChatSyncManager(::windows::runtime::IInspectable);
impl ChatSyncManager {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<ChatSyncConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatSyncConfiguration>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Security_Credentials`*"]
    pub fn AssociateAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(&self, webaccount: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn UnassociateAccountAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Security_Credentials`*"]
    pub fn IsAccountAssociated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccount>>(&self, webaccount: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), webaccount.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn StartSync(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SetConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatSyncConfiguration>>(&self, configuration: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ChatSyncManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.ChatSyncManager;{7ba52c63-2650-486f-b4b4-6bd9d3d63c84})");
}
unsafe impl ::windows::runtime::Interface for ChatSyncManager {
    type Vtable = IChatSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2074422371, 9808, 18543, [180, 180, 107, 217, 211, 214, 60, 132]);
}
impl ::windows::runtime::RuntimeName for ChatSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.ChatSyncManager";
}
unsafe impl ::std::marker::Send for ChatSyncManager {}
unsafe impl ::std::marker::Sync for ChatSyncManager {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatTransportErrorCodeCategory(pub i32);
impl ChatTransportErrorCodeCategory {
    pub const None: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(0i32);
    pub const Http: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(1i32);
    pub const Network: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(2i32);
    pub const MmsServer: ChatTransportErrorCodeCategory = ChatTransportErrorCodeCategory(3i32);
}
impl ::std::convert::From<i32> for ChatTransportErrorCodeCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatTransportErrorCodeCategory {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatTransportErrorCodeCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ChatTransportInterpretedErrorCode(pub i32);
impl ChatTransportInterpretedErrorCode {
    pub const None: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(0i32);
    pub const Unknown: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(1i32);
    pub const InvalidRecipientAddress: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(2i32);
    pub const NetworkConnectivity: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(3i32);
    pub const ServiceDenied: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(4i32);
    pub const Timeout: ChatTransportInterpretedErrorCode = ChatTransportInterpretedErrorCode(5i32);
}
impl ::std::convert::From<i32> for ChatTransportInterpretedErrorCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ChatTransportInterpretedErrorCode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ChatTransportInterpretedErrorCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatCapabilities {
    type Vtable = IChatCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(989820860, 14793, 19921, [173, 45, 57, 100, 221, 157, 64, 63]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatCapabilitiesManagerStatics {
    type Vtable = IChatCapabilitiesManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3044683568, 28737, 17806, [176, 207, 124, 13, 159, 234, 51, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatCapabilitiesManagerStatics2 {
    type Vtable = IChatCapabilitiesManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3809297012, 54721, 19145, [159, 252, 64, 230, 145, 132, 254, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatCapabilitiesManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, address: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatConversation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatConversation {
    type Vtable = IChatConversation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2777417741, 6767, 18140, [143, 61, 245, 2, 134, 96, 182, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, participantaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, iscomposing: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, participantaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, iscomposing: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatConversation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatConversation2 {
    type Vtable = IChatConversation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(167972049, 38970, 18346, [154, 144, 238, 72, 238, 153, 123, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversation2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatConversationReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatConversationReader {
    type Vtable = IChatConversationReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(89208530, 56882, 19015, [169, 58, 179, 220, 8, 51, 133, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatConversationThreadingInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatConversationThreadingInfo {
    type Vtable = IChatConversationThreadingInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(857481692, 31239, 17442, [163, 44, 36, 190, 124, 109, 171, 36]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatConversationThreadingInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatConversationThreadingKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ChatConversationThreadingKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `ApplicationModel_Chat`*"]
pub struct IChatItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatItem {
    type Vtable = IChatItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2270285824, 52913, 16963, [184, 3, 21, 212, 90, 29, 212, 40]);
}
impl IChatItem {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ItemKind(&self) -> ::windows::runtime::Result<ChatItemKind> {
        let this = self;
        unsafe {
            let mut result__: ChatItemKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ChatItemKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IChatItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{8751d000-ceb1-4243-b803-15d45a1dd428}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatItemKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessage {
    type Vtable = IChatMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1262028074, 4418, 20617, [118, 218, 242, 219, 61, 23, 205, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessage2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessage2 {
    type Vtable = IChatMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2254865202, 21567, 18933, [172, 113, 108, 42, 252, 101, 101, 253]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ChatMessageKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageOperatorKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ChatMessageOperatorKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ChatMessageStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessage3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessage3 {
    type Vtable = IChatMessage3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1961570224, 15271, 17823, [142, 11, 232, 175, 15, 235, 217, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessage4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessage4 {
    type Vtable = IChatMessage4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(756304655, 53951, 17932, [170, 104, 109, 63, 132, 131, 201, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessage4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageAttachment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageAttachment {
    type Vtable = IChatMessageAttachment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3351575924, 48995, 22763, [80, 140, 139, 134, 63, 241, 107, 103]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageAttachment2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageAttachment2 {
    type Vtable = IChatMessageAttachment2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1591317104, 32209, 19079, [168, 206, 172, 221, 135, 216, 13, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachment2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageAttachmentFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageAttachmentFactory {
    type Vtable = IChatMessageAttachmentFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(542659234, 41814, 23409, [108, 169, 102, 201, 133, 183, 208, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageAttachmentFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mimetype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, datastreamreference: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageBlockingStatic(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageBlockingStatic {
    type Vtable = IChatMessageBlockingStatic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4139361152, 52714, 4580, [136, 48, 8, 0, 32, 12, 154, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageBlockingStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, blocked: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageChange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageChange {
    type Vtable = IChatMessageChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(471384917, 16926, 21688, [109, 56, 107, 58, 108, 130, 252, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageChangeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageChangeReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageChangeReader {
    type Vtable = IChatMessageChangeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(338063392, 10446, 24358, [123, 5, 154, 92, 124, 206, 135, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastchangetoacknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageChangeTracker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageChangeTracker {
    type Vtable = IChatMessageChangeTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1622667366, 28832, 21028, [80, 140, 36, 46, 247, 193, 208, 111]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangeTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageChangedDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageChangedDeferral {
    type Vtable = IChatMessageChangedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4224103180, 30860, 19916, [172, 231, 98, 130, 56, 41, 104, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageChangedEventArgs {
    type Vtable = IChatMessageChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3065462317, 26908, 20191, [134, 96, 110, 185, 137, 104, 146, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageManager2Statics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageManager2Statics {
    type Vtable = IChatMessageManager2Statics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(491075855, 40783, 20021, [150, 78, 27, 156, 166, 26, 192, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManager2Statics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageManagerStatic(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageManagerStatic {
    type Vtable = IChatMessageManagerStatic_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4049363191, 54760, 24210, [85, 109, 224, 59, 96, 37, 49, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageManagerStatics3 {
    type Vtable = IChatMessageManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(546013965, 26453, 18636, [154, 179, 253, 3, 196, 99, 252, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageNotificationTriggerDetails {
    type Vtable = IChatMessageNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4248063483, 12387, 19991, [133, 134, 198, 192, 130, 98, 230, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageNotificationTriggerDetails2 {
    type Vtable = IChatMessageNotificationTriggerDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1807033056, 43527, 20433, [148, 113, 119, 147, 79, 183, 94, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageNotificationTriggerDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageReader {
    type Vtable = IChatMessageReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3068819662, 17545, 22265, [118, 170, 226, 4, 104, 37, 20, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageReader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageReader2 {
    type Vtable = IChatMessageReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2305046147, 25787, 18189, [157, 244, 13, 232, 190, 26, 5, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageStore {
    type Vtable = IChatMessageStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(838008065, 52470, 22539, [73, 118, 10, 7, 221, 93, 59, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recenttimelimit: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chatmessage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chatmessage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageStore2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageStore2 {
    type Vtable = IChatMessageStore2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2907555054, 15060, 18715, [179, 17, 171, 223, 155, 178, 39, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, addresses: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conversationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conversationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, transportids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadinginfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, remoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, chatmessage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localchatmessageid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageStore3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageStore3 {
    type Vtable = IChatMessageStore3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2598091529, 17221, 20161, [139, 116, 183, 51, 130, 67, 113, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStore3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, syncid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageStoreChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageStoreChangedEventArgs {
    type Vtable = IChatMessageStoreChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1707503532, 65164, 18132, [145, 25, 87, 184, 65, 3, 17, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageStoreChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatStoreChangedEventKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageTransport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageTransport {
    type Vtable = IChatMessageTransport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1672076280, 59059, 23706, [95, 133, 212, 121, 37, 185, 189, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageTransport2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageTransport2 {
    type Vtable = IChatMessageTransport2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426885666, 55370, 19490, [169, 77, 84, 68, 68, 237, 200, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransport2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageTransportKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageTransportConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageTransportConfiguration {
    type Vtable = IChatMessageTransportConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2275407653, 6664, 19146, [160, 117, 51, 85, 18, 99, 18, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageTransportConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_MediaProperties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatMessageValidationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatMessageValidationResult {
    type Vtable = IChatMessageValidationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(636041731, 10476, 22665, [86, 155, 126, 72, 107, 18, 111, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatMessageValidationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageValidationStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatQueryOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatQueryOptions {
    type Vtable = IChatQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(802383014, 48950, 17143, [183, 231, 146, 60, 10, 171, 254, 22]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatRecipientDeliveryInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatRecipientDeliveryInfo {
    type Vtable = IChatRecipientDeliveryInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4291277474, 10300, 19466, [138, 14, 140, 51, 189, 191, 5, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatRecipientDeliveryInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatTransportErrorCodeCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatTransportInterpretedErrorCode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatMessageStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatSearchReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatSearchReader {
    type Vtable = IChatSearchReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1181089353, 36896, 18258, [152, 13, 57, 97, 35, 37, 245, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSearchReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatSyncConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatSyncConfiguration {
    type Vtable = IChatSyncConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(167274930, 27124, 19199, [130, 182, 6, 153, 47, 244, 2, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ChatRestoreHistorySpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ChatRestoreHistorySpan) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IChatSyncManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IChatSyncManager {
    type Vtable = IChatSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2074422371, 9808, 18543, [180, 180, 107, 217, 211, 214, 60, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChatSyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, configuration: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsEndUserMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3620578795, 52183, 20283, [133, 38, 181, 6, 222, 195, 92, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: ::windows::runtime::RawPtr, pin: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453112631, 39746, 18131, [157, 94, 60, 27, 45, 174, 124, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(759541249, 16265, 16874, [151, 2, 158, 158, 212, 17, 170, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536652333, 13599, 18066, [180, 30, 27, 3, 93, 193, 137, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageAvailableTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsEndUserMessageManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810856026, 19743, 19289, [148, 51, 18, 108, 115, 78, 134, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsEndUserMessageManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsManagerStatics {
    type Vtable = IRcsManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2099710661, 2749, 20273, [155, 153, 165, 158, 113, 167, 183, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transportid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, conversation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsManagerStatics2 {
    type Vtable = IRcsManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3444157720, 44426, 17066, [142, 235, 167, 152, 168, 128, 137, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsServiceKindSupportedChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4101939780, 59267, 18534, [179, 167, 78, 92, 207, 2, 48, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsServiceKindSupportedChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RcsServiceKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsTransport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsTransport {
    type Vtable = IRcsTransport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4272113497, 62332, 17177, [133, 70, 236, 132, 210, 29, 48, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, servicekind: RcsServiceKind, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRcsTransportConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(533508354, 9330, 19385, [153, 136, 193, 33, 28, 131, 232, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRcsTransportConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRemoteParticipantComposingChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(515917223, 53193, 17865, [152, 118, 68, 159, 43, 193, 128, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteParticipantComposingChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsEndUserMessage(::windows::runtime::IInspectable);
impl RcsEndUserMessage {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsPinRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn Actions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<RcsEndUserMessageAction>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SendResponseAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RcsEndUserMessageAction>>(&self, action: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), action.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn SendResponseWithPinAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RcsEndUserMessageAction>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, action: Param0, pin: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), action.into_param().abi(), pin.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessage;{d7cda5eb-cbd7-4f3b-8526-b506dec35c53})");
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessage {
    type Vtable = IRcsEndUserMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3620578795, 52183, 20283, [133, 38, 181, 6, 222, 195, 92, 83]);
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessage";
}
unsafe impl ::std::marker::Send for RcsEndUserMessage {}
unsafe impl ::std::marker::Sync for RcsEndUserMessage {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsEndUserMessageAction(::windows::runtime::IInspectable);
impl RcsEndUserMessageAction {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessageAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAction;{92378737-9b42-46d3-9d5e-3c1b2dae7cb8})");
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessageAction {
    type Vtable = IRcsEndUserMessageAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2453112631, 39746, 18131, [157, 94, 60, 27, 45, 174, 124, 184]);
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessageAction {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
}
unsafe impl ::std::marker::Send for RcsEndUserMessageAction {}
unsafe impl ::std::marker::Sync for RcsEndUserMessageAction {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsEndUserMessageAvailableEventArgs(::windows::runtime::IInspectable);
impl RcsEndUserMessageAvailableEventArgs {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsMessageAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<RcsEndUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RcsEndUserMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessageAvailableEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs;{2d45ae01-3f89-41ea-9702-9e9ed411aa98})");
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessageAvailableEventArgs {
    type Vtable = IRcsEndUserMessageAvailableEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(759541249, 16265, 16874, [151, 2, 158, 158, 212, 17, 170, 152]);
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessageAvailableEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
}
unsafe impl ::std::marker::Send for RcsEndUserMessageAvailableEventArgs {}
unsafe impl ::std::marker::Sync for RcsEndUserMessageAvailableEventArgs {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsEndUserMessageAvailableTriggerDetails(::windows::runtime::IInspectable);
impl RcsEndUserMessageAvailableTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessageAvailableTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails;{5b97742d-351f-4692-b41e-1b035dc18986})");
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessageAvailableTriggerDetails {
    type Vtable = IRcsEndUserMessageAvailableTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536652333, 13599, 18066, [180, 30, 27, 3, 93, 193, 137, 134]);
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessageAvailableTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
}
unsafe impl ::std::marker::Send for RcsEndUserMessageAvailableTriggerDetails {}
unsafe impl ::std::marker::Sync for RcsEndUserMessageAvailableTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsEndUserMessageManager(::windows::runtime::IInspectable);
impl RcsEndUserMessageManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn MessageAvailableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RcsEndUserMessageManager, RcsEndUserMessageAvailableEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveMessageAvailableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsEndUserMessageManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsEndUserMessageManager;{3054ae5a-4d1f-4b59-9433-126c734e86a6})");
}
unsafe impl ::windows::runtime::Interface for RcsEndUserMessageManager {
    type Vtable = IRcsEndUserMessageManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810856026, 19743, 19289, [148, 51, 18, 108, 115, 78, 134, 166]);
}
impl ::windows::runtime::RuntimeName for RcsEndUserMessageManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
}
unsafe impl ::std::marker::Send for RcsEndUserMessageManager {}
unsafe impl ::std::marker::Sync for RcsEndUserMessageManager {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
pub struct RcsManager {}
impl RcsManager {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn GetEndUserMessageManager() -> ::windows::runtime::Result<RcsEndUserMessageManager> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RcsEndUserMessageManager>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetTransportsAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<RcsTransport>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn GetTransportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(transportid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RcsTransport>> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), transportid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<RcsTransport>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn LeaveConversationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ChatConversation>>(conversation: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IRcsManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), conversation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn TransportListChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IRcsManagerStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveTransportListChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IRcsManagerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    pub fn IRcsManagerStatics<R, F: FnOnce(&IRcsManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RcsManager, IRcsManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRcsManagerStatics2<R, F: FnOnce(&IRcsManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RcsManager, IRcsManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RcsManager {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsManager";
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RcsServiceKind(pub i32);
impl RcsServiceKind {
    pub const Chat: RcsServiceKind = RcsServiceKind(0i32);
    pub const GroupChat: RcsServiceKind = RcsServiceKind(1i32);
    pub const FileTransfer: RcsServiceKind = RcsServiceKind(2i32);
    pub const Capability: RcsServiceKind = RcsServiceKind(3i32);
}
impl ::std::convert::From<i32> for RcsServiceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RcsServiceKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RcsServiceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Chat.RcsServiceKind;i4)");
}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsServiceKindSupportedChangedEventArgs(::windows::runtime::IInspectable);
impl RcsServiceKindSupportedChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ServiceKind(&self) -> ::windows::runtime::Result<RcsServiceKind> {
        let this = self;
        unsafe {
            let mut result__: RcsServiceKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RcsServiceKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsServiceKindSupportedChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs;{f47ea244-e783-4866-b3a7-4e5ccf023070})");
}
unsafe impl ::windows::runtime::Interface for RcsServiceKindSupportedChangedEventArgs {
    type Vtable = IRcsServiceKindSupportedChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4101939780, 59267, 18534, [179, 167, 78, 92, 207, 2, 48, 112]);
}
impl ::windows::runtime::RuntimeName for RcsServiceKindSupportedChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
}
unsafe impl ::std::marker::Send for RcsServiceKindSupportedChangedEventArgs {}
unsafe impl ::std::marker::Sync for RcsServiceKindSupportedChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsTransport(::windows::runtime::IInspectable);
impl RcsTransport {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation_Collections`*"]
    pub fn ExtendedProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportFriendlyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<RcsTransportConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RcsTransportConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsStoreAndForwardEnabled(&self, servicekind: RcsServiceKind) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), servicekind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsServiceKindSupported(&self, servicekind: RcsServiceKind) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), servicekind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn ServiceKindSupportedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<RcsTransport, RcsServiceKindSupportedChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Chat`, `Foundation`*"]
    pub fn RemoveServiceKindSupportedChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsTransport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransport;{fea34759-f37c-4319-8546-ec84d21d30ff})");
}
unsafe impl ::windows::runtime::Interface for RcsTransport {
    type Vtable = IRcsTransport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4272113497, 62332, 17177, [133, 70, 236, 132, 210, 29, 48, 255]);
}
impl ::windows::runtime::RuntimeName for RcsTransport {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransport";
}
unsafe impl ::std::marker::Send for RcsTransport {}
unsafe impl ::std::marker::Sync for RcsTransport {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RcsTransportConfiguration(::windows::runtime::IInspectable);
impl RcsTransportConfiguration {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxAttachmentCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxMessageSizeInKilobytes(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxGroupMessageSizeInKilobytes(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxRecipientCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn MaxFileSizeInKilobytes(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn WarningFileSizeInKilobytes(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RcsTransportConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RcsTransportConfiguration;{1fccb102-2472-4bb9-9988-c1211c83e8a9})");
}
unsafe impl ::windows::runtime::Interface for RcsTransportConfiguration {
    type Vtable = IRcsTransportConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(533508354, 9330, 19385, [153, 136, 193, 33, 28, 131, 232, 169]);
}
impl ::windows::runtime::RuntimeName for RcsTransportConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RcsTransportConfiguration";
}
unsafe impl ::std::marker::Send for RcsTransportConfiguration {}
unsafe impl ::std::marker::Sync for RcsTransportConfiguration {}
#[doc = "*Required features: `ApplicationModel_Chat`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RemoteParticipantComposingChangedEventArgs(::windows::runtime::IInspectable);
impl RemoteParticipantComposingChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn TransportId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn ParticipantAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Chat`*"]
    pub fn IsComposing(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteParticipantComposingChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs;{1ec045a7-cfc9-45c9-9876-449f2bc180f5})");
}
unsafe impl ::windows::runtime::Interface for RemoteParticipantComposingChangedEventArgs {
    type Vtable = IRemoteParticipantComposingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(515917223, 53193, 17865, [152, 118, 68, 159, 43, 193, 128, 245]);
}
impl ::windows::runtime::RuntimeName for RemoteParticipantComposingChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
}
unsafe impl ::std::marker::Send for RemoteParticipantComposingChangedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteParticipantComposingChangedEventArgs {}
