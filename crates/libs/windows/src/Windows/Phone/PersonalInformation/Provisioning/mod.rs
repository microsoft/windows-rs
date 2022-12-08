#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, networkname: *mut ::core::ffi::c_void, networkaccountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateNetworkAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportVcardToSystemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IContactPartnerProvisioningManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc26155f7_55ed_475d_9334_c5d484c30f1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, networkname: *mut ::core::ffi::c_void, networkaccountid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateSocialNetworkAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessagePartnerProvisioningManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IMessagePartnerProvisioningManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, body: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, subject: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, attachments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
#[doc = "*Required features: `\"Phone_PersonalInformation_Provisioning\"`*"]
pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateNetworkAccountAsync(store: &super::ContactStore, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AssociateNetworkAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(store), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportVcardToSystemAsync<P0, E0>(stream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IInputStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportVcardToSystemAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateSocialNetworkAccountAsync(store: &super::ContactStore, networkname: &::windows::core::HSTRING, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AssociateSocialNetworkAccountAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(store), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[doc = "*Required features: `\"Phone_PersonalInformation_Provisioning\"`*"]
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportSmsToSystemAsync<P0, E0>(incoming: bool, read: bool, body: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportSmsToSystemAsync)(::windows::core::Vtable::as_raw(this), incoming, read, ::core::mem::transmute_copy(body), ::core::mem::transmute_copy(sender), recipients.try_into().map_err(|e| e.into())?.abi(), deliverytime, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportMmsToSystemAsync<P0, E0, P1, E1>(incoming: bool, read: bool, subject: &::windows::core::HSTRING, sender: &::windows::core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime, attachments: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ImportMmsToSystemAsync)(::windows::core::Vtable::as_raw(this), incoming, read, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(sender), recipients.try_into().map_err(|e| e.into())?.abi(), deliverytime, attachments.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
