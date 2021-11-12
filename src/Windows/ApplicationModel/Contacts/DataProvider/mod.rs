#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactDataProviderConnection(pub ::windows::core::IInspectable);
impl ContactDataProviderConnection {
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveServerSearchReadBatchRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteContactRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection;{1a398a52-8c9d-4d6f-a4e0-111e9a125a30})");
}
unsafe impl ::windows::core::Interface for ContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
}
impl ::windows::core::RuntimeName for ContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
}
impl ::core::convert::From<ContactDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: ContactDataProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactDataProviderConnection> for ::windows::core::IUnknown {
    fn from(value: &ContactDataProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: ContactDataProviderConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactDataProviderConnection> for ::windows::core::IInspectable {
    fn from(value: &ContactDataProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactDataProviderConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactDataProviderConnection {}
unsafe impl ::core::marker::Sync for ContactDataProviderConnection {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactDataProviderTriggerDetails(pub ::windows::core::IInspectable);
impl ContactDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows::core::Result<ContactDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactDataProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails;{527104be-3c62-43c8-9ae7-db531685cd99})");
}
unsafe impl ::windows::core::Interface for ContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x527104be_3c62_43c8_9ae7_db531685cd99);
}
impl ::windows::core::RuntimeName for ContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
}
impl ::core::convert::From<ContactDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactDataProviderTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactDataProviderTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for ContactDataProviderTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListCreateOrUpdateContactRequest(pub ::windows::core::IInspectable);
impl ContactListCreateOrUpdateContactRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Contact(&self) -> ::windows::core::Result<super::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Contact>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Contact>>(&self, createdorupdatedcontact: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), createdorupdatedcontact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest;{b4af411f-c849-47d0-b119-91cf605b2f2a})");
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4af411f_c849_47d0_b119_91cf605b2f2a);
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequest {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(pub ::windows::core::IInspectable);
impl ContactListCreateOrUpdateContactRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListCreateOrUpdateContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListCreateOrUpdateContactRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs;{851c1690-1a51-4b0c-aeef-1240ac5bed75})");
}
unsafe impl ::windows::core::Interface for ContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
}
impl ::windows::core::RuntimeName for ContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListCreateOrUpdateContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListCreateOrUpdateContactRequestEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListDeleteContactRequest(pub ::windows::core::IInspectable);
impl ContactListDeleteContactRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContactId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest;{5e114687-ce03-4de5-8557-9ccf552d472a})");
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e114687_ce03_4de5_8557_9ccf552d472a);
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
}
impl ::core::convert::From<ContactListDeleteContactRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListDeleteContactRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListDeleteContactRequest {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListDeleteContactRequestEventArgs(pub ::windows::core::IInspectable);
impl ContactListDeleteContactRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListDeleteContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListDeleteContactRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListDeleteContactRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs;{b22054a1-e8fa-4db5-9389-2d12ee7d15ee})");
}
unsafe impl ::windows::core::Interface for ContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
}
impl ::windows::core::RuntimeName for ContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
}
impl ::core::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListDeleteContactRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListDeleteContactRequestEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListServerSearchReadBatchRequest(pub ::windows::core::IInspectable);
impl ContactListServerSearchReadBatchRequest {
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<super::ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ContactQueryOptions>(result__)
        }
    }
    pub fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveContactAsync<'a, Param0: ::windows::core::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), batchstatus, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest;{ba776a97-4030-4925-9fb4-143b295e653b})");
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba776a97_4030_4925_9fb4_143b295e653b);
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequest {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(pub ::windows::core::IInspectable);
impl ContactListServerSearchReadBatchRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListServerSearchReadBatchRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs;{1a27e87b-69d7-4e4e-8042-861cba61471e})");
}
unsafe impl ::windows::core::Interface for ContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a27e87b_69d7_4e4e_8042_861cba61471e);
}
impl ::windows::core::RuntimeName for ContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListServerSearchReadBatchRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListServerSearchReadBatchRequestEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListSyncManagerSyncRequest(pub ::windows::core::IInspectable);
impl ContactListSyncManagerSyncRequest {
    pub fn ContactListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest;{3c0e57a4-c4e7-4970-9a8f-9a66a2bb6c1a})");
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
}
impl ::core::convert::From<ContactListSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::core::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::core::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequest {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactListSyncManagerSyncRequestEventArgs(pub ::windows::core::IInspectable);
impl ContactListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<ContactListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs;{158e4dac-446d-4f10-afc2-02683ec533a6})");
}
unsafe impl ::windows::core::Interface for ContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x158e4dac_446d_4f10_afc2_02683ec533a6);
}
impl ::windows::core::RuntimeName for ContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
}
impl ::core::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ContactListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for ContactListSyncManagerSyncRequestEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a398a52_8c9d_4d6f_a4e0_111e9a125a30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection_abi(
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
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactDataProviderConnection2 {
    type Vtable = IContactDataProviderConnection2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1d327b0_196c_4bfd_8f0f_c68d67f249d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x527104be_3c62_43c8_9ae7_db531685cd99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetails_abi(
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
pub struct IContactListCreateOrUpdateContactRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4af411f_c849_47d0_b119_91cf605b2f2a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, createdorupdatedcontact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x851c1690_1a51_4b0c_aeef_1240ac5bed75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs_abi(
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
pub struct IContactListDeleteContactRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e114687_ce03_4de5_8557_9ccf552d472a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequest_abi(
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
pub struct IContactListDeleteContactRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22054a1_e8fa_4db5_9389_2d12ee7d15ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequestEventArgs_abi(
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
pub struct IContactListServerSearchReadBatchRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xba776a97_4030_4925_9fb4_143b295e653b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contact: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, batchstatus: super::ContactBatchStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a27e87b_69d7_4e4e_8042_861cba61471e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgs_abi(
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
pub struct IContactListSyncManagerSyncRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c0e57a4_c4e7_4970_9a8f_9a66a2bb6c1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequest_abi(
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
pub struct IContactListSyncManagerSyncRequestEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x158e4dac_446d_4f10_afc2_02683ec533a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestEventArgs_abi(
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
