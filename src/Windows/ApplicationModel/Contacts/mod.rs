#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_Contacts_DataProvider")]
pub mod DataProvider;
#[cfg(feature = "ApplicationModel_Contacts_Provider")]
pub mod Provider;
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AggregateContactManager(::windows::runtime::IInspectable);
impl AggregateContactManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindRawContactsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn TryLinkContactsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, Contact>>(&self, primarycontact: Param0, secondarycontact: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), primarycontact.into_param().abi(), secondarycontact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn UnlinkRawContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn TrySetPreferredSourceForPictureAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, Contact>>(&self, aggregatecontact: Param0, rawcontact: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), aggregatecontact.into_param().abi(), rawcontact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetRemoteIdentificationInformationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactlistid: Param0, remotesourceid: Param1, accountid: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IAggregateContactManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contactlistid.into_param().abi(), remotesourceid.into_param().abi(), accountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AggregateContactManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.AggregateContactManager;{0379d5dd-db5a-4fd3-b54e-4df17917a212})");
}
unsafe impl ::windows::runtime::Interface for AggregateContactManager {
    type Vtable = IAggregateContactManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(58316253, 56154, 20435, [181, 78, 77, 241, 121, 23, 162, 18]);
}
impl ::windows::runtime::RuntimeName for AggregateContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.AggregateContactManager";
}
impl ::std::convert::From<AggregateContactManager> for ::windows::runtime::IUnknown {
    fn from(value: AggregateContactManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&AggregateContactManager> for ::windows::runtime::IUnknown {
    fn from(value: &AggregateContactManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for AggregateContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &AggregateContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<AggregateContactManager> for ::windows::runtime::IInspectable {
    fn from(value: AggregateContactManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&AggregateContactManager> for ::windows::runtime::IInspectable {
    fn from(value: &AggregateContactManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for AggregateContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a AggregateContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for AggregateContactManager {}
unsafe impl ::std::marker::Sync for AggregateContactManager {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Contact(::windows::runtime::IInspectable);
impl Contact {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Contact, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Fields(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<IContactField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IContactField>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Notes(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetNotes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Phones(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactPhone>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactPhone>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Emails(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactEmail>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactEmail>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Addresses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactAddress>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactAddress>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ConnectedServiceAccounts(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactConnectedServiceAccount>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ImportantDates(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactDate>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactDate>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn DataSuppliers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn JobInfo(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactJobInfo>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactJobInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn SignificantOthers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactSignificantOther>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactSignificantOther>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Websites(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactWebsite>> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactWebsite>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ProviderProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IContact2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DisplayPictureUserUpdateTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetDisplayPictureUserUpdateTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsMe(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn AggregateId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn RingToneToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRingToneToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsDisplayPictureManuallySet(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn LargeDisplayPicture(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn SmallDisplayPicture(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn SourceDisplayPicture(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Storage_Streams`*"]
    pub fn SetSourceDisplayPicture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn TextToneToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetTextToneToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsAggregate(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn FullName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DisplayNameOverride(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDisplayNameOverride<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Nickname(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetNickname<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SortName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContact3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn FirstName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetFirstName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn LastName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetLastName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn MiddleName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetMiddleName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn YomiGivenName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetYomiGivenName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn YomiFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetYomiFamilyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn HonorificNameSuffix(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetHonorificNameSuffix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn HonorificNamePrefix(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetHonorificNamePrefix<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn YomiDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactName>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Contact {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Contact;{ec0072f3-2118-4049-9ebc-17f0ab692b64})");
}
unsafe impl ::windows::runtime::Interface for Contact {
    type Vtable = IContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3959452403, 8472, 16457, [158, 188, 23, 240, 171, 105, 43, 100]);
}
impl ::windows::runtime::RuntimeName for Contact {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Contact";
}
impl ::std::convert::From<Contact> for ::windows::runtime::IUnknown {
    fn from(value: Contact) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Contact> for ::windows::runtime::IUnknown {
    fn from(value: &Contact) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Contact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Contact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Contact> for ::windows::runtime::IInspectable {
    fn from(value: Contact) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Contact> for ::windows::runtime::IInspectable {
    fn from(value: &Contact) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Contact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Contact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Contact {}
unsafe impl ::std::marker::Sync for Contact {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactAddress(::windows::runtime::IInspectable);
impl ContactAddress {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactAddress, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn StreetAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetStreetAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Locality(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetLocality<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Region(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRegion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Country(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCountry<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn PostalCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetPostalCode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ContactAddressKind> {
        let this = self;
        unsafe {
            let mut result__: ContactAddressKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactAddressKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetKind(&self, value: ContactAddressKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactAddress {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAddress;{9739d39a-42ce-4872-8d70-3063aa584b70})");
}
unsafe impl ::windows::runtime::Interface for ContactAddress {
    type Vtable = IContactAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2537149338, 17102, 18546, [141, 112, 48, 99, 170, 88, 75, 112]);
}
impl ::windows::runtime::RuntimeName for ContactAddress {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAddress";
}
impl ::std::convert::From<ContactAddress> for ::windows::runtime::IUnknown {
    fn from(value: ContactAddress) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactAddress> for ::windows::runtime::IUnknown {
    fn from(value: &ContactAddress) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactAddress> for ::windows::runtime::IInspectable {
    fn from(value: ContactAddress) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactAddress> for ::windows::runtime::IInspectable {
    fn from(value: &ContactAddress) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactAddress {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactAddress {}
unsafe impl ::std::marker::Sync for ContactAddress {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactAddressKind(pub i32);
impl ContactAddressKind {
    pub const Home: ContactAddressKind = ContactAddressKind(0i32);
    pub const Work: ContactAddressKind = ContactAddressKind(1i32);
    pub const Other: ContactAddressKind = ContactAddressKind(2i32);
}
impl ::std::convert::From<i32> for ContactAddressKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactAddressKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactAddressKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAddressKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactAddressKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactAnnotation(::windows::runtime::IInspectable);
impl ContactAnnotation {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactAnnotation, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn AnnotationListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ContactId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetContactId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SupportedOperations(&self) -> ::windows::runtime::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__: ContactAnnotationOperations = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactAnnotationOperations>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSupportedOperations(&self, value: ContactAnnotationOperations) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsDisabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ProviderProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ContactListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactAnnotation2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetContactListId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactAnnotation2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactAnnotation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotation;{821fc2ef-7d41-44a2-84c3-60a281dd7b86})");
}
unsafe impl ::windows::runtime::Interface for ContactAnnotation {
    type Vtable = IContactAnnotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2183119599, 32065, 17570, [132, 195, 96, 162, 129, 221, 123, 134]);
}
impl ::windows::runtime::RuntimeName for ContactAnnotation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotation";
}
impl ::std::convert::From<ContactAnnotation> for ::windows::runtime::IUnknown {
    fn from(value: ContactAnnotation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactAnnotation> for ::windows::runtime::IUnknown {
    fn from(value: &ContactAnnotation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactAnnotation> for ::windows::runtime::IInspectable {
    fn from(value: ContactAnnotation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactAnnotation> for ::windows::runtime::IInspectable {
    fn from(value: &ContactAnnotation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactAnnotation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactAnnotation {}
unsafe impl ::std::marker::Sync for ContactAnnotation {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactAnnotationList(::windows::runtime::IInspectable);
impl ContactAnnotationList {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ProviderPackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn TrySaveAnnotationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ContactAnnotation>>(&self, annotation: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), annotation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetAnnotationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, annotationid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), annotationid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAnnotationsByRemoteIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, remoteid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), remoteid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAnnotationsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DeleteAnnotationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ContactAnnotation>>(&self, annotation: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), annotation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactAnnotationList {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationList;{92a486aa-5c88-45b9-aad0-461888e68d8a})");
}
unsafe impl ::windows::runtime::Interface for ContactAnnotationList {
    type Vtable = IContactAnnotationList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460255914, 23688, 17849, [170, 208, 70, 24, 136, 230, 141, 138]);
}
impl ::windows::runtime::RuntimeName for ContactAnnotationList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationList";
}
impl ::std::convert::From<ContactAnnotationList> for ::windows::runtime::IUnknown {
    fn from(value: ContactAnnotationList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactAnnotationList> for ::windows::runtime::IUnknown {
    fn from(value: &ContactAnnotationList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactAnnotationList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactAnnotationList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactAnnotationList> for ::windows::runtime::IInspectable {
    fn from(value: ContactAnnotationList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactAnnotationList> for ::windows::runtime::IInspectable {
    fn from(value: &ContactAnnotationList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactAnnotationList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactAnnotationList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactAnnotationList {}
unsafe impl ::std::marker::Sync for ContactAnnotationList {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactAnnotationOperations(pub u32);
impl ContactAnnotationOperations {
    pub const None: ContactAnnotationOperations = ContactAnnotationOperations(0u32);
    pub const ContactProfile: ContactAnnotationOperations = ContactAnnotationOperations(1u32);
    pub const Message: ContactAnnotationOperations = ContactAnnotationOperations(2u32);
    pub const AudioCall: ContactAnnotationOperations = ContactAnnotationOperations(4u32);
    pub const VideoCall: ContactAnnotationOperations = ContactAnnotationOperations(8u32);
    pub const SocialFeeds: ContactAnnotationOperations = ContactAnnotationOperations(16u32);
    pub const Share: ContactAnnotationOperations = ContactAnnotationOperations(32u32);
}
impl ::std::convert::From<u32> for ContactAnnotationOperations {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactAnnotationOperations {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactAnnotationOperations {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationOperations;u4)");
}
impl ::windows::runtime::DefaultType for ContactAnnotationOperations {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ContactAnnotationOperations {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ContactAnnotationOperations {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ContactAnnotationOperations {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ContactAnnotationOperations {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ContactAnnotationOperations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactAnnotationStore(::windows::runtime::IInspectable);
impl ContactAnnotationStore {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactIdsByEmailAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, emailaddress: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), emailaddress.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactIdsByPhoneNumberAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, phonenumber: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), phonenumber.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAnnotationsForContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DisableAnnotationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ContactAnnotation>>(&self, annotation: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), annotation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateAnnotationListAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateAnnotationListInAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdataaccountid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetAnnotationListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, annotationlistid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), annotationlistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotationList>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAnnotationListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotationList>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAnnotationsForContactListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactlistid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>> {
        let this = &::windows::runtime::Interface::cast::<IContactAnnotationStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contactlistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactAnnotation>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactAnnotationStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactAnnotationStore;{23acf4aa-7a77-457d-8203-987f4b31af09})");
}
unsafe impl ::windows::runtime::Interface for ContactAnnotationStore {
    type Vtable = IContactAnnotationStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(598537386, 31351, 17789, [130, 3, 152, 127, 75, 49, 175, 9]);
}
impl ::windows::runtime::RuntimeName for ContactAnnotationStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactAnnotationStore";
}
impl ::std::convert::From<ContactAnnotationStore> for ::windows::runtime::IUnknown {
    fn from(value: ContactAnnotationStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactAnnotationStore> for ::windows::runtime::IUnknown {
    fn from(value: &ContactAnnotationStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactAnnotationStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactAnnotationStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactAnnotationStore> for ::windows::runtime::IInspectable {
    fn from(value: ContactAnnotationStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactAnnotationStore> for ::windows::runtime::IInspectable {
    fn from(value: &ContactAnnotationStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactAnnotationStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactAnnotationStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactAnnotationStore {}
unsafe impl ::std::marker::Sync for ContactAnnotationStore {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactAnnotationStoreAccessType(pub i32);
impl ContactAnnotationStoreAccessType {
    pub const AppAnnotationsReadWrite: ContactAnnotationStoreAccessType = ContactAnnotationStoreAccessType(0i32);
    pub const AllAnnotationsReadWrite: ContactAnnotationStoreAccessType = ContactAnnotationStoreAccessType(1i32);
}
impl ::std::convert::From<i32> for ContactAnnotationStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactAnnotationStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactAnnotationStoreAccessType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactAnnotationStoreAccessType;i4)");
}
impl ::windows::runtime::DefaultType for ContactAnnotationStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactBatch(::windows::runtime::IInspectable);
impl ContactBatch {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Contacts(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<Contact>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ContactBatchStatus> {
        let this = self;
        unsafe {
            let mut result__: ContactBatchStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactBatchStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactBatch {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactBatch;{35d1972d-bfce-46bb-93f8-a5b06ec5e201})");
}
unsafe impl ::windows::runtime::Interface for ContactBatch {
    type Vtable = IContactBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(902928173, 49102, 18107, [147, 248, 165, 176, 110, 197, 226, 1]);
}
impl ::windows::runtime::RuntimeName for ContactBatch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactBatch";
}
impl ::std::convert::From<ContactBatch> for ::windows::runtime::IUnknown {
    fn from(value: ContactBatch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactBatch> for ::windows::runtime::IUnknown {
    fn from(value: &ContactBatch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactBatch> for ::windows::runtime::IInspectable {
    fn from(value: ContactBatch) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactBatch> for ::windows::runtime::IInspectable {
    fn from(value: &ContactBatch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactBatch {}
unsafe impl ::std::marker::Sync for ContactBatch {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactBatchStatus(pub i32);
impl ContactBatchStatus {
    pub const Success: ContactBatchStatus = ContactBatchStatus(0i32);
    pub const ServerSearchSyncManagerError: ContactBatchStatus = ContactBatchStatus(1i32);
    pub const ServerSearchUnknownError: ContactBatchStatus = ContactBatchStatus(2i32);
}
impl ::std::convert::From<i32> for ContactBatchStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactBatchStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactBatchStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactBatchStatus;i4)");
}
impl ::windows::runtime::DefaultType for ContactBatchStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactCardDelayedDataLoader(::windows::runtime::IInspectable);
impl ContactCardDelayedDataLoader {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactCardDelayedDataLoader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader;{b60af902-1546-434d-869c-6e3520760ef3})");
}
unsafe impl ::windows::runtime::Interface for ContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3054172418, 5446, 17229, [134, 156, 110, 53, 32, 118, 14, 243]);
}
impl ::windows::runtime::RuntimeName for ContactCardDelayedDataLoader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardDelayedDataLoader";
}
impl ::std::convert::From<ContactCardDelayedDataLoader> for ::windows::runtime::IUnknown {
    fn from(value: ContactCardDelayedDataLoader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactCardDelayedDataLoader> for ::windows::runtime::IUnknown {
    fn from(value: &ContactCardDelayedDataLoader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactCardDelayedDataLoader> for ::windows::runtime::IInspectable {
    fn from(value: ContactCardDelayedDataLoader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactCardDelayedDataLoader> for ::windows::runtime::IInspectable {
    fn from(value: &ContactCardDelayedDataLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ContactCardDelayedDataLoader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactCardDelayedDataLoader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ContactCardDelayedDataLoader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactCardDelayedDataLoader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ContactCardDelayedDataLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContactCardDelayedDataLoader {}
unsafe impl ::std::marker::Sync for ContactCardDelayedDataLoader {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactCardHeaderKind(pub i32);
impl ContactCardHeaderKind {
    pub const Default: ContactCardHeaderKind = ContactCardHeaderKind(0i32);
    pub const Basic: ContactCardHeaderKind = ContactCardHeaderKind(1i32);
    pub const Enterprise: ContactCardHeaderKind = ContactCardHeaderKind(2i32);
}
impl ::std::convert::From<i32> for ContactCardHeaderKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactCardHeaderKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactCardHeaderKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardHeaderKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactCardHeaderKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactCardOptions(::windows::runtime::IInspectable);
impl ContactCardOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactCardOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn HeaderKind(&self) -> ::windows::runtime::Result<ContactCardHeaderKind> {
        let this = self;
        unsafe {
            let mut result__: ContactCardHeaderKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactCardHeaderKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetHeaderKind(&self, value: ContactCardHeaderKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn InitialTabKind(&self) -> ::windows::runtime::Result<ContactCardTabKind> {
        let this = self;
        unsafe {
            let mut result__: ContactCardTabKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactCardTabKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetInitialTabKind(&self, value: ContactCardTabKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ServerSearchContactListIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IContactCardOptions2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactCardOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactCardOptions;{8c0a4f7e-6ab6-4f3f-be72-817236eeea5b})");
}
unsafe impl ::windows::runtime::Interface for ContactCardOptions {
    type Vtable = IContactCardOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349485950, 27318, 20287, [190, 114, 129, 114, 54, 238, 234, 91]);
}
impl ::windows::runtime::RuntimeName for ContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactCardOptions";
}
impl ::std::convert::From<ContactCardOptions> for ::windows::runtime::IUnknown {
    fn from(value: ContactCardOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactCardOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ContactCardOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactCardOptions> for ::windows::runtime::IInspectable {
    fn from(value: ContactCardOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactCardOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ContactCardOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactCardOptions {}
unsafe impl ::std::marker::Sync for ContactCardOptions {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactCardTabKind(pub i32);
impl ContactCardTabKind {
    pub const Default: ContactCardTabKind = ContactCardTabKind(0i32);
    pub const Email: ContactCardTabKind = ContactCardTabKind(1i32);
    pub const Messaging: ContactCardTabKind = ContactCardTabKind(2i32);
    pub const Phone: ContactCardTabKind = ContactCardTabKind(3i32);
    pub const Video: ContactCardTabKind = ContactCardTabKind(4i32);
    pub const OrganizationalHierarchy: ContactCardTabKind = ContactCardTabKind(5i32);
}
impl ::std::convert::From<i32> for ContactCardTabKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactCardTabKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactCardTabKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactCardTabKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactCardTabKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactChange(::windows::runtime::IInspectable);
impl ContactChange {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ChangeType(&self) -> ::windows::runtime::Result<ContactChangeType> {
        let this = self;
        unsafe {
            let mut result__: ContactChangeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactChangeType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Contact(&self) -> ::windows::runtime::Result<Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Contact>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactChange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChange;{951d4b10-6a59-4720-a4e1-363d98c135d5})");
}
unsafe impl ::windows::runtime::Interface for ContactChange {
    type Vtable = IContactChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2501724944, 27225, 18208, [164, 225, 54, 61, 152, 193, 53, 213]);
}
impl ::windows::runtime::RuntimeName for ContactChange {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChange";
}
impl ::std::convert::From<ContactChange> for ::windows::runtime::IUnknown {
    fn from(value: ContactChange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactChange> for ::windows::runtime::IUnknown {
    fn from(value: &ContactChange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactChange> for ::windows::runtime::IInspectable {
    fn from(value: ContactChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactChange> for ::windows::runtime::IInspectable {
    fn from(value: &ContactChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactChange {}
unsafe impl ::std::marker::Sync for ContactChange {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactChangeReader(::windows::runtime::IInspectable);
impl ContactChangeReader {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn AcceptChanges(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn AcceptChangesThrough<'a, Param0: ::windows::runtime::IntoParam<'a, ContactChange>>(&self, lastchangetoaccept: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), lastchangetoaccept.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactChange>>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactChangeReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeReader;{217319fa-2d0c-42e0-a9da-3ecd56a78a47})");
}
unsafe impl ::windows::runtime::Interface for ContactChangeReader {
    type Vtable = IContactChangeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561191418, 11532, 17120, [169, 218, 62, 205, 86, 167, 138, 71]);
}
impl ::windows::runtime::RuntimeName for ContactChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeReader";
}
impl ::std::convert::From<ContactChangeReader> for ::windows::runtime::IUnknown {
    fn from(value: ContactChangeReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactChangeReader> for ::windows::runtime::IUnknown {
    fn from(value: &ContactChangeReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactChangeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactChangeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactChangeReader> for ::windows::runtime::IInspectable {
    fn from(value: ContactChangeReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactChangeReader> for ::windows::runtime::IInspectable {
    fn from(value: &ContactChangeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactChangeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactChangeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactChangeReader {}
unsafe impl ::std::marker::Sync for ContactChangeReader {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactChangeTracker(::windows::runtime::IInspectable);
impl ContactChangeTracker {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Enable(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetChangeReader(&self) -> ::windows::runtime::Result<ContactChangeReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactChangeReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Reset(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsTracking(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IContactChangeTracker2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactChangeTracker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangeTracker;{6e992952-309b-404d-9712-b37bd30278aa})");
}
unsafe impl ::windows::runtime::Interface for ContactChangeTracker {
    type Vtable = IContactChangeTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1855531346, 12443, 16461, [151, 18, 179, 123, 211, 2, 120, 170]);
}
impl ::windows::runtime::RuntimeName for ContactChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangeTracker";
}
impl ::std::convert::From<ContactChangeTracker> for ::windows::runtime::IUnknown {
    fn from(value: ContactChangeTracker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactChangeTracker> for ::windows::runtime::IUnknown {
    fn from(value: &ContactChangeTracker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactChangeTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactChangeTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactChangeTracker> for ::windows::runtime::IInspectable {
    fn from(value: ContactChangeTracker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactChangeTracker> for ::windows::runtime::IInspectable {
    fn from(value: &ContactChangeTracker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactChangeTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactChangeTracker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactChangeTracker {}
unsafe impl ::std::marker::Sync for ContactChangeTracker {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactChangeType(pub i32);
impl ContactChangeType {
    pub const Created: ContactChangeType = ContactChangeType(0i32);
    pub const Modified: ContactChangeType = ContactChangeType(1i32);
    pub const Deleted: ContactChangeType = ContactChangeType(2i32);
    pub const ChangeTrackingLost: ContactChangeType = ContactChangeType(3i32);
}
impl ::std::convert::From<i32> for ContactChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactChangeType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactChangeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactChangeType;i4)");
}
impl ::windows::runtime::DefaultType for ContactChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactChangedDeferral(::windows::runtime::IInspectable);
impl ContactChangedDeferral {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactChangedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedDeferral;{c5143ae8-1b03-46f8-b694-a523e83cfcb6})");
}
unsafe impl ::windows::runtime::Interface for ContactChangedDeferral {
    type Vtable = IContactChangedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3306437352, 6915, 18168, [182, 148, 165, 35, 232, 60, 252, 182]);
}
impl ::windows::runtime::RuntimeName for ContactChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedDeferral";
}
impl ::std::convert::From<ContactChangedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: ContactChangedDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactChangedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &ContactChangedDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactChangedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactChangedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactChangedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: ContactChangedDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactChangedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &ContactChangedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactChangedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactChangedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactChangedDeferral {}
unsafe impl ::std::marker::Sync for ContactChangedDeferral {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactChangedEventArgs(::windows::runtime::IInspectable);
impl ContactChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<ContactChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactChangedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactChangedEventArgs;{525e7fd1-73f3-4b7d-a918-580be4366121})");
}
unsafe impl ::windows::runtime::Interface for ContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1381924817, 29683, 19325, [169, 24, 88, 11, 228, 54, 97, 33]);
}
impl ::windows::runtime::RuntimeName for ContactChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactChangedEventArgs";
}
impl ::std::convert::From<ContactChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactChangedEventArgs {}
unsafe impl ::std::marker::Sync for ContactChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactConnectedServiceAccount(::windows::runtime::IInspectable);
impl ContactConnectedServiceAccount {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactConnectedServiceAccount, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetServiceName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactConnectedServiceAccount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount;{f6f83553-aa27-4731-8e4a-3dec5ce9eec9})");
}
unsafe impl ::windows::runtime::Interface for ContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4143461715, 43559, 18225, [142, 74, 61, 236, 92, 233, 238, 201]);
}
impl ::windows::runtime::RuntimeName for ContactConnectedServiceAccount {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactConnectedServiceAccount";
}
impl ::std::convert::From<ContactConnectedServiceAccount> for ::windows::runtime::IUnknown {
    fn from(value: ContactConnectedServiceAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactConnectedServiceAccount> for ::windows::runtime::IUnknown {
    fn from(value: &ContactConnectedServiceAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactConnectedServiceAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactConnectedServiceAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactConnectedServiceAccount> for ::windows::runtime::IInspectable {
    fn from(value: ContactConnectedServiceAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactConnectedServiceAccount> for ::windows::runtime::IInspectable {
    fn from(value: &ContactConnectedServiceAccount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactConnectedServiceAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactConnectedServiceAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactConnectedServiceAccount {}
unsafe impl ::std::marker::Sync for ContactConnectedServiceAccount {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactDate(::windows::runtime::IInspectable);
impl ContactDate {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactDate, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Day(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetDay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Month(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMonth<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<u32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Year(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetYear<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ContactDateKind> {
        let this = self;
        unsafe {
            let mut result__: ContactDateKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactDateKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetKind(&self, value: ContactDateKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactDate {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactDate;{fe98ae66-b205-4934-9174-0ff2b0565707})");
}
unsafe impl ::windows::runtime::Interface for ContactDate {
    type Vtable = IContactDate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4271418982, 45573, 18740, [145, 116, 15, 242, 176, 86, 87, 7]);
}
impl ::windows::runtime::RuntimeName for ContactDate {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactDate";
}
impl ::std::convert::From<ContactDate> for ::windows::runtime::IUnknown {
    fn from(value: ContactDate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactDate> for ::windows::runtime::IUnknown {
    fn from(value: &ContactDate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactDate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactDate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactDate> for ::windows::runtime::IInspectable {
    fn from(value: ContactDate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactDate> for ::windows::runtime::IInspectable {
    fn from(value: &ContactDate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactDate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactDate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactDate {}
unsafe impl ::std::marker::Sync for ContactDate {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactDateKind(pub i32);
impl ContactDateKind {
    pub const Birthday: ContactDateKind = ContactDateKind(0i32);
    pub const Anniversary: ContactDateKind = ContactDateKind(1i32);
    pub const Other: ContactDateKind = ContactDateKind(2i32);
}
impl ::std::convert::From<i32> for ContactDateKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactDateKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactDateKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactDateKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactDateKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactEmail(::windows::runtime::IInspectable);
impl ContactEmail {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactEmail, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ContactEmailKind> {
        let this = self;
        unsafe {
            let mut result__: ContactEmailKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactEmailKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetKind(&self, value: ContactEmailKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactEmail {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactEmail;{90a219a9-e3d3-4d63-993b-05b9a5393abf})");
}
unsafe impl ::windows::runtime::Interface for ContactEmail {
    type Vtable = IContactEmail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426542505, 58323, 19811, [153, 59, 5, 185, 165, 57, 58, 191]);
}
impl ::windows::runtime::RuntimeName for ContactEmail {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactEmail";
}
impl ::std::convert::From<ContactEmail> for ::windows::runtime::IUnknown {
    fn from(value: ContactEmail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactEmail> for ::windows::runtime::IUnknown {
    fn from(value: &ContactEmail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactEmail> for ::windows::runtime::IInspectable {
    fn from(value: ContactEmail) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactEmail> for ::windows::runtime::IInspectable {
    fn from(value: &ContactEmail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactEmail {}
unsafe impl ::std::marker::Sync for ContactEmail {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactEmailKind(pub i32);
impl ContactEmailKind {
    pub const Personal: ContactEmailKind = ContactEmailKind(0i32);
    pub const Work: ContactEmailKind = ContactEmailKind(1i32);
    pub const Other: ContactEmailKind = ContactEmailKind(2i32);
}
impl ::std::convert::From<i32> for ContactEmailKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactEmailKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactEmailKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactEmailKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactEmailKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactField(::windows::runtime::IInspectable);
impl ContactField {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__: ContactFieldType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Category(&self) -> ::windows::runtime::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__: ContactFieldCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldCategory>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0, r#type: ContactFieldType) -> ::windows::runtime::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, &mut result__).from_abi::<ContactField>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Custom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0, value: Param1, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        Self::IContactFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        })
    }
    pub fn IContactFieldFactory<R, F: FnOnce(&IContactFieldFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactField, IContactFieldFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactField {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactField;{b176486a-d293-492c-a058-db575b3e3c0f})");
}
unsafe impl ::windows::runtime::Interface for ContactField {
    type Vtable = IContactField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2977319018, 53907, 18732, [160, 88, 219, 87, 91, 62, 60, 15]);
}
impl ::windows::runtime::RuntimeName for ContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactField";
}
impl ::std::convert::From<ContactField> for ::windows::runtime::IUnknown {
    fn from(value: ContactField) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactField> for ::windows::runtime::IUnknown {
    fn from(value: &ContactField) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactField> for ::windows::runtime::IInspectable {
    fn from(value: ContactField) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactField> for ::windows::runtime::IInspectable {
    fn from(value: &ContactField) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ContactField> for IContactField {
    fn from(value: ContactField) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactField> for IContactField {
    fn from(value: &ContactField) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContactField>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for &ContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContactField>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for ContactField {}
unsafe impl ::std::marker::Sync for ContactField {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactFieldCategory(pub i32);
impl ContactFieldCategory {
    pub const None: ContactFieldCategory = ContactFieldCategory(0i32);
    pub const Home: ContactFieldCategory = ContactFieldCategory(1i32);
    pub const Work: ContactFieldCategory = ContactFieldCategory(2i32);
    pub const Mobile: ContactFieldCategory = ContactFieldCategory(3i32);
    pub const Other: ContactFieldCategory = ContactFieldCategory(4i32);
}
impl ::std::convert::From<i32> for ContactFieldCategory {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactFieldCategory {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactFieldCategory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldCategory;i4)");
}
impl ::windows::runtime::DefaultType for ContactFieldCategory {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactFieldFactory(::windows::runtime::IInspectable);
impl ContactFieldFactory {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactFieldFactory, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, r#type: ContactFieldType) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, &mut result__).from_abi::<ContactField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Custom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = &::windows::runtime::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), username.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = &::windows::runtime::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), username.into_param().abi(), category, &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateInstantMessage_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, username: Param0, category: ContactFieldCategory, service: Param2, displaytext: Param3, verb: Param4) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = &::windows::runtime::Interface::cast::<IContactInstantMessageFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), username.into_param().abi(), category, service.into_param().abi(), displaytext.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, unstructuredaddress: Param0) -> ::windows::runtime::Result<ContactLocationField> {
        let this = &::windows::runtime::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, unstructuredaddress: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactLocationField> {
        let this = &::windows::runtime::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        unstructuredaddress: Param0,
        category: ContactFieldCategory,
        street: Param2,
        city: Param3,
        region: Param4,
        country: Param5,
        postalcode: Param6,
    ) -> ::windows::runtime::Result<ContactLocationField> {
        let this = &::windows::runtime::Interface::cast::<IContactLocationFieldFactory>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, street.into_param().abi(), city.into_param().abi(), region.into_param().abi(), country.into_param().abi(), postalcode.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactFieldFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactFieldFactory;{85e2913f-0e4a-4a3e-8994-406ae7ed646e})");
}
unsafe impl ::windows::runtime::Interface for ContactFieldFactory {
    type Vtable = IContactFieldFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2246218047, 3658, 19006, [137, 148, 64, 106, 231, 237, 100, 110]);
}
impl ::windows::runtime::RuntimeName for ContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactFieldFactory";
}
impl ::std::convert::From<ContactFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: ContactFieldFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: &ContactFieldFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: ContactFieldFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: &ContactFieldFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ContactFieldFactory> for IContactFieldFactory {
    fn from(value: ContactFieldFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactFieldFactory> for IContactFieldFactory {
    fn from(value: &ContactFieldFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactFieldFactory> for ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactFieldFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContactFieldFactory>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactFieldFactory> for &ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactFieldFactory> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IContactFieldFactory>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<ContactFieldFactory> for IContactInstantMessageFieldFactory {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactFieldFactory) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContactFieldFactory> for IContactInstantMessageFieldFactory {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactInstantMessageFieldFactory> for ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactInstantMessageFieldFactory> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactInstantMessageFieldFactory> for &ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactInstantMessageFieldFactory> {
        ::std::convert::TryInto::<IContactInstantMessageFieldFactory>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<ContactFieldFactory> for IContactLocationFieldFactory {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactFieldFactory) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContactFieldFactory> for IContactLocationFieldFactory {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactFieldFactory) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactLocationFieldFactory> for ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactLocationFieldFactory> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactLocationFieldFactory> for &ContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactLocationFieldFactory> {
        ::std::convert::TryInto::<IContactLocationFieldFactory>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContactFieldFactory {}
unsafe impl ::std::marker::Sync for ContactFieldFactory {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactFieldType(pub i32);
impl ContactFieldType {
    pub const Email: ContactFieldType = ContactFieldType(0i32);
    pub const PhoneNumber: ContactFieldType = ContactFieldType(1i32);
    pub const Location: ContactFieldType = ContactFieldType(2i32);
    pub const InstantMessage: ContactFieldType = ContactFieldType(3i32);
    pub const Custom: ContactFieldType = ContactFieldType(4i32);
    pub const ConnectedServiceAccount: ContactFieldType = ContactFieldType(5i32);
    pub const ImportantDate: ContactFieldType = ContactFieldType(6i32);
    pub const Address: ContactFieldType = ContactFieldType(7i32);
    pub const SignificantOther: ContactFieldType = ContactFieldType(8i32);
    pub const Notes: ContactFieldType = ContactFieldType(9i32);
    pub const Website: ContactFieldType = ContactFieldType(10i32);
    pub const JobInfo: ContactFieldType = ContactFieldType(11i32);
}
impl ::std::convert::From<i32> for ContactFieldType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactFieldType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactFieldType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactFieldType;i4)");
}
impl ::windows::runtime::DefaultType for ContactFieldType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactGroup(::windows::runtime::IInspectable);
impl ContactGroup {}
unsafe impl ::windows::runtime::RuntimeType for ContactGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactGroup;{59bdeb01-9e9a-475d-bfe5-a37b806d852c})");
}
unsafe impl ::windows::runtime::Interface for ContactGroup {
    type Vtable = IContactGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1505618689, 40602, 18269, [191, 229, 163, 123, 128, 109, 133, 44]);
}
impl ::windows::runtime::RuntimeName for ContactGroup {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactGroup";
}
impl ::std::convert::From<ContactGroup> for ::windows::runtime::IUnknown {
    fn from(value: ContactGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactGroup> for ::windows::runtime::IUnknown {
    fn from(value: &ContactGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactGroup> for ::windows::runtime::IInspectable {
    fn from(value: ContactGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactGroup> for ::windows::runtime::IInspectable {
    fn from(value: &ContactGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactGroup {}
unsafe impl ::std::marker::Sync for ContactGroup {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactInformation(::windows::runtime::IInspectable);
impl ContactInformation {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn GetThumbnailAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Emails(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactField>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn PhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactField>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn Locations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactLocationField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactLocationField>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn InstantMessages(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactInstantMessageField>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn CustomFields(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactField>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn QueryCustomFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, customname: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactField>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), customname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactField>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInformation;{275eb6d4-6a2e-4278-a914-e460d5f088f6})");
}
unsafe impl ::windows::runtime::Interface for ContactInformation {
    type Vtable = IContactInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(660518612, 27182, 17016, [169, 20, 228, 96, 213, 240, 136, 246]);
}
impl ::windows::runtime::RuntimeName for ContactInformation {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInformation";
}
impl ::std::convert::From<ContactInformation> for ::windows::runtime::IUnknown {
    fn from(value: ContactInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactInformation> for ::windows::runtime::IUnknown {
    fn from(value: &ContactInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactInformation> for ::windows::runtime::IInspectable {
    fn from(value: ContactInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactInformation> for ::windows::runtime::IInspectable {
    fn from(value: &ContactInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactInstantMessageField(::windows::runtime::IInspectable);
impl ContactInstantMessageField {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Service(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DisplayText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn LaunchUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<ContactFieldType> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ContactFieldType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Category(&self) -> ::windows::runtime::Result<ContactFieldCategory> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ContactFieldCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldCategory>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(username: Param0) -> ::windows::runtime::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), username.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(username: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), username.into_param().abi(), category, &mut result__).from_abi::<ContactInstantMessageField>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateInstantMessage_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(username: Param0, category: ContactFieldCategory, service: Param2, displaytext: Param3, verb: Param4) -> ::windows::runtime::Result<ContactInstantMessageField> {
        Self::IContactInstantMessageFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), username.into_param().abi(), category, service.into_param().abi(), displaytext.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        })
    }
    pub fn IContactInstantMessageFieldFactory<R, F: FnOnce(&IContactInstantMessageFieldFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactInstantMessageField, IContactInstantMessageFieldFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactInstantMessageField {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactInstantMessageField;{cce33b37-0d85-41fa-b43d-da599c3eb009})");
}
unsafe impl ::windows::runtime::Interface for ContactInstantMessageField {
    type Vtable = IContactInstantMessageField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437443895, 3461, 16890, [180, 61, 218, 89, 156, 62, 176, 9]);
}
impl ::windows::runtime::RuntimeName for ContactInstantMessageField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactInstantMessageField";
}
impl ::std::convert::From<ContactInstantMessageField> for ::windows::runtime::IUnknown {
    fn from(value: ContactInstantMessageField) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactInstantMessageField> for ::windows::runtime::IUnknown {
    fn from(value: &ContactInstantMessageField) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactInstantMessageField> for ::windows::runtime::IInspectable {
    fn from(value: ContactInstantMessageField) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactInstantMessageField> for ::windows::runtime::IInspectable {
    fn from(value: &ContactInstantMessageField) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ContactInstantMessageField> for IContactField {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactInstantMessageField) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContactInstantMessageField> for IContactField {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactInstantMessageField) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for &ContactInstantMessageField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::std::convert::TryInto::<IContactField>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContactInstantMessageField {}
unsafe impl ::std::marker::Sync for ContactInstantMessageField {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactJobInfo(::windows::runtime::IInspectable);
impl ContactJobInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactJobInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CompanyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCompanyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CompanyYomiName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCompanyYomiName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Department(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDepartment<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Manager(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetManager<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Office(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetOffice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CompanyAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCompanyAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactJobInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactJobInfo;{6d117b4c-ce50-4b43-9e69-b18258ea5315})");
}
unsafe impl ::windows::runtime::Interface for ContactJobInfo {
    type Vtable = IContactJobInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1829862220, 52816, 19267, [158, 105, 177, 130, 88, 234, 83, 21]);
}
impl ::windows::runtime::RuntimeName for ContactJobInfo {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactJobInfo";
}
impl ::std::convert::From<ContactJobInfo> for ::windows::runtime::IUnknown {
    fn from(value: ContactJobInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactJobInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ContactJobInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactJobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactJobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactJobInfo> for ::windows::runtime::IInspectable {
    fn from(value: ContactJobInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactJobInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ContactJobInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactJobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactJobInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactJobInfo {}
unsafe impl ::std::marker::Sync for ContactJobInfo {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct ContactLaunchActionVerbs {}
impl ContactLaunchActionVerbs {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Call() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Message() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Map() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Post() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn VideoCall() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IContactLaunchActionVerbsStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IContactLaunchActionVerbsStatics<R, F: FnOnce(&IContactLaunchActionVerbsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactLaunchActionVerbs, IContactLaunchActionVerbsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ContactLaunchActionVerbs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLaunchActionVerbs";
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactList(::windows::runtime::IInspectable);
impl ContactList {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SourceDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsHidden(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetIsHidden(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::runtime::Result<ContactListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: ContactListOtherAppReadAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetOtherAppReadAccess(&self, value: ContactListOtherAppReadAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn OtherAppWriteAccess(&self) -> ::windows::runtime::Result<ContactListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: ContactListOtherAppWriteAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListOtherAppWriteAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetOtherAppWriteAccess(&self, value: ContactListOtherAppWriteAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ChangeTracker(&self) -> ::windows::runtime::Result<ContactChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactChangeTracker>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SyncManager(&self) -> ::windows::runtime::Result<ContactListSyncManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncManager>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SupportsServerSearch(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn ContactChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ContactList, ContactChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RemoveContactChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetContactFromRemoteIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, remoteid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), remoteid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetMeContactAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetContactReader(&self) -> ::windows::runtime::Result<ContactReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetContactReaderWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ContactQueryOptions>>(&self, options: Param0) -> ::windows::runtime::Result<ContactReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<ContactReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SaveContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn DeleteContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), contactid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSupportsServerSearch(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactList2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SyncConstraints(&self) -> ::windows::runtime::Result<ContactListSyncConstraints> {
        let this = &::windows::runtime::Interface::cast::<IContactList2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncConstraints>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn LimitedWriteOperations(&self) -> ::windows::runtime::Result<ContactListLimitedWriteOperations> {
        let this = &::windows::runtime::Interface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListLimitedWriteOperations>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetChangeTracker<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, identity: Param0) -> ::windows::runtime::Result<ContactChangeTracker> {
        let this = &::windows::runtime::Interface::cast::<IContactList3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<ContactChangeTracker>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactList {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactList;{16ddec75-392c-4845-9dfb-51a3e7ef3e42})");
}
unsafe impl ::windows::runtime::Interface for ContactList {
    type Vtable = IContactList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(383642741, 14636, 18501, [157, 251, 81, 163, 231, 239, 62, 66]);
}
impl ::windows::runtime::RuntimeName for ContactList {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactList";
}
impl ::std::convert::From<ContactList> for ::windows::runtime::IUnknown {
    fn from(value: ContactList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactList> for ::windows::runtime::IUnknown {
    fn from(value: &ContactList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactList> for ::windows::runtime::IInspectable {
    fn from(value: ContactList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactList> for ::windows::runtime::IInspectable {
    fn from(value: &ContactList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactList {}
unsafe impl ::std::marker::Sync for ContactList {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListLimitedWriteOperations(::windows::runtime::IInspectable);
impl ContactListLimitedWriteOperations {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn TryCreateOrUpdateContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn TryDeleteContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contactid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListLimitedWriteOperations {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations;{e19813da-4a0b-44b8-9a1f-a0f3d218175f})");
}
unsafe impl ::windows::runtime::Interface for ContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3784840154, 18955, 17592, [154, 31, 160, 243, 210, 24, 23, 95]);
}
impl ::windows::runtime::RuntimeName for ContactListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListLimitedWriteOperations";
}
impl ::std::convert::From<ContactListLimitedWriteOperations> for ::windows::runtime::IUnknown {
    fn from(value: ContactListLimitedWriteOperations) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactListLimitedWriteOperations> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListLimitedWriteOperations) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListLimitedWriteOperations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactListLimitedWriteOperations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactListLimitedWriteOperations> for ::windows::runtime::IInspectable {
    fn from(value: ContactListLimitedWriteOperations) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListLimitedWriteOperations> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListLimitedWriteOperations) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListLimitedWriteOperations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListLimitedWriteOperations {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListLimitedWriteOperations {}
unsafe impl ::std::marker::Sync for ContactListLimitedWriteOperations {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactListOtherAppReadAccess(pub i32);
impl ContactListOtherAppReadAccess {
    pub const SystemOnly: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(0i32);
    pub const Limited: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(1i32);
    pub const Full: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(2i32);
    pub const None: ContactListOtherAppReadAccess = ContactListOtherAppReadAccess(3i32);
}
impl ::std::convert::From<i32> for ContactListOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactListOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactListOtherAppReadAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppReadAccess;i4)");
}
impl ::windows::runtime::DefaultType for ContactListOtherAppReadAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactListOtherAppWriteAccess(pub i32);
impl ContactListOtherAppWriteAccess {
    pub const None: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(0i32);
    pub const SystemOnly: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(1i32);
    pub const Limited: ContactListOtherAppWriteAccess = ContactListOtherAppWriteAccess(2i32);
}
impl ::std::convert::From<i32> for ContactListOtherAppWriteAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactListOtherAppWriteAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactListOtherAppWriteAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListOtherAppWriteAccess;i4)");
}
impl ::windows::runtime::DefaultType for ContactListOtherAppWriteAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListSyncConstraints(::windows::runtime::IInspectable);
impl ContactListSyncConstraints {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CanSyncDescriptions(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCanSyncDescriptions(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxHomePhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxHomePhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxMobilePhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxMobilePhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxWorkPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxWorkPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxOtherPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxOtherPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxPagerPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxPagerPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxBusinessFaxPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxBusinessFaxPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxHomeFaxPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxHomeFaxPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxCompanyPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxCompanyPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxAssistantPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxAssistantPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxRadioPhoneNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxRadioPhoneNumbers<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxPersonalEmailAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxPersonalEmailAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxWorkEmailAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxWorkEmailAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxOtherEmailAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxOtherEmailAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxHomeAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxHomeAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxWorkAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxWorkAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxOtherAddresses(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxOtherAddresses<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxBirthdayDates(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxBirthdayDates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxAnniversaryDates(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxAnniversaryDates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxOtherDates(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxOtherDates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxOtherRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxOtherRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxSpouseRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxSpouseRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxPartnerRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxPartnerRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxSiblingRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxSiblingRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxParentRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxParentRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxChildRelationships(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxChildRelationships<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxJobInfo(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxJobInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn MaxWebsites(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetMaxWebsites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListSyncConstraints {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncConstraints;{b2b0bf01-3062-4e2e-969d-018d1987f314})");
}
unsafe impl ::windows::runtime::Interface for ContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2997927681, 12386, 20014, [150, 157, 1, 141, 25, 135, 243, 20]);
}
impl ::windows::runtime::RuntimeName for ContactListSyncConstraints {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncConstraints";
}
impl ::std::convert::From<ContactListSyncConstraints> for ::windows::runtime::IUnknown {
    fn from(value: ContactListSyncConstraints) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactListSyncConstraints> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListSyncConstraints) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListSyncConstraints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactListSyncConstraints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactListSyncConstraints> for ::windows::runtime::IInspectable {
    fn from(value: ContactListSyncConstraints) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListSyncConstraints> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListSyncConstraints) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListSyncConstraints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListSyncConstraints {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListSyncConstraints {}
unsafe impl ::std::marker::Sync for ContactListSyncConstraints {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactListSyncManager(::windows::runtime::IInspectable);
impl ContactListSyncManager {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ContactListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: ContactListSyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactListSyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SyncAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SyncStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ContactListSyncManager, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetStatus(&self, value: ContactListSyncStatus) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactListSyncManager2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactListSyncManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactListSyncManager;{146e83be-7925-4acc-9de5-21ddd06f8674})");
}
unsafe impl ::windows::runtime::Interface for ContactListSyncManager {
    type Vtable = IContactListSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342787006, 31013, 19148, [157, 229, 33, 221, 208, 111, 134, 116]);
}
impl ::windows::runtime::RuntimeName for ContactListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactListSyncManager";
}
impl ::std::convert::From<ContactListSyncManager> for ::windows::runtime::IUnknown {
    fn from(value: ContactListSyncManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactListSyncManager> for ::windows::runtime::IUnknown {
    fn from(value: &ContactListSyncManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactListSyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactListSyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactListSyncManager> for ::windows::runtime::IInspectable {
    fn from(value: ContactListSyncManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactListSyncManager> for ::windows::runtime::IInspectable {
    fn from(value: &ContactListSyncManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactListSyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactListSyncManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactListSyncManager {}
unsafe impl ::std::marker::Sync for ContactListSyncManager {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactListSyncStatus(pub i32);
impl ContactListSyncStatus {
    pub const Idle: ContactListSyncStatus = ContactListSyncStatus(0i32);
    pub const Syncing: ContactListSyncStatus = ContactListSyncStatus(1i32);
    pub const UpToDate: ContactListSyncStatus = ContactListSyncStatus(2i32);
    pub const AuthenticationError: ContactListSyncStatus = ContactListSyncStatus(3i32);
    pub const PolicyError: ContactListSyncStatus = ContactListSyncStatus(4i32);
    pub const UnknownError: ContactListSyncStatus = ContactListSyncStatus(5i32);
    pub const ManualAccountRemovalRequired: ContactListSyncStatus = ContactListSyncStatus(6i32);
}
impl ::std::convert::From<i32> for ContactListSyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactListSyncStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactListSyncStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactListSyncStatus;i4)");
}
impl ::windows::runtime::DefaultType for ContactListSyncStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactLocationField(::windows::runtime::IInspectable);
impl ContactLocationField {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn UnstructuredAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Street(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn City(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Region(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Country(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn PostalCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<ContactFieldType> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ContactFieldType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Category(&self) -> ::windows::runtime::Result<ContactFieldCategory> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ContactFieldCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldCategory>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactField>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(unstructuredaddress: Param0) -> ::windows::runtime::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(unstructuredaddress: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, &mut result__).from_abi::<ContactLocationField>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        unstructuredaddress: Param0,
        category: ContactFieldCategory,
        street: Param2,
        city: Param3,
        region: Param4,
        country: Param5,
        postalcode: Param6,
    ) -> ::windows::runtime::Result<ContactLocationField> {
        Self::IContactLocationFieldFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, street.into_param().abi(), city.into_param().abi(), region.into_param().abi(), country.into_param().abi(), postalcode.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        })
    }
    pub fn IContactLocationFieldFactory<R, F: FnOnce(&IContactLocationFieldFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactLocationField, IContactLocationFieldFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactLocationField {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactLocationField;{9ec00f82-ab6e-4b36-89e3-b23bc0a1dacc})");
}
unsafe impl ::windows::runtime::Interface for ContactLocationField {
    type Vtable = IContactLocationField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2663387010, 43886, 19254, [137, 227, 178, 59, 192, 161, 218, 204]);
}
impl ::windows::runtime::RuntimeName for ContactLocationField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactLocationField";
}
impl ::std::convert::From<ContactLocationField> for ::windows::runtime::IUnknown {
    fn from(value: ContactLocationField) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactLocationField> for ::windows::runtime::IUnknown {
    fn from(value: &ContactLocationField) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactLocationField> for ::windows::runtime::IInspectable {
    fn from(value: ContactLocationField) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactLocationField> for ::windows::runtime::IInspectable {
    fn from(value: &ContactLocationField) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ContactLocationField> for IContactField {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ContactLocationField) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ContactLocationField> for IContactField {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ContactLocationField) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IContactField> for &ContactLocationField {
    fn into_param(self) -> ::windows::runtime::Param<'a, IContactField> {
        ::std::convert::TryInto::<IContactField>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ContactLocationField {}
unsafe impl ::std::marker::Sync for ContactLocationField {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct ContactManager {}
impl ContactManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn ShowContactCard<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(contact: Param0, selection: Param1) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), selection.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI_Popups`*"]
    pub fn ShowContactCardWithPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(contact: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contact.into_param().abi(), selection.into_param().abi(), preferredplacement).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI_Popups`*"]
    pub fn ShowDelayLoadedContactCard<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(contact: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::Result<ContactCardDelayedDataLoader> {
        Self::IContactManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), contact.into_param().abi(), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<ContactCardDelayedDataLoader>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestStoreAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactStore>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertContactToVCardAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertContactToVCardAsyncWithMaxBytes<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(contact: Param0, maxbytes: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contact.into_param().abi(), maxbytes, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertVCardToContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(vcard: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), vcard.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestStoreAsyncWithAccessType(accesstype: ContactStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactStore>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestAnnotationStoreAsync(accesstype: ContactAnnotationStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsShowContactCardSupported() -> ::windows::runtime::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI_Popups`*"]
    pub fn ShowContactCardWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::runtime::IntoParam<'a, ContactCardOptions>>(contact: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: Param3) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), contact.into_param().abi(), selection.into_param().abi(), preferredplacement, contactcardoptions.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsShowDelayLoadedContactCardSupported() -> ::windows::runtime::Result<bool> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI_Popups`*"]
    pub fn ShowDelayLoadedContactCardWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>, Param3: ::windows::runtime::IntoParam<'a, ContactCardOptions>>(contact: Param0, selection: Param1, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: Param3) -> ::windows::runtime::Result<ContactCardDelayedDataLoader> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), contact.into_param().abi(), selection.into_param().abi(), preferredplacement, contactcardoptions.into_param().abi(), &mut result__).from_abi::<ContactCardDelayedDataLoader>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ShowFullContactCard<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, FullContactCardOptions>>(contact: Param0, fullcontactcardoptions: Param1) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), contact.into_param().abi(), fullcontactcardoptions.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SystemDisplayNameOrder() -> ::windows::runtime::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ContactNameOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactNameOrder>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSystemDisplayNameOrder(value: ContactNameOrder) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SystemSortOrder() -> ::windows::runtime::Result<ContactNameOrder> {
        Self::IContactManagerStatics3(|this| unsafe {
            let mut result__: ContactNameOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactNameOrder>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSystemSortOrder(value: ContactNameOrder) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics3(|this| unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<ContactManagerForUser> {
        Self::IContactManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<ContactManagerForUser>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn IsShowFullContactCardSupportedAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IncludeMiddleNameInSystemDisplayAndSort() -> ::windows::runtime::Result<bool> {
        Self::IContactManagerStatics5(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetIncludeMiddleNameInSystemDisplayAndSort(value: bool) -> ::windows::runtime::Result<()> {
        Self::IContactManagerStatics5(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() })
    }
    pub fn IContactManagerStatics<R, F: FnOnce(&IContactManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactManager, IContactManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactManagerStatics2<R, F: FnOnce(&IContactManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactManager, IContactManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactManagerStatics3<R, F: FnOnce(&IContactManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactManager, IContactManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactManagerStatics4<R, F: FnOnce(&IContactManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactManager, IContactManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactManagerStatics5<R, F: FnOnce(&IContactManagerStatics5) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactManager, IContactManagerStatics5> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManager";
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactManagerForUser(::windows::runtime::IInspectable);
impl ContactManagerForUser {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertContactToVCardAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertContactToVCardAsyncWithMaxBytes<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0, maxbytes: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contact.into_param().abi(), maxbytes, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Storage_Streams`*"]
    pub fn ConvertVCardToContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, vcard: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), vcard.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestStoreAsync(&self, accesstype: ContactStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactStore>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestAnnotationStoreAsync(&self, accesstype: ContactAnnotationStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactAnnotationStore>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SystemDisplayNameOrder(&self) -> ::windows::runtime::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__: ContactNameOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactNameOrder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSystemDisplayNameOrder(&self, value: ContactNameOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SystemSortOrder(&self) -> ::windows::runtime::Result<ContactNameOrder> {
        let this = self;
        unsafe {
            let mut result__: ContactNameOrder = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactNameOrder>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSystemSortOrder(&self, value: ContactNameOrder) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ShowFullContactCard<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>, Param1: ::windows::runtime::IntoParam<'a, FullContactCardOptions>>(&self, contact: Param0, fullcontactcardoptions: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactManagerForUser2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), contact.into_param().abi(), fullcontactcardoptions.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactManagerForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactManagerForUser;{b74bba57-1076-4bef-aef3-54686d18387d})");
}
unsafe impl ::windows::runtime::Interface for ContactManagerForUser {
    type Vtable = IContactManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3075193431, 4214, 19439, [174, 243, 84, 104, 109, 24, 56, 125]);
}
impl ::windows::runtime::RuntimeName for ContactManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactManagerForUser";
}
impl ::std::convert::From<ContactManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: ContactManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &ContactManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: ContactManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &ContactManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactManagerForUser {}
unsafe impl ::std::marker::Sync for ContactManagerForUser {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactMatchReason(::windows::runtime::IInspectable);
impl ContactMatchReason {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Field(&self) -> ::windows::runtime::Result<ContactMatchReasonKind> {
        let this = self;
        unsafe {
            let mut result__: ContactMatchReasonKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactMatchReasonKind>(result__)
        }
    }
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Data_Text`, `Foundation_Collections`*"]
    pub fn Segments(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactMatchReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactMatchReason;{bc922504-e7d8-413e-95f4-b75c54c74077})");
}
unsafe impl ::windows::runtime::Interface for ContactMatchReason {
    type Vtable = IContactMatchReason_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3163694340, 59352, 16702, [149, 244, 183, 92, 84, 199, 64, 119]);
}
impl ::windows::runtime::RuntimeName for ContactMatchReason {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactMatchReason";
}
impl ::std::convert::From<ContactMatchReason> for ::windows::runtime::IUnknown {
    fn from(value: ContactMatchReason) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactMatchReason> for ::windows::runtime::IUnknown {
    fn from(value: &ContactMatchReason) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactMatchReason {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactMatchReason {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactMatchReason> for ::windows::runtime::IInspectable {
    fn from(value: ContactMatchReason) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactMatchReason> for ::windows::runtime::IInspectable {
    fn from(value: &ContactMatchReason) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactMatchReason {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactMatchReason {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactMatchReason {}
unsafe impl ::std::marker::Sync for ContactMatchReason {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactMatchReasonKind(pub i32);
impl ContactMatchReasonKind {
    pub const Name: ContactMatchReasonKind = ContactMatchReasonKind(0i32);
    pub const EmailAddress: ContactMatchReasonKind = ContactMatchReasonKind(1i32);
    pub const PhoneNumber: ContactMatchReasonKind = ContactMatchReasonKind(2i32);
    pub const JobInfo: ContactMatchReasonKind = ContactMatchReasonKind(3i32);
    pub const YomiName: ContactMatchReasonKind = ContactMatchReasonKind(4i32);
    pub const Other: ContactMatchReasonKind = ContactMatchReasonKind(5i32);
}
impl ::std::convert::From<i32> for ContactMatchReasonKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactMatchReasonKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactMatchReasonKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactMatchReasonKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactMatchReasonKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactNameOrder(pub i32);
impl ContactNameOrder {
    pub const FirstNameLastName: ContactNameOrder = ContactNameOrder(0i32);
    pub const LastNameFirstName: ContactNameOrder = ContactNameOrder(1i32);
}
impl ::std::convert::From<i32> for ContactNameOrder {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactNameOrder {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactNameOrder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactNameOrder;i4)");
}
impl ::windows::runtime::DefaultType for ContactNameOrder {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactPanel(::windows::runtime::IInspectable);
impl ContactPanel {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ClosePanel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI`*"]
    pub fn HeaderColor(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::UI::Color>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `UI`*"]
    pub fn SetHeaderColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::UI::Color>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn LaunchFullAppRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelLaunchFullAppRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RemoveLaunchFullAppRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Closing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ContactPanel, ContactPanelClosingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RemoveClosing<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPanel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanel;{41bf1265-d2ee-4b97-a80a-7d8d64cca6f5})");
}
unsafe impl ::windows::runtime::Interface for ContactPanel {
    type Vtable = IContactPanel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1103041125, 53998, 19351, [168, 10, 125, 141, 100, 204, 166, 245]);
}
impl ::windows::runtime::RuntimeName for ContactPanel {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanel";
}
impl ::std::convert::From<ContactPanel> for ::windows::runtime::IUnknown {
    fn from(value: ContactPanel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactPanel> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPanel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactPanel> for ::windows::runtime::IInspectable {
    fn from(value: ContactPanel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactPanel> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactPanel {}
unsafe impl ::std::marker::Sync for ContactPanel {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactPanelClosingEventArgs(::windows::runtime::IInspectable);
impl ContactPanelClosingEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPanelClosingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs;{222174d3-cf4b-46d7-b739-6edc16110bfb})");
}
unsafe impl ::windows::runtime::Interface for ContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(572617939, 53067, 18135, [183, 57, 110, 220, 22, 17, 11, 251]);
}
impl ::windows::runtime::RuntimeName for ContactPanelClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelClosingEventArgs";
}
impl ::std::convert::From<ContactPanelClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactPanelClosingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactPanelClosingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPanelClosingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPanelClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactPanelClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactPanelClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactPanelClosingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactPanelClosingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPanelClosingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPanelClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPanelClosingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactPanelClosingEventArgs {}
unsafe impl ::std::marker::Sync for ContactPanelClosingEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactPanelLaunchFullAppRequestedEventArgs(::windows::runtime::IInspectable);
impl ContactPanelLaunchFullAppRequestedEventArgs {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPanelLaunchFullAppRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs;{88d61c0e-23b4-4be8-8afc-072c25a4190d})");
}
unsafe impl ::windows::runtime::Interface for ContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2295733262, 9140, 19432, [138, 252, 7, 44, 37, 164, 25, 13]);
}
impl ::windows::runtime::RuntimeName for ContactPanelLaunchFullAppRequestedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPanelLaunchFullAppRequestedEventArgs";
}
impl ::std::convert::From<ContactPanelLaunchFullAppRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ContactPanelLaunchFullAppRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactPanelLaunchFullAppRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPanelLaunchFullAppRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPanelLaunchFullAppRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactPanelLaunchFullAppRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactPanelLaunchFullAppRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ContactPanelLaunchFullAppRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactPanelLaunchFullAppRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPanelLaunchFullAppRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPanelLaunchFullAppRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPanelLaunchFullAppRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactPanelLaunchFullAppRequestedEventArgs {}
unsafe impl ::std::marker::Sync for ContactPanelLaunchFullAppRequestedEventArgs {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactPhone(::windows::runtime::IInspectable);
impl ContactPhone {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactPhone, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Number(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ContactPhoneKind> {
        let this = self;
        unsafe {
            let mut result__: ContactPhoneKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactPhoneKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetKind(&self, value: ContactPhoneKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPhone {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPhone;{467dab65-2712-4f52-b783-9ea8111c63cd})");
}
unsafe impl ::windows::runtime::Interface for ContactPhone {
    type Vtable = IContactPhone_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1182640997, 10002, 20306, [183, 131, 158, 168, 17, 28, 99, 205]);
}
impl ::windows::runtime::RuntimeName for ContactPhone {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPhone";
}
impl ::std::convert::From<ContactPhone> for ::windows::runtime::IUnknown {
    fn from(value: ContactPhone) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactPhone> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPhone) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPhone {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactPhone {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactPhone> for ::windows::runtime::IInspectable {
    fn from(value: ContactPhone) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactPhone> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPhone) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPhone {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPhone {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactPhone {}
unsafe impl ::std::marker::Sync for ContactPhone {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactPhoneKind(pub i32);
impl ContactPhoneKind {
    pub const Home: ContactPhoneKind = ContactPhoneKind(0i32);
    pub const Mobile: ContactPhoneKind = ContactPhoneKind(1i32);
    pub const Work: ContactPhoneKind = ContactPhoneKind(2i32);
    pub const Other: ContactPhoneKind = ContactPhoneKind(3i32);
    pub const Pager: ContactPhoneKind = ContactPhoneKind(4i32);
    pub const BusinessFax: ContactPhoneKind = ContactPhoneKind(5i32);
    pub const HomeFax: ContactPhoneKind = ContactPhoneKind(6i32);
    pub const Company: ContactPhoneKind = ContactPhoneKind(7i32);
    pub const Assistant: ContactPhoneKind = ContactPhoneKind(8i32);
    pub const Radio: ContactPhoneKind = ContactPhoneKind(9i32);
}
impl ::std::convert::From<i32> for ContactPhoneKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactPhoneKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactPhoneKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactPhoneKind;i4)");
}
impl ::windows::runtime::DefaultType for ContactPhoneKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactPicker(::windows::runtime::IInspectable);
impl ContactPicker {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactPicker, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CommitButtonText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetCommitButtonText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SelectionMode(&self) -> ::windows::runtime::Result<ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: ContactSelectionMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSelectionMode(&self, value: ContactSelectionMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn DesiredFields(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn PickSingleContactAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactInformation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactInformation>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn PickMultipleContactsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactInformation>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<ContactFieldType>> {
        let this = &::windows::runtime::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<ContactFieldType>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn PickContactAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows::runtime::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn PickContactsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>> {
        let this = &::windows::runtime::Interface::cast::<IContactPicker2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<Contact>>>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<IContactPicker3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn CreateForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<ContactPicker> {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<ContactPicker>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn IsSupportedAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IContactPickerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IContactPickerStatics<R, F: FnOnce(&IContactPickerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactPicker, IContactPickerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactPicker {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactPicker;{0e09fd91-42f8-4055-90a0-896f96738936})");
}
unsafe impl ::windows::runtime::Interface for ContactPicker {
    type Vtable = IContactPicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(235535761, 17144, 16469, [144, 160, 137, 111, 150, 115, 137, 54]);
}
impl ::windows::runtime::RuntimeName for ContactPicker {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactPicker";
}
impl ::std::convert::From<ContactPicker> for ::windows::runtime::IUnknown {
    fn from(value: ContactPicker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactPicker> for ::windows::runtime::IUnknown {
    fn from(value: &ContactPicker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactPicker> for ::windows::runtime::IInspectable {
    fn from(value: ContactPicker) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactPicker> for ::windows::runtime::IInspectable {
    fn from(value: &ContactPicker) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactPicker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactQueryDesiredFields(pub u32);
impl ContactQueryDesiredFields {
    pub const None: ContactQueryDesiredFields = ContactQueryDesiredFields(0u32);
    pub const PhoneNumber: ContactQueryDesiredFields = ContactQueryDesiredFields(1u32);
    pub const EmailAddress: ContactQueryDesiredFields = ContactQueryDesiredFields(2u32);
    pub const PostalAddress: ContactQueryDesiredFields = ContactQueryDesiredFields(4u32);
}
impl ::std::convert::From<u32> for ContactQueryDesiredFields {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactQueryDesiredFields {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactQueryDesiredFields {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQueryDesiredFields;u4)");
}
impl ::windows::runtime::DefaultType for ContactQueryDesiredFields {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ContactQueryDesiredFields {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ContactQueryDesiredFields {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ContactQueryDesiredFields {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ContactQueryDesiredFields {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ContactQueryDesiredFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactQueryOptions(::windows::runtime::IInspectable);
impl ContactQueryOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactQueryOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn TextSearch(&self) -> ::windows::runtime::Result<ContactQueryTextSearch> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactQueryTextSearch>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ContactListIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IncludeContactsFromHiddenLists(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetIncludeContactsFromHiddenLists(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DesiredFields(&self) -> ::windows::runtime::Result<ContactQueryDesiredFields> {
        let this = self;
        unsafe {
            let mut result__: ContactQueryDesiredFields = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactQueryDesiredFields>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDesiredFields(&self, value: ContactQueryDesiredFields) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn DesiredOperations(&self) -> ::windows::runtime::Result<ContactAnnotationOperations> {
        let this = self;
        unsafe {
            let mut result__: ContactAnnotationOperations = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactAnnotationOperations>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDesiredOperations(&self, value: ContactAnnotationOperations) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn AnnotationListIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateWithText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0) -> ::windows::runtime::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<ContactQueryOptions>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateWithTextAndFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(text: Param0, fields: ContactQuerySearchFields) -> ::windows::runtime::Result<ContactQueryOptions> {
        Self::IContactQueryOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), text.into_param().abi(), fields, &mut result__).from_abi::<ContactQueryOptions>(result__)
        })
    }
    pub fn IContactQueryOptionsFactory<R, F: FnOnce(&IContactQueryOptionsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactQueryOptions, IContactQueryOptionsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactQueryOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryOptions;{4408cc9e-7d7c-42f0-8ac7-f50733ecdbc1})");
}
unsafe impl ::windows::runtime::Interface for ContactQueryOptions {
    type Vtable = IContactQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1141427358, 32124, 17136, [138, 199, 245, 7, 51, 236, 219, 193]);
}
impl ::windows::runtime::RuntimeName for ContactQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryOptions";
}
impl ::std::convert::From<ContactQueryOptions> for ::windows::runtime::IUnknown {
    fn from(value: ContactQueryOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactQueryOptions> for ::windows::runtime::IUnknown {
    fn from(value: &ContactQueryOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactQueryOptions> for ::windows::runtime::IInspectable {
    fn from(value: ContactQueryOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactQueryOptions> for ::windows::runtime::IInspectable {
    fn from(value: &ContactQueryOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactQueryOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactQueryOptions {}
unsafe impl ::std::marker::Sync for ContactQueryOptions {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactQuerySearchFields(pub u32);
impl ContactQuerySearchFields {
    pub const None: ContactQuerySearchFields = ContactQuerySearchFields(0u32);
    pub const Name: ContactQuerySearchFields = ContactQuerySearchFields(1u32);
    pub const Email: ContactQuerySearchFields = ContactQuerySearchFields(2u32);
    pub const Phone: ContactQuerySearchFields = ContactQuerySearchFields(4u32);
    pub const All: ContactQuerySearchFields = ContactQuerySearchFields(4294967295u32);
}
impl ::std::convert::From<u32> for ContactQuerySearchFields {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactQuerySearchFields {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactQuerySearchFields {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchFields;u4)");
}
impl ::windows::runtime::DefaultType for ContactQuerySearchFields {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ContactQuerySearchFields {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ContactQuerySearchFields {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ContactQuerySearchFields {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ContactQuerySearchFields {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ContactQuerySearchFields {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactQuerySearchScope(pub i32);
impl ContactQuerySearchScope {
    pub const Local: ContactQuerySearchScope = ContactQuerySearchScope(0i32);
    pub const Server: ContactQuerySearchScope = ContactQuerySearchScope(1i32);
}
impl ::std::convert::From<i32> for ContactQuerySearchScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactQuerySearchScope {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactQuerySearchScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactQuerySearchScope;i4)");
}
impl ::windows::runtime::DefaultType for ContactQuerySearchScope {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactQueryTextSearch(::windows::runtime::IInspectable);
impl ContactQueryTextSearch {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Fields(&self) -> ::windows::runtime::Result<ContactQuerySearchFields> {
        let this = self;
        unsafe {
            let mut result__: ContactQuerySearchFields = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactQuerySearchFields>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetFields(&self, value: ContactQuerySearchFields) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SearchScope(&self) -> ::windows::runtime::Result<ContactQuerySearchScope> {
        let this = self;
        unsafe {
            let mut result__: ContactQuerySearchScope = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactQuerySearchScope>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetSearchScope(&self, value: ContactQuerySearchScope) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactQueryTextSearch {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactQueryTextSearch;{f7e3f9cb-a957-439b-a0b7-1c02a1963ff0})");
}
unsafe impl ::windows::runtime::Interface for ContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4158912971, 43351, 17307, [160, 183, 28, 2, 161, 150, 63, 240]);
}
impl ::windows::runtime::RuntimeName for ContactQueryTextSearch {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactQueryTextSearch";
}
impl ::std::convert::From<ContactQueryTextSearch> for ::windows::runtime::IUnknown {
    fn from(value: ContactQueryTextSearch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactQueryTextSearch> for ::windows::runtime::IUnknown {
    fn from(value: &ContactQueryTextSearch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactQueryTextSearch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactQueryTextSearch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactQueryTextSearch> for ::windows::runtime::IInspectable {
    fn from(value: ContactQueryTextSearch) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactQueryTextSearch> for ::windows::runtime::IInspectable {
    fn from(value: &ContactQueryTextSearch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactQueryTextSearch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactQueryTextSearch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactQueryTextSearch {}
unsafe impl ::std::marker::Sync for ContactQueryTextSearch {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactReader(::windows::runtime::IInspectable);
impl ContactReader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactBatch>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactBatch>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn GetMatchingPropertiesWithMatchReason<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ContactMatchReason>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ContactMatchReason>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactReader;{d397e42e-1488-42f2-bf64-253f4884bfed})");
}
unsafe impl ::windows::runtime::Interface for ContactReader {
    type Vtable = IContactReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3549946926, 5256, 17138, [191, 100, 37, 63, 72, 132, 191, 237]);
}
impl ::windows::runtime::RuntimeName for ContactReader {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactReader";
}
impl ::std::convert::From<ContactReader> for ::windows::runtime::IUnknown {
    fn from(value: ContactReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactReader> for ::windows::runtime::IUnknown {
    fn from(value: &ContactReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactReader> for ::windows::runtime::IInspectable {
    fn from(value: ContactReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactReader> for ::windows::runtime::IInspectable {
    fn from(value: &ContactReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactReader {}
unsafe impl ::std::marker::Sync for ContactReader {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactRelationship(pub i32);
impl ContactRelationship {
    pub const Other: ContactRelationship = ContactRelationship(0i32);
    pub const Spouse: ContactRelationship = ContactRelationship(1i32);
    pub const Partner: ContactRelationship = ContactRelationship(2i32);
    pub const Sibling: ContactRelationship = ContactRelationship(3i32);
    pub const Parent: ContactRelationship = ContactRelationship(4i32);
    pub const Child: ContactRelationship = ContactRelationship(5i32);
}
impl ::std::convert::From<i32> for ContactRelationship {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactRelationship {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactRelationship {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactRelationship;i4)");
}
impl ::windows::runtime::DefaultType for ContactRelationship {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactSelectionMode(pub i32);
impl ContactSelectionMode {
    pub const Contacts: ContactSelectionMode = ContactSelectionMode(0i32);
    pub const Fields: ContactSelectionMode = ContactSelectionMode(1i32);
}
impl ::std::convert::From<i32> for ContactSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactSelectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactSelectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactSelectionMode;i4)");
}
impl ::windows::runtime::DefaultType for ContactSelectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactSignificantOther(::windows::runtime::IInspectable);
impl ContactSignificantOther {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactSignificantOther, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Relationship(&self) -> ::windows::runtime::Result<ContactRelationship> {
        let this = &::windows::runtime::Interface::cast::<IContactSignificantOther2>(self)?;
        unsafe {
            let mut result__: ContactRelationship = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactRelationship>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRelationship(&self, value: ContactRelationship) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactSignificantOther2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactSignificantOther {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactSignificantOther;{8873b5ab-c5fb-46d8-93fe-da3ff1934054})");
}
unsafe impl ::windows::runtime::Interface for ContactSignificantOther {
    type Vtable = IContactSignificantOther_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289284523, 50683, 18136, [147, 254, 218, 63, 241, 147, 64, 84]);
}
impl ::windows::runtime::RuntimeName for ContactSignificantOther {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactSignificantOther";
}
impl ::std::convert::From<ContactSignificantOther> for ::windows::runtime::IUnknown {
    fn from(value: ContactSignificantOther) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactSignificantOther> for ::windows::runtime::IUnknown {
    fn from(value: &ContactSignificantOther) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactSignificantOther {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactSignificantOther {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactSignificantOther> for ::windows::runtime::IInspectable {
    fn from(value: ContactSignificantOther) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactSignificantOther> for ::windows::runtime::IInspectable {
    fn from(value: &ContactSignificantOther) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactSignificantOther {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactSignificantOther {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactSignificantOther {}
unsafe impl ::std::marker::Sync for ContactSignificantOther {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactStore(::windows::runtime::IInspectable);
impl ContactStore {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactsWithSearchTextAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, searchtext: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), searchtext.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Contact>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), contactid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ChangeTracker(&self) -> ::windows::runtime::Result<ContactChangeTracker> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactChangeTracker>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn ContactChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ContactStore, ContactChangedEventArgs>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RemoveContactChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn AggregateContactManager(&self) -> ::windows::runtime::Result<AggregateContactManager> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AggregateContactManager>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ContactList>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetContactListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, contactlistid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), contactlistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateContactListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, displayname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetMeContactAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Contact>> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Contact>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetContactReader(&self) -> ::windows::runtime::Result<ContactReader> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetContactReaderWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ContactQueryOptions>>(&self, options: Param0) -> ::windows::runtime::Result<ContactReader> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<ContactReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateContactListInAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, displayname: Param0, userdataaccountid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ContactList>> {
        let this = &::windows::runtime::Interface::cast::<IContactStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), displayname.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ContactList>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetChangeTracker<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, identity: Param0) -> ::windows::runtime::Result<ContactChangeTracker> {
        let this = &::windows::runtime::Interface::cast::<IContactStore3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), identity.into_param().abi(), &mut result__).from_abi::<ContactChangeTracker>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStore;{2c220b10-3a6c-4293-b9bc-fe987f6e0d52})");
}
unsafe impl ::windows::runtime::Interface for ContactStore {
    type Vtable = IContactStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(740428560, 14956, 17043, [185, 188, 254, 152, 127, 110, 13, 82]);
}
impl ::windows::runtime::RuntimeName for ContactStore {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStore";
}
impl ::std::convert::From<ContactStore> for ::windows::runtime::IUnknown {
    fn from(value: ContactStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactStore> for ::windows::runtime::IUnknown {
    fn from(value: &ContactStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactStore> for ::windows::runtime::IInspectable {
    fn from(value: ContactStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactStore> for ::windows::runtime::IInspectable {
    fn from(value: &ContactStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactStore {}
unsafe impl ::std::marker::Sync for ContactStore {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ContactStoreAccessType(pub i32);
impl ContactStoreAccessType {
    pub const AppContactsReadWrite: ContactStoreAccessType = ContactStoreAccessType(0i32);
    pub const AllContactsReadOnly: ContactStoreAccessType = ContactStoreAccessType(1i32);
    pub const AllContactsReadWrite: ContactStoreAccessType = ContactStoreAccessType(2i32);
}
impl ::std::convert::From<i32> for ContactStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ContactStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ContactStoreAccessType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.ContactStoreAccessType;i4)");
}
impl ::windows::runtime::DefaultType for ContactStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactStoreNotificationTriggerDetails(::windows::runtime::IInspectable);
impl ContactStoreNotificationTriggerDetails {}
unsafe impl ::windows::runtime::RuntimeType for ContactStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails;{abb298d6-878a-4f8b-a9ce-46bb7d1c84ce})");
}
unsafe impl ::windows::runtime::Interface for ContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2880608470, 34698, 20363, [169, 206, 70, 187, 125, 28, 132, 206]);
}
impl ::windows::runtime::RuntimeName for ContactStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactStoreNotificationTriggerDetails";
}
impl ::std::convert::From<ContactStoreNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: ContactStoreNotificationTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactStoreNotificationTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ContactStoreNotificationTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactStoreNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: ContactStoreNotificationTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactStoreNotificationTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ContactStoreNotificationTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactStoreNotificationTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactStoreNotificationTriggerDetails {}
unsafe impl ::std::marker::Sync for ContactStoreNotificationTriggerDetails {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContactWebsite(::windows::runtime::IInspectable);
impl ContactWebsite {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactWebsite, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn RawValue(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IContactWebsite2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SetRawValue<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IContactWebsite2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContactWebsite {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.ContactWebsite;{9f130176-dc1b-4055-ad66-652f39d990e8})");
}
unsafe impl ::windows::runtime::Interface for ContactWebsite {
    type Vtable = IContactWebsite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2668822902, 56347, 16469, [173, 102, 101, 47, 57, 217, 144, 232]);
}
impl ::windows::runtime::RuntimeName for ContactWebsite {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.ContactWebsite";
}
impl ::std::convert::From<ContactWebsite> for ::windows::runtime::IUnknown {
    fn from(value: ContactWebsite) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContactWebsite> for ::windows::runtime::IUnknown {
    fn from(value: &ContactWebsite) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContactWebsite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContactWebsite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContactWebsite> for ::windows::runtime::IInspectable {
    fn from(value: ContactWebsite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContactWebsite> for ::windows::runtime::IInspectable {
    fn from(value: &ContactWebsite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContactWebsite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContactWebsite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContactWebsite {}
unsafe impl ::std::marker::Sync for ContactWebsite {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FullContactCardOptions(::windows::runtime::IInspectable);
impl FullContactCardOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FullContactCardOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `UI_ViewManagement`*"]
    pub fn DesiredRemainingView(&self) -> ::windows::runtime::Result<super::super::UI::ViewManagement::ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__: super::super::UI::ViewManagement::ViewSizePreference = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::ViewManagement::ViewSizePreference>(result__)
        }
    }
    #[cfg(feature = "UI_ViewManagement")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `UI_ViewManagement`*"]
    pub fn SetDesiredRemainingView(&self, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FullContactCardOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.FullContactCardOptions;{8744436c-5cf9-4683-bdca-a1fdebf8dbce})");
}
unsafe impl ::windows::runtime::Interface for FullContactCardOptions {
    type Vtable = IFullContactCardOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269397868, 23801, 18051, [189, 202, 161, 253, 235, 248, 219, 206]);
}
impl ::windows::runtime::RuntimeName for FullContactCardOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.FullContactCardOptions";
}
impl ::std::convert::From<FullContactCardOptions> for ::windows::runtime::IUnknown {
    fn from(value: FullContactCardOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FullContactCardOptions> for ::windows::runtime::IUnknown {
    fn from(value: &FullContactCardOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FullContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FullContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FullContactCardOptions> for ::windows::runtime::IInspectable {
    fn from(value: FullContactCardOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FullContactCardOptions> for ::windows::runtime::IInspectable {
    fn from(value: &FullContactCardOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FullContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FullContactCardOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FullContactCardOptions {}
unsafe impl ::std::marker::Sync for FullContactCardOptions {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAggregateContactManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAggregateContactManager {
    type Vtable = IAggregateContactManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(58316253, 56154, 20435, [181, 78, 77, 241, 121, 23, 162, 18]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, primarycontact: ::windows::runtime::RawPtr, secondarycontact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, aggregatecontact: ::windows::runtime::RawPtr, rawcontact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAggregateContactManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAggregateContactManager2 {
    type Vtable = IAggregateContactManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1586283224, 43469, 17456, [156, 75, 1, 52, 141, 178, 202, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAggregateContactManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactlistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, remotesourceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContact(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContact {
    type Vtable = IContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3959452403, 8472, 16457, [158, 188, 23, 240, 171, 105, 43, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContact2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContact2 {
    type Vtable = IContact2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4078105445, 47991, 19604, [128, 45, 131, 40, 206, 228, 12, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact2_abi(
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
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContact3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContact3 {
    type Vtable = IContact3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1210064487, 57486, 17060, [181, 97, 65, 208, 140, 169, 87, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactAddress(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAddress {
    type Vtable = IContactAddress_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2537149338, 17102, 18546, [141, 112, 48, 99, 170, 88, 75, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAddress_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactAddressKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactAddressKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactAnnotation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAnnotation {
    type Vtable = IContactAnnotation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2183119599, 32065, 17570, [132, 195, 96, 162, 129, 221, 123, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactAnnotationOperations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactAnnotationOperations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactAnnotation2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAnnotation2 {
    type Vtable = IContactAnnotation2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3063016691, 19127, 18975, [153, 65, 12, 156, 243, 23, 27, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotation2_abi(
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
#[doc(hidden)]
pub struct IContactAnnotationList(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAnnotationList {
    type Vtable = IContactAnnotationList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460255914, 23688, 17849, [170, 208, 70, 24, 136, 230, 141, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationList_abi(
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
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, annotation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, annotationid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, annotation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactAnnotationStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAnnotationStore {
    type Vtable = IContactAnnotationStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(598537386, 31351, 17789, [130, 3, 152, 127, 75, 49, 175, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emailaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumber: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, annotation: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdataaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, annotationlistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactAnnotationStore2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactAnnotationStore2 {
    type Vtable = IContactAnnotationStore2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2128487421, 25063, 18791, [142, 197, 189, 242, 128, 162, 64, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAnnotationStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactlistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactBatch(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactBatch {
    type Vtable = IContactBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(902928173, 49102, 18107, [147, 248, 165, 176, 110, 197, 226, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactBatch_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactBatchStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactCardDelayedDataLoader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactCardDelayedDataLoader {
    type Vtable = IContactCardDelayedDataLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3054172418, 5446, 17229, [134, 156, 110, 53, 32, 118, 14, 243]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardDelayedDataLoader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactCardOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactCardOptions {
    type Vtable = IContactCardOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2349485950, 27318, 20287, [190, 114, 129, 114, 54, 238, 234, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactCardHeaderKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactCardHeaderKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactCardTabKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactCardTabKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactCardOptions2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactCardOptions2 {
    type Vtable = IContactCardOptions2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2401704864, 55115, 19654, [159, 83, 27, 14, 181, 209, 39, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCardOptions2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactChange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChange {
    type Vtable = IContactChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2501724944, 27225, 18208, [164, 225, 54, 61, 152, 193, 53, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactChangeType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactChangeReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChangeReader {
    type Vtable = IContactChangeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(561191418, 11532, 17120, [169, 218, 62, 205, 86, 167, 138, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastchangetoaccept: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactChangeTracker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChangeTracker {
    type Vtable = IContactChangeTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1855531346, 12443, 16461, [151, 18, 179, 123, 211, 2, 120, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker_abi(
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
#[doc(hidden)]
pub struct IContactChangeTracker2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChangeTracker2 {
    type Vtable = IContactChangeTracker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2139803900, 37665, 19736, [156, 9, 215, 8, 198, 63, 205, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangeTracker2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactChangedDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChangedDeferral {
    type Vtable = IContactChangedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3306437352, 6915, 18168, [182, 148, 165, 35, 232, 60, 252, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactChangedEventArgs {
    type Vtable = IContactChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1381924817, 29683, 19325, [169, 24, 88, 11, 228, 54, 97, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactChangedEventArgs_abi(
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
pub struct IContactConnectedServiceAccount(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactConnectedServiceAccount {
    type Vtable = IContactConnectedServiceAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4143461715, 43559, 18225, [142, 74, 61, 236, 92, 233, 238, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactConnectedServiceAccount_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactDate(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactDate {
    type Vtable = IContactDate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4271418982, 45573, 18740, [145, 116, 15, 242, 176, 86, 87, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactDate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactDateKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactDateKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactEmail(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactEmail {
    type Vtable = IContactEmail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2426542505, 58323, 19811, [153, 59, 5, 185, 165, 57, 58, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactEmail_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactEmailKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactEmailKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct IContactField(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactField {
    type Vtable = IContactField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2977319018, 53907, 18732, [160, 88, 219, 87, 91, 62, 60, 15]);
}
impl IContactField {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<ContactFieldType> {
        let this = self;
        unsafe {
            let mut result__: ContactFieldType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Category(&self) -> ::windows::runtime::Result<ContactFieldCategory> {
        let this = self;
        unsafe {
            let mut result__: ContactFieldCategory = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContactFieldCategory>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactField {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b176486a-d293-492c-a058-db575b3e3c0f}");
}
impl ::std::convert::From<IContactField> for ::windows::runtime::IUnknown {
    fn from(value: IContactField) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContactField> for ::windows::runtime::IUnknown {
    fn from(value: &IContactField) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IContactField> for ::windows::runtime::IInspectable {
    fn from(value: IContactField) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContactField> for ::windows::runtime::IInspectable {
    fn from(value: &IContactField) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactField {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactField_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactFieldType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactFieldCategory) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct IContactFieldFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactFieldFactory {
    type Vtable = IContactFieldFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2246218047, 3658, 19006, [137, 148, 64, 106, 231, 237, 100, 110]);
}
impl IContactFieldFactory {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, r#type: ContactFieldType) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, &mut result__).from_abi::<ContactField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateField_Custom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), r#type, category, &mut result__).from_abi::<ContactField>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactFieldFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{85e2913f-0e4a-4a3e-8994-406ae7ed646e}");
}
impl ::std::convert::From<IContactFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: IContactFieldFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContactFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IContactFieldFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IContactFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: IContactFieldFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContactFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IContactFieldFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactFieldFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, r#type: ContactFieldType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactGroup {
    type Vtable = IContactGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1505618689, 40602, 18269, [191, 229, 163, 123, 128, 109, 133, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactInformation {
    type Vtable = IContactInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(660518612, 27182, 17016, [169, 20, 228, 96, 213, 240, 136, 246]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactInstantMessageField(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactInstantMessageField {
    type Vtable = IContactInstantMessageField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437443895, 3461, 16890, [180, 61, 218, 89, 156, 62, 176, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageField_abi(
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
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct IContactInstantMessageFieldFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactInstantMessageFieldFactory {
    type Vtable = IContactInstantMessageFieldFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121309588, 37283, 19378, [177, 185, 105, 165, 223, 240, 186, 9]);
}
impl IContactInstantMessageFieldFactory {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), username.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateInstantMessage_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, username: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), username.into_param().abi(), category, &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn CreateInstantMessage_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, username: Param0, category: ContactFieldCategory, service: Param2, displaytext: Param3, verb: Param4) -> ::windows::runtime::Result<ContactInstantMessageField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), username.into_param().abi(), category, service.into_param().abi(), displaytext.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<ContactInstantMessageField>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactInstantMessageFieldFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ba0b6794-91a3-4bb2-b1b9-69a5dff0ba09}");
}
impl ::std::convert::From<IContactInstantMessageFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: IContactInstantMessageFieldFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContactInstantMessageFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IContactInstantMessageFieldFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactInstantMessageFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContactInstantMessageFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IContactInstantMessageFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: IContactInstantMessageFieldFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContactInstantMessageFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IContactInstantMessageFieldFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactInstantMessageFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactInstantMessageFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactInstantMessageFieldFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: ContactFieldCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: ContactFieldCategory, service: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displaytext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, verb: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactJobInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactJobInfo {
    type Vtable = IContactJobInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1829862220, 52816, 19267, [158, 105, 177, 130, 88, 234, 83, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactJobInfo_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactLaunchActionVerbsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactLaunchActionVerbsStatics {
    type Vtable = IContactLaunchActionVerbsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4212273878, 61043, 18151, [135, 97, 17, 205, 1, 87, 114, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLaunchActionVerbsStatics_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactList(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactList {
    type Vtable = IContactList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(383642741, 14636, 18501, [157, 251, 81, 163, 231, 239, 62, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactListOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactListOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactListOtherAppWriteAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactListOtherAppWriteAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, remoteid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactList2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactList2 {
    type Vtable = IContactList2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3409527732, 17744, 19915, [146, 41, 64, 255, 145, 251, 2, 3]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactList3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactList3 {
    type Vtable = IContactList3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(360246871, 9980, 16872, [168, 80, 90, 163, 37, 20, 172, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactList3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListLimitedWriteOperations(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListLimitedWriteOperations {
    type Vtable = IContactListLimitedWriteOperations_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3784840154, 18955, 17592, [154, 31, 160, 243, 210, 24, 23, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListLimitedWriteOperations_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListSyncConstraints(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListSyncConstraints {
    type Vtable = IContactListSyncConstraints_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2997927681, 12386, 20014, [150, 157, 1, 141, 25, 135, 243, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncConstraints_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListSyncManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListSyncManager {
    type Vtable = IContactListSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(342787006, 31013, 19148, [157, 229, 33, 221, 208, 111, 134, 116]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactListSyncStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactListSyncManager2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactListSyncManager2 {
    type Vtable = IContactListSyncManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2841186887, 47957, 20003, [129, 40, 55, 1, 52, 168, 93, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactListSyncManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactListSyncStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactLocationField(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactLocationField {
    type Vtable = IContactLocationField_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2663387010, 43886, 19254, [137, 227, 178, 59, 192, 161, 218, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationField_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct IContactLocationFieldFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactLocationFieldFactory {
    type Vtable = IContactLocationFieldFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4154012375, 12255, 17406, [143, 24, 65, 137, 115, 144, 188, 254]);
}
impl IContactLocationFieldFactory {
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Default<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, unstructuredaddress: Param0) -> ::windows::runtime::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_Category<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, unstructuredaddress: Param0, category: ContactFieldCategory) -> ::windows::runtime::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn CreateLocation_All<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        unstructuredaddress: Param0,
        category: ContactFieldCategory,
        street: Param2,
        city: Param3,
        region: Param4,
        country: Param5,
        postalcode: Param6,
    ) -> ::windows::runtime::Result<ContactLocationField> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), unstructuredaddress.into_param().abi(), category, street.into_param().abi(), city.into_param().abi(), region.into_param().abi(), country.into_param().abi(), postalcode.into_param().abi(), &mut result__).from_abi::<ContactLocationField>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IContactLocationFieldFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{f79932d7-2fdf-43fe-8f18-41897390bcfe}");
}
impl ::std::convert::From<IContactLocationFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: IContactLocationFieldFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContactLocationFieldFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IContactLocationFieldFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactLocationFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContactLocationFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IContactLocationFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: IContactLocationFieldFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IContactLocationFieldFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IContactLocationFieldFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IContactLocationFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IContactLocationFieldFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactLocationFieldFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unstructuredaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unstructuredaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, category: ContactFieldCategory, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        unstructuredaddress: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        category: ContactFieldCategory,
        street: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        city: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        region: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        country: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        postalcode: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerForUser {
    type Vtable = IContactManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3075193431, 4214, 19439, [174, 243, 84, 104, 109, 24, 56, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, maxbytes: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcard: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: ContactStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: ContactAnnotationStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactNameOrder) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerForUser2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerForUser2 {
    type Vtable = IContactManagerForUser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1296473134, 15221, 19059, [187, 48, 115, 102, 69, 71, 34, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerForUser2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, fullcontactcardoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerStatics {
    type Vtable = IContactManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2180127424, 63073, 18184, [186, 79, 211, 134, 189, 13, 98, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerStatics2 {
    type Vtable = IContactManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2709055008, 18392, 18636, [150, 60, 149, 146, 182, 229, 16, 198]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics2_abi(
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
#[doc(hidden)]
pub struct IContactManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerStatics3 {
    type Vtable = IContactManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3301719362, 30086, 18730, [147, 11, 123, 193, 56, 252, 33, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, maxbytes: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vcard: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: ContactStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: ContactAnnotationStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, contactcardoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, fullcontactcardoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactNameOrder) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactNameOrder) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerStatics4 {
    type Vtable = IContactManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(613950066, 13435, 18140, [141, 149, 81, 189, 65, 225, 90, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactManagerStatics5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactManagerStatics5 {
    type Vtable = IContactManagerStatics5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4149811847, 44215, 20397, [144, 242, 168, 171, 100, 205, 187, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManagerStatics5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactMatchReason(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactMatchReason {
    type Vtable = IContactMatchReason_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3163694340, 59352, 16702, [149, 244, 183, 92, 84, 199, 64, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactMatchReason_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactMatchReasonKind) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactName(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactName {
    type Vtable = IContactName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4093962619, 36916, 17724, [142, 191, 20, 10, 56, 200, 111, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactName_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPanel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPanel {
    type Vtable = IContactPanel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1103041125, 53998, 19351, [168, 10, 125, 141, 100, 204, 166, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPanelClosingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPanelClosingEventArgs {
    type Vtable = IContactPanelClosingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(572617939, 53067, 18135, [183, 57, 110, 220, 22, 17, 11, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelClosingEventArgs_abi(
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
#[doc(hidden)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPanelLaunchFullAppRequestedEventArgs {
    type Vtable = IContactPanelLaunchFullAppRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2295733262, 9140, 19432, [138, 252, 7, 44, 37, 164, 25, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPanelLaunchFullAppRequestedEventArgs_abi(
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
#[doc(hidden)]
pub struct IContactPhone(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPhone {
    type Vtable = IContactPhone_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1182640997, 10002, 20306, [183, 131, 158, 168, 17, 28, 99, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPhone_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactPhoneKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactPhoneKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPicker(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPicker {
    type Vtable = IContactPicker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(235535761, 17144, 16469, [144, 160, 137, 111, 150, 115, 137, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactSelectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactSelectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPicker2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPicker2 {
    type Vtable = IContactPicker2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3008369103, 23791, 19748, [170, 12, 52, 12, 82, 8, 114, 93]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPicker3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPicker3 {
    type Vtable = IContactPicker3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(242365205, 45635, 19437, [133, 22, 34, 177, 167, 172, 10, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPicker3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPickerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPickerStatics {
    type Vtable = IContactPickerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1955119145, 27219, 16984, [163, 233, 98, 223, 246, 120, 75, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactQueryOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactQueryOptions {
    type Vtable = IContactQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1141427358, 32124, 17136, [138, 199, 245, 7, 51, 236, 219, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactQueryDesiredFields) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactQueryDesiredFields) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactAnnotationOperations) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactAnnotationOperations) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactQueryOptionsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactQueryOptionsFactory {
    type Vtable = IContactQueryOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1413462599, 36071, 18123, [157, 172, 154, 164, 42, 27, 200, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ContactQuerySearchFields, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactQueryTextSearch(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactQueryTextSearch {
    type Vtable = IContactQueryTextSearch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4158912971, 43351, 17307, [160, 183, 28, 2, 161, 150, 63, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactQueryTextSearch_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactQuerySearchFields) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactQuerySearchFields) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactQuerySearchScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactQuerySearchScope) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactReader {
    type Vtable = IContactReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3549946926, 5256, 17138, [191, 100, 37, 63, 72, 132, 191, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactSignificantOther(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactSignificantOther {
    type Vtable = IContactSignificantOther_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2289284523, 50683, 18136, [147, 254, 218, 63, 241, 147, 64, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther_abi(
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
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactSignificantOther2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactSignificantOther2 {
    type Vtable = IContactSignificantOther2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2373702772, 16131, 17912, [186, 15, 196, 237, 55, 214, 66, 25]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactSignificantOther2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ContactRelationship) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ContactRelationship) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactStore {
    type Vtable = IContactStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(740428560, 14956, 17043, [185, 188, 254, 152, 127, 110, 13, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, searchtext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactStore2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactStore2 {
    type Vtable = IContactStore2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(416160802, 60373, 19451, [182, 144, 95, 79, 39, 196, 240, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contactlistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, userdataaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactStore3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactStore3 {
    type Vtable = IContactStore3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3414699116, 78, 16464, [135, 240, 132, 4, 7, 238, 104, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStore3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactStoreNotificationTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactStoreNotificationTriggerDetails {
    type Vtable = IContactStoreNotificationTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2880608470, 34698, 20363, [169, 206, 70, 187, 125, 28, 132, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactStoreNotificationTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactWebsite(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactWebsite {
    type Vtable = IContactWebsite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2668822902, 56347, 16469, [173, 102, 101, 47, 57, 217, 144, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactWebsite2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactWebsite2 {
    type Vtable = IContactWebsite2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169066782, 22087, 16488, [187, 94, 75, 111, 67, 124, 227, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactWebsite2_abi(
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
#[doc(hidden)]
pub struct IFullContactCardOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFullContactCardOptions {
    type Vtable = IFullContactCardOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2269397868, 23801, 18051, [189, 202, 161, 253, 235, 248, 219, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullContactCardOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
    #[cfg(feature = "UI_ViewManagement")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::ViewManagement::ViewSizePreference) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_ViewManagement"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IKnownContactFieldStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownContactFieldStatics {
    type Vtable = IKnownContactFieldStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(772676370, 54823, 20426, [186, 212, 31, 175, 22, 140, 125, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownContactFieldStatics_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ContactFieldType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ContactFieldType, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPinnedContactIdsQueryResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107319634, 5497, 19932, [135, 31, 163, 10, 58, 234, 155, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactIdsQueryResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPinnedContactManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPinnedContactManager {
    type Vtable = IPinnedContactManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4240208908, 57814, 17859, [184, 182, 163, 86, 4, 225, 103, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, surface: PinnedContactSurface, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contacts: ::windows::runtime::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr, surface: PinnedContactSurface, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contact: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPinnedContactManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPinnedContactManagerStatics {
    type Vtable = IPinnedContactManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4133276798, 65017, 18538, [172, 233, 188, 49, 29, 10, 231, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPinnedContactManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
pub struct KnownContactField {}
impl KnownContactField {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Email() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn PhoneNumber() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn Location() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn InstantMessage() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ConvertNameToType<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<ContactFieldType> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ContactFieldType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ContactFieldType>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn ConvertTypeToName(r#type: ContactFieldType) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownContactFieldStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), r#type, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownContactFieldStatics<R, F: FnOnce(&IKnownContactFieldStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownContactField, IKnownContactFieldStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.KnownContactField";
}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PinnedContactIdsQueryResult(::windows::runtime::IInspectable);
impl PinnedContactIdsQueryResult {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation_Collections`*"]
    pub fn ContactIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PinnedContactIdsQueryResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult;{7d9b2552-1579-4ddc-871f-a30a3aea9ba1})");
}
unsafe impl ::windows::runtime::Interface for PinnedContactIdsQueryResult {
    type Vtable = IPinnedContactIdsQueryResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107319634, 5497, 19932, [135, 31, 163, 10, 58, 234, 155, 161]);
}
impl ::windows::runtime::RuntimeName for PinnedContactIdsQueryResult {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactIdsQueryResult";
}
impl ::std::convert::From<PinnedContactIdsQueryResult> for ::windows::runtime::IUnknown {
    fn from(value: PinnedContactIdsQueryResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PinnedContactIdsQueryResult> for ::windows::runtime::IUnknown {
    fn from(value: &PinnedContactIdsQueryResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PinnedContactIdsQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PinnedContactIdsQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PinnedContactIdsQueryResult> for ::windows::runtime::IInspectable {
    fn from(value: PinnedContactIdsQueryResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PinnedContactIdsQueryResult> for ::windows::runtime::IInspectable {
    fn from(value: &PinnedContactIdsQueryResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PinnedContactIdsQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PinnedContactIdsQueryResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PinnedContactIdsQueryResult {}
unsafe impl ::std::marker::Sync for PinnedContactIdsQueryResult {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PinnedContactManager(::windows::runtime::IInspectable);
impl PinnedContactManager {
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsPinSurfaceSupported(&self, surface: PinnedContactSurface) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), surface, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsContactPinned<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0, surface: PinnedContactSurface) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), contact.into_param().abi(), surface, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestPinContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0, surface: PinnedContactSurface) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), contact.into_param().abi(), surface, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn RequestPinContactsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<Contact>>>(&self, contacts: Param0, surface: PinnedContactSurface) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), contacts.into_param().abi(), surface, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn RequestUnpinContactAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0, surface: PinnedContactSurface) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), contact.into_param().abi(), surface, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn SignalContactActivity<'a, Param0: ::windows::runtime::IntoParam<'a, Contact>>(&self, contact: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), contact.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `Foundation`*"]
    pub fn GetPinnedContactIdsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PinnedContactIdsQueryResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<PinnedContactManager> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PinnedContactManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_Contacts`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<PinnedContactManager> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<PinnedContactManager>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_Contacts`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IPinnedContactManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IPinnedContactManagerStatics<R, F: FnOnce(&IPinnedContactManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PinnedContactManager, IPinnedContactManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PinnedContactManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.PinnedContactManager;{fcbc740c-e1d6-45c3-b8b6-a35604e167a0})");
}
unsafe impl ::windows::runtime::Interface for PinnedContactManager {
    type Vtable = IPinnedContactManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4240208908, 57814, 17859, [184, 182, 163, 86, 4, 225, 103, 160]);
}
impl ::windows::runtime::RuntimeName for PinnedContactManager {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.PinnedContactManager";
}
impl ::std::convert::From<PinnedContactManager> for ::windows::runtime::IUnknown {
    fn from(value: PinnedContactManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PinnedContactManager> for ::windows::runtime::IUnknown {
    fn from(value: &PinnedContactManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PinnedContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PinnedContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PinnedContactManager> for ::windows::runtime::IInspectable {
    fn from(value: PinnedContactManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PinnedContactManager> for ::windows::runtime::IInspectable {
    fn from(value: &PinnedContactManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PinnedContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PinnedContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PinnedContactManager {}
unsafe impl ::std::marker::Sync for PinnedContactManager {}
#[doc = "*Required features: `ApplicationModel_Contacts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PinnedContactSurface(pub i32);
impl PinnedContactSurface {
    pub const StartMenu: PinnedContactSurface = PinnedContactSurface(0i32);
    pub const Taskbar: PinnedContactSurface = PinnedContactSurface(1i32);
}
impl ::std::convert::From<i32> for PinnedContactSurface {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PinnedContactSurface {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PinnedContactSurface {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.PinnedContactSurface;i4)");
}
impl ::windows::runtime::DefaultType for PinnedContactSurface {
    type DefaultType = Self;
}
