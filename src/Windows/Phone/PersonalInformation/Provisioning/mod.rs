#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct ContactPartnerProvisioningManager {}
impl ContactPartnerProvisioningManager {
    #[cfg(feature = "Foundation")]
    pub fn AssociateNetworkAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::ContactStore>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportVcardToSystemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(stream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn AssociateSocialNetworkAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::ContactStore>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc26155f7_55ed_475d_9334_c5d484c30f1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, incoming: bool, read: bool, body: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, incoming: bool, read: bool, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, attachments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
pub struct MessagePartnerProvisioningManager {}
impl MessagePartnerProvisioningManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ImportSmsToSystemAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(
        incoming: bool,
        read: bool,
        body: Param2,
        sender: Param3,
        recipients: Param4,
        deliverytime: Param5,
    ) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), incoming, read, body.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ImportMmsToSystemAsync<
        'a,
        Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>,
        Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>,
        Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>,
        Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>,
    >(
        incoming: bool,
        read: bool,
        subject: Param2,
        sender: Param3,
        recipients: Param4,
        deliverytime: Param5,
        attachments: Param6,
    ) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), incoming, read, subject.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), attachments.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
