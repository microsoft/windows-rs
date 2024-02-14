::windows_core::imp::com_interface!(IContactPartnerProvisioningManagerStatics, IContactPartnerProvisioningManagerStatics_Vtbl, 0xc0d79a21_01af_4fd3_98cd_b3d656de15f4);
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AssociateNetworkAccountAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ImportVcardToSystemAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ImportVcardToSystemAsync: usize,
}
::windows_core::imp::com_interface!(IContactPartnerProvisioningManagerStatics2, IContactPartnerProvisioningManagerStatics2_Vtbl, 0xc26155f7_55ed_475d_9334_c5d484c30f1a);
#[repr(C)]
pub struct IContactPartnerProvisioningManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AssociateSocialNetworkAccountAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMessagePartnerProvisioningManagerStatics, IMessagePartnerProvisioningManagerStatics_Vtbl, 0x8a1b0850_73c5_457c_bc59_ed7d615c05a4);
#[repr(C)]
pub struct IMessagePartnerProvisioningManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportSmsToSystemAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool, bool, ::std::mem::MaybeUninit<::windows_core::HSTRING>, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut ::core::ffi::c_void, super::super::super::Foundation::DateTime, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportSmsToSystemAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportMmsToSystemAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, bool, bool, ::std::mem::MaybeUninit<::windows_core::HSTRING>, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut ::core::ffi::c_void, super::super::super::Foundation::DateTime, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportMmsToSystemAsync: usize,
}
pub struct ContactPartnerProvisioningManager;
impl ContactPartnerProvisioningManager {
    pub fn AssociateNetworkAccountAsync<P0>(store: P0, networkname: &::windows_core::HSTRING, networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssociateNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ImportVcardToSystemAsync<P0>(stream: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Storage::Streams::IInputStream>,
    {
        Self::IContactPartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportVcardToSystemAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn AssociateSocialNetworkAccountAsync<P0>(store: P0, networkname: &::windows_core::HSTRING, networkaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::ContactStore>,
    {
        Self::IContactPartnerProvisioningManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AssociateSocialNetworkAccountAsync)(::windows_core::Interface::as_raw(this), store.into_param().abi(), ::core::mem::transmute_copy(networkname), ::core::mem::transmute_copy(networkaccountid), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
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
pub struct MessagePartnerProvisioningManager;
impl MessagePartnerProvisioningManager {
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportSmsToSystemAsync<P0>(incoming: bool, read: bool, body: &::windows_core::HSTRING, sender: &::windows_core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportSmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, ::core::mem::transmute_copy(body), ::core::mem::transmute_copy(sender), recipients.into_param().abi(), deliverytime, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportMmsToSystemAsync<P0, P1>(incoming: bool, read: bool, subject: &::windows_core::HSTRING, sender: &::windows_core::HSTRING, recipients: P0, deliverytime: super::super::super::Foundation::DateTime, attachments: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::Collections::IVectorView<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::IInspectable>>>,
    {
        Self::IMessagePartnerProvisioningManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ImportMmsToSystemAsync)(::windows_core::Interface::as_raw(this), incoming, read, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(sender), recipients.into_param().abi(), deliverytime, attachments.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
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
