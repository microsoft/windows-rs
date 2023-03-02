#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingMonitor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastingMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastingMonitor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00f95a68_8907_48a0_b8ef_24d208137542);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingMonitor_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCurrentAppBroadcasting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub IsCurrentAppBroadcastingChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IAppBroadcastingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastingStatus {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1225e4df_03a1_42f8_8b80_c9228cd9cf2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatus_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CanStartBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingStatusDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastingStatusDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x069dada4_b573_4e3c_8e19_1bafacd09713);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingStatusDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
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
}
impl ::core::clone::Clone for IAppBroadcastingUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastingUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe56f9f8f_ee99_4dca_a3c3_70af3db44f5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowBroadcastUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppBroadcastingUIStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppBroadcastingUIStatics {
    type Vtable = IAppBroadcastingUIStatics_Vtbl;
}
impl ::core::clone::Clone for IAppBroadcastingUIStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppBroadcastingUIStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x55a8a79d_23cb_4579_9c34_886fe02c045a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppBroadcastingUIStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingMonitor(::windows::core::IUnknown);
impl AppBroadcastingMonitor {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppBroadcastingMonitor, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsCurrentAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsCurrentAppBroadcasting)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsCurrentAppBroadcastingChanged(&self, handler: &super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).IsCurrentAppBroadcastingChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveIsCurrentAppBroadcastingChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveIsCurrentAppBroadcastingChanged)(::windows::core::Interface::as_raw(this), token).ok() }
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
impl ::windows::core::RuntimeType for AppBroadcastingMonitor {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingMonitor;{00f95a68-8907-48a0-b8ef-24d208137542})");
}
impl ::core::clone::Clone for AppBroadcastingMonitor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingMonitor {
    type Vtable = IAppBroadcastingMonitor_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastingMonitor {
    const IID: ::windows::core::GUID = <IAppBroadcastingMonitor as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingMonitor {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingMonitor";
}
::windows::imp::interface_hierarchy!(AppBroadcastingMonitor, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastingMonitor {}
unsafe impl ::core::marker::Sync for AppBroadcastingMonitor {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatus(::windows::core::IUnknown);
impl AppBroadcastingStatus {
    pub fn CanStartBroadcast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanStartBroadcast)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Details(&self) -> ::windows::core::Result<AppBroadcastingStatusDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastingStatusDetails>();
            (::windows::core::Interface::vtable(this).Details)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for AppBroadcastingStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatus;{1225e4df-03a1-42f8-8b80-c9228cd9cf2e})");
}
impl ::core::clone::Clone for AppBroadcastingStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingStatus {
    type Vtable = IAppBroadcastingStatus_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastingStatus {
    const IID: ::windows::core::GUID = <IAppBroadcastingStatus as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingStatus {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatus";
}
::windows::imp::interface_hierarchy!(AppBroadcastingStatus, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastingStatus {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatus {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingStatusDetails(::windows::core::IUnknown);
impl AppBroadcastingStatusDetails {
    pub fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAnyAppBroadcasting)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsCaptureResourceUnavailable)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsGameStreamInProgress)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsGpuConstrained(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsGpuConstrained)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAppInactive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAppInactive)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsBlockedForApp(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsBlockedForApp)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledByUser(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDisabledByUser)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsDisabledBySystem)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for AppBroadcastingStatusDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails;{069dada4-b573-4e3c-8e19-1bafacd09713})");
}
impl ::core::clone::Clone for AppBroadcastingStatusDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingStatusDetails {
    type Vtable = IAppBroadcastingStatusDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastingStatusDetails {
    const IID: ::windows::core::GUID = <IAppBroadcastingStatusDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingStatusDetails {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingStatusDetails";
}
::windows::imp::interface_hierarchy!(AppBroadcastingStatusDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastingStatusDetails {}
unsafe impl ::core::marker::Sync for AppBroadcastingStatusDetails {}
#[doc = "*Required features: `\"Media_AppBroadcasting\"`*"]
#[repr(transparent)]
pub struct AppBroadcastingUI(::windows::core::IUnknown);
impl AppBroadcastingUI {
    pub fn GetStatus(&self) -> ::windows::core::Result<AppBroadcastingStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastingStatus>();
            (::windows::core::Interface::vtable(this).GetStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ShowBroadcastUI(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowBroadcastUI)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn GetDefault() -> ::windows::core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastingUI>();
            (::windows::core::Interface::vtable(this).GetDefault)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::System::User) -> ::windows::core::Result<AppBroadcastingUI> {
        Self::IAppBroadcastingUIStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppBroadcastingUI>();
            (::windows::core::Interface::vtable(this).GetForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppBroadcastingUIStatics<R, F: FnOnce(&IAppBroadcastingUIStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppBroadcastingUI, IAppBroadcastingUIStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for AppBroadcastingUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Media.AppBroadcasting.AppBroadcastingUI;{e56f9f8f-ee99-4dca-a3c3-70af3db44f5f})");
}
impl ::core::clone::Clone for AppBroadcastingUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppBroadcastingUI {
    type Vtable = IAppBroadcastingUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppBroadcastingUI {
    const IID: ::windows::core::GUID = <IAppBroadcastingUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppBroadcastingUI {
    const NAME: &'static str = "Windows.Media.AppBroadcasting.AppBroadcastingUI";
}
::windows::imp::interface_hierarchy!(AppBroadcastingUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for AppBroadcastingUI {}
unsafe impl ::core::marker::Sync for AppBroadcastingUI {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
