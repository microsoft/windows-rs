#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_PersonalInformation_Provisioning`*"]
pub struct ContactPartnerProvisioningManager {}
impl ContactPartnerProvisioningManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_PersonalInformation_Provisioning`, `Foundation`*"]
    pub fn AssociateNetworkAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::ContactStore>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Phone_PersonalInformation_Provisioning`, `Foundation`, `Storage_Streams`*"]
    pub fn ImportVcardToSystemAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(stream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_PersonalInformation_Provisioning`, `Foundation`*"]
    pub fn AssociateSocialNetworkAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::ContactStore>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3235355169, 431, 20435, [152, 205, 179, 214, 86, 222, 21, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, store: ::windows::runtime::RawPtr, networkname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3261158903, 21997, 18269, [147, 52, 197, 212, 132, 195, 15, 26]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, store: ::windows::runtime::RawPtr, networkname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2317027408, 29637, 17788, [188, 89, 237, 125, 97, 92, 5, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, incoming: bool, read: bool, body: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sender: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, recipients: ::windows::runtime::RawPtr, deliverytime: super::super::super::Foundation::DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, incoming: bool, read: bool, subject: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sender: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, recipients: ::windows::runtime::RawPtr, deliverytime: super::super::super::Foundation::DateTime, attachments: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: `Phone_PersonalInformation_Provisioning`*"]
pub struct MessagePartnerProvisioningManager {}
impl MessagePartnerProvisioningManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_PersonalInformation_Provisioning`, `Foundation`, `Foundation_Collections`*"]
    pub fn ImportSmsToSystemAsync<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>>(
        incoming: bool,
        read: bool,
        body: Param2,
        sender: Param3,
        recipients: Param4,
        deliverytime: Param5,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), incoming, read, body.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Phone_PersonalInformation_Provisioning`, `Foundation`, `Foundation_Collections`*"]
    pub fn ImportMmsToSystemAsync<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::DateTime>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::IInspectable>>>,
    >(
        incoming: bool,
        read: bool,
        subject: Param2,
        sender: Param3,
        recipients: Param4,
        deliverytime: Param5,
        attachments: Param6,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), incoming, read, subject.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), attachments.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
