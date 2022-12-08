#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAccessManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICommunicationBlockingAccessManagerStatics {
    type Vtable = ICommunicationBlockingAccessManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommunicationBlockingAccessManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsBlockingActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBlockedNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBlockedNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICommunicationBlockingAppManagerStatics {
    type Vtable = ICommunicationBlockingAppManagerStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77db58ec_14a6_4baa_942a_6a673d999bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICommunicationBlockingAppManagerStatics2 {
    type Vtable = ICommunicationBlockingAppManagerStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14a68edd_ed88_457a_a364_a3634d6f166d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveBlockingAppAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
pub struct CommunicationBlockingAccessManager;
impl CommunicationBlockingAccessManager {
    pub fn IsBlockingActive() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlockingActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsBlockedNumberAsync(number: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsBlockedNumberAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(number), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowBlockNumbersUI<P0, E0>(phonenumbers: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowBlockNumbersUI)(::windows::core::Vtable::as_raw(this), phonenumbers.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowUnblockNumbersUI<P0, E0>(phonenumbers: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ShowUnblockNumbersUI)(::windows::core::Vtable::as_raw(this), phonenumbers.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShowBlockedCallsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowBlockedCallsUI)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    pub fn ShowBlockedMessagesUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowBlockedMessagesUI)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
#[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
pub struct CommunicationBlockingAppManager;
impl CommunicationBlockingAppManager {
    pub fn IsCurrentAppActiveBlockingApp() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsCurrentAppActiveBlockingApp)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ShowCommunicationBlockingSettingsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (::windows::core::Vtable::vtable(this).ShowCommunicationBlockingSettingsUI)(::windows::core::Vtable::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsActiveBlockingAppAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RequestSetAsActiveBlockingAppAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
