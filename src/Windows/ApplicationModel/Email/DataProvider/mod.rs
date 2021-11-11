#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailDataProviderConnection(pub ::windows::core::IInspectable);
impl EmailDataProviderConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn MailboxSyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveMailboxSyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DownloadMessageRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDownloadMessageRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DownloadAttachmentRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDownloadAttachmentRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn CreateFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveCreateFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DeleteFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDeleteFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn EmptyFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveEmptyFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn MoveFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveMoveFolderRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn UpdateMeetingResponseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveUpdateMeetingResponseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ForwardMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveForwardMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ProposeNewTimeForMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveProposeNewTimeForMeetingRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn SetAutoReplySettingsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveSetAutoReplySettingsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetAutoReplySettingsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveGetAutoReplySettingsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ResolveRecipientsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveResolveRecipientsRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ValidateCertificatesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveValidateCertificatesRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection;{3b9c9dc7-37b2-4bf0-ae30-7b644a1c96e1})");
}
unsafe impl ::windows::core::Interface for EmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b9c9dc7_37b2_4bf0_ae30_7b644a1c96e1);
}
impl ::windows::core::RuntimeName for EmailDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
}
impl ::core::convert::From<EmailDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: EmailDataProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &EmailDataProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: EmailDataProviderConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &EmailDataProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailDataProviderConnection {}
unsafe impl ::core::marker::Sync for EmailDataProviderConnection {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailDataProviderTriggerDetails(pub ::windows::core::IInspectable);
impl EmailDataProviderTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Connection(&self) -> ::windows::core::Result<EmailDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailDataProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails;{8f3e4e50-341e-45f3-bba0-84a005e1319a})");
}
unsafe impl ::windows::core::Interface for EmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f3e4e50_341e_45f3_bba0_84a005e1319a);
}
impl ::windows::core::RuntimeName for EmailDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
}
impl ::core::convert::From<EmailDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: EmailDataProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &EmailDataProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: EmailDataProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &EmailDataProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for EmailDataProviderTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxCreateFolderRequest(pub ::windows::core::IInspectable);
impl EmailMailboxCreateFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EmailFolder>>(&self, folder: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxCreateFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest;{184d3775-c921-4c39-a309-e16c9f22b04b})");
}
unsafe impl ::windows::core::Interface for EmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x184d3775_c921_4c39_a309_e16c9f22b04b);
}
impl ::windows::core::RuntimeName for EmailMailboxCreateFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
}
impl ::core::convert::From<EmailMailboxCreateFolderRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxCreateFolderRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxCreateFolderRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxCreateFolderRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxCreateFolderRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxCreateFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxCreateFolderRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxCreateFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxCreateFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxCreateFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxCreateFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs;{03e4c02c-241c-4ea9-a68f-ff20bc5afc85})");
}
unsafe impl ::windows::core::Interface for EmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e4c02c_241c_4ea9_a68f_ff20bc5afc85);
}
impl ::windows::core::RuntimeName for EmailMailboxCreateFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxCreateFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxCreateFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxCreateFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxCreateFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxCreateFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDeleteFolderRequest(pub ::windows::core::IInspectable);
impl EmailMailboxDeleteFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDeleteFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest;{9469e88a-a931-4779-923d-09a3ea292e29})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9469e88a_a931_4779_923d_09a3ea292e29);
}
impl ::windows::core::RuntimeName for EmailMailboxDeleteFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
}
impl ::core::convert::From<EmailMailboxDeleteFolderRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDeleteFolderRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDeleteFolderRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDeleteFolderRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDeleteFolderRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDeleteFolderRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDeleteFolderRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDeleteFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDeleteFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDeleteFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxDeleteFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDeleteFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDeleteFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDeleteFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs;{b4d32d06-2332-4678-8378-28b579336846})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4d32d06_2332_4678_8378_28b579336846);
}
impl ::windows::core::RuntimeName for EmailMailboxDeleteFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxDeleteFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDeleteFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDeleteFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDeleteFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDeleteFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDeleteFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDownloadAttachmentRequest(pub ::windows::core::IInspectable);
impl EmailMailboxDownloadAttachmentRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailAttachmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadAttachmentRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest;{0b1dbbb4-750c-48e1-bce4-8d589684ffbc})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b1dbbb4_750c_48e1_bce4_8d589684ffbc);
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadAttachmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
}
impl ::core::convert::From<EmailMailboxDownloadAttachmentRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadAttachmentRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDownloadAttachmentRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadAttachmentRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDownloadAttachmentRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadAttachmentRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxDownloadAttachmentRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadAttachmentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDownloadAttachmentRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadAttachmentRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs;{ccddc46d-ffa8-4877-9f9d-fed7bcaf4104})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccddc46d_ffa8_4877_9f9d_fed7bcaf4104);
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadAttachmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDownloadAttachmentRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadAttachmentRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDownloadMessageRequest(pub ::windows::core::IInspectable);
impl EmailMailboxDownloadMessageRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadMessageRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest;{497b4187-5b4d-4b23-816c-f3842beb753e})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x497b4187_5b4d_4b23_816c_f3842beb753e);
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadMessageRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
}
impl ::core::convert::From<EmailMailboxDownloadMessageRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDownloadMessageRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadMessageRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDownloadMessageRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDownloadMessageRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDownloadMessageRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadMessageRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDownloadMessageRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDownloadMessageRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadMessageRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxDownloadMessageRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadMessageRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDownloadMessageRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxDownloadMessageRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs;{470409ad-d0a0-4a5b-bb2a-37621039c53e})");
}
unsafe impl ::windows::core::Interface for EmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470409ad_d0a0_4a5b_bb2a_37621039c53e);
}
impl ::windows::core::RuntimeName for EmailMailboxDownloadMessageRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxDownloadMessageRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadMessageRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxDownloadMessageRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxDownloadMessageRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxDownloadMessageRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxDownloadMessageRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxEmptyFolderRequest(pub ::windows::core::IInspectable);
impl EmailMailboxEmptyFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxEmptyFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest;{fe4b03ab-f86d-46d9-b4ce-bc8a6d9e9268})");
}
unsafe impl ::windows::core::Interface for EmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe4b03ab_f86d_46d9_b4ce_bc8a6d9e9268);
}
impl ::windows::core::RuntimeName for EmailMailboxEmptyFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
}
impl ::core::convert::From<EmailMailboxEmptyFolderRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxEmptyFolderRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxEmptyFolderRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxEmptyFolderRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxEmptyFolderRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxEmptyFolderRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxEmptyFolderRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxEmptyFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxEmptyFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxEmptyFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxEmptyFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxEmptyFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxEmptyFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxEmptyFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs;{7183f484-985a-4ac0-b33f-ee0e2627a3c0})");
}
unsafe impl ::windows::core::Interface for EmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7183f484_985a_4ac0_b33f_ee0e2627a3c0);
}
impl ::windows::core::RuntimeName for EmailMailboxEmptyFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxEmptyFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxEmptyFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxEmptyFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxEmptyFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxEmptyFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxEmptyFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxForwardMeetingRequest(pub ::windows::core::IInspectable);
impl EmailMailboxForwardMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`*"]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ForwardHeaderType(&self) -> ::windows::core::Result<super::EmailMessageBodyKind> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMessageBodyKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMessageBodyKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxForwardMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest;{616d6af1-70d4-4832-b869-b80542ae9be8})");
}
unsafe impl ::windows::core::Interface for EmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616d6af1_70d4_4832_b869_b80542ae9be8);
}
impl ::windows::core::RuntimeName for EmailMailboxForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
}
impl ::core::convert::From<EmailMailboxForwardMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxForwardMeetingRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxForwardMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxForwardMeetingRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxForwardMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxForwardMeetingRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxForwardMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxForwardMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxForwardMeetingRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxForwardMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxForwardMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxForwardMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxForwardMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs;{2bd8f33a-2974-4759-a5a5-58f44d3c0275})");
}
unsafe impl ::windows::core::Interface for EmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bd8f33a_2974_4759_a5a5_58f44d3c0275);
}
impl ::windows::core::RuntimeName for EmailMailboxForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxForwardMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxForwardMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxForwardMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxForwardMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxForwardMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxForwardMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxGetAutoReplySettingsRequest(pub ::windows::core::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn RequestedFormat(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMailboxAutoReplyMessageResponseKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMailboxAutoReplyMessageResponseKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EmailMailboxAutoReplySettings>>(&self, autoreplysettings: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), autoreplysettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxGetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest;{9b380789-1e88-4e01-84cc-1386ad9a2c2f})");
}
unsafe impl ::windows::core::Interface for EmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b380789_1e88_4e01_84cc_1386ad9a2c2f);
}
impl ::windows::core::RuntimeName for EmailMailboxGetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
}
impl ::core::convert::From<EmailMailboxGetAutoReplySettingsRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxGetAutoReplySettingsRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxGetAutoReplySettingsRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxGetAutoReplySettingsRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxGetAutoReplySettingsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxGetAutoReplySettingsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxGetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxGetAutoReplySettingsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs;{d79f55c2-fd45-4004-8a91-9bacf38b7022})");
}
unsafe impl ::windows::core::Interface for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd79f55c2_fd45_4004_8a91_9bacf38b7022);
}
impl ::windows::core::RuntimeName for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxMoveFolderRequest(pub ::windows::core::IInspectable);
impl EmailMailboxMoveFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn NewParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn NewFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxMoveFolderRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest;{10ba2856-4a95-4068-91cc-67cc7acf454f})");
}
unsafe impl ::windows::core::Interface for EmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ba2856_4a95_4068_91cc_67cc7acf454f);
}
impl ::windows::core::RuntimeName for EmailMailboxMoveFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
}
impl ::core::convert::From<EmailMailboxMoveFolderRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxMoveFolderRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxMoveFolderRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxMoveFolderRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxMoveFolderRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxMoveFolderRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxMoveFolderRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxMoveFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxMoveFolderRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxMoveFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxMoveFolderRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxMoveFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxMoveFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxMoveFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxMoveFolderRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs;{38623020-14ba-4c88-8698-7239e3c8aaa7})");
}
unsafe impl ::windows::core::Interface for EmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38623020_14ba_4c88_8698_7239e3c8aaa7);
}
impl ::windows::core::RuntimeName for EmailMailboxMoveFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxMoveFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxMoveFolderRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxMoveFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxMoveFolderRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxMoveFolderRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxMoveFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(pub ::windows::core::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest;{5aeff152-9799-4f9f-a399-ff07f3eef04e})");
}
unsafe impl ::windows::core::Interface for EmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5aeff152_9799_4f9f_a399_ff07f3eef04e);
}
impl ::windows::core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
}
impl ::core::convert::From<EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxProposeNewTimeForMeetingRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxProposeNewTimeForMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs;{fb480b98-33ad-4a67-8251-0f9c249b6a20})");
}
unsafe impl ::windows::core::Interface for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb480b98_33ad_4a67_8251_0f9c249b6a20);
}
impl ::windows::core::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxResolveRecipientsRequest(pub ::windows::core::IInspectable);
impl EmailMailboxResolveRecipientsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`*"]
    pub fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>>(&self, resolutionresults: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), resolutionresults.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxResolveRecipientsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest;{efa4cf70-7b39-4c9b-811e-41eaf43a332d})");
}
unsafe impl ::windows::core::Interface for EmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa4cf70_7b39_4c9b_811e_41eaf43a332d);
}
impl ::windows::core::RuntimeName for EmailMailboxResolveRecipientsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
}
impl ::core::convert::From<EmailMailboxResolveRecipientsRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxResolveRecipientsRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxResolveRecipientsRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxResolveRecipientsRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxResolveRecipientsRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxResolveRecipientsRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxResolveRecipientsRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxResolveRecipientsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxResolveRecipientsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxResolveRecipientsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxResolveRecipientsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxResolveRecipientsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxResolveRecipientsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxResolveRecipientsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs;{260f9e02-b2cf-40f8-8c28-e3ed43b1e89a})");
}
unsafe impl ::windows::core::Interface for EmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260f9e02_b2cf_40f8_8c28_e3ed43b1e89a);
}
impl ::windows::core::RuntimeName for EmailMailboxResolveRecipientsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxResolveRecipientsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxResolveRecipientsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxServerSearchReadBatchRequest(pub ::windows::core::IInspectable);
impl EmailMailboxServerSearchReadBatchRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Options(&self) -> ::windows::core::Result<super::EmailQueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailQueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn SaveMessageAsync<'a, Param0: ::windows::core::IntoParam<'a, super::EmailMessage>>(&self, message: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), batchstatus, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest;{090eebdf-5a96-41d3-8ad8-34912f9aa60e})");
}
unsafe impl ::windows::core::Interface for EmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x090eebdf_5a96_41d3_8ad8_34912f9aa60e);
}
impl ::windows::core::RuntimeName for EmailMailboxServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
}
impl ::core::convert::From<EmailMailboxServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxServerSearchReadBatchRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxServerSearchReadBatchRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxServerSearchReadBatchRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxServerSearchReadBatchRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs;{14101b4e-ed9e-45d1-ad7a-cc9b7f643ae2})");
}
unsafe impl ::windows::core::Interface for EmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14101b4e_ed9e_45d1_ad7a_cc9b7f643ae2);
}
impl ::windows::core::RuntimeName for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxServerSearchReadBatchRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxSetAutoReplySettingsRequest(pub ::windows::core::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn AutoReplySettings(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMailboxAutoReplySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest;{75a422d0-a88e-4e54-8dc7-c243186b774e})");
}
unsafe impl ::windows::core::Interface for EmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a422d0_a88e_4e54_8dc7_c243186b774e);
}
impl ::windows::core::RuntimeName for EmailMailboxSetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
}
impl ::core::convert::From<EmailMailboxSetAutoReplySettingsRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxSetAutoReplySettingsRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxSetAutoReplySettingsRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxSetAutoReplySettingsRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxSetAutoReplySettingsRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxSetAutoReplySettingsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxSetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxSetAutoReplySettingsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs;{09da11ad-d7ca-4087-ac86-53fa67f76246})");
}
unsafe impl ::windows::core::Interface for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09da11ad_d7ca_4087_ac86_53fa67f76246);
}
impl ::windows::core::RuntimeName for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxSyncManagerSyncRequest(pub ::windows::core::IInspectable);
impl EmailMailboxSyncManagerSyncRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest;{4e10e8e4-7e67-405a-b673-dc60c91090fc})");
}
unsafe impl ::windows::core::Interface for EmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e10e8e4_7e67_405a_b673_dc60c91090fc);
}
impl ::windows::core::RuntimeName for EmailMailboxSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
}
impl ::core::convert::From<EmailMailboxSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManagerSyncRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs;{439a031a-8fcc-4ae5-b9b5-d434e0a65aa8})");
}
unsafe impl ::windows::core::Interface for EmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439a031a_8fcc_4ae5_b9b5_d434e0a65aa8);
}
impl ::windows::core::RuntimeName for EmailMailboxSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxSyncManagerSyncRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxUpdateMeetingResponseRequest(pub ::windows::core::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Response(&self) -> ::windows::core::Result<super::EmailMeetingResponseType> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMeetingResponseType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMeetingResponseType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SendUpdate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxUpdateMeetingResponseRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest;{5b99ac93-b2cf-4888-ba4f-306b6b66df30})");
}
unsafe impl ::windows::core::Interface for EmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b99ac93_b2cf_4888_ba4f_306b6b66df30);
}
impl ::windows::core::RuntimeName for EmailMailboxUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
}
impl ::core::convert::From<EmailMailboxUpdateMeetingResponseRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxUpdateMeetingResponseRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxUpdateMeetingResponseRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxUpdateMeetingResponseRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxUpdateMeetingResponseRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxUpdateMeetingResponseRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxUpdateMeetingResponseRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs;{6898d761-56c9-4f17-be31-66fda94ba159})");
}
unsafe impl ::windows::core::Interface for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6898d761_56c9_4f17_be31_66fda94ba159);
}
impl ::windows::core::RuntimeName for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxValidateCertificatesRequest(pub ::windows::core::IInspectable);
impl EmailMailboxValidateCertificatesRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`, `Security_Cryptography_Certificates`*"]
    pub fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>>(&self, validationstatuses: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), validationstatuses.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxValidateCertificatesRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest;{a94d3931-e11a-4f97-b81a-187a70a8f41a})");
}
unsafe impl ::windows::core::Interface for EmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94d3931_e11a_4f97_b81a_187a70a8f41a);
}
impl ::windows::core::RuntimeName for EmailMailboxValidateCertificatesRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
}
impl ::core::convert::From<EmailMailboxValidateCertificatesRequest> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxValidateCertificatesRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxValidateCertificatesRequest> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxValidateCertificatesRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxValidateCertificatesRequest> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxValidateCertificatesRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxValidateCertificatesRequest> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxValidateCertificatesRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxValidateCertificatesRequest {}
unsafe impl ::core::marker::Sync for EmailMailboxValidateCertificatesRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(pub ::windows::core::IInspectable);
impl EmailMailboxValidateCertificatesRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::core::Result<EmailMailboxValidateCertificatesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxValidateCertificatesRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for EmailMailboxValidateCertificatesRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs;{2583bf17-02ff-49fe-a73c-03f37566c691})");
}
unsafe impl ::windows::core::Interface for EmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2583bf17_02ff_49fe_a73c_03f37566c691);
}
impl ::windows::core::RuntimeName for EmailMailboxValidateCertificatesRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
}
impl ::core::convert::From<EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EmailMailboxValidateCertificatesRequestEventArgs {}
unsafe impl ::core::marker::Sync for EmailMailboxValidateCertificatesRequestEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailDataProviderConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b9c9dc7_37b2_4bf0_ae30_7b644a1c96e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailDataProviderTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f3e4e50_341e_45f3_bba0_84a005e1319a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x184d3775_c921_4c39_a309_e16c9f22b04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: super::EmailMailboxCreateFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e4c02c_241c_4ea9_a68f_ff20bc5afc85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9469e88a_a931_4779_923d_09a3ea292e29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: super::EmailMailboxDeleteFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4d32d06_2332_4678_8378_28b579336846);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b1dbbb4_750c_48e1_bce4_8d589684ffbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccddc46d_ffa8_4877_9f9d_fed7bcaf4104);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x497b4187_5b4d_4b23_816c_f3842beb753e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x470409ad_d0a0_4a5b_bb2a_37621039c53e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe4b03ab_f86d_46d9_b4ce_bc8a6d9e9268);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: super::EmailMailboxEmptyFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7183f484_985a_4ac0_b33f_ee0e2627a3c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x616d6af1_70d4_4832_b869_b80542ae9be8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::EmailMessageBodyKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bd8f33a_2974_4759_a5a5_58f44d3c0275);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b380789_1e88_4e01_84cc_1386ad9a2c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, autoreplysettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd79f55c2_fd45_4004_8a91_9bacf38b7022);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10ba2856_4a95_4068_91cc_67cc7acf454f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x38623020_14ba_4c88_8698_7239e3c8aaa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5aeff152_9799_4f9f_a399_ff07f3eef04e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb480b98_33ad_4a67_8251_0f9c249b6a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefa4cf70_7b39_4c9b_811e_41eaf43a332d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resolutionresults: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x260f9e02_b2cf_40f8_8c28_e3ed43b1e89a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x090eebdf_5a96_41d3_8ad8_34912f9aa60e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, batchstatus: super::EmailBatchStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14101b4e_ed9e_45d1_ad7a_cc9b7f643ae2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75a422d0_a88e_4e54_8dc7_c243186b774e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09da11ad_d7ca_4087_ac86_53fa67f76246);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e10e8e4_7e67_405a_b673_dc60c91090fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439a031a_8fcc_4ae5_b9b5_d434e0a65aa8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b99ac93_b2cf_4888_ba4f_306b6b66df30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::EmailMeetingResponseType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6898d761_56c9_4f17_be31_66fda94ba159);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa94d3931_e11a_4f97_b81a_187a70a8f41a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, validationstatuses: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2583bf17_02ff_49fe_a73c_03f37566c691);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
