#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlAcceleratorKeyPressedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a2a53e_7c74_437d_a290_3ac0d8cd5655);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CoreAcceleratorKeyEventType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    EventType: usize,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub RoutingStage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlAcceleratorKeyRoutingStage) -> ::windows::core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlMoveFocusRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlMoveFocusRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2a340d_4bd0_405e_b7c1_1e72a492f446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlMoveFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlMoveFocusReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlProcess(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlProcess {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02c723ec_98d6_424a_b63e_c6136c36a0f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcess_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsPrivateNetworkClientServerCapabilityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWebViewControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebViewControlAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWebViewControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWebViewControls: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlProcessFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlProcessFactory {
    type Vtable = IWebViewControlProcessFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlProcessFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47b65cf9_a2d2_453c_b097_f6779d4b8e02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlProcessOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlProcessOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cca72a7_3bd6_4826_8261_6c8189505d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT,
    pub PrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSite(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlSite {
    type Vtable = IWebViewControlSite_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlSite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x133f47c6_12dc_4898_bd47_04967de648ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: WebViewControlMoveFocusReason) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoveFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AcceleratorKeyPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceleratorKeyPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAcceleratorKeyPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSite2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebViewControlSite2 {
    type Vtable = IWebViewControlSite2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebViewControlSite2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd13b2e3f_48ee_4730_8243_d2ed0c05606a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControl(::windows::core::IUnknown);
impl WebViewControl {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Source(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Source)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSource(&self, source: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSource)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source)).ok() }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentTitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanGoBack)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanGoForward)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDefaultBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DefaultBackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsFullScreenElement)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Settings(&self) -> ::windows::core::Result<super::WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Settings)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeferredPermissionRequests)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GoForward)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GoBack)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Refresh)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Stop)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Navigate(&self, source: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Navigate)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source)).ok() }
    }
    pub fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateToString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateToLocalStreamUri<P0, E0>(&self, source: &super::super::super::Foundation::Uri, streamresolver: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::IUriToStreamResolver>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateToLocalStreamUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), streamresolver.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage(&self, requestmessage: &super::super::Http::HttpRequestMessage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).NavigateWithHttpRequestMessage)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(requestmessage)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<P0, E0>(&self, scriptname: &::windows::core::HSTRING, arguments: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InvokeScriptAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(scriptname), arguments.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CapturePreviewToStreamAsync<P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapturePreviewToStreamAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BuildLocalStreamUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(contentidentifier), ::core::mem::transmute_copy(relativepath), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<super::WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetDeferredPermissionRequestById)(::windows::core::Vtable::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarting(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigationStarting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNavigationStarting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLoading(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentLoading)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContentLoading)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DOMContentLoaded(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DOMContentLoaded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDOMContentLoaded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NavigationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNavigationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationStarting(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameNavigationStarting)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameNavigationStarting)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameContentLoading(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameContentLoading)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameContentLoading)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDOMContentLoaded(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameDOMContentLoaded)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameDOMContentLoaded)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationCompleted(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameNavigationCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveFrameNavigationCompleted)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScriptNotify(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlScriptNotifyEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScriptNotify)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScriptNotify(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveScriptNotify)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LongRunningScriptDetected(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlLongRunningScriptDetectedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LongRunningScriptDetected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLongRunningScriptDetected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsafeContentWarningDisplaying(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnsafeContentWarningDisplaying)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnviewableContentIdentified(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnviewableContentIdentifiedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnviewableContentIdentified)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnviewableContentIdentified)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PermissionRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlPermissionRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PermissionRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePermissionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePermissionRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsupportedUriSchemeIdentified(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnsupportedUriSchemeIdentified)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewWindowRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNewWindowRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NewWindowRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWindowRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveNewWindowRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsFullScreenElementChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContainsFullScreenElementChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveContainsFullScreenElementChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WebResourceRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlWebResourceRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WebResourceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWebResourceRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveWebResourceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn AddInitializeScript(&self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IWebViewControl2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).AddInitializeScript)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(script)).ok() }
    }
    pub fn Process(&self) -> ::windows::core::Result<WebViewControlProcess> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Process)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetScale)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Scale)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBounds(&self, value: super::super::super::Foundation::Rect) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetBounds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsVisible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).MoveFocus)(::windows::core::Vtable::as_raw(this), reason).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveFocusRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MoveFocusRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMoveFocusRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMoveFocusRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyPressed(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AcceleratorKeyPressed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyPressed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveAcceleratorKeyPressed)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GotFocus)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveGotFocus)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LostFocus)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveLostFocus)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for WebViewControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControl {}
