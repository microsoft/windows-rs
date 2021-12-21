#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_WindowManagement_Preview")]
pub mod Preview;
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindow(::windows::core::IUnknown);
impl AppWindow {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Content(&self) -> ::windows::core::Result<super::UIContentRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIContentRoot>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'System'*"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::DispatcherQueue>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Frame(&self) -> ::windows::core::Result<AppWindowFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowFrame>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetPersistedStateId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresenter>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowTitleBar>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn UIContext(&self) -> ::windows::core::Result<super::UIContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::UIContext>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowingEnvironment>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn GetPlacement(&self) -> ::windows::core::Result<AppWindowPlacement> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPlacement>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DisplayRegion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn RequestMoveToDisplayRegion<'a, Param0: ::windows::core::IntoParam<'a, DisplayRegion>>(&self, displayregion: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), displayregion.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn RequestMoveAdjacentToCurrentView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn RequestMoveAdjacentToWindow<'a, Param0: ::windows::core::IntoParam<'a, AppWindow>>(&self, anchorwindow: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), anchorwindow.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToWindowContent<'a, Param0: ::windows::core::IntoParam<'a, AppWindow>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, anchorwindow: Param0, contentoffset: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), anchorwindow.into_param().abi(), contentoffset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToCurrentViewContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, contentoffset: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), contentoffset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestMoveRelativeToDisplayRegion<'a, Param0: ::windows::core::IntoParam<'a, DisplayRegion>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, displayregion: Param0, displayregionoffset: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), displayregion.into_param().abi(), displayregionoffset.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, framesize: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), framesize.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCloseRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppWindow>> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<AppWindow>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn ClearAllPersistedState() -> ::windows::core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() })
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn ClearPersistedState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(key: Param0) -> ::windows::core::Result<()> {
        Self::IAppWindowStatics(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AppWindow, IAppWindowStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindow {}
impl ::core::fmt::Debug for AppWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindow;{663014a6-b75e-5dbd-995c-f0117fa3fb61})");
}
unsafe impl ::windows::core::Interface for AppWindow {
    type Vtable = IAppWindowVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x663014a6_b75e_5dbd_995c_f0117fa3fb61);
}
impl ::windows::core::RuntimeName for AppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindow";
}
impl ::core::convert::From<AppWindow> for ::windows::core::IUnknown {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IUnknown {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindow> for ::windows::core::IInspectable {
    fn from(value: AppWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindow> for ::windows::core::IInspectable {
    fn from(value: &AppWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(::windows::core::IUnknown);
impl AppWindowChangedEventArgs {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidAvailableWindowPresentationsChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidDisplayRegionsChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidFrameChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidSizeChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidTitleBarChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidVisibilityChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidWindowingEnvironmentChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DidWindowPresentationChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowChangedEventArgs {}
impl ::core::fmt::Debug for AppWindowChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowChangedEventArgs;{1de1f3be-a655-55ad-b2b6-eb240f880356})");
}
unsafe impl ::windows::core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1de1f3be_a655_55ad_b2b6_eb240f880356);
}
impl ::windows::core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowChangedEventArgs";
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowCloseRequestedEventArgs(::windows::core::IUnknown);
impl AppWindowCloseRequestedEventArgs {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowCloseRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowCloseRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowCloseRequestedEventArgs {}
impl ::core::fmt::Debug for AppWindowCloseRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowCloseRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowCloseRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs;{e9ff01da-e7a2-57a8-8b5e-39c4003afdbb})");
}
unsafe impl ::windows::core::Interface for AppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9ff01da_e7a2_57a8_8b5e_39c4003afdbb);
}
impl ::windows::core::RuntimeName for AppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowCloseRequestedEventArgs";
}
impl ::core::convert::From<AppWindowCloseRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowCloseRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowCloseRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowCloseRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowCloseRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowCloseRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowCloseRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowCloseRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowCloseRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowCloseRequestedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowCloseRequestedEventArgs {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowClosedEventArgs(::windows::core::IUnknown);
impl AppWindowClosedEventArgs {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Reason(&self) -> ::windows::core::Result<AppWindowClosedReason> {
        let this = self;
        unsafe {
            let mut result__: AppWindowClosedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowClosedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosedEventArgs {}
impl ::core::fmt::Debug for AppWindowClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowClosedEventArgs;{cc7df816-9520-5a06-821e-456ad8b358aa})");
}
unsafe impl ::windows::core::Interface for AppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc7df816_9520_5a06_821e_456ad8b358aa);
}
impl ::windows::core::RuntimeName for AppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowClosedEventArgs";
}
impl ::core::convert::From<AppWindowClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: AppWindowClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AppWindowClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: AppWindowClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AppWindowClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowClosedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosedEventArgs {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowClosedReason(pub i32);
impl AppWindowClosedReason {
    pub const Other: Self = Self(0i32);
    pub const AppInitiated: Self = Self(1i32);
    pub const UserInitiated: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowClosedReason {}
impl ::core::clone::Clone for AppWindowClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppWindowClosedReason {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppWindowClosedReason {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosedReason {}
impl ::core::fmt::Debug for AppWindowClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowClosedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowClosedReason;i4)");
}
impl ::windows::core::DefaultType for AppWindowClosedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowFrame(::windows::core::IUnknown);
impl AppWindowFrame {
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections', 'UI_Composition'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))]
    pub fn DragRegionVisuals(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn GetFrameStyle(&self) -> ::windows::core::Result<AppWindowFrameStyle> {
        let this = &::windows::core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe {
            let mut result__: AppWindowFrameStyle = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowFrameStyle>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindowFrameStyle>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), framestyle).ok() }
    }
}
impl ::core::clone::Clone for AppWindowFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowFrame {}
impl ::core::fmt::Debug for AppWindowFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrame").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowFrame;{9ee22601-7e5d-52af-846b-01dc6c296567})");
}
unsafe impl ::windows::core::Interface for AppWindowFrame {
    type Vtable = IAppWindowFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ee22601_7e5d_52af_846b_01dc6c296567);
}
impl ::windows::core::RuntimeName for AppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowFrame";
}
impl ::core::convert::From<AppWindowFrame> for ::windows::core::IUnknown {
    fn from(value: AppWindowFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowFrame> for ::windows::core::IUnknown {
    fn from(value: &AppWindowFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowFrame> for ::windows::core::IInspectable {
    fn from(value: AppWindowFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowFrame> for ::windows::core::IInspectable {
    fn from(value: &AppWindowFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowFrame {}
unsafe impl ::core::marker::Sync for AppWindowFrame {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowFrameStyle(pub i32);
impl AppWindowFrameStyle {
    pub const Default: Self = Self(0i32);
    pub const NoFrame: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowFrameStyle {}
impl ::core::clone::Clone for AppWindowFrameStyle {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppWindowFrameStyle {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppWindowFrameStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowFrameStyle {}
impl ::core::fmt::Debug for AppWindowFrameStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowFrameStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowFrameStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowFrameStyle;i4)");
}
impl ::windows::core::DefaultType for AppWindowFrameStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowPlacement(::windows::core::IUnknown);
impl AppWindowPlacement {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DisplayRegion(&self) -> ::windows::core::Result<DisplayRegion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DisplayRegion>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPlacement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPlacement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPlacement {}
impl ::core::fmt::Debug for AppWindowPlacement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPlacement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPlacement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPlacement;{03dc815e-e7a9-5857-9c03-7d670594410e})");
}
unsafe impl ::windows::core::Interface for AppWindowPlacement {
    type Vtable = IAppWindowPlacementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03dc815e_e7a9_5857_9c03_7d670594410e);
}
impl ::windows::core::RuntimeName for AppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPlacement";
}
impl ::core::convert::From<AppWindowPlacement> for ::windows::core::IUnknown {
    fn from(value: AppWindowPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPlacement> for ::windows::core::IUnknown {
    fn from(value: &AppWindowPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPlacement> for ::windows::core::IInspectable {
    fn from(value: AppWindowPlacement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPlacement> for ::windows::core::IInspectable {
    fn from(value: &AppWindowPlacement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowPlacement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPlacement {}
unsafe impl ::core::marker::Sync for AppWindowPlacement {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowPresentationConfiguration(::windows::core::IUnknown);
impl AppWindowPresentationConfiguration {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = self;
        unsafe {
            let mut result__: AppWindowPresentationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresentationConfiguration {}
impl ::core::fmt::Debug for AppWindowPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresentationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresentationConfiguration;{b5a43ee3-df33-5e67-bd31-1072457300df})");
}
unsafe impl ::windows::core::Interface for AppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a43ee3_df33_5e67_bd31_1072457300df);
}
impl ::windows::core::RuntimeName for AppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresentationConfiguration";
}
impl ::core::convert::From<AppWindowPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: AppWindowPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &AppWindowPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: AppWindowPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &AppWindowPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPresentationConfiguration {}
unsafe impl ::core::marker::Sync for AppWindowPresentationConfiguration {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowPresentationKind(pub i32);
impl AppWindowPresentationKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
}
impl ::core::marker::Copy for AppWindowPresentationKind {}
impl ::core::clone::Clone for AppWindowPresentationKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppWindowPresentationKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppWindowPresentationKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresentationKind {}
impl ::core::fmt::Debug for AppWindowPresentationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresentationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresentationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowPresentationKind;i4)");
}
impl ::windows::core::DefaultType for AppWindowPresentationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowPresenter(::windows::core::IUnknown);
impl AppWindowPresenter {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn GetConfiguration(&self) -> ::windows::core::Result<AppWindowPresentationConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresentationConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), presentationkind, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn RequestPresentation<'a, Param0: ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration>>(&self, configuration: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), configuration.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), presentationkind, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresenter {}
impl ::core::fmt::Debug for AppWindowPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowPresenter;{5ae9ed73-e1fd-5317-ad78-5a3ed271bbde})");
}
unsafe impl ::windows::core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ae9ed73_e1fd_5317_ad78_5a3ed271bbde);
}
impl ::windows::core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowPresenter";
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IUnknown {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: AppWindowPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowPresenter> for ::windows::core::IInspectable {
    fn from(value: &AppWindowPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowTitleBar(::windows::core::IUnknown);
impl AppWindowTitleBar {
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonHoverForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonInactiveForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetButtonPressedForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::Color>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInactiveForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::Color>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTitleBarOcclusions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn GetPreferredVisibility(&self) -> ::windows::core::Result<AppWindowTitleBarVisibility> {
        let this = &::windows::core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe {
            let mut result__: AppWindowTitleBarVisibility = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowTitleBarVisibility>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAppWindowTitleBarVisibility>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), visibilitymode).ok() }
    }
}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBar {}
impl ::core::fmt::Debug for AppWindowTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBar;{6e932c84-f644-541d-a2d7-0c262437842d})");
}
unsafe impl ::windows::core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e932c84_f644_541d_a2d7_0c262437842d);
}
impl ::windows::core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBar";
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IUnknown {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: AppWindowTitleBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBar> for ::windows::core::IInspectable {
    fn from(value: &AppWindowTitleBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowTitleBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowTitleBarOcclusion(::windows::core::IUnknown);
impl AppWindowTitleBarOcclusion {
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn OccludingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
}
impl ::core::clone::Clone for AppWindowTitleBarOcclusion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBarOcclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBarOcclusion {}
impl ::core::fmt::Debug for AppWindowTitleBarOcclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarOcclusion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowTitleBarOcclusion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.AppWindowTitleBarOcclusion;{fea3cffd-2ccf-5fc3-aeae-f843876bf37e})");
}
unsafe impl ::windows::core::Interface for AppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea3cffd_2ccf_5fc3_aeae_f843876bf37e);
}
impl ::windows::core::RuntimeName for AppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.AppWindowTitleBarOcclusion";
}
impl ::core::convert::From<AppWindowTitleBarOcclusion> for ::windows::core::IUnknown {
    fn from(value: AppWindowTitleBarOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBarOcclusion> for ::windows::core::IUnknown {
    fn from(value: &AppWindowTitleBarOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AppWindowTitleBarOcclusion> for ::windows::core::IInspectable {
    fn from(value: AppWindowTitleBarOcclusion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AppWindowTitleBarOcclusion> for ::windows::core::IInspectable {
    fn from(value: &AppWindowTitleBarOcclusion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &AppWindowTitleBarOcclusion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AppWindowTitleBarOcclusion {}
unsafe impl ::core::marker::Sync for AppWindowTitleBarOcclusion {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct AppWindowTitleBarVisibility(pub i32);
impl AppWindowTitleBarVisibility {
    pub const Default: Self = Self(0i32);
    pub const AlwaysHidden: Self = Self(1i32);
}
impl ::core::marker::Copy for AppWindowTitleBarVisibility {}
impl ::core::clone::Clone for AppWindowTitleBarVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for AppWindowTitleBarVisibility {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for AppWindowTitleBarVisibility {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBarVisibility {}
impl ::core::fmt::Debug for AppWindowTitleBarVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBarVisibility").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AppWindowTitleBarVisibility {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.AppWindowTitleBarVisibility;i4)");
}
impl ::windows::core::DefaultType for AppWindowTitleBarVisibility {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct CompactOverlayPresentationConfiguration(::windows::core::IUnknown);
impl CompactOverlayPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompactOverlayPresentationConfiguration, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__: AppWindowPresentationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for CompactOverlayPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompactOverlayPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompactOverlayPresentationConfiguration {}
impl ::core::fmt::Debug for CompactOverlayPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlayPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompactOverlayPresentationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration;{a7e5750f-5730-56c6-8e1f-d63ff4d7980d})");
}
unsafe impl ::windows::core::Interface for CompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e5750f_5730_56c6_8e1f_d63ff4d7980d);
}
impl ::windows::core::RuntimeName for CompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.CompactOverlayPresentationConfiguration";
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CompactOverlayPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: CompactOverlayPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CompactOverlayPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &CompactOverlayPresentationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for &CompactOverlayPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for CompactOverlayPresentationConfiguration {}
unsafe impl ::core::marker::Sync for CompactOverlayPresentationConfiguration {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct DefaultPresentationConfiguration(::windows::core::IUnknown);
impl DefaultPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DefaultPresentationConfiguration, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__: AppWindowPresentationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
}
impl ::core::clone::Clone for DefaultPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DefaultPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DefaultPresentationConfiguration {}
impl ::core::fmt::Debug for DefaultPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DefaultPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DefaultPresentationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DefaultPresentationConfiguration;{d8c2b53b-2168-5703-a853-d525589fe2b9})");
}
unsafe impl ::windows::core::Interface for DefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c2b53b_2168_5703_a853_d525589fe2b9);
}
impl ::windows::core::RuntimeName for DefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.DefaultPresentationConfiguration";
}
impl ::core::convert::From<DefaultPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DefaultPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DefaultPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: DefaultPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&DefaultPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &DefaultPresentationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for &DefaultPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for DefaultPresentationConfiguration {}
unsafe impl ::core::marker::Sync for DefaultPresentationConfiguration {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct DisplayRegion(::windows::core::IUnknown);
impl DisplayRegion {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn DisplayMonitorDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WorkAreaOffset(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn WorkAreaSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowingEnvironment>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for DisplayRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DisplayRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayRegion {}
impl ::core::fmt::Debug for DisplayRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayRegion").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DisplayRegion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.DisplayRegion;{db50c3a2-4094-5f47-8cb1-ea01ddafaa94})");
}
unsafe impl ::windows::core::Interface for DisplayRegion {
    type Vtable = IDisplayRegionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb50c3a2_4094_5f47_8cb1_ea01ddafaa94);
}
impl ::windows::core::RuntimeName for DisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.DisplayRegion";
}
impl ::core::convert::From<DisplayRegion> for ::windows::core::IUnknown {
    fn from(value: DisplayRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRegion> for ::windows::core::IUnknown {
    fn from(value: &DisplayRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DisplayRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &DisplayRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DisplayRegion> for ::windows::core::IInspectable {
    fn from(value: DisplayRegion) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DisplayRegion> for ::windows::core::IInspectable {
    fn from(value: &DisplayRegion) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DisplayRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &DisplayRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DisplayRegion {}
unsafe impl ::core::marker::Sync for DisplayRegion {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct FullScreenPresentationConfiguration(::windows::core::IUnknown);
impl FullScreenPresentationConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<FullScreenPresentationConfiguration, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind> {
        let this = &::windows::core::Interface::cast::<IAppWindowPresentationConfiguration>(self)?;
        unsafe {
            let mut result__: AppWindowPresentationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AppWindowPresentationKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsExclusive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn SetIsExclusive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for FullScreenPresentationConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FullScreenPresentationConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullScreenPresentationConfiguration {}
impl ::core::fmt::Debug for FullScreenPresentationConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenPresentationConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FullScreenPresentationConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.FullScreenPresentationConfiguration;{43d3dcd8-d2a8-503d-a626-15533d6d5f62})");
}
unsafe impl ::windows::core::Interface for FullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43d3dcd8_d2a8_503d_a626_15533d6d5f62);
}
impl ::windows::core::RuntimeName for FullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.FullScreenPresentationConfiguration";
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for ::windows::core::IUnknown {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for ::windows::core::IInspectable {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FullScreenPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: FullScreenPresentationConfiguration) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&FullScreenPresentationConfiguration> for AppWindowPresentationConfiguration {
    fn from(value: &FullScreenPresentationConfiguration) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, AppWindowPresentationConfiguration> for &FullScreenPresentationConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, AppWindowPresentationConfiguration> {
        ::windows::core::Param::Owned(::core::convert::Into::<AppWindowPresentationConfiguration>::into(self))
    }
}
unsafe impl ::core::marker::Send for FullScreenPresentationConfiguration {}
unsafe impl ::core::marker::Sync for FullScreenPresentationConfiguration {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow {
    type Vtable = IAppWindowVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x663014a6_b75e_5dbd_995c_f0117fa3fb61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, displayregionoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesize: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1de1f3be_a655_55ad_b2b6_eb240f880356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowCloseRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowCloseRequestedEventArgs {
    type Vtable = IAppWindowCloseRequestedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9ff01da_e7a2_57a8_8b5e_39c4003afdbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowCloseRequestedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowClosedEventArgs {
    type Vtable = IAppWindowClosedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc7df816_9520_5a06_821e_456ad8b358aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowClosedReason) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrame(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowFrame {
    type Vtable = IAppWindowFrameVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ee22601_7e5d_52af_846b_01dc6c296567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrameVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Composition")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowFrameStyle(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowFrameStyle {
    type Vtable = IAppWindowFrameStyleVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac412946_e1ac_5230_944a_c60873dcf4a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowFrameStyleVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowFrameStyle) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framestyle: AppWindowFrameStyle) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPlacement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPlacement {
    type Vtable = IAppWindowPlacementVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03dc815e_e7a9_5857_9c03_7d670594410e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPlacementVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresentationConfiguration {
    type Vtable = IAppWindowPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5a43ee3_df33_5e67_bd31_1072457300df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowPresentationKind) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresentationConfigurationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresentationConfigurationFactory {
    type Vtable = IAppWindowPresentationConfigurationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd3606a6_7875_5de8_84ff_6351ee13dd0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresentationConfigurationFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ae9ed73_e1fd_5317_ad78_5a3ed271bbde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff1f3ea3_b769_50ef_9873_108cd0e89746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBarVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e932c84_f644_541d_a2d7_0c262437842d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarOcclusion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarOcclusion {
    type Vtable = IAppWindowTitleBarOcclusionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfea3cffd_2ccf_5fc3_aeae_f843876bf37e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarOcclusionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarVisibility(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarVisibility {
    type Vtable = IAppWindowTitleBarVisibilityVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa215a4e3_6e7e_5651_8c3b_624819528154);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarVisibilityVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppWindowTitleBarVisibility) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresentationConfiguration {
    type Vtable = ICompactOverlayPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7e5750f_5730_56c6_8e1f_d63ff4d7980d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresentationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDefaultPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDefaultPresentationConfiguration {
    type Vtable = IDefaultPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8c2b53b_2168_5703_a853_d525589fe2b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultPresentationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayRegion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayRegion {
    type Vtable = IDisplayRegionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb50c3a2_4094_5f47_8cb1_ea01ddafaa94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayRegionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresentationConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresentationConfiguration {
    type Vtable = IFullScreenPresentationConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43d3dcd8_d2a8_503d_a626_15533d6d5f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresentationConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowServicesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowServicesStatics {
    type Vtable = IWindowServicesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcff4d519_50a6_5c64_97f6_c2d96add7f42);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowServicesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironment {
    type Vtable = IWindowingEnvironmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x264363c0_2a49_5417_b3ae_48a71c63a3bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WindowingEnvironmentKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentAddedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a5b7f_f183_5c66_99b2_429082069299);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentAddedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4160cfc6_023d_5e9a_b431_350e67dc978a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentChangedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e5b5473_beff_5e53_9316_7e775fe568b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentRemovedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowingEnvironmentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowingEnvironmentStatics {
    type Vtable = IWindowingEnvironmentStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x874e9fb7_c642_55ab_8aa2_162f734a9a72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowingEnvironmentStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: WindowingEnvironmentKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: 'UI_WindowManagement'*"]
pub struct WindowServices {}
impl WindowServices {
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllTopLevelWindowIds() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>> {
        Self::IWindowServicesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::WindowId>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowServicesStatics<R, F: FnOnce(&IWindowServicesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowServices, IWindowServicesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WindowServices {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowServices";
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct WindowingEnvironment(::windows::core::IUnknown);
impl WindowingEnvironment {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn Kind(&self) -> ::windows::core::Result<WindowingEnvironmentKind> {
        let this = self;
        unsafe {
            let mut result__: WindowingEnvironmentKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowingEnvironmentKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<DisplayRegion>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAll() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>(result__)
        })
    }
    #[doc = "*Required features: 'UI_WindowManagement', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllWithKind(kind: WindowingEnvironmentKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>> {
        Self::IWindowingEnvironmentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWindowingEnvironmentStatics<R, F: FnOnce(&IWindowingEnvironmentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowingEnvironment, IWindowingEnvironmentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowingEnvironment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironment {}
impl ::core::fmt::Debug for WindowingEnvironment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowingEnvironment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironment;{264363c0-2a49-5417-b3ae-48a71c63a3bd})");
}
unsafe impl ::windows::core::Interface for WindowingEnvironment {
    type Vtable = IWindowingEnvironmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x264363c0_2a49_5417_b3ae_48a71c63a3bd);
}
impl ::windows::core::RuntimeName for WindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironment";
}
impl ::core::convert::From<WindowingEnvironment> for ::windows::core::IUnknown {
    fn from(value: WindowingEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironment> for ::windows::core::IUnknown {
    fn from(value: &WindowingEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowingEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowingEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironment> for ::windows::core::IInspectable {
    fn from(value: WindowingEnvironment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironment> for ::windows::core::IInspectable {
    fn from(value: &WindowingEnvironment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowingEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowingEnvironment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironment {}
unsafe impl ::core::marker::Sync for WindowingEnvironment {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct WindowingEnvironmentAddedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentAddedEventArgs {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowingEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for WindowingEnvironmentAddedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentAddedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentAddedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowingEnvironmentAddedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs;{ff2a5b7f-f183-5c66-99b2-429082069299})");
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentAddedEventArgs {
    type Vtable = IWindowingEnvironmentAddedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff2a5b7f_f183_5c66_99b2_429082069299);
}
impl ::windows::core::RuntimeName for WindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentAddedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WindowingEnvironmentAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentAddedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WindowingEnvironmentAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WindowingEnvironmentAddedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentAddedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WindowingEnvironmentAddedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowingEnvironmentAddedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentAddedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentAddedEventArgs {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct WindowingEnvironmentChangedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentChangedEventArgs {}
impl ::core::clone::Clone for WindowingEnvironmentChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentChangedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowingEnvironmentChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs;{4160cfc6-023d-5e9a-b431-350e67dc978a})");
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentChangedEventArgs {
    type Vtable = IWindowingEnvironmentChangedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4160cfc6_023d_5e9a_b431_350e67dc978a);
}
impl ::windows::core::RuntimeName for WindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentChangedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WindowingEnvironmentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WindowingEnvironmentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WindowingEnvironmentChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WindowingEnvironmentChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowingEnvironmentChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentChangedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentChangedEventArgs {}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct WindowingEnvironmentKind(pub i32);
impl WindowingEnvironmentKind {
    pub const Unknown: Self = Self(0i32);
    pub const Overlapped: Self = Self(1i32);
    pub const Tiled: Self = Self(2i32);
}
impl ::core::marker::Copy for WindowingEnvironmentKind {}
impl ::core::clone::Clone for WindowingEnvironmentKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WindowingEnvironmentKind {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WindowingEnvironmentKind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentKind {}
impl ::core::fmt::Debug for WindowingEnvironmentKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowingEnvironmentKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WindowManagement.WindowingEnvironmentKind;i4)");
}
impl ::windows::core::DefaultType for WindowingEnvironmentKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_WindowManagement'*"]
#[repr(transparent)]
pub struct WindowingEnvironmentRemovedEventArgs(::windows::core::IUnknown);
impl WindowingEnvironmentRemovedEventArgs {
    #[doc = "*Required features: 'UI_WindowManagement'*"]
    pub fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WindowingEnvironment>(result__)
        }
    }
}
impl ::core::clone::Clone for WindowingEnvironmentRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowingEnvironmentRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowingEnvironmentRemovedEventArgs {}
impl ::core::fmt::Debug for WindowingEnvironmentRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowingEnvironmentRemovedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowingEnvironmentRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs;{2e5b5473-beff-5e53-9316-7e775fe568b3})");
}
unsafe impl ::windows::core::Interface for WindowingEnvironmentRemovedEventArgs {
    type Vtable = IWindowingEnvironmentRemovedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e5b5473_beff_5e53_9316_7e775fe568b3);
}
impl ::windows::core::RuntimeName for WindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.WindowingEnvironmentRemovedEventArgs";
}
impl ::core::convert::From<WindowingEnvironmentRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WindowingEnvironmentRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WindowingEnvironmentRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowingEnvironmentRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WindowingEnvironmentRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowingEnvironmentRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WindowingEnvironmentRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WindowingEnvironmentRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowingEnvironmentRemovedEventArgs {}
unsafe impl ::core::marker::Sync for WindowingEnvironmentRemovedEventArgs {}
