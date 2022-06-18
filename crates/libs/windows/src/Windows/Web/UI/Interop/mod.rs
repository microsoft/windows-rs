#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77a2a53e_7c74_437d_a290_3ac0d8cd5655);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IWebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b2a340d_4bd0_405e_b7c1_1e72a492f446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlMoveFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlMoveFocusReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlProcess(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02c723ec_98d6_424a_b63e_c6136c36a0f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcess_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IWebViewControlProcessFactory {
    type Vtable = IWebViewControlProcessFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47b65cf9_a2d2_453c_b097_f6779d4b8e02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlProcessOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cca72a7_3bd6_4826_8261_6c8189505d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessOptions_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT,
    pub PrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlProcessCapabilityState) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebViewControlSite(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebViewControlSite {
    type Vtable = IWebViewControlSite_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x133f47c6_12dc_4898_bd47_04967de648ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
unsafe impl ::windows::core::Interface for IWebViewControlSite2 {
    type Vtable = IWebViewControlSite2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd13b2e3f_48ee_4730_8243_d2ed0c05606a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Source(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DocumentTitle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanGoBack)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanGoForward)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::UI::Color>::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::UI::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Settings(&self) -> ::windows::core::Result<super::WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Settings)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WebViewControlSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DeferredPermissionRequests)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::WebViewControlDeferredPermissionRequest>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GoForward)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GoBack)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Refresh)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Navigate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, source: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Navigate)(::windows::core::Interface::as_raw(this), source.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn NavigateToString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToString)(::windows::core::Interface::as_raw(this), text.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateToLocalStreamUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::IUriToStreamResolver>>(&self, source: Param0, streamresolver: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToLocalStreamUri)(::windows::core::Interface::as_raw(this), source.into_param().abi(), streamresolver.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Web_Http\"`*"]
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Http::HttpRequestMessage>>(&self, requestmessage: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::windows::core::Interface::as_raw(this), requestmessage.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, scriptname: Param0, arguments: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).InvokeScriptAsync)(::windows::core::Interface::as_raw(this), scriptname.into_param().abi(), arguments.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CapturePreviewToStreamAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStream>>(&self, stream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CapturePreviewToStreamAsync)(::windows::core::Interface::as_raw(this), stream.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BuildLocalStreamUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, contentidentifier: Param0, relativepath: Param1) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).BuildLocalStreamUri)(::windows::core::Interface::as_raw(this), contentidentifier.into_param().abi(), relativepath.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<super::WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetDeferredPermissionRequestById)(::windows::core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).NavigationStarting)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationStarting)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ContentLoading)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentLoading)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).DOMContentLoaded)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDOMContentLoaded)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).NavigationCompleted)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationCompleted)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationStarting)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationStarting<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationStarting)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).FrameContentLoading)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameContentLoading<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameContentLoading)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).FrameDOMContentLoaded)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameDOMContentLoaded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationCompleted)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScriptNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlScriptNotifyEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ScriptNotify)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScriptNotify<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScriptNotify)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LongRunningScriptDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlLongRunningScriptDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).LongRunningScriptDetected)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLongRunningScriptDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsafeContentWarningDisplaying<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsafeContentWarningDisplaying<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnviewableContentIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnviewableContentIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).UnviewableContentIdentified)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnviewableContentIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PermissionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlPermissionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).PermissionRequested)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePermissionRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePermissionRequested)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UnsupportedUriSchemeIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsupportedUriSchemeIdentified<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewWindowRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNewWindowRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).NewWindowRequested)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWindowRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNewWindowRequested)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsFullScreenElementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElementChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContainsFullScreenElementChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn WebResourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlWebResourceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).WebResourceRequested)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWebResourceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveWebResourceRequested)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn AddInitializeScript<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, script: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::IWebViewControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddInitializeScript)(::windows::core::Interface::as_raw(this), script.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Process(&self) -> ::windows::core::Result<WebViewControlProcess> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Process)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlProcess>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn SetScale(&self, value: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetScale)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Scale(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).Scale)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBounds)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::Rect>::zeroed();
            (::windows::core::Interface::vtable(this).Bounds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsVisible)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsVisible)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).MoveFocus)(::windows::core::Interface::as_raw(this), reason).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).MoveFocusRequested)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMoveFocusRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMoveFocusRequested)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).AcceleratorKeyPressed)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAcceleratorKeyPressed)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).GotFocus)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGotFocus)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).LostFocus)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IWebViewControlSite2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLostFocus)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControl {
    type Vtable = super::IWebViewControl_Vtbl;
    const IID: ::windows::core::GUID = <super::IWebViewControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControl {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControl";
}
impl ::core::convert::From<WebViewControl> for ::windows::core::IUnknown {
    fn from(value: WebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControl> for ::windows::core::IUnknown {
    fn from(value: &WebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControl> for ::windows::core::IInspectable {
    fn from(value: WebViewControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControl> for ::windows::core::IInspectable {
    fn from(value: &WebViewControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::IWebViewControl> for WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWebViewControl> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IWebViewControl> for &WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWebViewControl> {
        ::core::convert::TryInto::<super::IWebViewControl>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::IWebViewControl2> for WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWebViewControl2> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IWebViewControl2> for &WebViewControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWebViewControl2> {
        ::core::convert::TryInto::<super::IWebViewControl2>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
pub struct WebViewControlAcceleratorKeyPressedEventArgs(::windows::core::IUnknown);
impl WebViewControlAcceleratorKeyPressedEventArgs {
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn EventType(&self) -> ::windows::core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::UI::Core::CoreAcceleratorKeyEventType>::zeroed();
            (::windows::core::Interface::vtable(this).EventType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::UI::Core::CoreAcceleratorKeyEventType>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::System::VirtualKey>::zeroed();
            (::windows::core::Interface::vtable(this).VirtualKey)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::System::VirtualKey>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows::core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::UI::Core::CorePhysicalKeyStatus>::zeroed();
            (::windows::core::Interface::vtable(this).KeyStatus)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::UI::Core::CorePhysicalKeyStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn RoutingStage(&self) -> ::windows::core::Result<WebViewControlAcceleratorKeyRoutingStage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlAcceleratorKeyRoutingStage>::zeroed();
            (::windows::core::Interface::vtable(this).RoutingStage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlAcceleratorKeyRoutingStage>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Handled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Handled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn SetHandled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHandled)(::windows::core::Interface::as_raw(this), value).ok() }
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
unsafe impl ::windows::core::Interface for WebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlAcceleratorKeyPressedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlAcceleratorKeyPressedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs";
}
impl ::core::convert::From<WebViewControlAcceleratorKeyPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlAcceleratorKeyPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlAcceleratorKeyPressedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlAcceleratorKeyPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlAcceleratorKeyPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlAcceleratorKeyPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlAcceleratorKeyPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlAcceleratorKeyPressedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlAcceleratorKeyPressedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlAcceleratorKeyPressedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlAcceleratorKeyPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlAcceleratorKeyPressedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct WebViewControlMoveFocusRequestedEventArgs(::windows::core::IUnknown);
impl WebViewControlMoveFocusRequestedEventArgs {
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<WebViewControlMoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlMoveFocusReason>::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlMoveFocusReason>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlMoveFocusRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlMoveFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs";
}
impl ::core::convert::From<WebViewControlMoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebViewControlMoveFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlMoveFocusRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlMoveFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlMoveFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlMoveFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlMoveFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebViewControlMoveFocusRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlMoveFocusRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlMoveFocusRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlMoveFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlMoveFocusRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn ProcessId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).ProcessId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsPrivateNetworkClientServerCapabilityEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebViewControlAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, hostwindowhandle: i64, bounds: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWebViewControlAsync)(::windows::core::Interface::as_raw(this), hostwindowhandle, bounds.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<WebViewControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWebViewControls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetWebViewControls)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<WebViewControl>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn Terminate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Terminate)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProcessExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ProcessExited)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProcessExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveProcessExited)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn CreateWithOptions<'a, Param0: ::windows::core::IntoParam<'a, WebViewControlProcessOptions>>(processoptions: Param0) -> ::windows::core::Result<WebViewControlProcess> {
        Self::IWebViewControlProcessFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithOptions)(::windows::core::Interface::as_raw(this), processoptions.into_param().abi(), result__.as_mut_ptr()).from_abi::<WebViewControlProcess>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlProcess as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlProcess {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcess";
}
impl ::core::convert::From<WebViewControlProcess> for ::windows::core::IUnknown {
    fn from(value: WebViewControlProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlProcess> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlProcess> for ::windows::core::IInspectable {
    fn from(value: WebViewControlProcess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlProcess> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlProcess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlProcess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Web_UI_Interop\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn SetEnterpriseId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnterpriseId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrivateNetworkClientServerCapability)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI_Interop\"`*"]
    pub fn PrivateNetworkClientServerCapability(&self) -> ::windows::core::Result<WebViewControlProcessCapabilityState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<WebViewControlProcessCapabilityState>::zeroed();
            (::windows::core::Interface::vtable(this).PrivateNetworkClientServerCapability)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebViewControlProcessCapabilityState>(result__)
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
unsafe impl ::windows::core::Interface for WebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
    const IID: ::windows::core::GUID = <IWebViewControlProcessOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebViewControlProcessOptions {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcessOptions";
}
impl ::core::convert::From<WebViewControlProcessOptions> for ::windows::core::IUnknown {
    fn from(value: WebViewControlProcessOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlProcessOptions> for ::windows::core::IUnknown {
    fn from(value: &WebViewControlProcessOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebViewControlProcessOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebViewControlProcessOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebViewControlProcessOptions> for ::windows::core::IInspectable {
    fn from(value: WebViewControlProcessOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebViewControlProcessOptions> for ::windows::core::IInspectable {
    fn from(value: &WebViewControlProcessOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebViewControlProcessOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebViewControlProcessOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
