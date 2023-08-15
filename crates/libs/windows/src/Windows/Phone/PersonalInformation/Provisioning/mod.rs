#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPartnerProvisioningManagerStatics {
    type Vtable = IContactPartnerProvisioningManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IContactPartnerProvisioningManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPartnerProvisioningManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, networkname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateNetworkAccountAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ImportVcardToSystemAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPartnerProvisioningManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IContactPartnerProvisioningManagerStatics2 {
    type Vtable = IContactPartnerProvisioningManagerStatics2_Vtbl;
}
impl ::core::clone::Clone for IContactPartnerProvisioningManagerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IContactPartnerProvisioningManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc26155f7_55ed_475d_9334_c5d484c30f1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, store: *mut ::core::ffi::c_void, networkname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, networkaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AssociateSocialNetworkAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessagePartnerProvisioningManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMessagePartnerProvisioningManagerStatics {
    type Vtable = IMessagePartnerProvisioningManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IMessagePartnerProvisioningManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMessagePartnerProvisioningManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, body: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sender: ::std::mem::MaybeUninit<::windows_core::HSTRING>, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incoming: bool, read: bool, subject: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sender: ::std::mem::MaybeUninit<::windows_core::HSTRING>, recipients: *mut ::core::ffi::c_void, deliverytime: super::super::super::Foundation::DateTime, attachments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
#[doc = "*Required features: `\"Phone_PersonalInformation_Provisioning\"`*"]
pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateNetworkAccountAsync<P0>(store: P0, networkname: &::windows_core::HSTRING, networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssociateNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ImportVcardToSystemAsync<P0>(stream: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IInputStream>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportVcardToSystemAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AssociateSocialNetworkAccountAsync<P0>(store: P0, networkname: &::windows_core::HSTRING, networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssociateSocialNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IContactPartnerProvisioningManagerStatics2<R, F: FnOnce(&IContactPartnerProvisioningManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContactPartnerProvisioningManager, IContactPartnerProvisioningManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ContactPartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.ContactPartnerProvisioningManager";
}
#[doc = "*Required features: `\"Phone_PersonalInformation_Provisioning\"`*"]
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportSmsToSystemAsync<P0>(incoming: bool, read: bool, body: &::windows_core::HSTRING, sender: &::windows_core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportSmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, ::core::mem::transmute_copy(body), ::core::mem::transmute_copy(sender), recipients.try_into_param()?.abi(), deliverytime, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportMmsToSystemAsync<P0, P1>(incoming: bool, read: bool, subject: &::windows_core::HSTRING, sender: &::windows_core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime, attachments: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
        P1: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportMmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(sender), recipients.try_into_param()?.abi(), deliverytime, attachments.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMessagePartnerProvisioningManagerStatics<R, F: FnOnce(&IMessagePartnerProvisioningManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MessagePartnerProvisioningManager, IMessagePartnerProvisioningManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MessagePartnerProvisioningManager {
    const NAME: &'static str = "Windows.Phone.PersonalInformation.Provisioning.MessagePartnerProvisioningManager";
}
