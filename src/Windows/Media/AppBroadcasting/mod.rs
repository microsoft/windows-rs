#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct AppBroadcastingContract(pub u8);
#[doc = "*Required features: `Media_AppBroadcasting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastingMonitor(::windows::runtime::IInspectable);
impl AppBroadcastingMonitor {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppBroadcastingMonitor, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsCurrentAppBroadcasting(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_AppBroadcasting`, `Foundation`*"]
    pub fn IsCurrentAppBroadcastingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Media_AppBroadcasting`, `Foundation`*"]
    pub fn RemoveIsCurrentAppBroadcastingChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastingMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingMonitor;{00f95a68-8907-48a0-b8ef-24d208137542})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(16341608, 35079, 18592, [184, 239, 36, 210, 8, 19, 117, 66]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
}
unsafe impl ::std::marker::Send for AppBroadcastingMonitor {}
unsafe impl ::std::marker::Sync for AppBroadcastingMonitor {}
#[doc = "*Required features: `Media_AppBroadcasting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastingStatus(::windows::runtime::IInspectable);
impl AppBroadcastingStatus {
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn CanStartBroadcast(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<AppBroadcastingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingStatusDetails>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastingStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatus;{1225e4df-03a1-42f8-8b80-c9228cd9cf2e})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(304473311, 929, 17144, [139, 128, 201, 34, 140, 217, 207, 46]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatus";
}
unsafe impl ::std::marker::Send for AppBroadcastingStatus {}
unsafe impl ::std::marker::Sync for AppBroadcastingStatus {}
#[doc = "*Required features: `Media_AppBroadcasting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastingStatusDetails(::windows::runtime::IInspectable);
impl AppBroadcastingStatusDetails {
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsGameStreamInProgress(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsAppInactive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsBlockedForApp(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsDisabledByUser(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn IsDisabledBySystem(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastingStatusDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails;{069dada4-b573-4e3c-8e19-1bafacd09713})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(110996900, 46451, 20028, [142, 25, 27, 175, 172, 208, 151, 19]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
}
unsafe impl ::std::marker::Send for AppBroadcastingStatusDetails {}
unsafe impl ::std::marker::Sync for AppBroadcastingStatusDetails {}
#[doc = "*Required features: `Media_AppBroadcasting`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AppBroadcastingUI(::windows::runtime::IInspectable);
impl AppBroadcastingUI {
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn GetStatus(&self) -> ::windows::runtime::Result<AppBroadcastingStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingStatus>(result__)
        }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn ShowBroadcastUI(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Media_AppBroadcasting`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Media_AppBroadcasting`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    pub fn IAppBroadcastingUIStatics<R, F: FnOnce(&IAppBroadcastingUIStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AppBroadcastingUI, IAppBroadcastingUIStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AppBroadcastingUI {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingUI;{e56f9f8f-ee99-4dca-a3c3-70af3db44f5f})");
}
unsafe impl ::windows::runtime::Interface for AppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3849297807, 61081, 19914, [163, 195, 112, 175, 61, 180, 79, 95]);
}
impl ::windows::runtime::RuntimeName for AppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingUI";
}
unsafe impl ::std::marker::Send for AppBroadcastingUI {}
unsafe impl ::std::marker::Sync for AppBroadcastingUI {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastingMonitor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(16341608, 35079, 18592, [184, 239, 36, 210, 8, 19, 117, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastingStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(304473311, 929, 17144, [139, 128, 201, 34, 140, 217, 207, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastingStatusDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(110996900, 46451, 20028, [142, 25, 27, 175, 172, 208, 151, 19]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatusDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastingUI(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3849297807, 61081, 19914, [163, 195, 112, 175, 61, 180, 79, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IAppBroadcastingUIStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAppBroadcastingUIStatics {
    type Vtable = IAppBroadcastingUIStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1437116317, 9163, 17785, [156, 52, 136, 111, 224, 44, 4, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUIStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
