#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccessibilitySettings_Impl: Sized {
    fn HighContrast(&mut self) -> ::windows::core::Result<bool>;
    fn HighContrastScheme(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HighContrastChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccessibilitySettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHighContrastChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccessibilitySettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IAccessibilitySettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccessibilitySettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessibilitySettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessibilitySettings_Vtbl {
        unsafe extern "system" fn HighContrast<Impl: IAccessibilitySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HighContrastScheme<Impl: IAccessibilitySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HighContrastChanged<Impl: IAccessibilitySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHighContrastChanged<Impl: IAccessibilitySettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IActivationViewSwitcher_Impl: Sized {
    fn ShowAsStandaloneAsync(&mut self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAsStandaloneWithSizePreferenceAsync(&mut self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn IsViewPresentedOnActivationVirtualDesktop(&mut self, viewid: i32) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationViewSwitcher {
    const NAME: &'static str = "Windows.UI.ViewManagement.IActivationViewSwitcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IActivationViewSwitcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationViewSwitcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationViewSwitcher_Vtbl {
        unsafe extern "system" fn ShowAsStandaloneAsync<Impl: IActivationViewSwitcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowAsStandaloneWithSizePreferenceAsync<Impl: IActivationViewSwitcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsViewPresentedOnActivationVirtualDesktop<Impl: IActivationViewSwitcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationView_Impl: Sized {
    fn Orientation(&mut self) -> ::windows::core::Result<ApplicationViewOrientation>;
    fn AdjacentToLeftDisplayEdge(&mut self) -> ::windows::core::Result<bool>;
    fn AdjacentToRightDisplayEdge(&mut self) -> ::windows::core::Result<bool>;
    fn IsFullScreen(&mut self) -> ::windows::core::Result<bool>;
    fn IsOnLockScreen(&mut self) -> ::windows::core::Result<bool>;
    fn IsScreenCaptureEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsScreenCaptureEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<i32>;
    fn Consolidated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ApplicationViewConsolidatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConsolidated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView_Vtbl {
        unsafe extern "system" fn Orientation<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewOrientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdjacentToLeftDisplayEdge<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdjacentToRightDisplayEdge<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsFullScreen<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOnLockScreen<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsScreenCaptureEnabled<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsScreenCaptureEnabled<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsScreenCaptureEnabled(value).into()
        }
        unsafe extern "system" fn SetTitle<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Title<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Consolidated<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConsolidated<Impl: IApplicationView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IApplicationView2_Impl: Sized {
    fn SuppressSystemOverlays(&mut self) -> ::windows::core::Result<bool>;
    fn SetSuppressSystemOverlays(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn VisibleBounds(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn VisibleBoundsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ApplicationView, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibleBoundsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetDesiredBoundsMode(&mut self, boundsmode: ApplicationViewBoundsMode) -> ::windows::core::Result<bool>;
    fn DesiredBoundsMode(&mut self) -> ::windows::core::Result<ApplicationViewBoundsMode>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView2_Vtbl {
        unsafe extern "system" fn SuppressSystemOverlays<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuppressSystemOverlays<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuppressSystemOverlays(value).into()
        }
        unsafe extern "system" fn VisibleBounds<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VisibleBoundsChanged<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVisibleBoundsChanged<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisibleBoundsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetDesiredBoundsMode<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, boundsmode: ApplicationViewBoundsMode, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredBoundsMode<Impl: IApplicationView2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewBoundsMode) -> ::windows::core::HRESULT {
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
pub trait IApplicationView3_Impl: Sized {
    fn TitleBar(&mut self) -> ::windows::core::Result<ApplicationViewTitleBar>;
    fn FullScreenSystemOverlayMode(&mut self) -> ::windows::core::Result<FullScreenSystemOverlayMode>;
    fn SetFullScreenSystemOverlayMode(&mut self, value: FullScreenSystemOverlayMode) -> ::windows::core::Result<()>;
    fn IsFullScreenMode(&mut self) -> ::windows::core::Result<bool>;
    fn TryEnterFullScreenMode(&mut self) -> ::windows::core::Result<bool>;
    fn ExitFullScreenMode(&mut self) -> ::windows::core::Result<()>;
    fn ShowStandardSystemOverlays(&mut self) -> ::windows::core::Result<()>;
    fn TryResizeView(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn SetPreferredMinSize(&mut self, minsize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView3_Vtbl {
        unsafe extern "system" fn TitleBar<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FullScreenSystemOverlayMode<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FullScreenSystemOverlayMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFullScreenSystemOverlayMode<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FullScreenSystemOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullScreenSystemOverlayMode(value).into()
        }
        unsafe extern "system" fn IsFullScreenMode<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryEnterFullScreenMode<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExitFullScreenMode<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExitFullScreenMode().into()
        }
        unsafe extern "system" fn ShowStandardSystemOverlays<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowStandardSystemOverlays().into()
        }
        unsafe extern "system" fn TryResizeView<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredMinSize<Impl: IApplicationView3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minsize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
pub trait IApplicationView4_Impl: Sized {
    fn ViewMode(&mut self) -> ::windows::core::Result<ApplicationViewMode>;
    fn IsViewModeSupported(&mut self, viewmode: ApplicationViewMode) -> ::windows::core::Result<bool>;
    fn TryEnterViewModeAsync(&mut self, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryEnterViewModeWithPreferencesAsync(&mut self, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryConsolidateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationView4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView4_Vtbl {
        unsafe extern "system" fn ViewMode<Impl: IApplicationView4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsViewModeSupported<Impl: IApplicationView4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryEnterViewModeAsync<Impl: IApplicationView4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryEnterViewModeWithPreferencesAsync<Impl: IApplicationView4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewmode: ApplicationViewMode, viewmodepreferences: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryConsolidateAsync<Impl: IApplicationView4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IApplicationView7_Impl: Sized {
    fn PersistedStateId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationView7 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView7";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationView7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView7_Vtbl {
        unsafe extern "system" fn PersistedStateId<Impl: IApplicationView7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPersistedStateId<Impl: IApplicationView7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IApplicationView9_Impl: Sized {
    fn WindowingEnvironment(&mut self) -> ::windows::core::Result<super::WindowManagement::WindowingEnvironment>;
    fn GetDisplayRegions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowManagement::DisplayRegion>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationView9 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationView9";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IApplicationView9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationView9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationView9_Vtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IApplicationView9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDisplayRegions<Impl: IApplicationView9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewConsolidatedEventArgs_Impl: Sized {
    fn IsUserInitiated(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewConsolidatedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewConsolidatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewConsolidatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewConsolidatedEventArgs_Vtbl {
        unsafe extern "system" fn IsUserInitiated<Impl: IApplicationViewConsolidatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewConsolidatedEventArgs2_Impl: Sized {
    fn IsAppInitiated(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewConsolidatedEventArgs2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewConsolidatedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewConsolidatedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewConsolidatedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewConsolidatedEventArgs2_Vtbl {
        unsafe extern "system" fn IsAppInitiated<Impl: IApplicationViewConsolidatedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewFullscreenStatics_Impl: Sized {
    fn TryUnsnapToFullscreen(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewFullscreenStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewFullscreenStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IApplicationViewFullscreenStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewFullscreenStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewFullscreenStatics_Vtbl {
        unsafe extern "system" fn TryUnsnapToFullscreen<Impl: IApplicationViewFullscreenStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewInteropStatics_Impl: Sized {
    fn GetApplicationViewIdForWindow(&mut self, window: &::core::option::Option<super::Core::ICoreWindow>) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewInteropStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewInteropStatics";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl IApplicationViewInteropStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewInteropStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewInteropStatics_Vtbl {
        unsafe extern "system" fn GetApplicationViewIdForWindow<Impl: IApplicationViewInteropStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewScaling_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewScaling {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewScaling";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewScaling_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewScaling_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewScaling_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationViewScaling, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationViewScaling as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationViewScalingStatics_Impl: Sized {
    fn DisableLayoutScaling(&mut self) -> ::windows::core::Result<bool>;
    fn TrySetDisableLayoutScaling(&mut self, disablelayoutscaling: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewScalingStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewScalingStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewScalingStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewScalingStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewScalingStatics_Vtbl {
        unsafe extern "system" fn DisableLayoutScaling<Impl: IApplicationViewScalingStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetDisableLayoutScaling<Impl: IApplicationViewScalingStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disablelayoutscaling: bool, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewStatics_Impl: Sized {
    fn Value(&mut self) -> ::windows::core::Result<ApplicationViewState>;
    fn TryUnsnap(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IApplicationViewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics_Vtbl {
        unsafe extern "system" fn Value<Impl: IApplicationViewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryUnsnap<Impl: IApplicationViewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewStatics2_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<ApplicationView>;
    fn TerminateAppOnFinalViewClose(&mut self) -> ::windows::core::Result<bool>;
    fn SetTerminateAppOnFinalViewClose(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics2_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IApplicationViewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TerminateAppOnFinalViewClose<Impl: IApplicationViewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTerminateAppOnFinalViewClose<Impl: IApplicationViewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewStatics3_Impl: Sized {
    fn PreferredLaunchWindowingMode(&mut self) -> ::windows::core::Result<ApplicationViewWindowingMode>;
    fn SetPreferredLaunchWindowingMode(&mut self, value: ApplicationViewWindowingMode) -> ::windows::core::Result<()>;
    fn PreferredLaunchViewSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetPreferredLaunchViewSize(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewStatics3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics3_Vtbl {
        unsafe extern "system" fn PreferredLaunchWindowingMode<Impl: IApplicationViewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationViewWindowingMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredLaunchWindowingMode<Impl: IApplicationViewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ApplicationViewWindowingMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredLaunchWindowingMode(value).into()
        }
        unsafe extern "system" fn PreferredLaunchViewSize<Impl: IApplicationViewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredLaunchViewSize<Impl: IApplicationViewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewStatics4_Impl: Sized {
    fn ClearAllPersistedState(&mut self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&mut self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewStatics4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewStatics4_Vtbl {
        unsafe extern "system" fn ClearAllPersistedState<Impl: IApplicationViewStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllPersistedState().into()
        }
        unsafe extern "system" fn ClearPersistedState<Impl: IApplicationViewStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewSwitcherStatics_Impl: Sized {
    fn DisableShowingMainViewOnActivation(&mut self) -> ::windows::core::Result<()>;
    fn TryShowAsStandaloneAsync(&mut self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithSizePreferenceAsync(&mut self, viewid: i32, sizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync(&mut self, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SwitchAsync(&mut self, viewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewAsync(&mut self, toviewid: i32, fromviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwitchFromViewWithOptionsAsync(&mut self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PrepareForCustomAnimatedSwitchAsync(&mut self, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewSwitcherStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStatics_Vtbl {
        unsafe extern "system" fn DisableShowingMainViewOnActivation<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableShowingMainViewOnActivation().into()
        }
        unsafe extern "system" fn TryShowAsStandaloneAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryShowAsStandaloneWithSizePreferenceAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryShowAsStandaloneWithAnchorViewAndSizePreferenceAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, sizepreference: ViewSizePreference, anchorviewid: i32, anchorsizepreference: ViewSizePreference, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SwitchAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SwitchFromViewAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SwitchFromViewWithOptionsAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrepareForCustomAnimatedSwitchAsync<Impl: IApplicationViewSwitcherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toviewid: i32, fromviewid: i32, options: ApplicationViewSwitchingOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewSwitcherStatics2_Impl: Sized {
    fn DisableSystemViewActivationPolicy(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewSwitcherStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStatics2_Vtbl {
        unsafe extern "system" fn DisableSystemViewActivationPolicy<Impl: IApplicationViewSwitcherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewSwitcherStatics3_Impl: Sized {
    fn TryShowAsViewModeAsync(&mut self, viewid: i32, viewmode: ApplicationViewMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryShowAsViewModeWithPreferencesAsync(&mut self, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: &::core::option::Option<ViewModePreferences>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewSwitcherStatics3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewSwitcherStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewSwitcherStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewSwitcherStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewSwitcherStatics3_Vtbl {
        unsafe extern "system" fn TryShowAsViewModeAsync<Impl: IApplicationViewSwitcherStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryShowAsViewModeWithPreferencesAsync<Impl: IApplicationViewSwitcherStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewid: i32, viewmode: ApplicationViewMode, viewmodepreferences: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewTitleBar_Impl: Sized {
    fn SetForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationViewTitleBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTitleBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IApplicationViewTitleBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTitleBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTitleBar_Vtbl {
        unsafe extern "system" fn SetForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveForegroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveBackgroundColor<Impl: IApplicationViewTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewTransferContext_Impl: Sized {
    fn ViewId(&mut self) -> ::windows::core::Result<i32>;
    fn SetViewId(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewTransferContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTransferContext";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewTransferContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTransferContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTransferContext_Vtbl {
        unsafe extern "system" fn ViewId<Impl: IApplicationViewTransferContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewId<Impl: IApplicationViewTransferContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewTransferContextStatics_Impl: Sized {
    fn DataPackageFormatId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewTransferContextStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewTransferContextStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewTransferContextStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewTransferContextStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewTransferContextStatics_Vtbl {
        unsafe extern "system" fn DataPackageFormatId<Impl: IApplicationViewTransferContextStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IApplicationViewWithContext_Impl: Sized {
    fn UIContext(&mut self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationViewWithContext {
    const NAME: &'static str = "Windows.UI.ViewManagement.IApplicationViewWithContext";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationViewWithContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationViewWithContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationViewWithContext_Vtbl {
        unsafe extern "system" fn UIContext<Impl: IApplicationViewWithContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInputPane_Impl: Sized {
    fn Showing(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<InputPane, InputPaneVisibilityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn OccludedRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputPane {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPane";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInputPane_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPane_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPane_Vtbl {
        unsafe extern "system" fn Showing<Impl: IInputPane_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShowing<Impl: IInputPane_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hiding<Impl: IInputPane_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHiding<Impl: IInputPane_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHiding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OccludedRect<Impl: IInputPane_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
pub trait IInputPane2_Impl: Sized {
    fn TryShow(&mut self) -> ::windows::core::Result<bool>;
    fn TryHide(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPane2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPane2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPane2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPane2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPane2_Vtbl {
        unsafe extern "system" fn TryShow<Impl: IInputPane2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryHide<Impl: IInputPane2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IInputPaneControl_Impl: Sized {
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn SetVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneControl {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneControl";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneControl_Vtbl {
        unsafe extern "system" fn Visible<Impl: IInputPaneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVisible<Impl: IInputPaneControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IInputPaneStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IInputPaneStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInputPaneStatics2_Impl: Sized {
    fn GetForUIContext(&mut self, context: &::core::option::Option<super::UIContext>) -> ::windows::core::Result<InputPane>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputPaneStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IInputPaneStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneStatics2_Vtbl {
        unsafe extern "system" fn GetForUIContext<Impl: IInputPaneStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInputPaneVisibilityEventArgs_Impl: Sized {
    fn OccludedRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetEnsuredFocusedElementInView(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn EnsuredFocusedElementInView(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputPaneVisibilityEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IInputPaneVisibilityEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IInputPaneVisibilityEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneVisibilityEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneVisibilityEventArgs_Vtbl {
        unsafe extern "system" fn OccludedRect<Impl: IInputPaneVisibilityEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnsuredFocusedElementInView<Impl: IInputPaneVisibilityEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnsuredFocusedElementInView(value).into()
        }
        unsafe extern "system" fn EnsuredFocusedElementInView<Impl: IInputPaneVisibilityEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IProjectionManagerStatics_Impl: Sized {
    fn StartProjectingAsync(&mut self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SwapDisplaysForViewsAsync(&mut self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopProjectingAsync(&mut self, projectionviewid: i32, anchorviewid: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ProjectionDisplayAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn ProjectionDisplayAvailableChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProjectionDisplayAvailableChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProjectionManagerStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IProjectionManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProjectionManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjectionManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjectionManagerStatics_Vtbl {
        unsafe extern "system" fn StartProjectingAsync<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SwapDisplaysForViewsAsync<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StopProjectingAsync<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProjectionDisplayAvailable<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProjectionDisplayAvailableChanged<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveProjectionDisplayAvailableChanged<Impl: IProjectionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IProjectionManagerStatics2_Impl: Sized {
    fn StartProjectingWithDeviceInfoAsync(&mut self, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: &::core::option::Option<super::super::Devices::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestStartProjectingAsync(&mut self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestStartProjectingWithPlacementAsync(&mut self, projectionviewid: i32, anchorviewid: i32, selection: &super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProjectionManagerStatics2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IProjectionManagerStatics2";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl IProjectionManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProjectionManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProjectionManagerStatics2_Vtbl {
        unsafe extern "system" fn StartProjectingWithDeviceInfoAsync<Impl: IProjectionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, displaydeviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestStartProjectingAsync<Impl: IProjectionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestStartProjectingWithPlacementAsync<Impl: IProjectionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectionviewid: i32, anchorviewid: i32, selection: super::super::Foundation::Rect, prefferedplacement: super::Popups::Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IProjectionManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IStatusBar_Impl: Sized {
    fn ShowAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BackgroundOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetBackgroundOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ProgressIndicator(&mut self) -> ::windows::core::Result<StatusBarProgressIndicator>;
    fn OccludedRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Showing(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Hiding(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StatusBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHiding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStatusBar {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStatusBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBar_Vtbl {
        unsafe extern "system" fn ShowAsync<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HideAsync<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BackgroundOpacity<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundOpacity<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundOpacity(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForegroundColor<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BackgroundColor<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProgressIndicator<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OccludedRect<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Showing<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShowing<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hiding<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHiding<Impl: IStatusBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IStatusBarProgressIndicator_Impl: Sized {
    fn ShowAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HideAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ProgressValue(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SetProgressValue(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStatusBarProgressIndicator {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBarProgressIndicator";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStatusBarProgressIndicator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBarProgressIndicator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBarProgressIndicator_Vtbl {
        unsafe extern "system" fn ShowAsync<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HideAsync<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetText<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProgressValue<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProgressValue<Impl: IStatusBarProgressIndicator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IStatusBarStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<StatusBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStatusBarStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IStatusBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStatusBarStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatusBarStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatusBarStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IStatusBarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IUISettings_Impl: Sized {
    fn HandPreference(&mut self) -> ::windows::core::Result<HandPreference>;
    fn CursorSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarArrowSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ScrollBarThumbBoxSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MessageDuration(&mut self) -> ::windows::core::Result<u32>;
    fn AnimationsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn CaretBrowsingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn CaretBlinkRate(&mut self) -> ::windows::core::Result<u32>;
    fn CaretWidth(&mut self) -> ::windows::core::Result<u32>;
    fn DoubleClickTime(&mut self) -> ::windows::core::Result<u32>;
    fn MouseHoverTime(&mut self) -> ::windows::core::Result<u32>;
    fn UIElementColor(&mut self, desiredelement: UIElementType) -> ::windows::core::Result<super::Color>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings_Vtbl {
        unsafe extern "system" fn HandPreference<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HandPreference) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CursorSize<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollBarSize<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollBarArrowSize<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollBarThumbBoxSize<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MessageDuration<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AnimationsEnabled<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CaretBrowsingEnabled<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CaretBlinkRate<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CaretWidth<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DoubleClickTime<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MouseHoverTime<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UIElementColor<Impl: IUISettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredelement: UIElementType, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
pub trait IUISettings2_Impl: Sized {
    fn TextScaleFactor(&mut self) -> ::windows::core::Result<f64>;
    fn TextScaleFactorChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTextScaleFactorChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings2_Vtbl {
        unsafe extern "system" fn TextScaleFactor<Impl: IUISettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TextScaleFactorChanged<Impl: IUISettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTextScaleFactorChanged<Impl: IUISettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IUISettings3_Impl: Sized {
    fn GetColorValue(&mut self, desiredcolor: UIColorType) -> ::windows::core::Result<super::Color>;
    fn ColorValuesChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorValuesChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings3_Vtbl {
        unsafe extern "system" fn GetColorValue<Impl: IUISettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredcolor: UIColorType, result__: *mut super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ColorValuesChanged<Impl: IUISettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveColorValuesChanged<Impl: IUISettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IUISettings4_Impl: Sized {
    fn AdvancedEffectsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn AdvancedEffectsEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvancedEffectsEnabledChanged(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings4_Vtbl {
        unsafe extern "system" fn AdvancedEffectsEnabled<Impl: IUISettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdvancedEffectsEnabledChanged<Impl: IUISettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAdvancedEffectsEnabledChanged<Impl: IUISettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IUISettings5_Impl: Sized {
    fn AutoHideScrollBars(&mut self) -> ::windows::core::Result<bool>;
    fn AutoHideScrollBarsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoHideScrollBarsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings5 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings5_Vtbl {
        unsafe extern "system" fn AutoHideScrollBars<Impl: IUISettings5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AutoHideScrollBarsChanged<Impl: IUISettings5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAutoHideScrollBarsChanged<Impl: IUISettings5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IUISettings6_Impl: Sized {
    fn AnimationsEnabledChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAnimationsEnabledChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MessageDurationChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageDurationChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUISettings6 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUISettings6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettings6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettings6_Vtbl {
        unsafe extern "system" fn AnimationsEnabledChanged<Impl: IUISettings6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAnimationsEnabledChanged<Impl: IUISettings6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAnimationsEnabledChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MessageDurationChanged<Impl: IUISettings6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageDurationChanged<Impl: IUISettings6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IUISettingsAnimationsEnabledChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsAnimationsEnabledChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsAnimationsEnabledChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsAnimationsEnabledChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsAutoHideScrollBarsChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsAutoHideScrollBarsChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsAutoHideScrollBarsChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsAutoHideScrollBarsChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUISettingsMessageDurationChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUISettingsMessageDurationChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUISettingsMessageDurationChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUISettingsMessageDurationChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUISettingsMessageDurationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIViewSettings_Impl: Sized {
    fn UserInteractionMode(&mut self) -> ::windows::core::Result<UserInteractionMode>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIViewSettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUIViewSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IUIViewSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIViewSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIViewSettings_Vtbl {
        unsafe extern "system" fn UserInteractionMode<Impl: IUIViewSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserInteractionMode) -> ::windows::core::HRESULT {
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
pub trait IUIViewSettingsStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<UIViewSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIViewSettingsStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUIViewSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUIViewSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIViewSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIViewSettingsStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IUIViewSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IViewModePreferences_Impl: Sized {
    fn ViewSizePreference(&mut self) -> ::windows::core::Result<ViewSizePreference>;
    fn SetViewSizePreference(&mut self, value: ViewSizePreference) -> ::windows::core::Result<()>;
    fn CustomSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetCustomSize(&mut self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IViewModePreferences {
    const NAME: &'static str = "Windows.UI.ViewManagement.IViewModePreferences";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IViewModePreferences_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewModePreferences_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewModePreferences_Vtbl {
        unsafe extern "system" fn ViewSizePreference<Impl: IViewModePreferences_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ViewSizePreference) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewSizePreference<Impl: IViewModePreferences_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewSizePreference(value).into()
        }
        unsafe extern "system" fn CustomSize<Impl: IViewModePreferences_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCustomSize<Impl: IViewModePreferences_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
pub trait IViewModePreferencesStatics_Impl: Sized {
    fn CreateDefault(&mut self, mode: ApplicationViewMode) -> ::windows::core::Result<ViewModePreferences>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IViewModePreferencesStatics {
    const NAME: &'static str = "Windows.UI.ViewManagement.IViewModePreferencesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IViewModePreferencesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IViewModePreferencesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IViewModePreferencesStatics_Vtbl {
        unsafe extern "system" fn CreateDefault<Impl: IViewModePreferencesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: ApplicationViewMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
