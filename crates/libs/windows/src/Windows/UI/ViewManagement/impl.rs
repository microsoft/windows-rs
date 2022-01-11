#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccessibilitySettingsImpl: Sized {
    fn HighContrast(&self) -> ::windows::core::Result<bool>;
    fn HighContrastScheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighContrastChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHighContrastChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccessibilitySettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IAccessibilitySettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccessibilitySettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibilitySettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibilitySettingsVtbl {
        unsafe extern "system" fn HighContrast<Impl: IAccessibilitySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrast() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighContrastScheme<Impl: IAccessibilitySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastScheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighContrastChanged<Impl: IAccessibilitySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHighContrastChanged<Impl: IAccessibilitySettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHighContrastChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessibilitySettings, BASE_OFFSET>(),
            HighContrast: HighContrast::<Impl, IMPL_OFFSET>,
            HighContrastScheme: HighContrastScheme::<Impl, IMPL_OFFSET>,
            HighContrastChanged: HighContrastChanged::<Impl, IMPL_OFFSET>,
            RemoveHighContrastChanged: RemoveHighContrastChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessibilitySettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IActivationViewSwitcherImpl: Sized {
    fn ShowAsStandaloneAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn IsViewPresentedOnActivationVirtualDesktop(&self, viewid: i32) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.IActivationViewSwitcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivationViewSwitcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationViewSwitcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationViewSwitcherVtbl {
        unsafe extern "system" fn ShowAsStandaloneAsync<Impl: IActivationViewSwitcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsStandaloneAsync(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAsStandaloneWithSizePreferenceAsync<Impl: IActivationViewSwitcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsStandaloneWithSizePreferenceAsync(viewid, sizepreference) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsViewPresentedOnActivationVirtualDesktop<Impl: IActivationViewSwitcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsViewPresentedOnActivationVirtualDesktop(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationViewSwitcher, BASE_OFFSET>(),
            ShowAsStandaloneAsync: ShowAsStandaloneAsync::<Impl, IMPL_OFFSET>,
            ShowAsStandaloneWithSizePreferenceAsync: ShowAsStandaloneWithSizePreferenceAsync::<Impl, IMPL_OFFSET>,
            IsViewPresentedOnActivationVirtualDesktop: IsViewPresentedOnActivationVirtualDesktop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationViewSwitcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationViewImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<ApplicationViewOrientation>;
    fn AdjacentToLeftDisplayEdge(&self) -> ::windows::core::Result<bool>;
    fn AdjacentToRightDisplayEdge(&self) -> ::windows::core::Result<bool>;
    fn IsFullScreen(&self) -> ::windows::core::Result<bool>;
    fn IsOnLockScreen(&self) -> ::windows::core::Result<bool>;
    fn IsScreenCaptureEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsScreenCaptureEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Consolidated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConsolidated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewVtbl {
        unsafe extern "system" fn Orientation<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjacentToLeftDisplayEdge<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjacentToLeftDisplayEdge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdjacentToRightDisplayEdge<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdjacentToRightDisplayEdge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFullScreen<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullScreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOnLockScreen<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOnLockScreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScreenCaptureEnabled<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScreenCaptureEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsScreenCaptureEnabled<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsScreenCaptureEnabled(value).into()
        }
        unsafe extern "system" fn SetTitle<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Consolidated<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Consolidated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConsolidated<Impl: IApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConsolidated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView, BASE_OFFSET>(),
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            AdjacentToLeftDisplayEdge: AdjacentToLeftDisplayEdge::<Impl, IMPL_OFFSET>,
            AdjacentToRightDisplayEdge: AdjacentToRightDisplayEdge::<Impl, IMPL_OFFSET>,
            IsFullScreen: IsFullScreen::<Impl, IMPL_OFFSET>,
            IsOnLockScreen: IsOnLockScreen::<Impl, IMPL_OFFSET>,
            IsScreenCaptureEnabled: IsScreenCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetIsScreenCaptureEnabled: SetIsScreenCaptureEnabled::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Consolidated: Consolidated::<Impl, IMPL_OFFSET>,
            RemoveConsolidated: RemoveConsolidated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationView2Impl: Sized {
    fn SuppressSystemOverlays(&self) -> ::windows::core::Result<bool>;
    fn SetSuppressSystemOverlays(&self, value: bool) -> ::windows::core::Result<()>;
    fn VisibleBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn VisibleBoundsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibleBoundsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDesiredBoundsMode(&self, boundsmode: ApplicationViewBoundsMode) -> ::windows::core::Result<bool>;
    fn DesiredBoundsMode(&self) -> ::windows::core::Result<ApplicationViewBoundsMode>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView2Vtbl {
        unsafe extern "system" fn SuppressSystemOverlays<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressSystemOverlays() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressSystemOverlays<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuppressSystemOverlays(value).into()
        }
        unsafe extern "system" fn VisibleBounds<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibleBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleBoundsChanged<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibleBoundsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibleBoundsChanged<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisibleBoundsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDesiredBoundsMode<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundsmode: ApplicationViewBoundsMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredBoundsMode(boundsmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredBoundsMode<Impl: IApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewBoundsMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredBoundsMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView2, BASE_OFFSET>(),
            SuppressSystemOverlays: SuppressSystemOverlays::<Impl, IMPL_OFFSET>,
            SetSuppressSystemOverlays: SetSuppressSystemOverlays::<Impl, IMPL_OFFSET>,
            VisibleBounds: VisibleBounds::<Impl, IMPL_OFFSET>,
            VisibleBoundsChanged: VisibleBoundsChanged::<Impl, IMPL_OFFSET>,
            RemoveVisibleBoundsChanged: RemoveVisibleBoundsChanged::<Impl, IMPL_OFFSET>,
            SetDesiredBoundsMode: SetDesiredBoundsMode::<Impl, IMPL_OFFSET>,
            DesiredBoundsMode: DesiredBoundsMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationView3Impl: Sized {
    fn TitleBar(&self) -> ::windows::core::Result<ApplicationViewTitleBar>;
    fn FullScreenSystemOverlayMode(&self) -> ::windows::core::Result<FullScreenSystemOverlayMode>;
    fn SetFullScreenSystemOverlayMode(&self, value: FullScreenSystemOverlayMode) -> ::windows::core::Result<()>;
    fn IsFullScreenMode(&self) -> ::windows::core::Result<bool>;
    fn TryEnterFullScreenMode(&self) -> ::windows::core::Result<bool>;
    fn ExitFullScreenMode(&self) -> ::windows::core::Result<()>;
    fn ShowStandardSystemOverlays(&self) -> ::windows::core::Result<()>;
    fn TryResizeView(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetPreferredMinSize(&self, minsize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView3Vtbl {
        unsafe extern "system" fn TitleBar<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullScreenSystemOverlayMode<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FullScreenSystemOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullScreenSystemOverlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullScreenSystemOverlayMode<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FullScreenSystemOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullScreenSystemOverlayMode(value).into()
        }
        unsafe extern "system" fn IsFullScreenMode<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullScreenMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnterFullScreenMode<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnterFullScreenMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitFullScreenMode<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExitFullScreenMode().into()
        }
        unsafe extern "system" fn ShowStandardSystemOverlays<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowStandardSystemOverlays().into()
        }
        unsafe extern "system" fn TryResizeView<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryResizeView(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredMinSize<Impl: IApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minsize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredMinSize(&*(&minsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView3, BASE_OFFSET>(),
            TitleBar: TitleBar::<Impl, IMPL_OFFSET>,
            FullScreenSystemOverlayMode: FullScreenSystemOverlayMode::<Impl, IMPL_OFFSET>,
            SetFullScreenSystemOverlayMode: SetFullScreenSystemOverlayMode::<Impl, IMPL_OFFSET>,
            IsFullScreenMode: IsFullScreenMode::<Impl, IMPL_OFFSET>,
            TryEnterFullScreenMode: TryEnterFullScreenMode::<Impl, IMPL_OFFSET>,
            ExitFullScreenMode: ExitFullScreenMode::<Impl, IMPL_OFFSET>,
            ShowStandardSystemOverlays: ShowStandardSystemOverlays::<Impl, IMPL_OFFSET>,
            TryResizeView: TryResizeView::<Impl, IMPL_OFFSET>,
            SetPreferredMinSize: SetPreferredMinSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationView4Impl: Sized {
    fn ViewMode(&self) -> ::windows::core::Result<ApplicationViewMode>;
    fn IsViewModeSupported(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<bool>;
    fn TryEnterViewModeAsync(&self, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryEnterViewModeWithPreferencesAsync(&self, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryConsolidateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView4Vtbl {
        unsafe extern "system" fn ViewMode<Impl: IApplicationView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsViewModeSupported<Impl: IApplicationView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsViewModeSupported(viewmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnterViewModeAsync<Impl: IApplicationView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnterViewModeAsync(viewmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnterViewModeWithPreferencesAsync<Impl: IApplicationView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, viewmodepreferences: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnterViewModeWithPreferencesAsync(viewmode, &*(&viewmodepreferences as *const <ViewModePreferences as ::windows::core::Abi>::Abi as *const <ViewModePreferences as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryConsolidateAsync<Impl: IApplicationView4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryConsolidateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView4, BASE_OFFSET>(),
            ViewMode: ViewMode::<Impl, IMPL_OFFSET>,
            IsViewModeSupported: IsViewModeSupported::<Impl, IMPL_OFFSET>,
            TryEnterViewModeAsync: TryEnterViewModeAsync::<Impl, IMPL_OFFSET>,
            TryEnterViewModeWithPreferencesAsync: TryEnterViewModeWithPreferencesAsync::<Impl, IMPL_OFFSET>,
            TryConsolidateAsync: TryConsolidateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationView7Impl: Sized {
    fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationView7 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView7";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationView7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView7Vtbl {
        unsafe extern "system" fn PersistedStateId<Impl: IApplicationView7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PersistedStateId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistedStateId<Impl: IApplicationView7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersistedStateId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView7, BASE_OFFSET>(),
            PersistedStateId: PersistedStateId::<Impl, IMPL_OFFSET>,
            SetPersistedStateId: SetPersistedStateId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait IApplicationView9Impl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<super::WindowManagement::WindowingEnvironment>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView9 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView9";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IApplicationView9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView9Vtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IApplicationView9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowingEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayRegions<Impl: IApplicationView9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationView9, BASE_OFFSET>(),
            WindowingEnvironment: WindowingEnvironment::<Impl, IMPL_OFFSET>,
            GetDisplayRegions: GetDisplayRegions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationView9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewConsolidatedEventArgsImpl: Sized {
    fn IsUserInitiated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewConsolidatedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewConsolidatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewConsolidatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewConsolidatedEventArgsVtbl {
        unsafe extern "system" fn IsUserInitiated<Impl: IApplicationViewConsolidatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserInitiated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewConsolidatedEventArgs, BASE_OFFSET>(),
            IsUserInitiated: IsUserInitiated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewConsolidatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewConsolidatedEventArgs2Impl: Sized {
    fn IsAppInitiated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewConsolidatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewConsolidatedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewConsolidatedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewConsolidatedEventArgs2Vtbl {
        unsafe extern "system" fn IsAppInitiated<Impl: IApplicationViewConsolidatedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppInitiated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewConsolidatedEventArgs2, BASE_OFFSET>(),
            IsAppInitiated: IsAppInitiated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewConsolidatedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IApplicationViewFullscreenStaticsImpl: Sized {
    fn TryUnsnapToFullscreen(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewFullscreenStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewFullscreenStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IApplicationViewFullscreenStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewFullscreenStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewFullscreenStaticsVtbl {
        unsafe extern "system" fn TryUnsnapToFullscreen<Impl: IApplicationViewFullscreenStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUnsnapToFullscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewFullscreenStatics, BASE_OFFSET>(),
            TryUnsnapToFullscreen: TryUnsnapToFullscreen::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewFullscreenStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IApplicationViewInteropStaticsImpl: Sized {
    fn GetApplicationViewIdForWindow(&self, window: &::core::option::Option<super::Core::ICoreWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewInteropStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewInteropStatics";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl IApplicationViewInteropStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewInteropStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewInteropStaticsVtbl {
        unsafe extern "system" fn GetApplicationViewIdForWindow<Impl: IApplicationViewInteropStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationViewIdForWindow(&*(&window as *const <super::Core::ICoreWindow as ::windows::core::Abi>::Abi as *const <super::Core::ICoreWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewInteropStatics, BASE_OFFSET>(),
            GetApplicationViewIdForWindow: GetApplicationViewIdForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewInteropStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewScalingImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewScaling {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewScaling";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewScalingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewScalingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewScalingVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewScaling, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewScaling as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewScalingStaticsImpl: Sized {
    fn DisableLayoutScaling(&self) -> ::windows::core::Result<bool>;
    fn TrySetDisableLayoutScaling(&self, disablelayoutscaling: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewScalingStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewScalingStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewScalingStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewScalingStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewScalingStaticsVtbl {
        unsafe extern "system" fn DisableLayoutScaling<Impl: IApplicationViewScalingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableLayoutScaling() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetDisableLayoutScaling<Impl: IApplicationViewScalingStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablelayoutscaling: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetDisableLayoutScaling(disablelayoutscaling) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewScalingStatics, BASE_OFFSET>(),
            DisableLayoutScaling: DisableLayoutScaling::<Impl, IMPL_OFFSET>,
            TrySetDisableLayoutScaling: TrySetDisableLayoutScaling::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewScalingStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IApplicationViewStaticsImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<ApplicationViewState>;
    fn TryUnsnap(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IApplicationViewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStaticsVtbl {
        unsafe extern "system" fn Value<Impl: IApplicationViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryUnsnap<Impl: IApplicationViewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUnsnap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewStatics, BASE_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            TryUnsnap: TryUnsnap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewStatics2Impl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<ApplicationView>;
    fn TerminateAppOnFinalViewClose(&self) -> ::windows::core::Result<bool>;
    fn SetTerminateAppOnFinalViewClose(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics2Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IApplicationViewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateAppOnFinalViewClose<Impl: IApplicationViewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TerminateAppOnFinalViewClose() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTerminateAppOnFinalViewClose<Impl: IApplicationViewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTerminateAppOnFinalViewClose(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewStatics2, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            TerminateAppOnFinalViewClose: TerminateAppOnFinalViewClose::<Impl, IMPL_OFFSET>,
            SetTerminateAppOnFinalViewClose: SetTerminateAppOnFinalViewClose::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationViewStatics3Impl: Sized {
    fn PreferredLaunchWindowingMode(&self) -> ::windows::core::Result<ApplicationViewWindowingMode>;
    fn SetPreferredLaunchWindowingMode(&self, value: ApplicationViewWindowingMode) -> ::windows::core::Result<()>;
    fn PreferredLaunchViewSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetPreferredLaunchViewSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewStatics3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics3Vtbl {
        unsafe extern "system" fn PreferredLaunchWindowingMode<Impl: IApplicationViewStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewWindowingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredLaunchWindowingMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredLaunchWindowingMode<Impl: IApplicationViewStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ApplicationViewWindowingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredLaunchWindowingMode(value).into()
        }
        unsafe extern "system" fn PreferredLaunchViewSize<Impl: IApplicationViewStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredLaunchViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredLaunchViewSize<Impl: IApplicationViewStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredLaunchViewSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewStatics3, BASE_OFFSET>(),
            PreferredLaunchWindowingMode: PreferredLaunchWindowingMode::<Impl, IMPL_OFFSET>,
            SetPreferredLaunchWindowingMode: SetPreferredLaunchWindowingMode::<Impl, IMPL_OFFSET>,
            PreferredLaunchViewSize: PreferredLaunchViewSize::<Impl, IMPL_OFFSET>,
            SetPreferredLaunchViewSize: SetPreferredLaunchViewSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewStatics4Impl: Sized {
    fn ClearAllPersistedState(&self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewStatics4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics4Vtbl {
        unsafe extern "system" fn ClearAllPersistedState<Impl: IApplicationViewStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllPersistedState().into()
        }
        unsafe extern "system" fn ClearPersistedState<Impl: IApplicationViewStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPersistedState(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewStatics4, BASE_OFFSET>(),
            ClearAllPersistedState: ClearAllPersistedState::<Impl, IMPL_OFFSET>,
            ClearPersistedState: ClearPersistedState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationViewSwitcherStaticsImpl: Sized {
    fn DisableShowingMainViewOnActivation(&self) -> ::windows::core::Result<()>;
    fn TryShowAsStandaloneAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(&self, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SwitchAsync(&self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewAsync(&self, toviewid: i32, fromviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewWithOptionsAsync(&self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PrepareForCustomAnimatedSwitchAsync(&self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewSwitcherStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStaticsVtbl {
        unsafe extern "system" fn DisableShowingMainViewOnActivation<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableShowingMainViewOnActivation().into()
        }
        unsafe extern "system" fn TryShowAsStandaloneAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsStandaloneAsync(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowAsStandaloneWithSizePreferenceAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsStandaloneWithSizePreferenceAsync(viewid, sizepreference) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(viewid, sizepreference, anchorviewid, anchorsizepreference) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchAsync(viewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchFromViewAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchFromViewAsync(toviewid, fromviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwitchFromViewWithOptionsAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwitchFromViewWithOptionsAsync(toviewid, fromviewid, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareForCustomAnimatedSwitchAsync<Impl: IApplicationViewSwitcherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareForCustomAnimatedSwitchAsync(toviewid, fromviewid, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewSwitcherStatics, BASE_OFFSET>(),
            DisableShowingMainViewOnActivation: DisableShowingMainViewOnActivation::<Impl, IMPL_OFFSET>,
            TryShowAsStandaloneAsync: TryShowAsStandaloneAsync::<Impl, IMPL_OFFSET>,
            TryShowAsStandaloneWithSizePreferenceAsync: TryShowAsStandaloneWithSizePreferenceAsync::<Impl, IMPL_OFFSET>,
            TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync: TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync::<Impl, IMPL_OFFSET>,
            SwitchAsync: SwitchAsync::<Impl, IMPL_OFFSET>,
            SwitchFromViewAsync: SwitchFromViewAsync::<Impl, IMPL_OFFSET>,
            SwitchFromViewWithOptionsAsync: SwitchFromViewWithOptionsAsync::<Impl, IMPL_OFFSET>,
            PrepareForCustomAnimatedSwitchAsync: PrepareForCustomAnimatedSwitchAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewSwitcherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewSwitcherStatics2Impl: Sized {
    fn DisableSystemViewActivationPolicy(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewSwitcherStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStatics2Vtbl {
        unsafe extern "system" fn DisableSystemViewActivationPolicy<Impl: IApplicationViewSwitcherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableSystemViewActivationPolicy().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewSwitcherStatics2, BASE_OFFSET>(),
            DisableSystemViewActivationPolicy: DisableSystemViewActivationPolicy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewSwitcherStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationViewSwitcherStatics3Impl: Sized {
    fn TryShowAsViewModeAsync(&self, viewid: i32, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsViewModeWithPreferencesAsync(&self, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewSwitcherStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStatics3Vtbl {
        unsafe extern "system" fn TryShowAsViewModeAsync<Impl: IApplicationViewSwitcherStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsViewModeAsync(viewid, viewmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryShowAsViewModeWithPreferencesAsync<Impl: IApplicationViewSwitcherStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsViewModeWithPreferencesAsync(viewid, viewmode, &*(&viewmodepreferences as *const <ViewModePreferences as ::windows::core::Abi>::Abi as *const <ViewModePreferences as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewSwitcherStatics3, BASE_OFFSET>(),
            TryShowAsViewModeAsync: TryShowAsViewModeAsync::<Impl, IMPL_OFFSET>,
            TryShowAsViewModeWithPreferencesAsync: TryShowAsViewModeWithPreferencesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewSwitcherStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplicationViewTitleBarImpl: Sized {
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewTitleBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTitleBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewTitleBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTitleBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTitleBarVtbl {
        unsafe extern "system" fn SetForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonHoverForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonHoverForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonHoverBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonHoverBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonPressedForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonPressedForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonPressedBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonPressedBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInactiveForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InactiveForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInactiveBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InactiveBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonInactiveForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveForegroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonInactiveForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonInactiveBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveBackgroundColor<Impl: IApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ButtonInactiveBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewTitleBar, BASE_OFFSET>(),
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonForegroundColor: SetButtonForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonForegroundColor: ButtonForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonBackgroundColor: SetButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonBackgroundColor: ButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverForegroundColor: SetButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverForegroundColor: ButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverBackgroundColor: SetButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverBackgroundColor: ButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedForegroundColor: SetButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedForegroundColor: ButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedBackgroundColor: SetButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedBackgroundColor: ButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveForegroundColor: SetInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            InactiveForegroundColor: InactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveBackgroundColor: SetInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            InactiveBackgroundColor: InactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveForegroundColor: SetButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveForegroundColor: ButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveBackgroundColor: SetButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveBackgroundColor: ButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewTitleBar as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewTransferContextImpl: Sized {
    fn ViewId(&self) -> ::windows::core::Result<i32>;
    fn SetViewId(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewTransferContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTransferContext";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewTransferContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTransferContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTransferContextVtbl {
        unsafe extern "system" fn ViewId<Impl: IApplicationViewTransferContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewId<Impl: IApplicationViewTransferContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewId(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewTransferContext, BASE_OFFSET>(),
            ViewId: ViewId::<Impl, IMPL_OFFSET>,
            SetViewId: SetViewId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewTransferContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewTransferContextStaticsImpl: Sized {
    fn DataPackageFormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewTransferContextStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTransferContextStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewTransferContextStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTransferContextStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTransferContextStaticsVtbl {
        unsafe extern "system" fn DataPackageFormatId<Impl: IApplicationViewTransferContextStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataPackageFormatId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewTransferContextStatics, BASE_OFFSET>(),
            DataPackageFormatId: DataPackageFormatId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewTransferContextStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewWithContextImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewWithContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewWithContext";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewWithContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewWithContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewWithContextVtbl {
        unsafe extern "system" fn UIContext<Impl: IApplicationViewWithContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewWithContext, BASE_OFFSET>(), UIContext: UIContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewWithContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInputPaneImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputPane {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPane";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInputPaneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneVtbl {
        unsafe extern "system" fn Showing<Impl: IInputPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Showing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShowing<Impl: IInputPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hiding<Impl: IInputPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hiding(&*(&handler as *const <super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHiding<Impl: IInputPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHiding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OccludedRect<Impl: IInputPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OccludedRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPane, BASE_OFFSET>(),
            Showing: Showing::<Impl, IMPL_OFFSET>,
            RemoveShowing: RemoveShowing::<Impl, IMPL_OFFSET>,
            Hiding: Hiding::<Impl, IMPL_OFFSET>,
            RemoveHiding: RemoveHiding::<Impl, IMPL_OFFSET>,
            OccludedRect: OccludedRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPane as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPane2Impl: Sized {
    fn TryShow(&self) -> ::windows::core::Result<bool>;
    fn TryHide(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPane2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPane2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPane2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPane2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPane2Vtbl {
        unsafe extern "system" fn TryShow<Impl: IInputPane2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryHide<Impl: IInputPane2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryHide() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPane2, BASE_OFFSET>(),
            TryShow: TryShow::<Impl, IMPL_OFFSET>,
            TryHide: TryHide::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPane2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneControlImpl: Sized {
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn SetVisible(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneControl {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneControl";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneControlVtbl {
        unsafe extern "system" fn Visible<Impl: IInputPaneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisible<Impl: IInputPaneControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisible(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPaneControl, BASE_OFFSET>(),
            Visible: Visible::<Impl, IMPL_OFFSET>,
            SetVisible: SetVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IInputPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPaneStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputPaneStatics2Impl: Sized {
    fn GetForUIContext(&self, context: &::core::option::Option<super::UIContext>) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneStatics2Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IInputPaneStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUIContext(&*(&context as *const <super::UIContext as ::windows::core::Abi>::Abi as *const <super::UIContext as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPaneStatics2, BASE_OFFSET>(),
            GetForUIContext: GetForUIContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IInputPaneVisibilityEventArgsImpl: Sized {
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetEnsuredFocusedElementInView(&self, value: bool) -> ::windows::core::Result<()>;
    fn EnsuredFocusedElementInView(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputPaneVisibilityEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInputPaneVisibilityEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneVisibilityEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneVisibilityEventArgsVtbl {
        unsafe extern "system" fn OccludedRect<Impl: IInputPaneVisibilityEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OccludedRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnsuredFocusedElementInView<Impl: IInputPaneVisibilityEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnsuredFocusedElementInView(value).into()
        }
        unsafe extern "system" fn EnsuredFocusedElementInView<Impl: IInputPaneVisibilityEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnsuredFocusedElementInView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPaneVisibilityEventArgs, BASE_OFFSET>(),
            OccludedRect: OccludedRect::<Impl, IMPL_OFFSET>,
            SetEnsuredFocusedElementInView: SetEnsuredFocusedElementInView::<Impl, IMPL_OFFSET>,
            EnsuredFocusedElementInView: EnsuredFocusedElementInView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneVisibilityEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProjectionManagerStaticsImpl: Sized {
    fn StartProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwapDisplaysForViewsAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ProjectionDisplayAvailable(&self) -> ::windows::core::Result<bool>;
    fn ProjectionDisplayAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProjectionDisplayAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProjectionManagerStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IProjectionManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProjectionManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjectionManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjectionManagerStaticsVtbl {
        unsafe extern "system" fn StartProjectingAsync<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProjectingAsync(projectionviewid, anchorviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SwapDisplaysForViewsAsync<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SwapDisplaysForViewsAsync(projectionviewid, anchorviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopProjectingAsync<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopProjectingAsync(projectionviewid, anchorviewid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionDisplayAvailable<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionDisplayAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionDisplayAvailableChanged<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionDisplayAvailableChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProjectionDisplayAvailableChanged<Impl: IProjectionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProjectionDisplayAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProjectionManagerStatics, BASE_OFFSET>(),
            StartProjectingAsync: StartProjectingAsync::<Impl, IMPL_OFFSET>,
            SwapDisplaysForViewsAsync: SwapDisplaysForViewsAsync::<Impl, IMPL_OFFSET>,
            StopProjectingAsync: StopProjectingAsync::<Impl, IMPL_OFFSET>,
            ProjectionDisplayAvailable: ProjectionDisplayAvailable::<Impl, IMPL_OFFSET>,
            ProjectionDisplayAvailableChanged: ProjectionDisplayAvailableChanged::<Impl, IMPL_OFFSET>,
            RemoveProjectionDisplayAvailableChanged: RemoveProjectionDisplayAvailableChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProjectionManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait IProjectionManagerStatics2Impl: Sized {
    fn StartProjectingWithDeviceInfoAsync(&self, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestStartProjectingAsync(&self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestStartProjectingWithPlacementAsync(&self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProjectionManagerStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IProjectionManagerStatics2";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IProjectionManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjectionManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjectionManagerStatics2Vtbl {
        unsafe extern "system" fn StartProjectingWithDeviceInfoAsync<Impl: IProjectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProjectingWithDeviceInfoAsync(projectionviewid, anchorviewid, &*(&displaydeviceinfo as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Devices::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStartProjectingAsync<Impl: IProjectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStartProjectingAsync(projectionviewid, anchorviewid, &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestStartProjectingWithPlacementAsync<Impl: IProjectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStartProjectingWithPlacementAsync(projectionviewid, anchorviewid, &*(&selection as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), prefferedplacement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IProjectionManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProjectionManagerStatics2, BASE_OFFSET>(),
            StartProjectingWithDeviceInfoAsync: StartProjectingWithDeviceInfoAsync::<Impl, IMPL_OFFSET>,
            RequestStartProjectingAsync: RequestStartProjectingAsync::<Impl, IMPL_OFFSET>,
            RequestStartProjectingWithPlacementAsync: RequestStartProjectingWithPlacementAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProjectionManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStatusBarImpl: Sized {
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BackgroundOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetBackgroundOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ProgressIndicator(&self) -> ::windows::core::Result<StatusBarProgressIndicator>;
    fn OccludedRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Showing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStatusBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStatusBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBarVtbl {
        unsafe extern "system" fn ShowAsync<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HideAsync<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HideAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackgroundOpacity<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundOpacity<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundOpacity(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProgressIndicator<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressIndicator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OccludedRect<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OccludedRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Showing<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Showing(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShowing<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hiding<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hiding(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHiding<Impl: IStatusBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHiding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStatusBar, BASE_OFFSET>(),
            ShowAsync: ShowAsync::<Impl, IMPL_OFFSET>,
            HideAsync: HideAsync::<Impl, IMPL_OFFSET>,
            BackgroundOpacity: BackgroundOpacity::<Impl, IMPL_OFFSET>,
            SetBackgroundOpacity: SetBackgroundOpacity::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            ProgressIndicator: ProgressIndicator::<Impl, IMPL_OFFSET>,
            OccludedRect: OccludedRect::<Impl, IMPL_OFFSET>,
            Showing: Showing::<Impl, IMPL_OFFSET>,
            RemoveShowing: RemoveShowing::<Impl, IMPL_OFFSET>,
            Hiding: Hiding::<Impl, IMPL_OFFSET>,
            RemoveHiding: RemoveHiding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStatusBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStatusBarProgressIndicatorImpl: Sized {
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProgressValue(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SetProgressValue(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStatusBarProgressIndicator {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBarProgressIndicator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStatusBarProgressIndicatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBarProgressIndicatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBarProgressIndicatorVtbl {
        unsafe extern "system" fn ShowAsync<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HideAsync<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HideAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProgressValue<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgressValue<Impl: IStatusBarProgressIndicatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgressValue(&*(&value as *const <super::super::Foundation::IReference<f64> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<f64> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStatusBarProgressIndicator, BASE_OFFSET>(),
            ShowAsync: ShowAsync::<Impl, IMPL_OFFSET>,
            HideAsync: HideAsync::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
            SetText: SetText::<Impl, IMPL_OFFSET>,
            ProgressValue: ProgressValue::<Impl, IMPL_OFFSET>,
            SetProgressValue: SetProgressValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStatusBarProgressIndicator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusBarStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<StatusBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStatusBarStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStatusBarStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBarStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBarStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IStatusBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStatusBarStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStatusBarStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettingsImpl: Sized {
    fn HandPreference(&self) -> ::windows::core::Result<HandPreference>;
    fn CursorSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarArrowSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarThumbBoxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MessageDuration(&self) -> ::windows::core::Result<u32>;
    fn AnimationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn CaretBrowsingEnabled(&self) -> ::windows::core::Result<bool>;
    fn CaretBlinkRate(&self) -> ::windows::core::Result<u32>;
    fn CaretWidth(&self) -> ::windows::core::Result<u32>;
    fn DoubleClickTime(&self) -> ::windows::core::Result<u32>;
    fn MouseHoverTime(&self) -> ::windows::core::Result<u32>;
    fn UIElementColor(&self, desiredelement: UIElementType) -> ::windows::core::Result<super::Color>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsVtbl {
        unsafe extern "system" fn HandPreference<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HandPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandPreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CursorSize<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CursorSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollBarSize<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollBarSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollBarArrowSize<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollBarArrowSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollBarThumbBoxSize<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollBarThumbBoxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageDuration<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AnimationsEnabled<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaretBrowsingEnabled<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaretBrowsingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaretBlinkRate<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaretBlinkRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CaretWidth<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CaretWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoubleClickTime<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoubleClickTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MouseHoverTime<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MouseHoverTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIElementColor<Impl: IUISettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredelement: UIElementType, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIElementColor(desiredelement) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings, BASE_OFFSET>(),
            HandPreference: HandPreference::<Impl, IMPL_OFFSET>,
            CursorSize: CursorSize::<Impl, IMPL_OFFSET>,
            ScrollBarSize: ScrollBarSize::<Impl, IMPL_OFFSET>,
            ScrollBarArrowSize: ScrollBarArrowSize::<Impl, IMPL_OFFSET>,
            ScrollBarThumbBoxSize: ScrollBarThumbBoxSize::<Impl, IMPL_OFFSET>,
            MessageDuration: MessageDuration::<Impl, IMPL_OFFSET>,
            AnimationsEnabled: AnimationsEnabled::<Impl, IMPL_OFFSET>,
            CaretBrowsingEnabled: CaretBrowsingEnabled::<Impl, IMPL_OFFSET>,
            CaretBlinkRate: CaretBlinkRate::<Impl, IMPL_OFFSET>,
            CaretWidth: CaretWidth::<Impl, IMPL_OFFSET>,
            DoubleClickTime: DoubleClickTime::<Impl, IMPL_OFFSET>,
            MouseHoverTime: MouseHoverTime::<Impl, IMPL_OFFSET>,
            UIElementColor: UIElementColor::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettings2Impl: Sized {
    fn TextScaleFactor(&self) -> ::windows::core::Result<f64>;
    fn TextScaleFactorChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextScaleFactorChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings2Vtbl {
        unsafe extern "system" fn TextScaleFactor<Impl: IUISettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TextScaleFactorChanged<Impl: IUISettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextScaleFactorChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTextScaleFactorChanged<Impl: IUISettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTextScaleFactorChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings2, BASE_OFFSET>(),
            TextScaleFactor: TextScaleFactor::<Impl, IMPL_OFFSET>,
            TextScaleFactorChanged: TextScaleFactorChanged::<Impl, IMPL_OFFSET>,
            RemoveTextScaleFactorChanged: RemoveTextScaleFactorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettings3Impl: Sized {
    fn GetColorValue(&self, desiredcolor: UIColorType) -> ::windows::core::Result<super::Color>;
    fn ColorValuesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorValuesChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings3Vtbl {
        unsafe extern "system" fn GetColorValue<Impl: IUISettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: UIColorType, result__: *mut super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorValue(desiredcolor) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ColorValuesChanged<Impl: IUISettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorValuesChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorValuesChanged<Impl: IUISettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveColorValuesChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings3, BASE_OFFSET>(),
            GetColorValue: GetColorValue::<Impl, IMPL_OFFSET>,
            ColorValuesChanged: ColorValuesChanged::<Impl, IMPL_OFFSET>,
            RemoveColorValuesChanged: RemoveColorValuesChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettings4Impl: Sized {
    fn AdvancedEffectsEnabled(&self) -> ::windows::core::Result<bool>;
    fn AdvancedEffectsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedEffectsEnabledChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings4Vtbl {
        unsafe extern "system" fn AdvancedEffectsEnabled<Impl: IUISettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvancedEffectsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvancedEffectsEnabledChanged<Impl: IUISettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvancedEffectsEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdvancedEffectsEnabledChanged<Impl: IUISettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdvancedEffectsEnabledChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings4, BASE_OFFSET>(),
            AdvancedEffectsEnabled: AdvancedEffectsEnabled::<Impl, IMPL_OFFSET>,
            AdvancedEffectsEnabledChanged: AdvancedEffectsEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveAdvancedEffectsEnabledChanged: RemoveAdvancedEffectsEnabledChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettings5Impl: Sized {
    fn AutoHideScrollBars(&self) -> ::windows::core::Result<bool>;
    fn AutoHideScrollBarsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoHideScrollBarsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings5 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings5Vtbl {
        unsafe extern "system" fn AutoHideScrollBars<Impl: IUISettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoHideScrollBars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoHideScrollBarsChanged<Impl: IUISettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoHideScrollBarsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutoHideScrollBarsChanged<Impl: IUISettings5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoHideScrollBarsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings5, BASE_OFFSET>(),
            AutoHideScrollBars: AutoHideScrollBars::<Impl, IMPL_OFFSET>,
            AutoHideScrollBarsChanged: AutoHideScrollBarsChanged::<Impl, IMPL_OFFSET>,
            RemoveAutoHideScrollBarsChanged: RemoveAutoHideScrollBarsChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUISettings6Impl: Sized {
    fn AnimationsEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnimationsEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageDurationChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageDurationChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings6 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings6Vtbl {
        unsafe extern "system" fn AnimationsEnabledChanged<Impl: IUISettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationsEnabledChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAnimationsEnabledChanged<Impl: IUISettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAnimationsEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MessageDurationChanged<Impl: IUISettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageDurationChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageDurationChanged<Impl: IUISettings6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageDurationChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettings6, BASE_OFFSET>(),
            AnimationsEnabledChanged: AnimationsEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveAnimationsEnabledChanged: RemoveAnimationsEnabledChanged::<Impl, IMPL_OFFSET>,
            MessageDurationChanged: MessageDurationChanged::<Impl, IMPL_OFFSET>,
            RemoveMessageDurationChanged: RemoveMessageDurationChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettings6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsAnimationsEnabledChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsAnimationsEnabledChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsAnimationsEnabledChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsAnimationsEnabledChangedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsAnimationsEnabledChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsAnimationsEnabledChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsAutoHideScrollBarsChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsAutoHideScrollBarsChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsAutoHideScrollBarsChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsAutoHideScrollBarsChangedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsAutoHideScrollBarsChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsAutoHideScrollBarsChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsMessageDurationChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsMessageDurationChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsMessageDurationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsMessageDurationChangedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsMessageDurationChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsMessageDurationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIViewSettingsImpl: Sized {
    fn UserInteractionMode(&self) -> ::windows::core::Result<UserInteractionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIViewSettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUIViewSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IUIViewSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIViewSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIViewSettingsVtbl {
        unsafe extern "system" fn UserInteractionMode<Impl: IUIViewSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserInteractionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserInteractionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIViewSettings, BASE_OFFSET>(),
            UserInteractionMode: UserInteractionMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIViewSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIViewSettingsStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<UIViewSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIViewSettingsStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUIViewSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUIViewSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIViewSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIViewSettingsStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IUIViewSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIViewSettingsStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIViewSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IViewModePreferencesImpl: Sized {
    fn ViewSizePreference(&self) -> ::windows::core::Result<ViewSizePreference>;
    fn SetViewSizePreference(&self, value: ViewSizePreference) -> ::windows::core::Result<()>;
    fn CustomSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCustomSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IViewModePreferences {
    const NAME: &'static str = "Windows.UI.ViewManagement.IViewModePreferences";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IViewModePreferencesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewModePreferencesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewModePreferencesVtbl {
        unsafe extern "system" fn ViewSizePreference<Impl: IViewModePreferencesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewSizePreference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewSizePreference<Impl: IViewModePreferencesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewSizePreference(value).into()
        }
        unsafe extern "system" fn CustomSize<Impl: IViewModePreferencesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomSize<Impl: IViewModePreferencesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IViewModePreferences, BASE_OFFSET>(),
            ViewSizePreference: ViewSizePreference::<Impl, IMPL_OFFSET>,
            SetViewSizePreference: SetViewSizePreference::<Impl, IMPL_OFFSET>,
            CustomSize: CustomSize::<Impl, IMPL_OFFSET>,
            SetCustomSize: SetCustomSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewModePreferences as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IViewModePreferencesStaticsImpl: Sized {
    fn CreateDefault(&self, mode: ApplicationViewMode) -> ::windows::core::Result<ViewModePreferences>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IViewModePreferencesStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IViewModePreferencesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IViewModePreferencesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewModePreferencesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewModePreferencesStaticsVtbl {
        unsafe extern "system" fn CreateDefault<Impl: IViewModePreferencesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDefault(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IViewModePreferencesStatics, BASE_OFFSET>(),
            CreateDefault: CreateDefault::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IViewModePreferencesStatics as ::windows::core::Interface>::IID
    }
}
