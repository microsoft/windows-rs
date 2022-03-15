#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingMonitor(::windows::core::IUnknown);
impl AppBroadcastingMonitor {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastingMonitor, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsCurrentAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAppBroadcasting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppBroadcastingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAppBroadcastingChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsCurrentAppBroadcastingChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsCurrentAppBroadcastingChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AppBroadcastingMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingMonitor {}
impl ::core::fmt::Debug for AppBroadcastingMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingMonitor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastingMonitor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingMonitor;{00f95a68-8907-48a0-b8ef-24d208137542})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastingMonitor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
}
impl ::core::convert::From<AppBroadcastingMonitor> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastingMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingMonitor> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastingMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastingMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastingMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingMonitor> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastingMonitor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingMonitor> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastingMonitor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastingMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastingMonitor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingMonitor {}
unsafe impl ::core::marker::Sync for AppBroadcastingMonitor {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatus(::windows::core::IUnknown);
impl AppBroadcastingStatus {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn CanStartBroadcast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanStartBroadcast)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn Details(&self) -> ::windows::core::Result<AppBroadcastingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Details)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingStatusDetails>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingStatus {}
impl ::core::fmt::Debug for AppBroadcastingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatus;{1225e4df-03a1-42f8-8b80-c9228cd9cf2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastingStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatus";
}
impl ::core::convert::From<AppBroadcastingStatus> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatus> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingStatus> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastingStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatus> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastingStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastingStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingStatus {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatus {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatusDetails(::windows::core::IUnknown);
impl AppBroadcastingStatusDetails {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAnyAppBroadcasting)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCaptureResourceUnavailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGameStreamInProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsGpuConstrained)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsAppInactive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAppInactive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsBlockedForApp(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlockedForApp)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsDisabledByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledBySystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingStatusDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingStatusDetails {}
impl ::core::fmt::Debug for AppBroadcastingStatusDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingStatusDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastingStatusDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails;{069dada4-b573-4e3c-8e19-1bafacd09713})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastingStatusDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
}
impl ::core::convert::From<AppBroadcastingStatusDetails> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatusDetails> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingStatusDetails> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastingStatusDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingStatusDetails> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastingStatusDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastingStatusDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingStatusDetails {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatusDetails {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingUI(::windows::core::IUnknown);
impl AppBroadcastingUI {
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn GetStatus(&self) -> ::windows::core::Result<AppBroadcastingStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn ShowBroadcastUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowBroadcastUI)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
    pub fn GetDefault() -> ::windows::core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    #[doc = "*Required features: `\"Media_AppBroadcasting\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<AppBroadcastingUI>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastingUIStatics<R, F: FnOnce(&IAppBroadcastingUIStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppBroadcastingUI, IAppBroadcastingUIStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppBroadcastingUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppBroadcastingUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppBroadcastingUI {}
impl ::core::fmt::Debug for AppBroadcastingUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppBroadcastingUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppBroadcastingUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingUI;{e56f9f8f-ee99-4dca-a3c3-70af3db44f5f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
    const IID: ::windows::core::GUID = <IAppBroadcastingUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingUI";
}
impl ::core::convert::From<AppBroadcastingUI> for ::windows::core::IUnknown {
    fn from(value: AppBroadcastingUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingUI> for ::windows::core::IUnknown {
    fn from(value: &AppBroadcastingUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppBroadcastingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AppBroadcastingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppBroadcastingUI> for ::windows::core::IInspectable {
    fn from(value: AppBroadcastingUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppBroadcastingUI> for ::windows::core::IInspectable {
    fn from(value: &AppBroadcastingUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppBroadcastingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AppBroadcastingUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppBroadcastingUI {}
unsafe impl ::core::marker::Sync for AppBroadcastingUI {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f95a68_8907_48a0_b8ef_24d208137542);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingMonitor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCurrentAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsCurrentAppBroadcastingChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveIsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveIsCurrentAppBroadcastingChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1225e4df_03a1_42f8_8b80_c9228cd9cf2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanStartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingStatusDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x069dada4_b573_4e3c_8e19_1bafacd09713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatusDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsAnyAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCaptureResourceUnavailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGameStreamInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsGpuConstrained: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsAppInactive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlockedForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDisabledBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe56f9f8f_ee99_4dca_a3c3_70af3db44f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingUIStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingUIStatics {
    type Vtable = IAppBroadcastingUIStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55a8a79d_23cb_4579_9c34_886fe02c045a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUIStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
