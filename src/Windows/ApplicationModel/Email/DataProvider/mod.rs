#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailDataProviderConnection(::windows::runtime::IInspectable);
impl EmailDataProviderConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn MailboxSyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveMailboxSyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DownloadMessageRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDownloadMessageRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DownloadAttachmentRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDownloadAttachmentRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn CreateFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveCreateFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn DeleteFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveDeleteFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn EmptyFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveEmptyFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn MoveFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveMoveFolderRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn UpdateMeetingResponseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveUpdateMeetingResponseRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ForwardMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveForwardMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ProposeNewTimeForMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveProposeNewTimeForMeetingRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn SetAutoReplySettingsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveSetAutoReplySettingsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetAutoReplySettingsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveGetAutoReplySettingsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ResolveRecipientsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveResolveRecipientsRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ValidateCertificatesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveValidateCertificatesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ServerSearchReadBatchRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn RemoveServerSearchReadBatchRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailDataProviderConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection;{3b9c9dc7-37b2-4bf0-ae30-7b644a1c96e1})");
}
unsafe impl ::windows::runtime::Interface for EmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1000119751, 14258, 19440, [174, 48, 123, 100, 74, 28, 150, 225]);
}
impl ::windows::runtime::RuntimeName for EmailDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
}
impl ::std::convert::From<EmailDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: EmailDataProviderConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: &EmailDataProviderConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: EmailDataProviderConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: &EmailDataProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailDataProviderConnection {}
unsafe impl ::std::marker::Sync for EmailDataProviderConnection {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailDataProviderTriggerDetails(::windows::runtime::IInspectable);
impl EmailDataProviderTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<EmailDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailDataProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailDataProviderTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails;{8f3e4e50-341e-45f3-bba0-84a005e1319a})");
}
unsafe impl ::windows::runtime::Interface for EmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2403225168, 13342, 17907, [187, 160, 132, 160, 5, 225, 49, 154]);
}
impl ::windows::runtime::RuntimeName for EmailDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
}
impl ::std::convert::From<EmailDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: EmailDataProviderTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &EmailDataProviderTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: EmailDataProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &EmailDataProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailDataProviderTriggerDetails {}
unsafe impl ::std::marker::Sync for EmailDataProviderTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxCreateFolderRequest(::windows::runtime::IInspectable);
impl EmailMailboxCreateFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ParentFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::EmailFolder>>(&self, folder: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxCreateFolderRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest;{184d3775-c921-4c39-a309-e16c9f22b04b})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(407713653, 51489, 19513, [163, 9, 225, 108, 159, 34, 176, 75]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxCreateFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
}
impl ::std::convert::From<EmailMailboxCreateFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxCreateFolderRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxCreateFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxCreateFolderRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxCreateFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxCreateFolderRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxCreateFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxCreateFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxCreateFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxCreateFolderRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxCreateFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxCreateFolderRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxCreateFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxCreateFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxCreateFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxCreateFolderRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs;{03e4c02c-241c-4ea9-a68f-ff20bc5afc85})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65323052, 9244, 20137, [166, 143, 255, 32, 188, 90, 252, 133]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxCreateFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxCreateFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxCreateFolderRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxCreateFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxCreateFolderRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxCreateFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxCreateFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxCreateFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxCreateFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxCreateFolderRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxCreateFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDeleteFolderRequest(::windows::runtime::IInspectable);
impl EmailMailboxDeleteFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDeleteFolderRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest;{9469e88a-a931-4779-923d-09a3ea292e29})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2489968778, 43313, 18297, [146, 61, 9, 163, 234, 41, 46, 41]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDeleteFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
}
impl ::std::convert::From<EmailMailboxDeleteFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDeleteFolderRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDeleteFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDeleteFolderRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDeleteFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDeleteFolderRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDeleteFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDeleteFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDeleteFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDeleteFolderRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxDeleteFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDeleteFolderRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxDeleteFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxDeleteFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDeleteFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDeleteFolderRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs;{b4d32d06-2332-4678-8378-28b579336846})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3033738502, 9010, 18040, [131, 120, 40, 181, 121, 51, 104, 70]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDeleteFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxDeleteFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDeleteFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDeleteFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDeleteFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDeleteFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDeleteFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDeleteFolderRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxDeleteFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDownloadAttachmentRequest(::windows::runtime::IInspectable);
impl EmailMailboxDownloadAttachmentRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailAttachmentId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDownloadAttachmentRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest;{0b1dbbb4-750c-48e1-bce4-8d589684ffbc})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(186497972, 29964, 18657, [188, 228, 141, 88, 150, 132, 255, 188]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDownloadAttachmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
}
impl ::std::convert::From<EmailMailboxDownloadAttachmentRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDownloadAttachmentRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDownloadAttachmentRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDownloadAttachmentRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDownloadAttachmentRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDownloadAttachmentRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDownloadAttachmentRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDownloadAttachmentRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDownloadAttachmentRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxDownloadAttachmentRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDownloadAttachmentRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxDownloadAttachmentRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxDownloadAttachmentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDownloadAttachmentRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDownloadAttachmentRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs;{ccddc46d-ffa8-4877-9f9d-fed7bcaf4104})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437085805, 65448, 18551, [159, 157, 254, 215, 188, 175, 65, 4]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDownloadAttachmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDownloadAttachmentRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDownloadAttachmentRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDownloadAttachmentRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDownloadAttachmentRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxDownloadAttachmentRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDownloadMessageRequest(::windows::runtime::IInspectable);
impl EmailMailboxDownloadMessageRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDownloadMessageRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest;{497b4187-5b4d-4b23-816c-f3842beb753e})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1232814471, 23373, 19235, [129, 108, 243, 132, 43, 235, 117, 62]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDownloadMessageRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
}
impl ::std::convert::From<EmailMailboxDownloadMessageRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDownloadMessageRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDownloadMessageRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDownloadMessageRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDownloadMessageRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDownloadMessageRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDownloadMessageRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDownloadMessageRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDownloadMessageRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDownloadMessageRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxDownloadMessageRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxDownloadMessageRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxDownloadMessageRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxDownloadMessageRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxDownloadMessageRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxDownloadMessageRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs;{470409ad-d0a0-4a5b-bb2a-37621039c53e})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1191446957, 53408, 19035, [187, 42, 55, 98, 16, 57, 197, 62]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxDownloadMessageRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxDownloadMessageRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxDownloadMessageRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxDownloadMessageRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxDownloadMessageRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxDownloadMessageRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxDownloadMessageRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxDownloadMessageRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxDownloadMessageRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxEmptyFolderRequest(::windows::runtime::IInspectable);
impl EmailMailboxEmptyFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), status, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxEmptyFolderRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest;{fe4b03ab-f86d-46d9-b4ce-bc8a6d9e9268})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4266329003, 63597, 18137, [180, 206, 188, 138, 109, 158, 146, 104]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxEmptyFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
}
impl ::std::convert::From<EmailMailboxEmptyFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxEmptyFolderRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxEmptyFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxEmptyFolderRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxEmptyFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxEmptyFolderRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxEmptyFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxEmptyFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxEmptyFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxEmptyFolderRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxEmptyFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxEmptyFolderRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxEmptyFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxEmptyFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxEmptyFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxEmptyFolderRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs;{7183f484-985a-4ac0-b33f-ee0e2627a3c0})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904473220, 39002, 19136, [179, 63, 238, 14, 38, 39, 163, 192]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxEmptyFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxEmptyFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxEmptyFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxEmptyFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxEmptyFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxEmptyFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxEmptyFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxEmptyFolderRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxEmptyFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxForwardMeetingRequest(::windows::runtime::IInspectable);
impl EmailMailboxForwardMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`*"]
    pub fn Recipients(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ForwardHeaderType(&self) -> ::windows::runtime::Result<super::EmailMessageBodyKind> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMessageBodyKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMessageBodyKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn ForwardHeader(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxForwardMeetingRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest;{616d6af1-70d4-4832-b869-b80542ae9be8})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1634560753, 28884, 18482, [184, 105, 184, 5, 66, 174, 155, 232]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
}
impl ::std::convert::From<EmailMailboxForwardMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxForwardMeetingRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxForwardMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxForwardMeetingRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxForwardMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxForwardMeetingRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxForwardMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxForwardMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxForwardMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxForwardMeetingRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxForwardMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxForwardMeetingRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxForwardMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxForwardMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxForwardMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxForwardMeetingRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs;{2bd8f33a-2974-4759-a5a5-58f44d3c0275})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(735638330, 10612, 18265, [165, 165, 88, 244, 77, 60, 2, 117]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxForwardMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxForwardMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxForwardMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxForwardMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxForwardMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxForwardMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxForwardMeetingRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxForwardMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxGetAutoReplySettingsRequest(::windows::runtime::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn RequestedFormat(&self) -> ::windows::runtime::Result<super::EmailMailboxAutoReplyMessageResponseKind> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMailboxAutoReplyMessageResponseKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMailboxAutoReplyMessageResponseKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::EmailMailboxAutoReplySettings>>(&self, autoreplysettings: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), autoreplysettings.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxGetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest;{9b380789-1e88-4e01-84cc-1386ad9a2c2f})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604140425, 7816, 19969, [132, 204, 19, 134, 173, 154, 44, 47]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxGetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
}
impl ::std::convert::From<EmailMailboxGetAutoReplySettingsRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxGetAutoReplySettingsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxGetAutoReplySettingsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxGetAutoReplySettingsRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxGetAutoReplySettingsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxGetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxGetAutoReplySettingsRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxGetAutoReplySettingsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxGetAutoReplySettingsRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxGetAutoReplySettingsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxGetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxGetAutoReplySettingsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs;{d79f55c2-fd45-4004-8a91-9bacf38b7022})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3617543618, 64837, 16388, [138, 145, 155, 172, 243, 139, 112, 34]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxGetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxGetAutoReplySettingsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxGetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxGetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxMoveFolderRequest(::windows::runtime::IInspectable);
impl EmailMailboxMoveFolderRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn NewParentFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn NewFolderName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxMoveFolderRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest;{10ba2856-4a95-4068-91cc-67cc7acf454f})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(280635478, 19093, 16488, [145, 204, 103, 204, 122, 207, 69, 79]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxMoveFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
}
impl ::std::convert::From<EmailMailboxMoveFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxMoveFolderRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxMoveFolderRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxMoveFolderRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxMoveFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxMoveFolderRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxMoveFolderRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxMoveFolderRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxMoveFolderRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxMoveFolderRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxMoveFolderRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxMoveFolderRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxMoveFolderRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxMoveFolderRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxMoveFolderRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxMoveFolderRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs;{38623020-14ba-4c88-8698-7239e3c8aaa7})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945958944, 5306, 19592, [134, 152, 114, 57, 227, 200, 170, 167]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxMoveFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxMoveFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxMoveFolderRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxMoveFolderRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxMoveFolderRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxMoveFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxMoveFolderRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxMoveFolderRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxMoveFolderRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxMoveFolderRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxMoveFolderRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxProposeNewTimeForMeetingRequest(::windows::runtime::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn NewStartTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn NewDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest;{5aeff152-9799-4f9f-a399-ff07f3eef04e})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1525674322, 38809, 20383, [163, 153, 255, 7, 243, 238, 240, 78]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
}
impl ::std::convert::From<EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxProposeNewTimeForMeetingRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxProposeNewTimeForMeetingRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxProposeNewTimeForMeetingRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxProposeNewTimeForMeetingRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxProposeNewTimeForMeetingRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxProposeNewTimeForMeetingRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs;{fb480b98-33ad-4a67-8251-0f9c249b6a20})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4215802776, 13229, 19047, [130, 81, 15, 156, 36, 155, 106, 32]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxProposeNewTimeForMeetingRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxProposeNewTimeForMeetingRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxProposeNewTimeForMeetingRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxResolveRecipientsRequest(::windows::runtime::IInspectable);
impl EmailMailboxResolveRecipientsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`*"]
    pub fn Recipients(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>>(&self, resolutionresults: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), resolutionresults.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxResolveRecipientsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest;{efa4cf70-7b39-4c9b-811e-41eaf43a332d})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020555632, 31545, 19611, [129, 30, 65, 234, 244, 58, 51, 45]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxResolveRecipientsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
}
impl ::std::convert::From<EmailMailboxResolveRecipientsRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxResolveRecipientsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxResolveRecipientsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxResolveRecipientsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxResolveRecipientsRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxResolveRecipientsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxResolveRecipientsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxResolveRecipientsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxResolveRecipientsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxResolveRecipientsRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxResolveRecipientsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxResolveRecipientsRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxResolveRecipientsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxResolveRecipientsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxResolveRecipientsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxResolveRecipientsRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs;{260f9e02-b2cf-40f8-8c28-e3ed43b1e89a})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(638557698, 45775, 16632, [140, 40, 227, 237, 67, 177, 232, 154]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxResolveRecipientsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxResolveRecipientsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxResolveRecipientsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxResolveRecipientsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxResolveRecipientsRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxResolveRecipientsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxServerSearchReadBatchRequest(::windows::runtime::IInspectable);
impl EmailMailboxServerSearchReadBatchRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailFolderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<super::EmailQueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailQueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SuggestedBatchSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn SaveMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::EmailMessage>>(&self, message: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), batchstatus, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest;{090eebdf-5a96-41d3-8ad8-34912f9aa60e})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(151972831, 23190, 16851, [138, 216, 52, 145, 47, 154, 166, 14]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
}
impl ::std::convert::From<EmailMailboxServerSearchReadBatchRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxServerSearchReadBatchRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxServerSearchReadBatchRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxServerSearchReadBatchRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxServerSearchReadBatchRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxServerSearchReadBatchRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxServerSearchReadBatchRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxServerSearchReadBatchRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxServerSearchReadBatchRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxServerSearchReadBatchRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxServerSearchReadBatchRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxServerSearchReadBatchRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs;{14101b4e-ed9e-45d1-ad7a-cc9b7f643ae2})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(336599886, 60830, 17873, [173, 122, 204, 155, 127, 100, 58, 226]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxServerSearchReadBatchRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxServerSearchReadBatchRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxSetAutoReplySettingsRequest(::windows::runtime::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn AutoReplySettings(&self) -> ::windows::runtime::Result<super::EmailMailboxAutoReplySettings> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMailboxAutoReplySettings>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxSetAutoReplySettingsRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest;{75a422d0-a88e-4e54-8dc7-c243186b774e})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1973691088, 43150, 20052, [141, 199, 194, 67, 24, 107, 119, 78]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxSetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
}
impl ::std::convert::From<EmailMailboxSetAutoReplySettingsRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxSetAutoReplySettingsRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxSetAutoReplySettingsRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxSetAutoReplySettingsRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxSetAutoReplySettingsRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxSetAutoReplySettingsRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxSetAutoReplySettingsRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxSetAutoReplySettingsRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxSetAutoReplySettingsRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxSetAutoReplySettingsRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxSetAutoReplySettingsRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxSetAutoReplySettingsRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs;{09da11ad-d7ca-4087-ac86-53fa67f76246})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(165286317, 55242, 16519, [172, 134, 83, 250, 103, 247, 98, 70]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxSetAutoReplySettingsRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxSetAutoReplySettingsRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxSetAutoReplySettingsRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxSetAutoReplySettingsRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxSyncManagerSyncRequest(::windows::runtime::IInspectable);
impl EmailMailboxSyncManagerSyncRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxSyncManagerSyncRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest;{4e10e8e4-7e67-405a-b673-dc60c91090fc})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1309731044, 32359, 16474, [182, 115, 220, 96, 201, 16, 144, 252]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
}
impl ::std::convert::From<EmailMailboxSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxSyncManagerSyncRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxSyncManagerSyncRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxSyncManagerSyncRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxSyncManagerSyncRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxSyncManagerSyncRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxSyncManagerSyncRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs;{439a031a-8fcc-4ae5-b9b5-d434e0a65aa8})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1134166810, 36812, 19173, [185, 181, 212, 52, 224, 166, 90, 168]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxSyncManagerSyncRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxSyncManagerSyncRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxSyncManagerSyncRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxUpdateMeetingResponseRequest(::windows::runtime::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMessageId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Response(&self) -> ::windows::runtime::Result<super::EmailMeetingResponseType> {
        let this = self;
        unsafe {
            let mut result__: super::EmailMeetingResponseType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::EmailMeetingResponseType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Comment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn SendUpdate(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxUpdateMeetingResponseRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest;{5b99ac93-b2cf-4888-ba4f-306b6b66df30})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536797843, 45775, 18568, [186, 79, 48, 107, 107, 102, 223, 48]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
}
impl ::std::convert::From<EmailMailboxUpdateMeetingResponseRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxUpdateMeetingResponseRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxUpdateMeetingResponseRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxUpdateMeetingResponseRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxUpdateMeetingResponseRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxUpdateMeetingResponseRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxUpdateMeetingResponseRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxUpdateMeetingResponseRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxUpdateMeetingResponseRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxUpdateMeetingResponseRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxUpdateMeetingResponseRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxUpdateMeetingResponseRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs;{6898d761-56c9-4f17-be31-66fda94ba159})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1754847073, 22217, 20247, [190, 49, 102, 253, 169, 75, 161, 89]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxUpdateMeetingResponseRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxUpdateMeetingResponseRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxUpdateMeetingResponseRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxUpdateMeetingResponseRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxValidateCertificatesRequest(::windows::runtime::IInspectable);
impl EmailMailboxValidateCertificatesRequest {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn EmailMailboxId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation_Collections`, `Security_Cryptography_Certificates`*"]
    pub fn Certificates(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>>(&self, validationstatuses: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), validationstatuses.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxValidateCertificatesRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest;{a94d3931-e11a-4f97-b81a-187a70a8f41a})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2840410417, 57626, 20375, [184, 26, 24, 122, 112, 168, 244, 26]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxValidateCertificatesRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
}
impl ::std::convert::From<EmailMailboxValidateCertificatesRequest> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxValidateCertificatesRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxValidateCertificatesRequest> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxValidateCertificatesRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxValidateCertificatesRequest> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxValidateCertificatesRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxValidateCertificatesRequest> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxValidateCertificatesRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxValidateCertificatesRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxValidateCertificatesRequest {}
unsafe impl ::std::marker::Sync for EmailMailboxValidateCertificatesRequest {}
#[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EmailMailboxValidateCertificatesRequestEventArgs(::windows::runtime::IInspectable);
impl EmailMailboxValidateCertificatesRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<EmailMailboxValidateCertificatesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<EmailMailboxValidateCertificatesRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Email_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EmailMailboxValidateCertificatesRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs;{2583bf17-02ff-49fe-a73c-03f37566c691})");
}
unsafe impl ::windows::runtime::Interface for EmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(629391127, 767, 18942, [167, 60, 3, 243, 117, 102, 198, 145]);
}
impl ::windows::runtime::RuntimeName for EmailMailboxValidateCertificatesRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
}
impl ::std::convert::From<EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmailMailboxValidateCertificatesRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EmailMailboxValidateCertificatesRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EmailMailboxValidateCertificatesRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EmailMailboxValidateCertificatesRequestEventArgs {}
unsafe impl ::std::marker::Sync for EmailMailboxValidateCertificatesRequestEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailDataProviderConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailDataProviderConnection {
    type Vtable = IEmailDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1000119751, 14258, 19440, [174, 48, 123, 100, 74, 28, 150, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailDataProviderTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailDataProviderTriggerDetails {
    type Vtable = IEmailDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2403225168, 13342, 17907, [187, 160, 132, 160, 5, 225, 49, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailDataProviderTriggerDetails_abi(
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
pub struct IEmailMailboxCreateFolderRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxCreateFolderRequest {
    type Vtable = IEmailMailboxCreateFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(407713653, 51489, 19513, [163, 9, 225, 108, 159, 34, 176, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: super::EmailMailboxCreateFolderStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxCreateFolderRequestEventArgs {
    type Vtable = IEmailMailboxCreateFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65323052, 9244, 20137, [166, 143, 255, 32, 188, 90, 252, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxCreateFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDeleteFolderRequest {
    type Vtable = IEmailMailboxDeleteFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2489968778, 43313, 18297, [146, 61, 9, 163, 234, 41, 46, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: super::EmailMailboxDeleteFolderStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDeleteFolderRequestEventArgs {
    type Vtable = IEmailMailboxDeleteFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3033738502, 9010, 18040, [131, 120, 40, 181, 121, 51, 104, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDeleteFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDownloadAttachmentRequest {
    type Vtable = IEmailMailboxDownloadAttachmentRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(186497972, 29964, 18657, [188, 228, 141, 88, 150, 132, 255, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDownloadAttachmentRequestEventArgs {
    type Vtable = IEmailMailboxDownloadAttachmentRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437085805, 65448, 18551, [159, 157, 254, 215, 188, 175, 65, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadAttachmentRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDownloadMessageRequest {
    type Vtable = IEmailMailboxDownloadMessageRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1232814471, 23373, 19235, [129, 108, 243, 132, 43, 235, 117, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxDownloadMessageRequestEventArgs {
    type Vtable = IEmailMailboxDownloadMessageRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1191446957, 53408, 19035, [187, 42, 55, 98, 16, 57, 197, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxDownloadMessageRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxEmptyFolderRequest {
    type Vtable = IEmailMailboxEmptyFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4266329003, 63597, 18137, [180, 206, 188, 138, 109, 158, 146, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: super::EmailMailboxEmptyFolderStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxEmptyFolderRequestEventArgs {
    type Vtable = IEmailMailboxEmptyFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904473220, 39002, 19136, [179, 63, 238, 14, 38, 39, 163, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxEmptyFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxForwardMeetingRequest {
    type Vtable = IEmailMailboxForwardMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1634560753, 28884, 18482, [184, 105, 184, 5, 66, 174, 155, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::EmailMessageBodyKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxForwardMeetingRequestEventArgs {
    type Vtable = IEmailMailboxForwardMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(735638330, 10612, 18265, [165, 165, 88, 244, 77, 60, 2, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxForwardMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxGetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2604140425, 7816, 19969, [132, 204, 19, 134, 173, 154, 44, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::EmailMailboxAutoReplyMessageResponseKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoreplysettings: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3617543618, 64837, 16388, [138, 145, 155, 172, 243, 139, 112, 34]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxGetAutoReplySettingsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxMoveFolderRequest {
    type Vtable = IEmailMailboxMoveFolderRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(280635478, 19093, 16488, [145, 204, 103, 204, 122, 207, 69, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxMoveFolderRequestEventArgs {
    type Vtable = IEmailMailboxMoveFolderRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(945958944, 5306, 19592, [134, 152, 114, 57, 227, 200, 170, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxMoveFolderRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxProposeNewTimeForMeetingRequest {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1525674322, 38809, 20383, [163, 153, 255, 7, 243, 238, 240, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    type Vtable = IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4215802776, 13229, 19047, [130, 81, 15, 156, 36, 155, 106, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxProposeNewTimeForMeetingRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxResolveRecipientsRequest {
    type Vtable = IEmailMailboxResolveRecipientsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4020555632, 31545, 19611, [129, 30, 65, 234, 244, 58, 51, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resolutionresults: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxResolveRecipientsRequestEventArgs {
    type Vtable = IEmailMailboxResolveRecipientsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(638557698, 45775, 16632, [140, 40, 227, 237, 67, 177, 232, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxResolveRecipientsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxServerSearchReadBatchRequest {
    type Vtable = IEmailMailboxServerSearchReadBatchRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(151972831, 23190, 16851, [138, 216, 52, 145, 47, 154, 166, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, batchstatus: super::EmailBatchStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    type Vtable = IEmailMailboxServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(336599886, 60830, 17873, [173, 122, 204, 155, 127, 100, 58, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxServerSearchReadBatchRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxSetAutoReplySettingsRequest {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1973691088, 43150, 20052, [141, 199, 194, 67, 24, 107, 119, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    type Vtable = IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(165286317, 55242, 16519, [172, 134, 83, 250, 103, 247, 98, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSetAutoReplySettingsRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxSyncManagerSyncRequest {
    type Vtable = IEmailMailboxSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1309731044, 32359, 16474, [182, 115, 220, 96, 201, 16, 144, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxSyncManagerSyncRequestEventArgs {
    type Vtable = IEmailMailboxSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1134166810, 36812, 19173, [185, 181, 212, 52, 224, 166, 90, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxSyncManagerSyncRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxUpdateMeetingResponseRequest {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1536797843, 45775, 18568, [186, 79, 48, 107, 107, 102, 223, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::EmailMeetingResponseType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    type Vtable = IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1754847073, 22217, 20247, [190, 49, 102, 253, 169, 75, 161, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxUpdateMeetingResponseRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxValidateCertificatesRequest {
    type Vtable = IEmailMailboxValidateCertificatesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2840410417, 57626, 20375, [184, 26, 24, 122, 112, 168, 244, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, validationstatuses: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEmailMailboxValidateCertificatesRequestEventArgs {
    type Vtable = IEmailMailboxValidateCertificatesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(629391127, 767, 18942, [167, 60, 3, 243, 117, 102, 198, 145]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEmailMailboxValidateCertificatesRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
