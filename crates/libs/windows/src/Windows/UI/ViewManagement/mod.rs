#[cfg(feature = "UI_ViewManagement_Core")]
pub mod Core;
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct AccessibilitySettings(::windows::core::IUnknown);
impl AccessibilitySettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<AccessibilitySettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn HighContrast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HighContrast)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn HighContrastScheme(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).HighContrastScheme)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HighContrastChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).HighContrastChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHighContrastChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHighContrastChanged)(::windows::core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for AccessibilitySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AccessibilitySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AccessibilitySettings {}
impl ::core::fmt::Debug for AccessibilitySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessibilitySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AccessibilitySettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.AccessibilitySettings;{fe0e8147-c4c0-4562-b962-1327b52ad5b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AccessibilitySettings {
    type Vtable = IAccessibilitySettings_Vtbl;
    const IID: ::windows::core::GUID = <IAccessibilitySettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AccessibilitySettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.AccessibilitySettings";
}
impl ::core::convert::From<AccessibilitySettings> for ::windows::core::IUnknown {
    fn from(value: AccessibilitySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessibilitySettings> for ::windows::core::IUnknown {
    fn from(value: &AccessibilitySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AccessibilitySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AccessibilitySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AccessibilitySettings> for ::windows::core::IInspectable {
    fn from(value: AccessibilitySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AccessibilitySettings> for ::windows::core::IInspectable {
    fn from(value: &AccessibilitySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AccessibilitySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AccessibilitySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AccessibilitySettings {}
unsafe impl ::core::marker::Sync for AccessibilitySettings {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ActivationViewSwitcher(::windows::core::IUnknown);
impl ActivationViewSwitcher {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsStandaloneAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsStandaloneAsync)(::windows::core::Interface::as_raw(this), viewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsStandaloneWithSizePreferenceAsync)(::windows::core::Interface::as_raw(this), viewid, sizepreference, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsViewPresentedOnActivationVirtualDesktop(&self, viewid: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsViewPresentedOnActivationVirtualDesktop)(::windows::core::Interface::as_raw(this), viewid, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationViewSwitcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationViewSwitcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationViewSwitcher {}
impl ::core::fmt::Debug for ActivationViewSwitcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationViewSwitcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationViewSwitcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ActivationViewSwitcher;{dca71bb6-7350-492b-aac7-c8a13d7224ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivationViewSwitcher {
    type Vtable = IActivationViewSwitcher_Vtbl;
    const IID: ::windows::core::GUID = <IActivationViewSwitcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.ActivationViewSwitcher";
}
impl ::core::convert::From<ActivationViewSwitcher> for ::windows::core::IUnknown {
    fn from(value: ActivationViewSwitcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationViewSwitcher> for ::windows::core::IUnknown {
    fn from(value: &ActivationViewSwitcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationViewSwitcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationViewSwitcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationViewSwitcher> for ::windows::core::IInspectable {
    fn from(value: ActivationViewSwitcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationViewSwitcher> for ::windows::core::IInspectable {
    fn from(value: &ActivationViewSwitcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationViewSwitcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationViewSwitcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivationViewSwitcher {}
unsafe impl ::core::marker::Sync for ActivationViewSwitcher {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationView(::windows::core::IUnknown);
impl ApplicationView {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn Orientation(&self) -> ::windows::core::Result<ApplicationViewOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationViewOrientation>::zeroed();
            (::windows::core::Interface::vtable(this).Orientation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewOrientation>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn AdjacentToLeftDisplayEdge(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).AdjacentToLeftDisplayEdge)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn AdjacentToRightDisplayEdge(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).AdjacentToRightDisplayEdge)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn IsFullScreen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsFullScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsOnLockScreen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsOnLockScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsScreenCaptureEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsScreenCaptureEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetIsScreenCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsScreenCaptureEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Consolidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Consolidated)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConsolidated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConsolidated)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SuppressSystemOverlays(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).SuppressSystemOverlays)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn SetSuppressSystemOverlays(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSuppressSystemOverlays)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibleBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows::core::Interface::vtable(this).VisibleBounds)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VisibleBoundsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).VisibleBoundsChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVisibleBoundsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVisibleBoundsChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetDesiredBoundsMode(&self, boundsmode: ApplicationViewBoundsMode) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).SetDesiredBoundsMode)(::windows::core::Interface::as_raw(this), boundsmode, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DesiredBoundsMode(&self) -> ::windows::core::Result<ApplicationViewBoundsMode> {
        let this = &::windows::core::Interface::cast::<IApplicationView2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationViewBoundsMode>::zeroed();
            (::windows::core::Interface::vtable(this).DesiredBoundsMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewBoundsMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TitleBar(&self) -> ::windows::core::Result<ApplicationViewTitleBar> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TitleBar)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewTitleBar>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn FullScreenSystemOverlayMode(&self) -> ::windows::core::Result<FullScreenSystemOverlayMode> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<FullScreenSystemOverlayMode>::zeroed();
            (::windows::core::Interface::vtable(this).FullScreenSystemOverlayMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FullScreenSystemOverlayMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetFullScreenSystemOverlayMode(&self, value: FullScreenSystemOverlayMode) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFullScreenSystemOverlayMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsFullScreenMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsFullScreenMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TryEnterFullScreenMode(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryEnterFullScreenMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ExitFullScreenMode(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ExitFullScreenMode)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ShowStandardSystemOverlays(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ShowStandardSystemOverlays)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryResizeView<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryResizeView)(::windows::core::Interface::as_raw(this), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, minsize: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPreferredMinSize)(::windows::core::Interface::as_raw(this), minsize.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ViewMode(&self) -> ::windows::core::Result<ApplicationViewMode> {
        let this = &::windows::core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationViewMode>::zeroed();
            (::windows::core::Interface::vtable(this).ViewMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsViewModeSupported(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsViewModeSupported)(::windows::core::Interface::as_raw(this), viewmode, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryEnterViewModeAsync(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryEnterViewModeAsync)(::windows::core::Interface::as_raw(this), viewmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryEnterViewModeWithPreferencesAsync<'a, Param1: ::windows::core::IntoParam<'a, ViewModePreferences>>(&self, viewmode: ApplicationViewMode, viewmodepreferences: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryEnterViewModeWithPreferencesAsync)(::windows::core::Interface::as_raw(this), viewmode, viewmodepreferences.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryConsolidateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IApplicationView4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryConsolidateAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IApplicationView7>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).PersistedStateId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetPersistedStateId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IApplicationView7>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPersistedStateId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(feature = "UI_WindowManagement")]
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<super::WindowManagement::WindowingEnvironment> {
        let this = &::windows::core::Interface::cast::<IApplicationView9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).WindowingEnvironment)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WindowManagement::WindowingEnvironment>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation_Collections\"`, `\"UI_WindowManagement\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement"))]
    pub fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>> {
        let this = &::windows::core::Interface::cast::<IApplicationView9>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetDisplayRegions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryUnsnapToFullscreen() -> ::windows::core::Result<bool> {
        Self::IApplicationViewFullscreenStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryUnsnapToFullscreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"UI_Core\"`*"]
    #[cfg(feature = "UI_Core")]
    pub fn GetApplicationViewIdForWindow<'a, Param0: ::windows::core::IntoParam<'a, super::Core::ICoreWindow>>(window: Param0) -> ::windows::core::Result<i32> {
        Self::IApplicationViewInteropStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).GetApplicationViewIdForWindow)(::windows::core::Interface::as_raw(this), window.into_param().abi(), result__.as_mut_ptr()).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn Value() -> ::windows::core::Result<ApplicationViewState> {
        Self::IApplicationViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationViewState>::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewState>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn TryUnsnap() -> ::windows::core::Result<bool> {
        Self::IApplicationViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryUnsnap)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<ApplicationView> {
        Self::IApplicationViewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationView>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TerminateAppOnFinalViewClose() -> ::windows::core::Result<bool> {
        Self::IApplicationViewStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TerminateAppOnFinalViewClose)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetTerminateAppOnFinalViewClose(value: bool) -> ::windows::core::Result<()> {
        Self::IApplicationViewStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).SetTerminateAppOnFinalViewClose)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn PreferredLaunchWindowingMode() -> ::windows::core::Result<ApplicationViewWindowingMode> {
        Self::IApplicationViewStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ApplicationViewWindowingMode>::zeroed();
            (::windows::core::Interface::vtable(this).PreferredLaunchWindowingMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ApplicationViewWindowingMode>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetPreferredLaunchWindowingMode(value: ApplicationViewWindowingMode) -> ::windows::core::Result<()> {
        Self::IApplicationViewStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).SetPreferredLaunchWindowingMode)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredLaunchViewSize() -> ::windows::core::Result<super::super::Foundation::Size> {
        Self::IApplicationViewStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).PreferredLaunchViewSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredLaunchViewSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(value: Param0) -> ::windows::core::Result<()> {
        Self::IApplicationViewStatics3(|this| unsafe { (::windows::core::Interface::vtable(this).SetPreferredLaunchViewSize)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ClearAllPersistedState() -> ::windows::core::Result<()> {
        Self::IApplicationViewStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).ClearAllPersistedState)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ClearPersistedState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(key: Param0) -> ::windows::core::Result<()> {
        Self::IApplicationViewStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).ClearPersistedState)(::windows::core::Interface::as_raw(this), key.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn UIContext(&self) -> ::windows::core::Result<super::UIContext> {
        let this = &::windows::core::Interface::cast::<IApplicationViewWithContext>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).UIContext)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::UIContext>(result__)
        }
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IApplicationViewFullscreenStatics<R, F: FnOnce(&IApplicationViewFullscreenStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewFullscreenStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewInteropStatics<R, F: FnOnce(&IApplicationViewInteropStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewInteropStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IApplicationViewStatics<R, F: FnOnce(&IApplicationViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics2<R, F: FnOnce(&IApplicationViewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics3<R, F: FnOnce(&IApplicationViewStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewStatics4<R, F: FnOnce(&IApplicationViewStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationView, IApplicationViewStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ApplicationView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationView {}
impl ::core::fmt::Debug for ApplicationView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ApplicationView;{d222d519-4361-451e-96c4-60f4f9742db0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationView {
    type Vtable = IApplicationView_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationView {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationView";
}
impl ::core::convert::From<ApplicationView> for ::windows::core::IUnknown {
    fn from(value: ApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationView> for ::windows::core::IUnknown {
    fn from(value: &ApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationView> for ::windows::core::IInspectable {
    fn from(value: ApplicationView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationView> for ::windows::core::IInspectable {
    fn from(value: &ApplicationView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationView {}
unsafe impl ::core::marker::Sync for ApplicationView {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewBoundsMode(pub i32);
impl ApplicationViewBoundsMode {
    pub const UseVisible: Self = Self(0i32);
    pub const UseCoreWindow: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewBoundsMode {}
impl ::core::clone::Clone for ApplicationViewBoundsMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationViewBoundsMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationViewBoundsMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationViewBoundsMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewBoundsMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewBoundsMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewBoundsMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewConsolidatedEventArgs(::windows::core::IUnknown);
impl ApplicationViewConsolidatedEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsUserInitiated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsUserInitiated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn IsAppInitiated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IApplicationViewConsolidatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsAppInitiated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ApplicationViewConsolidatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationViewConsolidatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewConsolidatedEventArgs {}
impl ::core::fmt::Debug for ApplicationViewConsolidatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewConsolidatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewConsolidatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs;{514449ec-7ea2-4de7-a6a6-7dfbaaebb6fb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationViewConsolidatedEventArgs {
    type Vtable = IApplicationViewConsolidatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationViewConsolidatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationViewConsolidatedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewConsolidatedEventArgs";
}
impl ::core::convert::From<ApplicationViewConsolidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ApplicationViewConsolidatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewConsolidatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ApplicationViewConsolidatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationViewConsolidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationViewConsolidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationViewConsolidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ApplicationViewConsolidatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewConsolidatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ApplicationViewConsolidatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationViewConsolidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationViewConsolidatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationViewConsolidatedEventArgs {}
unsafe impl ::core::marker::Sync for ApplicationViewConsolidatedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewMode(pub i32);
impl ApplicationViewMode {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewMode {}
impl ::core::clone::Clone for ApplicationViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationViewMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationViewMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewOrientation(pub i32);
impl ApplicationViewOrientation {
    pub const Landscape: Self = Self(0i32);
    pub const Portrait: Self = Self(1i32);
}
impl ::core::marker::Copy for ApplicationViewOrientation {}
impl ::core::clone::Clone for ApplicationViewOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationViewOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationViewOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationViewOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewOrientation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewScaling(::windows::core::IUnknown);
impl ApplicationViewScaling {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DisableLayoutScaling() -> ::windows::core::Result<bool> {
        Self::IApplicationViewScalingStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).DisableLayoutScaling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TrySetDisableLayoutScaling(disablelayoutscaling: bool) -> ::windows::core::Result<bool> {
        Self::IApplicationViewScalingStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TrySetDisableLayoutScaling)(::windows::core::Interface::as_raw(this), disablelayoutscaling, result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewScalingStatics<R, F: FnOnce(&IApplicationViewScalingStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewScaling, IApplicationViewScalingStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ApplicationViewScaling {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationViewScaling {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewScaling {}
impl ::core::fmt::Debug for ApplicationViewScaling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewScaling").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewScaling {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ApplicationViewScaling;{1d0ddc23-23f3-4b2d-84fe-74bf37b48b66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationViewScaling {
    type Vtable = IApplicationViewScaling_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationViewScaling as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationViewScaling {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewScaling";
}
impl ::core::convert::From<ApplicationViewScaling> for ::windows::core::IUnknown {
    fn from(value: ApplicationViewScaling) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewScaling> for ::windows::core::IUnknown {
    fn from(value: &ApplicationViewScaling) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationViewScaling {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationViewScaling {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationViewScaling> for ::windows::core::IInspectable {
    fn from(value: ApplicationViewScaling) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewScaling> for ::windows::core::IInspectable {
    fn from(value: &ApplicationViewScaling) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationViewScaling {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationViewScaling {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`, `\"deprecated\"`*"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewState(pub i32);
#[cfg(feature = "deprecated")]
impl ApplicationViewState {
    pub const FullScreenLandscape: Self = Self(0i32);
    pub const Filled: Self = Self(1i32);
    pub const Snapped: Self = Self(2i32);
    pub const FullScreenPortrait: Self = Self(3i32);
}
#[cfg(feature = "deprecated")]
impl ::core::marker::Copy for ApplicationViewState {}
#[cfg(feature = "deprecated")]
impl ::core::clone::Clone for ApplicationViewState {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "deprecated")]
impl ::core::default::Default for ApplicationViewState {
    fn default() -> Self {
        Self(0)
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Abi for ApplicationViewState {
    type Abi = Self;
}
#[cfg(feature = "deprecated")]
impl ::core::fmt::Debug for ApplicationViewState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewState").field(&self.0).finish()
    }
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::RuntimeType for ApplicationViewState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
pub struct ApplicationViewSwitcher;
impl ApplicationViewSwitcher {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DisableShowingMainViewOnActivation() -> ::windows::core::Result<()> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe { (::windows::core::Interface::vtable(this).DisableShowingMainViewOnActivation)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsStandaloneAsync(viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryShowAsStandaloneAsync)(::windows::core::Interface::as_raw(this), viewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsStandaloneWithSizePreferenceAsync(viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryShowAsStandaloneWithSizePreferenceAsync)(::windows::core::Interface::as_raw(this), viewid, sizepreference, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync)(::windows::core::Interface::as_raw(this), viewid, sizepreference, anchorviewid, anchorsizepreference, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SwitchAsync(viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SwitchAsync)(::windows::core::Interface::as_raw(this), viewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SwitchFromViewAsync(toviewid: i32, fromviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SwitchFromViewAsync)(::windows::core::Interface::as_raw(this), toviewid, fromviewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SwitchFromViewWithOptionsAsync(toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SwitchFromViewWithOptionsAsync)(::windows::core::Interface::as_raw(this), toviewid, fromviewid, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrepareForCustomAnimatedSwitchAsync(toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).PrepareForCustomAnimatedSwitchAsync)(::windows::core::Interface::as_raw(this), toviewid, fromviewid, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DisableSystemViewActivationPolicy() -> ::windows::core::Result<()> {
        Self::IApplicationViewSwitcherStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).DisableSystemViewActivationPolicy)(::windows::core::Interface::as_raw(this)).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsViewModeAsync(viewid: i32, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryShowAsViewModeAsync)(::windows::core::Interface::as_raw(this), viewid, viewmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsViewModeWithPreferencesAsync<'a, Param2: ::windows::core::IntoParam<'a, ViewModePreferences>>(viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IApplicationViewSwitcherStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TryShowAsViewModeWithPreferencesAsync)(::windows::core::Interface::as_raw(this), viewid, viewmode, viewmodepreferences.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics<R, F: FnOnce(&IApplicationViewSwitcherStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics2<R, F: FnOnce(&IApplicationViewSwitcherStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IApplicationViewSwitcherStatics3<R, F: FnOnce(&IApplicationViewSwitcherStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewSwitcher, IApplicationViewSwitcherStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ApplicationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewSwitcher";
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewSwitchingOptions(pub u32);
impl ApplicationViewSwitchingOptions {
    pub const Default: Self = Self(0u32);
    pub const SkipAnimation: Self = Self(1u32);
    pub const ConsolidateViews: Self = Self(2u32);
}
impl ::core::marker::Copy for ApplicationViewSwitchingOptions {}
impl ::core::clone::Clone for ApplicationViewSwitchingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationViewSwitchingOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationViewSwitchingOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationViewSwitchingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewSwitchingOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ApplicationViewSwitchingOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ApplicationViewSwitchingOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ApplicationViewSwitchingOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewSwitchingOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewSwitchingOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewTitleBar(::windows::core::IUnknown);
impl ApplicationViewTitleBar {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonHoverForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonHoverForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonHoverBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonHoverBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonPressedForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressedForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonPressedBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonPressedBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInactiveForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).InactiveForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).InactiveBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonInactiveForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonInactiveForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ButtonInactiveBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
}
impl ::core::clone::Clone for ApplicationViewTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationViewTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewTitleBar {}
impl ::core::fmt::Debug for ApplicationViewTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ApplicationViewTitleBar;{00924ac0-932b-4a6b-9c4b-dc38c82478ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationViewTitleBar {
    type Vtable = IApplicationViewTitleBar_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationViewTitleBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationViewTitleBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewTitleBar";
}
impl ::core::convert::From<ApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: ApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewTitleBar> for ::windows::core::IUnknown {
    fn from(value: &ApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: ApplicationViewTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewTitleBar> for ::windows::core::IInspectable {
    fn from(value: &ApplicationViewTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationViewTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ApplicationViewTitleBar {}
unsafe impl ::core::marker::Sync for ApplicationViewTitleBar {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ApplicationViewTransferContext(::windows::core::IUnknown);
impl ApplicationViewTransferContext {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewTransferContext, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).ViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetViewId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DataPackageFormatId() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IApplicationViewTransferContextStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DataPackageFormatId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IApplicationViewTransferContextStatics<R, F: FnOnce(&IApplicationViewTransferContextStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ApplicationViewTransferContext, IApplicationViewTransferContextStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ApplicationViewTransferContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ApplicationViewTransferContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ApplicationViewTransferContext {}
impl ::core::fmt::Debug for ApplicationViewTransferContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewTransferContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewTransferContext {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ApplicationViewTransferContext;{8574bc63-3c17-408e-9408-8a1a9ea81bfa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ApplicationViewTransferContext {
    type Vtable = IApplicationViewTransferContext_Vtbl;
    const IID: ::windows::core::GUID = <IApplicationViewTransferContext as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ApplicationViewTransferContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.ApplicationViewTransferContext";
}
impl ::core::convert::From<ApplicationViewTransferContext> for ::windows::core::IUnknown {
    fn from(value: ApplicationViewTransferContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewTransferContext> for ::windows::core::IUnknown {
    fn from(value: &ApplicationViewTransferContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ApplicationViewTransferContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ApplicationViewTransferContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ApplicationViewTransferContext> for ::windows::core::IInspectable {
    fn from(value: ApplicationViewTransferContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ApplicationViewTransferContext> for ::windows::core::IInspectable {
    fn from(value: &ApplicationViewTransferContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ApplicationViewTransferContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ApplicationViewTransferContext {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ApplicationViewWindowingMode(pub i32);
impl ApplicationViewWindowingMode {
    pub const Auto: Self = Self(0i32);
    pub const PreferredLaunchViewSize: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const CompactOverlay: Self = Self(3i32);
    pub const Maximized: Self = Self(4i32);
}
impl ::core::marker::Copy for ApplicationViewWindowingMode {}
impl ::core::clone::Clone for ApplicationViewWindowingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ApplicationViewWindowingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ApplicationViewWindowingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ApplicationViewWindowingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ApplicationViewWindowingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ApplicationViewWindowingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ApplicationViewWindowingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FullScreenSystemOverlayMode(pub i32);
impl FullScreenSystemOverlayMode {
    pub const Standard: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
}
impl ::core::marker::Copy for FullScreenSystemOverlayMode {}
impl ::core::clone::Clone for FullScreenSystemOverlayMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FullScreenSystemOverlayMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FullScreenSystemOverlayMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FullScreenSystemOverlayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenSystemOverlayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullScreenSystemOverlayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.FullScreenSystemOverlayMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HandPreference(pub i32);
impl HandPreference {
    pub const LeftHanded: Self = Self(0i32);
    pub const RightHanded: Self = Self(1i32);
}
impl ::core::marker::Copy for HandPreference {}
impl ::core::clone::Clone for HandPreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HandPreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HandPreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for HandPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandPreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HandPreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.HandPreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAccessibilitySettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAccessibilitySettings {
    type Vtable = IAccessibilitySettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe0e8147_c4c0_4562_b962_1327b52ad5b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibilitySettings_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HighContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HighContrastScheme: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub HighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HighContrastChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHighContrastChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHighContrastChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationViewSwitcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationViewSwitcher {
    type Vtable = IActivationViewSwitcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdca71bb6_7350_492b_aac7_c8a13d7224ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationViewSwitcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAsStandaloneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsStandaloneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsStandaloneWithSizePreferenceAsync: usize,
    pub IsViewPresentedOnActivationVirtualDesktop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView {
    type Vtable = IApplicationView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd222d519_4361_451e_96c4_60f4f9742db0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Orientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewOrientation) -> ::windows::core::HRESULT,
    pub AdjacentToLeftDisplayEdge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AdjacentToRightDisplayEdge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "deprecated")]
    pub IsFullScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    IsFullScreen: usize,
    pub IsOnLockScreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsScreenCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsScreenCaptureEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Consolidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Consolidated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveConsolidated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConsolidated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView2 {
    type Vtable = IApplicationView2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe876b196_a545_40dc_b594_450cba68cc00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub SuppressSystemOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SuppressSystemOverlays: usize,
    #[cfg(feature = "deprecated")]
    pub SetSuppressSystemOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    SetSuppressSystemOverlays: usize,
    #[cfg(feature = "Foundation")]
    pub VisibleBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibleBounds: usize,
    #[cfg(feature = "Foundation")]
    pub VisibleBoundsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VisibleBoundsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVisibleBoundsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVisibleBoundsChanged: usize,
    pub SetDesiredBoundsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, boundsmode: ApplicationViewBoundsMode, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DesiredBoundsMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewBoundsMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView3 {
    type Vtable = IApplicationView3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x903c9ce5_793a_4fdf_a2b2_af1ac21e3108);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TitleBar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FullScreenSystemOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FullScreenSystemOverlayMode) -> ::windows::core::HRESULT,
    pub SetFullScreenSystemOverlayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FullScreenSystemOverlayMode) -> ::windows::core::HRESULT,
    pub IsFullScreenMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryEnterFullScreenMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExitFullScreenMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowStandardSystemOverlays: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryResizeView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryResizeView: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minsize: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView4 {
    type Vtable = IApplicationView4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15e5cbec_9e0f_46b5_bc3f_9bf653e74b5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewMode) -> ::windows::core::HRESULT,
    pub IsViewModeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryEnterViewModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnterViewModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryEnterViewModeWithPreferencesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, viewmodepreferences: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryEnterViewModeWithPreferencesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryConsolidateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryConsolidateAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView7(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView7 {
    type Vtable = IApplicationView7_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0369647_5faf_5aa6_9c38_befbb12a071e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView7_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPersistedStateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationView9(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationView9 {
    type Vtable = IApplicationView9_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c6516f9_021a_5f01_93e5_9bdad2647574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationView9_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_WindowManagement")]
    pub WindowingEnvironment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_WindowManagement"))]
    WindowingEnvironment: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement"))]
    pub GetDisplayRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_WindowManagement")))]
    GetDisplayRegions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewConsolidatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewConsolidatedEventArgs {
    type Vtable = IApplicationViewConsolidatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x514449ec_7ea2_4de7_a6a6_7dfbaaebb6fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewConsolidatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsUserInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewConsolidatedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewConsolidatedEventArgs2 {
    type Vtable = IApplicationViewConsolidatedEventArgs2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c199ecc_6dc1_40f4_afee_07d9ea296430);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewConsolidatedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsAppInitiated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IApplicationViewFullscreenStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IApplicationViewFullscreenStatics {
    type Vtable = IApplicationViewFullscreenStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc792ebd_64fe_4b65_a0c0_901ce2b68636);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewFullscreenStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub TryUnsnapToFullscreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnapToFullscreen: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewInteropStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewInteropStatics {
    type Vtable = IApplicationViewInteropStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc446fb5d_4793_4896_a8e2_be57a8bb0f50);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewInteropStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Core")]
    pub GetApplicationViewIdForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    GetApplicationViewIdForWindow: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewScaling(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewScaling {
    type Vtable = IApplicationViewScaling_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d0ddc23_23f3_4b2d_84fe_74bf37b48b66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewScaling_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewScalingStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewScalingStatics {
    type Vtable = IApplicationViewScalingStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb08fecf0_b946_45c8_a5e3_71f5aa78f861);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewScalingStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisableLayoutScaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TrySetDisableLayoutScaling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, disablelayoutscaling: bool, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
pub struct IApplicationViewStatics(::windows::core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows::core::Interface for IApplicationViewStatics {
    type Vtable = IApplicationViewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x010a6306_c433_44e5_a9f2_bd84d4030a95);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Value: usize,
    #[cfg(feature = "deprecated")]
    pub TryUnsnap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    TryUnsnap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewStatics2 {
    type Vtable = IApplicationViewStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf338ae5_cf64_423c_85e5_f3e72448fb23);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TerminateAppOnFinalViewClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetTerminateAppOnFinalViewClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewStatics3 {
    type Vtable = IApplicationViewStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa28d7594_8c41_4e13_9719_5164796fe4c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub PreferredLaunchWindowingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewWindowingMode) -> ::windows::core::HRESULT,
    pub SetPreferredLaunchWindowingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ApplicationViewWindowingMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PreferredLaunchViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredLaunchViewSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredLaunchViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredLaunchViewSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewStatics4 {
    type Vtable = IApplicationViewStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08fd8d33_2611_5336_a315_d98e6366c9db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewStatics4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ClearAllPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ClearPersistedState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewSwitcherStatics {
    type Vtable = IApplicationViewSwitcherStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x975f2f1e_e656_4c5e_a0a1_717c6ffa7d64);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewSwitcherStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisableShowingMainViewOnActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneWithSizePreferenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneWithSizePreferenceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchFromViewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchFromViewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchFromViewWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchFromViewWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PrepareForCustomAnimatedSwitchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrepareForCustomAnimatedSwitchAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewSwitcherStatics2 {
    type Vtable = IApplicationViewSwitcherStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60e995cd_4fc2_48c4_b8e3_395f2b9f0fc1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewSwitcherStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DisableSystemViewActivationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewSwitcherStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewSwitcherStatics3 {
    type Vtable = IApplicationViewSwitcherStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92059420_80a7_486d_b21f_c7a4a242a383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewSwitcherStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub TryShowAsViewModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsViewModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowAsViewModeWithPreferencesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowAsViewModeWithPreferencesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewTitleBar {
    type Vtable = IApplicationViewTitleBar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00924ac0_932b_4a6b_9c4b_dc38c82478ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewTitleBar_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonHoverBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonPressedBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub InactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetButtonInactiveBackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ButtonInactiveBackgroundColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewTransferContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewTransferContext {
    type Vtable = IApplicationViewTransferContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8574bc63_3c17_408e_9408_8a1a9ea81bfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewTransferContext_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewTransferContextStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewTransferContextStatics {
    type Vtable = IApplicationViewTransferContextStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15a89d92_dd79_4b0b_bc47_d5f195f14661);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewTransferContextStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DataPackageFormatId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IApplicationViewWithContext(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IApplicationViewWithContext {
    type Vtable = IApplicationViewWithContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd55d512_9dc1_44fc_8501_666625df60dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewWithContext_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPane(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPane {
    type Vtable = IInputPane_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x640ada70_06f3_4c87_a678_9829c9127c28);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPane_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub Hiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Hiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHiding: usize,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPane2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPane2 {
    type Vtable = IInputPane2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a6b3f26_7090_4793_944c_c3f2cde26276);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPane2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TryShow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub TryHide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPaneControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPaneControl {
    type Vtable = IInputPaneControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x088bb24f_962f_489d_aa6e_c6be1a0a6e52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPaneStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPaneStatics {
    type Vtable = IInputPaneStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95f4af3a_ef47_424a_9741_fd2815eba2bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPaneStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPaneStatics2 {
    type Vtable = IInputPaneStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b63529b_d9ec_4531_8445_71bab9fb828e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInputPaneVisibilityEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInputPaneVisibilityEventArgs {
    type Vtable = IInputPaneVisibilityEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd243e016_d907_4fcc_bb8d_f77baa5028f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneVisibilityEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
    pub SetEnsuredFocusedElementInView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub EnsuredFocusedElementInView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjectionManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProjectionManagerStatics {
    type Vtable = IProjectionManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb65f913d_e2f0_4ffd_ba95_34241647e45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartProjectingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartProjectingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SwapDisplaysForViewsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwapDisplaysForViewsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopProjectingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopProjectingAsync: usize,
    pub ProjectionDisplayAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProjectionDisplayAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProjectionDisplayAvailableChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProjectionDisplayAvailableChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProjectionDisplayAvailableChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IProjectionManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IProjectionManagerStatics2 {
    type Vtable = IProjectionManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf33d2f43_2749_4cde_b977_c0c41e7415d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProjectionManagerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub StartProjectingWithDeviceInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    StartProjectingWithDeviceInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartProjectingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartProjectingAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub RequestStartProjectingWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    RequestStartProjectingWithPlacementAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatusBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStatusBar {
    type Vtable = IStatusBar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ffcc5bf_98d0_4864_b1e8_b3f4020be8b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusBar_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HideAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HideAsync: usize,
    pub BackgroundOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetBackgroundOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetForegroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BackgroundColor: usize,
    #[cfg(feature = "Foundation")]
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBackgroundColor: usize,
    pub ProgressIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OccludedRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OccludedRect: usize,
    #[cfg(feature = "Foundation")]
    pub Showing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Showing: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveShowing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveShowing: usize,
    #[cfg(feature = "Foundation")]
    pub Hiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Hiding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveHiding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveHiding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatusBarProgressIndicator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStatusBarProgressIndicator {
    type Vtable = IStatusBarProgressIndicator_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x76cb2670_a3d7_49cf_8200_4f3eedca27bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusBarProgressIndicator_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub HideAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HideAsync: usize,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProgressValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProgressValue: usize,
    #[cfg(feature = "Foundation")]
    pub SetProgressValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetProgressValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStatusBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStatusBarStatics {
    type Vtable = IStatusBarStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b463fdf_422f_4561_8806_fb1289cadfb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStatusBarStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings {
    type Vtable = IUISettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85361600_1c63_4627_bcb1_3a89e0bc9c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HandPreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HandPreference) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CursorSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CursorSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarArrowSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarArrowSize: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollBarThumbBoxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollBarThumbBoxSize: usize,
    pub MessageDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AnimationsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CaretBrowsingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CaretBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CaretWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DoubleClickTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MouseHoverTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UIElementColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredelement: UIElementType, result__: *mut super::Color) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings2 {
    type Vtable = IUISettings2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad82401_2721_44f9_bb91_2bb228be442f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TextScaleFactor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TextScaleFactorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TextScaleFactorChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTextScaleFactorChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTextScaleFactorChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings3 {
    type Vtable = IUISettings3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03021be4_5254_4781_8194_5168f7d06d7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetColorValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: UIColorType, result__: *mut super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ColorValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ColorValuesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveColorValuesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveColorValuesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings4 {
    type Vtable = IUISettings4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52bb3002_919b_4d6b_9b78_8dd66ff4b93b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AdvancedEffectsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AdvancedEffectsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AdvancedEffectsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdvancedEffectsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdvancedEffectsEnabledChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings5 {
    type Vtable = IUISettings5_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5349d588_0cb5_5f05_bd34_706b3231f0bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings5_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AutoHideScrollBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AutoHideScrollBarsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AutoHideScrollBarsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAutoHideScrollBarsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAutoHideScrollBarsChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettings6(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettings6 {
    type Vtable = IUISettings6_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaef19bd7_fe31_5a04_ada4_469aaec6dfa9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings6_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AnimationsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnimationsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAnimationsEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAnimationsEnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub MessageDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MessageDurationChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMessageDurationChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMessageDurationChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsAnimationsEnabledChangedEventArgs {
    type Vtable = IUISettingsAnimationsEnabledChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c7b4b3d_2ea1_533e_894d_415bc5243c29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsAutoHideScrollBarsChangedEventArgs {
    type Vtable = IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87afd4b2_9146_5f02_8f6b_06d454174c0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUISettingsMessageDurationChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUISettingsMessageDurationChangedEventArgs {
    type Vtable = IUISettingsMessageDurationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338aad52_4a5d_5b59_8002_d930f608fd6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIViewSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIViewSettings {
    type Vtable = IUIViewSettings_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc63657f6_8850_470d_88f8_455e16ea2c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettings_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub UserInteractionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserInteractionMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUIViewSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUIViewSettingsStatics {
    type Vtable = IUIViewSettingsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x595c97a5_f8f6_41cf_b0fb_aacdb81fd5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IViewModePreferences(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IViewModePreferences {
    type Vtable = IViewModePreferences_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x878fcd3a_0b99_42c9_84d0_d3f1d403554b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewModePreferences_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ViewSizePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ViewSizePreference) -> ::windows::core::HRESULT,
    pub SetViewSizePreference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ViewSizePreference) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CustomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CustomSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetCustomSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCustomSize: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IViewModePreferencesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IViewModePreferencesStatics {
    type Vtable = IViewModePreferencesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69b60a65_5de5_40d8_8306_3833df7a2274);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewModePreferencesStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ApplicationViewMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct InputPane(::windows::core::IUnknown);
impl InputPane {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Showing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Showing)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShowing)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Hiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Hiding)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHiding)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows::core::Interface::vtable(this).OccludedRect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TryShow(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInputPane2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryShow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TryHide(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInputPane2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).TryHide)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInputPaneControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Visible)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInputPaneControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<InputPane> {
        Self::IInputPaneStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<InputPane>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetForUIContext<'a, Param0: ::windows::core::IntoParam<'a, super::UIContext>>(context: Param0) -> ::windows::core::Result<InputPane> {
        Self::IInputPaneStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetForUIContext)(::windows::core::Interface::as_raw(this), context.into_param().abi(), result__.as_mut_ptr()).from_abi::<InputPane>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInputPaneStatics<R, F: FnOnce(&IInputPaneStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InputPane, IInputPaneStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IInputPaneStatics2<R, F: FnOnce(&IInputPaneStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<InputPane, IInputPaneStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for InputPane {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputPane {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputPane {}
impl ::core::fmt::Debug for InputPane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPane").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputPane {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.InputPane;{640ada70-06f3-4c87-a678-9829c9127c28})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputPane {
    type Vtable = IInputPane_Vtbl;
    const IID: ::windows::core::GUID = <IInputPane as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputPane {
    const NAME: &'static str = "Windows.UI.ViewManagement.InputPane";
}
impl ::core::convert::From<InputPane> for ::windows::core::IUnknown {
    fn from(value: InputPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPane> for ::windows::core::IUnknown {
    fn from(value: &InputPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputPane> for ::windows::core::IInspectable {
    fn from(value: InputPane) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPane> for ::windows::core::IInspectable {
    fn from(value: &InputPane) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputPane {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct InputPaneVisibilityEventArgs(::windows::core::IUnknown);
impl InputPaneVisibilityEventArgs {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows::core::Interface::vtable(this).OccludedRect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetEnsuredFocusedElementInView(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnsuredFocusedElementInView)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn EnsuredFocusedElementInView(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).EnsuredFocusedElementInView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for InputPaneVisibilityEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InputPaneVisibilityEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputPaneVisibilityEventArgs {}
impl ::core::fmt::Debug for InputPaneVisibilityEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputPaneVisibilityEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InputPaneVisibilityEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.InputPaneVisibilityEventArgs;{d243e016-d907-4fcc-bb8d-f77baa5028f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for InputPaneVisibilityEventArgs {
    type Vtable = IInputPaneVisibilityEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IInputPaneVisibilityEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for InputPaneVisibilityEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.InputPaneVisibilityEventArgs";
}
impl ::core::convert::From<InputPaneVisibilityEventArgs> for ::windows::core::IUnknown {
    fn from(value: InputPaneVisibilityEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPaneVisibilityEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InputPaneVisibilityEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InputPaneVisibilityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InputPaneVisibilityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InputPaneVisibilityEventArgs> for ::windows::core::IInspectable {
    fn from(value: InputPaneVisibilityEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InputPaneVisibilityEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InputPaneVisibilityEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InputPaneVisibilityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InputPaneVisibilityEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
pub struct ProjectionManager;
impl ProjectionManager {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartProjectingAsync(projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).StartProjectingAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SwapDisplaysForViewsAsync(projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SwapDisplaysForViewsAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopProjectingAsync(projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).StopProjectingAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ProjectionDisplayAvailable() -> ::windows::core::Result<bool> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).ProjectionDisplayAvailable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProjectionDisplayAvailableChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IProjectionManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ProjectionDisplayAvailableChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProjectionDisplayAvailableChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::core::Result<()> {
        Self::IProjectionManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveProjectionDisplayAvailableChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Devices_Enumeration\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn StartProjectingWithDeviceInfoAsync<'a, Param2: ::windows::core::IntoParam<'a, super::super::Devices::Enumeration::DeviceInformation>>(projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).StartProjectingWithDeviceInfoAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, displaydeviceinfo.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartProjectingAsync<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(projectionviewid: i32, anchorviewid: i32, selection: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RequestStartProjectingAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, selection.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`, `\"UI_Popups\"`*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn RequestStartProjectingWithPlacementAsync<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(projectionviewid: i32, anchorviewid: i32, selection: Param2, prefferedplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RequestStartProjectingWithPlacementAsync)(::windows::core::Interface::as_raw(this), projectionviewid, anchorviewid, selection.into_param().abi(), prefferedplacement, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IProjectionManagerStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProjectionManagerStatics<R, F: FnOnce(&IProjectionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProjectionManager, IProjectionManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProjectionManagerStatics2<R, F: FnOnce(&IProjectionManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ProjectionManager, IProjectionManagerStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ProjectionManager {
    const NAME: &'static str = "Windows.UI.ViewManagement.ProjectionManager";
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct StatusBar(::windows::core::IUnknown);
impl StatusBar {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).HideAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn BackgroundOpacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundOpacity)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetBackgroundOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundOpacity)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ProgressIndicator(&self) -> ::windows::core::Result<StatusBarProgressIndicator> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ProgressIndicator)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StatusBarProgressIndicator>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Rect>::zeroed();
            (::windows::core::Interface::vtable(this).OccludedRect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Showing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Showing)(::windows::core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveShowing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveShowing)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Hiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).Hiding)(::windows::core::Interface::as_raw(this), eventhandler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveHiding<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveHiding)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<StatusBar> {
        Self::IStatusBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<StatusBar>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStatusBarStatics<R, F: FnOnce(&IStatusBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<StatusBar, IStatusBarStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for StatusBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatusBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusBar {}
impl ::core::fmt::Debug for StatusBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StatusBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.StatusBar;{0ffcc5bf-98d0-4864-b1e8-b3f4020be8b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StatusBar {
    type Vtable = IStatusBar_Vtbl;
    const IID: ::windows::core::GUID = <IStatusBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StatusBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.StatusBar";
}
impl ::core::convert::From<StatusBar> for ::windows::core::IUnknown {
    fn from(value: StatusBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusBar> for ::windows::core::IUnknown {
    fn from(value: &StatusBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StatusBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StatusBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StatusBar> for ::windows::core::IInspectable {
    fn from(value: StatusBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusBar> for ::windows::core::IInspectable {
    fn from(value: &StatusBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StatusBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StatusBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StatusBar {}
unsafe impl ::core::marker::Sync for StatusBar {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct StatusBarProgressIndicator(::windows::core::IUnknown);
impl StatusBarProgressIndicator {
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).HideAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ProgressValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ProgressValue)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<f64>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProgressValue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<f64>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProgressValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StatusBarProgressIndicator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StatusBarProgressIndicator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StatusBarProgressIndicator {}
impl ::core::fmt::Debug for StatusBarProgressIndicator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StatusBarProgressIndicator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StatusBarProgressIndicator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.StatusBarProgressIndicator;{76cb2670-a3d7-49cf-8200-4f3eedca27bb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StatusBarProgressIndicator {
    type Vtable = IStatusBarProgressIndicator_Vtbl;
    const IID: ::windows::core::GUID = <IStatusBarProgressIndicator as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StatusBarProgressIndicator {
    const NAME: &'static str = "Windows.UI.ViewManagement.StatusBarProgressIndicator";
}
impl ::core::convert::From<StatusBarProgressIndicator> for ::windows::core::IUnknown {
    fn from(value: StatusBarProgressIndicator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusBarProgressIndicator> for ::windows::core::IUnknown {
    fn from(value: &StatusBarProgressIndicator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StatusBarProgressIndicator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StatusBarProgressIndicator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StatusBarProgressIndicator> for ::windows::core::IInspectable {
    fn from(value: StatusBarProgressIndicator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StatusBarProgressIndicator> for ::windows::core::IInspectable {
    fn from(value: &StatusBarProgressIndicator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StatusBarProgressIndicator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StatusBarProgressIndicator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StatusBarProgressIndicator {}
unsafe impl ::core::marker::Sync for StatusBarProgressIndicator {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
    pub const AccentDark3: Self = Self(2i32);
    pub const AccentDark2: Self = Self(3i32);
    pub const AccentDark1: Self = Self(4i32);
    pub const Accent: Self = Self(5i32);
    pub const AccentLight1: Self = Self(6i32);
    pub const AccentLight2: Self = Self(7i32);
    pub const AccentLight3: Self = Self(8i32);
    pub const Complement: Self = Self(9i32);
}
impl ::core::marker::Copy for UIColorType {}
impl ::core::clone::Clone for UIColorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIColorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UIColorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UIColorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIColorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIColorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UIColorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UIElementType(pub i32);
impl UIElementType {
    pub const ActiveCaption: Self = Self(0i32);
    pub const Background: Self = Self(1i32);
    pub const ButtonFace: Self = Self(2i32);
    pub const ButtonText: Self = Self(3i32);
    pub const CaptionText: Self = Self(4i32);
    pub const GrayText: Self = Self(5i32);
    pub const Highlight: Self = Self(6i32);
    pub const HighlightText: Self = Self(7i32);
    pub const Hotlight: Self = Self(8i32);
    pub const InactiveCaption: Self = Self(9i32);
    pub const InactiveCaptionText: Self = Self(10i32);
    pub const Window: Self = Self(11i32);
    pub const WindowText: Self = Self(12i32);
    pub const AccentColor: Self = Self(1000i32);
    pub const TextHigh: Self = Self(1001i32);
    pub const TextMedium: Self = Self(1002i32);
    pub const TextLow: Self = Self(1003i32);
    pub const TextContrastWithHigh: Self = Self(1004i32);
    pub const NonTextHigh: Self = Self(1005i32);
    pub const NonTextMediumHigh: Self = Self(1006i32);
    pub const NonTextMedium: Self = Self(1007i32);
    pub const NonTextMediumLow: Self = Self(1008i32);
    pub const NonTextLow: Self = Self(1009i32);
    pub const PageBackground: Self = Self(1010i32);
    pub const PopupBackground: Self = Self(1011i32);
    pub const OverlayOutsidePopup: Self = Self(1012i32);
}
impl ::core::marker::Copy for UIElementType {}
impl ::core::clone::Clone for UIElementType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIElementType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UIElementType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UIElementType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIElementType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIElementType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UIElementType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UISettings(::windows::core::IUnknown);
impl UISettings {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UISettings, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn HandPreference(&self) -> ::windows::core::Result<HandPreference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<HandPreference>::zeroed();
            (::windows::core::Interface::vtable(this).HandPreference)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<HandPreference>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CursorSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).CursorSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScrollBarSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).ScrollBarSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScrollBarArrowSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).ScrollBarArrowSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScrollBarThumbBoxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).ScrollBarThumbBoxSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn MessageDuration(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).MessageDuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn AnimationsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).AnimationsEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn CaretBrowsingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CaretBrowsingEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn CaretBlinkRate(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).CaretBlinkRate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn CaretWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).CaretWidth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn DoubleClickTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).DoubleClickTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn MouseHoverTime(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
            (::windows::core::Interface::vtable(this).MouseHoverTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn UIElementColor(&self, desiredelement: UIElementType) -> ::windows::core::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Color>::zeroed();
            (::windows::core::Interface::vtable(this).UIElementColor)(::windows::core::Interface::as_raw(this), desiredelement, result__.as_mut_ptr()).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn TextScaleFactor(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IUISettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).TextScaleFactor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TextScaleFactorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).TextScaleFactorChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextScaleFactorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTextScaleFactorChanged)(::windows::core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetColorValue(&self, desiredcolor: UIColorType) -> ::windows::core::Result<super::Color> {
        let this = &::windows::core::Interface::cast::<IUISettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::Color>::zeroed();
            (::windows::core::Interface::vtable(this).GetColorValue)(::windows::core::Interface::as_raw(this), desiredcolor, result__.as_mut_ptr()).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ColorValuesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings3>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).ColorValuesChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveColorValuesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveColorValuesChanged)(::windows::core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn AdvancedEffectsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUISettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).AdvancedEffectsEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AdvancedEffectsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings4>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).AdvancedEffectsEnabledChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdvancedEffectsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdvancedEffectsEnabledChanged)(::windows::core::Interface::as_raw(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn AutoHideScrollBars(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUISettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).AutoHideScrollBars)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AutoHideScrollBarsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings5>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).AutoHideScrollBarsChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAutoHideScrollBarsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings5>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAutoHideScrollBarsChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnimationsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).AnimationsEnabledChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAnimationsEnabledChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAnimationsEnabledChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MessageDurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUISettings6>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::EventRegistrationToken>::zeroed();
            (::windows::core::Interface::vtable(this).MessageDurationChanged)(::windows::core::Interface::as_raw(this), handler.into_param().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMessageDurationChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUISettings6>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMessageDurationChanged)(::windows::core::Interface::as_raw(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UISettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettings {}
impl ::core::fmt::Debug for UISettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UISettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.UISettings;{85361600-1c63-4627-bcb1-3a89e0bc9c55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UISettings {
    type Vtable = IUISettings_Vtbl;
    const IID: ::windows::core::GUID = <IUISettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
}
impl ::core::convert::From<UISettings> for ::windows::core::IUnknown {
    fn from(value: UISettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettings> for ::windows::core::IUnknown {
    fn from(value: &UISettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettings> for ::windows::core::IInspectable {
    fn from(value: UISettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettings> for ::windows::core::IInspectable {
    fn from(value: &UISettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettings {}
unsafe impl ::core::marker::Sync for UISettings {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UISettingsAnimationsEnabledChangedEventArgs(::windows::core::IUnknown);
impl UISettingsAnimationsEnabledChangedEventArgs {}
impl ::core::clone::Clone for UISettingsAnimationsEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettingsAnimationsEnabledChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsAnimationsEnabledChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsAnimationsEnabledChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsAnimationsEnabledChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs;{0c7b4b3d-2ea1-533e-894d-415bc5243c29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UISettingsAnimationsEnabledChangedEventArgs {
    type Vtable = IUISettingsAnimationsEnabledChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUISettingsAnimationsEnabledChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
}
impl ::core::convert::From<UISettingsAnimationsEnabledChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UISettingsAnimationsEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsAnimationsEnabledChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UISettingsAnimationsEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettingsAnimationsEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettingsAnimationsEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettingsAnimationsEnabledChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UISettingsAnimationsEnabledChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsAnimationsEnabledChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UISettingsAnimationsEnabledChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettingsAnimationsEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettingsAnimationsEnabledChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsAnimationsEnabledChangedEventArgs {}
unsafe impl ::core::marker::Sync for UISettingsAnimationsEnabledChangedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UISettingsAutoHideScrollBarsChangedEventArgs(::windows::core::IUnknown);
impl UISettingsAutoHideScrollBarsChangedEventArgs {}
impl ::core::clone::Clone for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsAutoHideScrollBarsChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsAutoHideScrollBarsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs;{87afd4b2-9146-5f02-8f6b-06d454174c0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UISettingsAutoHideScrollBarsChangedEventArgs {
    type Vtable = IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUISettingsAutoHideScrollBarsChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
}
impl ::core::convert::From<UISettingsAutoHideScrollBarsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UISettingsAutoHideScrollBarsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsAutoHideScrollBarsChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UISettingsAutoHideScrollBarsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettingsAutoHideScrollBarsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettingsAutoHideScrollBarsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UISettingsAutoHideScrollBarsChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsAutoHideScrollBarsChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UISettingsAutoHideScrollBarsChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettingsAutoHideScrollBarsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettingsAutoHideScrollBarsChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsAutoHideScrollBarsChangedEventArgs {}
unsafe impl ::core::marker::Sync for UISettingsAutoHideScrollBarsChangedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UISettingsMessageDurationChangedEventArgs(::windows::core::IUnknown);
impl UISettingsMessageDurationChangedEventArgs {}
impl ::core::clone::Clone for UISettingsMessageDurationChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UISettingsMessageDurationChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UISettingsMessageDurationChangedEventArgs {}
impl ::core::fmt::Debug for UISettingsMessageDurationChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UISettingsMessageDurationChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs;{338aad52-4a5d-5b59-8002-d930f608fd6e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UISettingsMessageDurationChangedEventArgs {
    type Vtable = IUISettingsMessageDurationChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUISettingsMessageDurationChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
}
impl ::core::convert::From<UISettingsMessageDurationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UISettingsMessageDurationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsMessageDurationChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UISettingsMessageDurationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UISettingsMessageDurationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UISettingsMessageDurationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UISettingsMessageDurationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UISettingsMessageDurationChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UISettingsMessageDurationChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UISettingsMessageDurationChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UISettingsMessageDurationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UISettingsMessageDurationChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UISettingsMessageDurationChangedEventArgs {}
unsafe impl ::core::marker::Sync for UISettingsMessageDurationChangedEventArgs {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct UIViewSettings(::windows::core::IUnknown);
impl UIViewSettings {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn UserInteractionMode(&self) -> ::windows::core::Result<UserInteractionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<UserInteractionMode>::zeroed();
            (::windows::core::Interface::vtable(this).UserInteractionMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UserInteractionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<UIViewSettings> {
        Self::IUIViewSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<UIViewSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUIViewSettingsStatics<R, F: FnOnce(&IUIViewSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<UIViewSettings, IUIViewSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for UIViewSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UIViewSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UIViewSettings {}
impl ::core::fmt::Debug for UIViewSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIViewSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIViewSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.UIViewSettings;{c63657f6-8850-470d-88f8-455e16ea2c26})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UIViewSettings {
    type Vtable = IUIViewSettings_Vtbl;
    const IID: ::windows::core::GUID = <IUIViewSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UIViewSettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UIViewSettings";
}
impl ::core::convert::From<UIViewSettings> for ::windows::core::IUnknown {
    fn from(value: UIViewSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIViewSettings> for ::windows::core::IUnknown {
    fn from(value: &UIViewSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UIViewSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UIViewSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UIViewSettings> for ::windows::core::IInspectable {
    fn from(value: UIViewSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UIViewSettings> for ::windows::core::IInspectable {
    fn from(value: &UIViewSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UIViewSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UIViewSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UIViewSettings {}
unsafe impl ::core::marker::Sync for UIViewSettings {}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserInteractionMode(pub i32);
impl UserInteractionMode {
    pub const Mouse: Self = Self(0i32);
    pub const Touch: Self = Self(1i32);
}
impl ::core::marker::Copy for UserInteractionMode {}
impl ::core::clone::Clone for UserInteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserInteractionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserInteractionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserInteractionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserInteractionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserInteractionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UserInteractionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
pub struct ViewModePreferences(::windows::core::IUnknown);
impl ViewModePreferences {
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn ViewSizePreference(&self) -> ::windows::core::Result<ViewSizePreference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<ViewSizePreference>::zeroed();
            (::windows::core::Interface::vtable(this).ViewSizePreference)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ViewSizePreference>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn SetViewSizePreference(&self, value: ViewSizePreference) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetViewSizePreference)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CustomSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::Size>::zeroed();
            (::windows::core::Interface::vtable(this).CustomSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCustomSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomSize)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_ViewManagement\"`*"]
    pub fn CreateDefault(mode: ApplicationViewMode) -> ::windows::core::Result<ViewModePreferences> {
        Self::IViewModePreferencesStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).CreateDefault)(::windows::core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<ViewModePreferences>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IViewModePreferencesStatics<R, F: FnOnce(&IViewModePreferencesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ViewModePreferences, IViewModePreferencesStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ViewModePreferences {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ViewModePreferences {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ViewModePreferences {}
impl ::core::fmt::Debug for ViewModePreferences {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ViewModePreferences").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ViewModePreferences {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.ViewManagement.ViewModePreferences;{878fcd3a-0b99-42c9-84d0-d3f1d403554b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ViewModePreferences {
    type Vtable = IViewModePreferences_Vtbl;
    const IID: ::windows::core::GUID = <IViewModePreferences as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ViewModePreferences {
    const NAME: &'static str = "Windows.UI.ViewManagement.ViewModePreferences";
}
impl ::core::convert::From<ViewModePreferences> for ::windows::core::IUnknown {
    fn from(value: ViewModePreferences) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ViewModePreferences> for ::windows::core::IUnknown {
    fn from(value: &ViewModePreferences) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ViewModePreferences {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ViewModePreferences {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ViewModePreferences> for ::windows::core::IInspectable {
    fn from(value: ViewModePreferences) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ViewModePreferences> for ::windows::core::IInspectable {
    fn from(value: &ViewModePreferences) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ViewModePreferences {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ViewModePreferences {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_ViewManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ViewSizePreference(pub i32);
impl ViewSizePreference {
    pub const Default: Self = Self(0i32);
    pub const UseLess: Self = Self(1i32);
    pub const UseHalf: Self = Self(2i32);
    pub const UseMore: Self = Self(3i32);
    pub const UseMinimum: Self = Self(4i32);
    pub const UseNone: Self = Self(5i32);
    pub const Custom: Self = Self(6i32);
}
impl ::core::marker::Copy for ViewSizePreference {}
impl ::core::clone::Clone for ViewSizePreference {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ViewSizePreference {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ViewSizePreference {
    type Abi = Self;
}
impl ::core::fmt::Debug for ViewSizePreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ViewSizePreference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ViewSizePreference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.ViewSizePreference;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
