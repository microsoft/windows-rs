#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub struct CommunicationBlockingAccessManager {}
impl CommunicationBlockingAccessManager {
    pub fn IsBlockingActive() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn IsBlockedNumberAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(number: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowBlockNumbersUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumbers: Param0) -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ShowUnblockNumbersUI<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(phonenumbers: Param0) -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ShowBlockedCallsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn ShowBlockedMessagesUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() })
    }
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
pub struct CommunicationBlockingAppManager {}
impl CommunicationBlockingAppManager {
    pub fn IsCurrentAppActiveBlockingApp() -> ::windows::core::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn ShowCommunicationBlockingSettingsUI() -> ::windows::core::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestSetAsActiveBlockingAppAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct CommunicationBlockingContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAccessManagerStatics {
    type Vtable = ICommunicationBlockingAccessManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c969998_9d2a_5db7_edd5_0ce407fc2595);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phonenumbers: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics {
    type Vtable = ICommunicationBlockingAppManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77db58ec_14a6_4baa_942a_6a673d999bf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommunicationBlockingAppManagerStatics2 {
    type Vtable = ICommunicationBlockingAppManagerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14a68edd_ed88_457a_a364_a3634d6f166d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
