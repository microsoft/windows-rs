#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
pub struct CommunicationBlockingAccessManager {}
impl CommunicationBlockingAccessManager {
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
    pub fn IsBlockingActive() -> ::windows::runtime::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`, `Foundation`*"]
    pub fn IsBlockedNumberAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(number: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), number.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`, `Foundation_Collections`*"]
    pub fn ShowBlockNumbersUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(phonenumbers: Param0) -> ::windows::runtime::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`, `Foundation_Collections`*"]
    pub fn ShowUnblockNumbersUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(phonenumbers: Param0) -> ::windows::runtime::Result<bool> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), phonenumbers.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
    pub fn ShowBlockedCallsUI() -> ::windows::runtime::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
    pub fn ShowBlockedMessagesUI() -> ::windows::runtime::Result<()> {
        Self::ICommunicationBlockingAccessManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() })
    }
    pub fn ICommunicationBlockingAccessManagerStatics<R, F: FnOnce(&ICommunicationBlockingAccessManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CommunicationBlockingAccessManager, ICommunicationBlockingAccessManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CommunicationBlockingAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAccessManager";
}
#[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
pub struct CommunicationBlockingAppManager {}
impl CommunicationBlockingAppManager {
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
    pub fn IsCurrentAppActiveBlockingApp() -> ::windows::runtime::Result<bool> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`*"]
    pub fn ShowCommunicationBlockingSettingsUI() -> ::windows::runtime::Result<()> {
        Self::ICommunicationBlockingAppManagerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_CommunicationBlocking`, `Foundation`*"]
    pub fn RequestSetAsActiveBlockingAppAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::ICommunicationBlockingAppManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn ICommunicationBlockingAppManagerStatics<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICommunicationBlockingAppManagerStatics2<R, F: FnOnce(&ICommunicationBlockingAppManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CommunicationBlockingAppManager, ICommunicationBlockingAppManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CommunicationBlockingAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.CommunicationBlocking.CommunicationBlockingAppManager";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct CommunicationBlockingContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommunicationBlockingAccessManagerStatics {
    type Vtable = ICommunicationBlockingAccessManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(479631768, 40234, 23991, [237, 213, 12, 228, 7, 252, 37, 149]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAccessManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, number: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumbers: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phonenumbers: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommunicationBlockingAppManagerStatics {
    type Vtable = ICommunicationBlockingAppManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2010863852, 5286, 19370, [148, 42, 106, 103, 61, 153, 155, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommunicationBlockingAppManagerStatics2 {
    type Vtable = ICommunicationBlockingAppManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(346459869, 60808, 17786, [163, 100, 163, 99, 77, 111, 22, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommunicationBlockingAppManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
