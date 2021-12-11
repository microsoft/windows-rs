#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Phone_PersonalInformation_Provisioning'*"]
pub struct ContactPartnerProvisioningManager {}
impl ContactPartnerProvisioningManager {
    #[doc = "*Required features: 'Phone_PersonalInformation_Provisioning', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateNetworkAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::ContactStore>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_PersonalInformation_Provisioning', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportVcardToSystemAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(stream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_PersonalInformation_Provisioning', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateSocialNetworkAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, super::ContactStore>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(store: Param0, networkname: Param1, networkaccountid: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), store.into_param().abi(), networkname.into_param().abi(), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc26155f7_55ed_475d_9334_c5d484c30f1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: ::windows::core::RawPtr, networkname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, networkaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessagePartnerProvisioningManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, body: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, subject: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sender: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, recipients: ::windows::core::RawPtr, deliverytime: super::super::super::Foundation::DateTime, attachments: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc = "*Required features: 'Phone_PersonalInformation_Provisioning'*"]
pub struct MessagePartnerProvisioningManager {}
impl MessagePartnerProvisioningManager {
    #[doc = "*Required features: 'Phone_PersonalInformation_Provisioning', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ImportSmsToSystemAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>>(incoming: bool, read: bool, body: Param2, sender: Param3, recipients: Param4, deliverytime: Param5) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), incoming, read, body.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: 'Phone_PersonalInformation_Provisioning', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn ImportMmsToSystemAsync<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param4: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::DateTime>, Param6: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(incoming: bool, read: bool, subject: Param2, sender: Param3, recipients: Param4, deliverytime: Param5, attachments: Param6) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), incoming, read, subject.into_param().abi(), sender.into_param().abi(), recipients.into_param().abi(), deliverytime.into_param().abi(), attachments.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