impl ::core::fmt::Debug for WebViewControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControl;{3f921316-bc70-4bda-9136-c94370899fab})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControl {
    type Vtable = super::IWebViewControl_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControl {
    const IID: ::windows::core::GUID = <super::IWebViewControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControl {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControl";
}
::windows::core::interface_hierarchy!(WebViewControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WebViewControl> for super::IWebViewControl {
    type Error = ::windows::core::Error;
    fn try_from(value: WebViewControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebViewControl> for super::IWebViewControl {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebViewControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebViewControl> for ::windows::core::InParam<super::IWebViewControl> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebViewControl) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebViewControl> for super::IWebViewControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebViewControl) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebViewControl> for super::IWebViewControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebViewControl) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebViewControl> for ::windows::core::InParam<super::IWebViewControl2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebViewControl) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlAcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
impl WebViewControlAcceleratorKeyPressedEventArgs {
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn EventType(&self) -> ::windows::core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EventType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VirtualKey)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).KeyStatus)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RoutingStage(&self) -> ::windows::core::Result<WebViewControlAcceleratorKeyRoutingStage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RoutingStage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Handled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetHandled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for WebViewControlAcceleratorKeyPressedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlAcceleratorKeyPressedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlAcceleratorKeyPressedEventArgs {}
impl ::core::fmt::Debug for WebViewControlAcceleratorKeyPressedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlAcceleratorKeyPressedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlAcceleratorKeyPressedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs;{77a2a53e-7c74-437d-a290-3ac0d8cd5655})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlAcceleratorKeyPressedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlAcceleratorKeyPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlAcceleratorKeyPressedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlAcceleratorKeyPressedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlMoveFocusRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlMoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<WebViewControlMoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlMoveFocusRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlMoveFocusRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlMoveFocusRequestedEventArgs {}
impl ::core::fmt::Debug for WebViewControlMoveFocusRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlMoveFocusRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlMoveFocusRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs;{6b2a340d-4bd0-405e-b7c1-1e72a492f446})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlMoveFocusRequestedEventArgs {
    const IID: ::windows::core::GUID = <IWebViewControlMoveFocusRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlMoveFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs";
}
::windows::core::interface_hierarchy!(WebViewControlMoveFocusRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlProcess(::windows::core::IUnknown);
impl WebViewControlProcess {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebViewControlProcess, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnterpriseId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrivateNetworkClientServerCapabilityEnabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebViewControlAsync(&self, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWebViewControlAsync)(::windows::core::Vtable::as_raw(this), hostwindowhandle, bounds, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWebViewControls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetWebViewControls)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Terminate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Terminate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProcessExited(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProcessExited)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProcessExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveProcessExited)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn CreateWithOptions(processoptions: &WebViewControlProcessOptions) -> ::windows::core::Result<WebViewControlProcess> {
        Self::IWebViewControlProcessFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(processoptions), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebViewControlProcessFactory<R, F: FnOnce(&IWebViewControlProcessFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebViewControlProcess, IWebViewControlProcessFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebViewControlProcess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlProcess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlProcess {}
impl ::core::fmt::Debug for WebViewControlProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlProcess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlProcess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlProcess;{02c723ec-98d6-424a-b63e-c6136c36a0f2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlProcess {
    const IID: ::windows::core::GUID = <IWebViewControlProcess as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlProcess {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcess";
}
::windows::core::interface_hierarchy!(WebViewControlProcess, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlProcessOptions(::windows::core::IUnknown);
impl WebViewControlProcessOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebViewControlProcessOptions, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnterpriseId)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EnterpriseId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrivateNetworkClientServerCapability)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn PrivateNetworkClientServerCapability(&self) -> ::windows::core::Result<WebViewControlProcessCapabilityState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrivateNetworkClientServerCapability)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebViewControlProcessOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebViewControlProcessOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebViewControlProcessOptions {}
impl ::core::fmt::Debug for WebViewControlProcessOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlProcessOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlProcessOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlProcessOptions;{1cca72a7-3bd6-4826-8261-6c8189505d89})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for WebViewControlProcessOptions {
    const IID: ::windows::core::GUID = <IWebViewControlProcessOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlProcessOptions {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcessOptions";
}
::windows::core::interface_hierarchy!(WebViewControlProcessOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlAcceleratorKeyRoutingStage(pub i32);
impl WebViewControlAcceleratorKeyRoutingStage {
    pub const Tunneling: Self = Self(0i32);
    pub const Bubbling: Self = Self(1i32);
}
impl ::core::marker::Copy for WebViewControlAcceleratorKeyRoutingStage {}
impl ::core::clone::Clone for WebViewControlAcceleratorKeyRoutingStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlAcceleratorKeyRoutingStage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebViewControlAcceleratorKeyRoutingStage {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlAcceleratorKeyRoutingStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlAcceleratorKeyRoutingStage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlAcceleratorKeyRoutingStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlAcceleratorKeyRoutingStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlMoveFocusReason {}
impl ::core::clone::Clone for WebViewControlMoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlMoveFocusReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebViewControlMoveFocusReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlMoveFocusReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlMoveFocusReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlMoveFocusReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlMoveFocusReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlProcessCapabilityState {}
impl ::core::clone::Clone for WebViewControlProcessCapabilityState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlProcessCapabilityState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebViewControlProcessCapabilityState {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebViewControlProcessCapabilityState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlProcessCapabilityState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebViewControlProcessCapabilityState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlProcessCapabilityState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
