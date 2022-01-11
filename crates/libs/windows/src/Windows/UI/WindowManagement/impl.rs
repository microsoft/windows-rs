#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IAppWindowImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<super::UIContentRoot>;
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn Frame(&self) -> ::windows::core::Result<AppWindowFrame>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn PersistedStateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
    fn CloseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetPlacement(&self) -> ::windows::core::Result<AppWindowPlacement>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn RequestMoveToDisplayRegion(&self, displayregion: &::core::option::Option<DisplayRegion>) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToCurrentView(&self) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToWindow(&self, anchorwindow: &::core::option::Option<AppWindow>) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToWindowContent(&self, anchorwindow: &::core::option::Option<AppWindow>, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToCurrentViewContent(&self, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToDisplayRegion(&self, displayregion: &::core::option::Option<DisplayRegion>, displayregionoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestSize(&self, framesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn TryShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindow";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IAppWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowVtbl {
        unsafe extern "system" fn Content<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DispatcherQueue<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Frame<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Frame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PersistedStateId<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPersistedStateId<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersistedStateId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Presenter<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Presenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TitleBar<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UIContext<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WindowingEnvironment<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CloseAsync<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPlacement<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayRegions<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestMoveToDisplayRegion<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveToDisplayRegion(&*(&displayregion as *const <DisplayRegion as ::windows::core::Abi>::Abi as *const <DisplayRegion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveAdjacentToCurrentView<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveAdjacentToCurrentView().into()
        }
        unsafe extern "system" fn RequestMoveAdjacentToWindow<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveAdjacentToWindow(&*(&anchorwindow as *const <AppWindow as ::windows::core::Abi>::Abi as *const <AppWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToWindowContent<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToWindowContent(&*(&anchorwindow as *const <AppWindow as ::windows::core::Abi>::Abi as *const <AppWindow as ::windows::core::DefaultType>::DefaultType), &*(&contentoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToCurrentViewContent<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToCurrentViewContent(&*(&contentoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToDisplayRegion<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, displayregionoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToDisplayRegion(&*(&displayregion as *const <DisplayRegion as ::windows::core::Abi>::Abi as *const <DisplayRegion as ::windows::core::DefaultType>::DefaultType), &*(&displayregionoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestSize<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framesize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSize(&*(&framesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryShowAsync<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseRequested<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCloseRequested<Impl: IAppWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCloseRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppWindow>,
            ::windows::core::GetTrustLevel,
            Content::<Impl, IMPL_OFFSET>,
            DispatcherQueue::<Impl, IMPL_OFFSET>,
            Frame::<Impl, IMPL_OFFSET>,
            IsVisible::<Impl, IMPL_OFFSET>,
            PersistedStateId::<Impl, IMPL_OFFSET>,
            SetPersistedStateId::<Impl, IMPL_OFFSET>,
            Presenter::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            TitleBar::<Impl, IMPL_OFFSET>,
            UIContext::<Impl, IMPL_OFFSET>,
            WindowingEnvironment::<Impl, IMPL_OFFSET>,
            CloseAsync::<Impl, IMPL_OFFSET>,
            GetPlacement::<Impl, IMPL_OFFSET>,
            GetDisplayRegions::<Impl, IMPL_OFFSET>,
            RequestMoveToDisplayRegion::<Impl, IMPL_OFFSET>,
            RequestMoveAdjacentToCurrentView::<Impl, IMPL_OFFSET>,
            RequestMoveAdjacentToWindow::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToWindowContent::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToCurrentViewContent::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToDisplayRegion::<Impl, IMPL_OFFSET>,
            RequestSize::<Impl, IMPL_OFFSET>,
            TryShowAsync::<Impl, IMPL_OFFSET>,
            Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged::<Impl, IMPL_OFFSET>,
            Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseRequested::<Impl, IMPL_OFFSET>,
            RemoveCloseRequested::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowChangedEventArgsImpl: Sized {
    fn DidAvailableWindowPresentationsChange(&self) -> ::windows::core::Result<bool>;
    fn DidDisplayRegionsChange(&self) -> ::windows::core::Result<bool>;
    fn DidFrameChange(&self) -> ::windows::core::Result<bool>;
    fn DidSizeChange(&self) -> ::windows::core::Result<bool>;
    fn DidTitleBarChange(&self) -> ::windows::core::Result<bool>;
    fn DidVisibilityChange(&self) -> ::windows::core::Result<bool>;
    fn DidWindowingEnvironmentChange(&self) -> ::windows::core::Result<bool>;
    fn DidWindowPresentationChange(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowChangedEventArgsVtbl {
        unsafe extern "system" fn DidAvailableWindowPresentationsChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidAvailableWindowPresentationsChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidDisplayRegionsChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidDisplayRegionsChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidFrameChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidFrameChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidSizeChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidSizeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidTitleBarChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidTitleBarChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidVisibilityChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidVisibilityChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidWindowingEnvironmentChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidWindowingEnvironmentChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidWindowPresentationChange<Impl: IAppWindowChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DidWindowPresentationChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppWindowChangedEventArgs>,
            ::windows::core::GetTrustLevel,
            DidAvailableWindowPresentationsChange::<Impl, IMPL_OFFSET>,
            DidDisplayRegionsChange::<Impl, IMPL_OFFSET>,
            DidFrameChange::<Impl, IMPL_OFFSET>,
            DidSizeChange::<Impl, IMPL_OFFSET>,
            DidTitleBarChange::<Impl, IMPL_OFFSET>,
            DidVisibilityChange::<Impl, IMPL_OFFSET>,
            DidWindowingEnvironmentChange::<Impl, IMPL_OFFSET>,
            DidWindowPresentationChange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowCloseRequestedEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowCloseRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowCloseRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowCloseRequestedEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IAppWindowCloseRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IAppWindowCloseRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IAppWindowCloseRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowCloseRequestedEventArgs>, ::windows::core::GetTrustLevel, Cancel::<Impl, IMPL_OFFSET>, SetCancel::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowCloseRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowClosedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AppWindowClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowClosedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowClosedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IAppWindowClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowClosedEventArgs>, ::windows::core::GetTrustLevel, Reason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IAppWindowFrameImpl: Sized {
    fn DragRegionVisuals(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowFrame";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IAppWindowFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowFrameVtbl {
        unsafe extern "system" fn DragRegionVisuals<Impl: IAppWindowFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragRegionVisuals() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowFrame>, ::windows::core::GetTrustLevel, DragRegionVisuals::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowFrameStyleImpl: Sized {
    fn GetFrameStyle(&self) -> ::windows::core::Result<AppWindowFrameStyle>;
    fn SetFrameStyle(&self, framestyle: AppWindowFrameStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowFrameStyle {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowFrameStyle";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowFrameStyleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowFrameStyleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowFrameStyleVtbl {
        unsafe extern "system" fn GetFrameStyle<Impl: IAppWindowFrameStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowFrameStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStyle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrameStyle<Impl: IAppWindowFrameStyleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framestyle: AppWindowFrameStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameStyle(framestyle).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowFrameStyle>, ::windows::core::GetTrustLevel, GetFrameStyle::<Impl, IMPL_OFFSET>, SetFrameStyle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowFrameStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowPlacementImpl: Sized {
    fn DisplayRegion(&self) -> ::windows::core::Result<DisplayRegion>;
    fn Offset(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPlacement";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowPlacementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPlacementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPlacementVtbl {
        unsafe extern "system" fn DisplayRegion<Impl: IAppWindowPlacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Offset<Impl: IAppWindowPlacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Offset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IAppWindowPlacementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowPlacement>, ::windows::core::GetTrustLevel, DisplayRegion::<Impl, IMPL_OFFSET>, Offset::<Impl, IMPL_OFFSET>, Size::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPlacement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfigurationImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AppWindowPresentationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresentationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresentationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresentationConfigurationVtbl {
        unsafe extern "system" fn Kind<Impl: IAppWindowPresentationConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowPresentationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowPresentationConfiguration>, ::windows::core::GetTrustLevel, Kind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfigurationFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresentationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresentationConfigurationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresentationConfigurationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresentationConfigurationFactoryVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowPresentationConfigurationFactory>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresentationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresenterImpl: Sized {
    fn GetConfiguration(&self) -> ::windows::core::Result<AppWindowPresentationConfiguration>;
    fn IsPresentationSupported(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
    fn RequestPresentation(&self, configuration: &::core::option::Option<AppWindowPresentationConfiguration>) -> ::windows::core::Result<bool>;
    fn RequestPresentationByKind(&self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresenterVtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IAppWindowPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresentationSupported<Impl: IAppWindowPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPresentationSupported(presentationkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPresentation<Impl: IAppWindowPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPresentation(&*(&configuration as *const <AppWindowPresentationConfiguration as ::windows::core::Abi>::Abi as *const <AppWindowPresentationConfiguration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPresentationByKind<Impl: IAppWindowPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPresentationByKind(presentationkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowPresenter>, ::windows::core::GetTrustLevel, GetConfiguration::<Impl, IMPL_OFFSET>, IsPresentationSupported::<Impl, IMPL_OFFSET>, RequestPresentation::<Impl, IMPL_OFFSET>, RequestPresentationByKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowStaticsImpl: Sized {
    fn TryCreateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppWindow>>;
    fn ClearAllPersistedState(&self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowStaticsVtbl {
        unsafe extern "system" fn TryCreateAsync<Impl: IAppWindowStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearAllPersistedState<Impl: IAppWindowStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllPersistedState().into()
        }
        unsafe extern "system" fn ClearPersistedState<Impl: IAppWindowStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPersistedState(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowStatics>, ::windows::core::GetTrustLevel, TryCreateAsync::<Impl, IMPL_OFFSET>, ClearAllPersistedState::<Impl, IMPL_OFFSET>, ClearPersistedState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppWindowTitleBarImpl: Sized {
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool>;
    fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn GetTitleBarOcclusions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBar";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppWindowTitleBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBarVtbl {
        unsafe extern "system" fn BackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendsContentIntoTitleBar<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendsContentIntoTitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendsContentIntoTitleBar<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendsContentIntoTitleBar(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveBackgroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveForegroundColor<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitleBarOcclusions<Impl: IAppWindowTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitleBarOcclusions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppWindowTitleBar>,
            ::windows::core::GetTrustLevel,
            BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            ExtendsContentIntoTitleBar::<Impl, IMPL_OFFSET>,
            SetExtendsContentIntoTitleBar::<Impl, IMPL_OFFSET>,
            ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor::<Impl, IMPL_OFFSET>,
            InactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            InactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            IsVisible::<Impl, IMPL_OFFSET>,
            GetTitleBarOcclusions::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowTitleBarOcclusionImpl: Sized {
    fn OccludingRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowTitleBarOcclusionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBarOcclusionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBarOcclusionVtbl {
        unsafe extern "system" fn OccludingRect<Impl: IAppWindowTitleBarOcclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OccludingRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowTitleBarOcclusion>, ::windows::core::GetTrustLevel, OccludingRect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBarOcclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowTitleBarVisibilityImpl: Sized {
    fn GetPreferredVisibility(&self) -> ::windows::core::Result<AppWindowTitleBarVisibility>;
    fn SetPreferredVisibility(&self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowTitleBarVisibility {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBarVisibility";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowTitleBarVisibilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBarVisibilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBarVisibilityVtbl {
        unsafe extern "system" fn GetPreferredVisibility<Impl: IAppWindowTitleBarVisibilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowTitleBarVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreferredVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredVisibility<Impl: IAppWindowTitleBarVisibilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVisibility(visibilitymode).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppWindowTitleBarVisibility>, ::windows::core::GetTrustLevel, GetPreferredVisibility::<Impl, IMPL_OFFSET>, SetPreferredVisibility::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBarVisibility as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompactOverlayPresentationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl ICompactOverlayPresentationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompactOverlayPresentationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompactOverlayPresentationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompactOverlayPresentationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompactOverlayPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDefaultPresentationConfigurationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IDefaultPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDefaultPresentationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultPresentationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultPresentationConfigurationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDefaultPresentationConfiguration>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayRegionImpl: Sized {
    fn DisplayMonitorDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn WorkAreaOffset(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WorkAreaSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.IDisplayRegion";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayRegionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayRegionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayRegionVtbl {
        unsafe extern "system" fn DisplayMonitorDeviceId<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayMonitorDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVisible<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkAreaOffset<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkAreaOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkAreaSize<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkAreaSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowingEnvironment<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IDisplayRegionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDisplayRegion>,
            ::windows::core::GetTrustLevel,
            DisplayMonitorDeviceId::<Impl, IMPL_OFFSET>,
            IsVisible::<Impl, IMPL_OFFSET>,
            WorkAreaOffset::<Impl, IMPL_OFFSET>,
            WorkAreaSize::<Impl, IMPL_OFFSET>,
            WindowingEnvironment::<Impl, IMPL_OFFSET>,
            Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullScreenPresentationConfigurationImpl: Sized {
    fn IsExclusive(&self) -> ::windows::core::Result<bool>;
    fn SetIsExclusive(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IFullScreenPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IFullScreenPresentationConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullScreenPresentationConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFullScreenPresentationConfigurationVtbl {
        unsafe extern "system" fn IsExclusive<Impl: IFullScreenPresentationConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsExclusive<Impl: IFullScreenPresentationConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExclusive(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFullScreenPresentationConfiguration>, ::windows::core::GetTrustLevel, IsExclusive::<Impl, IMPL_OFFSET>, SetIsExclusive::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullScreenPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowServicesStaticsImpl: Sized {
    fn FindAllTopLevelWindowIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowServicesStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowServicesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowServicesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowServicesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowServicesStaticsVtbl {
        unsafe extern "system" fn FindAllTopLevelWindowIds<Impl: IWindowServicesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllTopLevelWindowIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowServicesStatics>, ::windows::core::GetTrustLevel, FindAllTopLevelWindowIds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowServicesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowingEnvironmentImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Kind(&self) -> ::windows::core::Result<WindowingEnvironmentKind>;
    fn GetDisplayRegions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowingEnvironmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWindowingEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IWindowingEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WindowingEnvironmentKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayRegions<Impl: IWindowingEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: IWindowingEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IWindowingEnvironmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowingEnvironment>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, Kind::<Impl, IMPL_OFFSET>, GetDisplayRegions::<Impl, IMPL_OFFSET>, Changed::<Impl, IMPL_OFFSET>, RemoveChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentAddedEventArgsImpl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentAddedEventArgsVtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IWindowingEnvironmentAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowingEnvironmentAddedEventArgs>, ::windows::core::GetTrustLevel, WindowingEnvironment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentChangedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowingEnvironmentChangedEventArgs>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentRemovedEventArgsImpl: Sized {
    fn WindowingEnvironment(&self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentRemovedEventArgsVtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IWindowingEnvironmentRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowingEnvironmentRemovedEventArgs>, ::windows::core::GetTrustLevel, WindowingEnvironment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowingEnvironmentStaticsImpl: Sized {
    fn FindAll(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
    fn FindAllWithKind(&self, kind: WindowingEnvironmentKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowingEnvironmentStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowingEnvironmentStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentStaticsVtbl {
        unsafe extern "system" fn FindAll<Impl: IWindowingEnvironmentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllWithKind<Impl: IWindowingEnvironmentStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: WindowingEnvironmentKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllWithKind(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowingEnvironmentStatics>, ::windows::core::GetTrustLevel, FindAll::<Impl, IMPL_OFFSET>, FindAllWithKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentStatics as ::windows::core::Interface>::IID
    }
}
