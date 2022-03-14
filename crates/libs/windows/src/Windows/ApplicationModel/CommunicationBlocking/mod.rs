#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
pub struct CommunicationBlockingAccessManager {}
impl CommunicationBlockingAccessManager {
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
    pub fn IsBlockingActive() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlockingActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsBlockedNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlockedNumberAsync)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowBlockNumbersUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumbers: Param0) -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowBlockNumbersUI)(::core::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowUnblockNumbersUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumbers: Param0) -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowUnblockNumbersUI)(::core::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
    pub fn ShowBlockedCallsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowBlockedCallsUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
    pub fn ShowBlockedMessagesUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowBlockedMessagesUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
#[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
pub struct CommunicationBlockingAppManager {}
impl CommunicationBlockingAppManager {
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
    pub fn IsCurrentAppActiveBlockingApp() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAppActiveBlockingApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`*"]
    pub fn ShowCommunicationBlockingSettingsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).ShowCommunicationBlockingSettingsUI)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_CommunicationBlocking\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsActiveBlockingAppAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSetAsActiveBlockingAppAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAccessManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAccessManagerStatics {
    type Vtable = ICommunicationBlockingAccessManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBlockingActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsBlockedNumberAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBlockedNumberAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowBlockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowBlockNumbersUI: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ShowUnblockNumbersUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ShowUnblockNumbersUI: usize,
    pub ShowBlockedCallsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowBlockedMessagesUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics {
    type Vtable = ICommunicationBlockingAppManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77db58ec_14a6_4baa_942a_6a673d999bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCurrentAppActiveBlockingApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ShowCommunicationBlockingSettingsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICommunicationBlockingAppManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics2 {
    type Vtable = ICommunicationBlockingAppManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14a68edd_ed88_457a_a364_a3634d6f166d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestSetAsActiveBlockingAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetAsActiveBlockingAppAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
