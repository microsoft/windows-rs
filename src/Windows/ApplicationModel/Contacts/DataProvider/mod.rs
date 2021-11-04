#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactDataProviderConnection(pub ::windows::runtime::IInspectable);
impl ContactDataProviderConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn SyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListSyncManagerSyncRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn RemoveSyncRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ServerSearchReadBatchRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListServerSearchReadBatchRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn RemoveServerSearchReadBatchRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn CreateOrUpdateContactRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListCreateOrUpdateContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn RemoveCreateOrUpdateContactRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn DeleteContactRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactDataProviderConnection, ContactListDeleteContactRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn RemoveDeleteContactRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactDataProviderConnection2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactDataProviderConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection;{1a398a52-8c9d-4d6f-a4e0-111e9a125a30})");
}
unsafe impl ::windows::runtime::Interface for ContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(439978578, 35997, 19823, [164, 224, 17, 30, 154, 18, 90, 48]);
}
impl ::windows::runtime::RuntimeName for ContactDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
}
impl ::std::convert::From<ContactDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: ContactDataProviderConnection) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactDataProviderConnection> for ::windows::runtime::IUnknown {
    fn from(value: &ContactDataProviderConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: ContactDataProviderConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactDataProviderConnection> for ::windows::runtime::IInspectable {
    fn from(value: &ContactDataProviderConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactDataProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactDataProviderConnection {}
unsafe impl ::std::marker::Sync for ContactDataProviderConnection {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactDataProviderTriggerDetails(pub ::windows::runtime::IInspectable);
impl ContactDataProviderTriggerDetails {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<ContactDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactDataProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactDataProviderTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails;{527104be-3c62-43c8-9ae7-db531685cd99})");
}
unsafe impl ::windows::runtime::Interface for ContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1383138494, 15458, 17352, [154, 231, 219, 83, 22, 133, 205, 153]);
}
impl ::windows::runtime::RuntimeName for ContactDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
}
impl ::std::convert::From<ContactDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactDataProviderTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ContactDataProviderTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactDataProviderTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ContactDataProviderTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactDataProviderTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactDataProviderTriggerDetails {}
unsafe impl ::std::marker::Sync for ContactDataProviderTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListCreateOrUpdateContactRequest(pub ::windows::runtime::IInspectable);
impl ContactListCreateOrUpdateContactRequest {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<super::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Contact>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Contact>>(&self, createdorupdatedcontact: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), createdorupdatedcontact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListCreateOrUpdateContactRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest;{b4af411f-c849-47d0-b119-91cf605b2f2a})");
}
unsafe impl ::windows::runtime::Interface for ContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3031384351, 51273, 18384, [177, 25, 145, 207, 96, 91, 47, 42]);
}
impl ::windows::runtime::RuntimeName for ContactListCreateOrUpdateContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
}
impl ::std::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::runtime::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListCreateOrUpdateContactRequest> for ::windows::runtime::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListCreateOrUpdateContactRequest> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListCreateOrUpdateContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListCreateOrUpdateContactRequest {}
unsafe impl ::std::marker::Sync for ContactListCreateOrUpdateContactRequest {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListCreateOrUpdateContactRequestEventArgs(pub ::windows::runtime::IInspectable);
impl ContactListCreateOrUpdateContactRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<ContactListCreateOrUpdateContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListCreateOrUpdateContactRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListCreateOrUpdateContactRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs;{851c1690-1a51-4b0c-aeef-1240ac5bed75})");
}
unsafe impl ::windows::runtime::Interface for ContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2233210512, 6737, 19212, [174, 239, 18, 64, 172, 91, 237, 117]);
}
impl ::windows::runtime::RuntimeName for ContactListCreateOrUpdateContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
}
impl ::std::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListCreateOrUpdateContactRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListCreateOrUpdateContactRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListCreateOrUpdateContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListCreateOrUpdateContactRequestEventArgs {}
unsafe impl ::std::marker::Sync for ContactListCreateOrUpdateContactRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListDeleteContactRequest(pub ::windows::runtime::IInspectable);
impl ContactListDeleteContactRequest {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn ContactId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListDeleteContactRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest;{5e114687-ce03-4de5-8557-9ccf552d472a})");
}
unsafe impl ::windows::runtime::Interface for ContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1578190471, 52739, 19941, [133, 87, 156, 207, 85, 45, 71, 42]);
}
impl ::windows::runtime::RuntimeName for ContactListDeleteContactRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
}
impl ::std::convert::From<ContactListDeleteContactRequest> for ::windows::runtime::IUnknown {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListDeleteContactRequest> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListDeleteContactRequest> for ::windows::runtime::IInspectable {
    fn from(value: ContactListDeleteContactRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListDeleteContactRequest> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListDeleteContactRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListDeleteContactRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListDeleteContactRequest {}
unsafe impl ::std::marker::Sync for ContactListDeleteContactRequest {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListDeleteContactRequestEventArgs(pub ::windows::runtime::IInspectable);
impl ContactListDeleteContactRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<ContactListDeleteContactRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListDeleteContactRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListDeleteContactRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs;{b22054a1-e8fa-4db5-9389-2d12ee7d15ee})");
}
unsafe impl ::windows::runtime::Interface for ContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2988463265, 59642, 19893, [147, 137, 45, 18, 238, 125, 21, 238]);
}
impl ::windows::runtime::RuntimeName for ContactListDeleteContactRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
}
impl ::std::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListDeleteContactRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactListDeleteContactRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListDeleteContactRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListDeleteContactRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListDeleteContactRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListDeleteContactRequestEventArgs {}
unsafe impl ::std::marker::Sync for ContactListDeleteContactRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListServerSearchReadBatchRequest(pub ::windows::runtime::IInspectable);
impl ContactListServerSearchReadBatchRequest {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<super::ContactQueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::ContactQueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn SuggestedBatchSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn SaveContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self, batchstatus: super::ContactBatchStatus) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), batchstatus, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListServerSearchReadBatchRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest;{ba776a97-4030-4925-9fb4-143b295e653b})");
}
unsafe impl ::windows::runtime::Interface for ContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3128388247, 16432, 18725, [159, 180, 20, 59, 41, 94, 101, 59]);
}
impl ::windows::runtime::RuntimeName for ContactListServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
}
impl ::std::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::runtime::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListServerSearchReadBatchRequest> for ::windows::runtime::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListServerSearchReadBatchRequest> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListServerSearchReadBatchRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListServerSearchReadBatchRequest {}
unsafe impl ::std::marker::Sync for ContactListServerSearchReadBatchRequest {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListServerSearchReadBatchRequestEventArgs(pub ::windows::runtime::IInspectable);
impl ContactListServerSearchReadBatchRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<ContactListServerSearchReadBatchRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListServerSearchReadBatchRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListServerSearchReadBatchRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs;{1a27e87b-69d7-4e4e-8042-861cba61471e})");
}
unsafe impl ::windows::runtime::Interface for ContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(438823035, 27095, 20046, [128, 66, 134, 28, 186, 97, 71, 30]);
}
impl ::windows::runtime::RuntimeName for ContactListServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
}
impl ::std::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListServerSearchReadBatchRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListServerSearchReadBatchRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListServerSearchReadBatchRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListServerSearchReadBatchRequestEventArgs {}
unsafe impl ::std::marker::Sync for ContactListServerSearchReadBatchRequestEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListSyncManagerSyncRequest(pub ::windows::runtime::IInspectable);
impl ContactListSyncManagerSyncRequest {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListSyncManagerSyncRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest;{3c0e57a4-c4e7-4970-9a8f-9a66a2bb6c1a})");
}
unsafe impl ::windows::runtime::Interface for ContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007572900, 50407, 18800, [154, 143, 154, 102, 162, 187, 108, 26]);
}
impl ::windows::runtime::RuntimeName for ContactListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
}
impl ::std::convert::From<ContactListSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListSyncManagerSyncRequest> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListSyncManagerSyncRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListSyncManagerSyncRequest {}
unsafe impl ::std::marker::Sync for ContactListSyncManagerSyncRequest {}
#[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListSyncManagerSyncRequestEventArgs(pub ::windows::runtime::IInspectable);
impl ContactListSyncManagerSyncRequestEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<ContactListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncManagerSyncRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts_DataProvider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs;{158e4dac-446d-4f10-afc2-02683ec533a6})");
}
unsafe impl ::windows::runtime::Interface for ContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(361647532, 17517, 20240, [175, 194, 2, 104, 62, 197, 51, 166]);
}
impl ::windows::runtime::RuntimeName for ContactListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
}
impl ::std::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ContactListSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListSyncManagerSyncRequestEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListSyncManagerSyncRequestEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListSyncManagerSyncRequestEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListSyncManagerSyncRequestEventArgs {}
unsafe impl ::std::marker::Sync for ContactListSyncManagerSyncRequestEventArgs {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderConnection(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactDataProviderConnection {
    type Vtable = IContactDataProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(439978578, 35997, 19823, [164, 224, 17, 30, 154, 18, 90, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactDataProviderConnection2 {
    type Vtable = IContactDataProviderConnection2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2714970032, 6508, 19453, [143, 15, 198, 141, 103, 242, 73, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderConnection2_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactDataProviderTriggerDetails {
    type Vtable = IContactDataProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1383138494, 15458, 17352, [154, 231, 219, 83, 22, 133, 205, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDataProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListCreateOrUpdateContactRequest {
    type Vtable = IContactListCreateOrUpdateContactRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3031384351, 51273, 18384, [177, 25, 145, 207, 96, 91, 47, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, createdorupdatedcontact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListCreateOrUpdateContactRequestEventArgs {
    type Vtable = IContactListCreateOrUpdateContactRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2233210512, 6737, 19212, [174, 239, 18, 64, 172, 91, 237, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListCreateOrUpdateContactRequestEventArgs_abi(
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
#[doc(hidden)]
pub struct IContactListDeleteContactRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListDeleteContactRequest {
    type Vtable = IContactListDeleteContactRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1578190471, 52739, 19941, [133, 87, 156, 207, 85, 45, 71, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequest_abi(
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
#[doc(hidden)]
pub struct IContactListDeleteContactRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListDeleteContactRequestEventArgs {
    type Vtable = IContactListDeleteContactRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2988463265, 59642, 19893, [147, 137, 45, 18, 238, 125, 21, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListDeleteContactRequestEventArgs_abi(
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
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListServerSearchReadBatchRequest {
    type Vtable = IContactListServerSearchReadBatchRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3128388247, 16432, 18725, [159, 180, 20, 59, 41, 94, 101, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, batchstatus: super::ContactBatchStatus, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListServerSearchReadBatchRequestEventArgs {
    type Vtable = IContactListServerSearchReadBatchRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(438823035, 27095, 20046, [128, 66, 134, 28, 186, 97, 71, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListServerSearchReadBatchRequestEventArgs_abi(
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
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListSyncManagerSyncRequest {
    type Vtable = IContactListSyncManagerSyncRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1007572900, 50407, 18800, [154, 143, 154, 102, 162, 187, 108, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequest_abi(
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
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListSyncManagerSyncRequestEventArgs {
    type Vtable = IContactListSyncManagerSyncRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(361647532, 17517, 20240, [175, 194, 2, 104, 62, 197, 51, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManagerSyncRequestEventArgs_abi(
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
