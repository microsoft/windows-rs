#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IAppWindow_Impl: Sized {
    fn Content(&mut self) -> ::windows::core::Result<super::UIContentRoot>;
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn Frame(&mut self) -> ::windows::core::Result<AppWindowFrame>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn PersistedStateId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPersistedStateId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Presenter(&mut self) -> ::windows::core::Result<AppWindowPresenter>;
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TitleBar(&mut self) -> ::windows::core::Result<AppWindowTitleBar>;
    fn UIContext(&mut self) -> ::windows::core::Result<super::UIContext>;
    fn WindowingEnvironment(&mut self) -> ::windows::core::Result<WindowingEnvironment>;
    fn CloseAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetPlacement(&mut self) -> ::windows::core::Result<AppWindowPlacement>;
    fn GetDisplayRegions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn RequestMoveToDisplayRegion(&mut self, displayregion: &::core::option::Option<DisplayRegion>) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToCurrentView(&mut self) -> ::windows::core::Result<()>;
    fn RequestMoveAdjacentToWindow(&mut self, anchorwindow: &::core::option::Option<AppWindow>) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToWindowContent(&mut self, anchorwindow: &::core::option::Option<AppWindow>, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToCurrentViewContent(&mut self, contentoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestMoveRelativeToDisplayRegion(&mut self, displayregion: &::core::option::Option<DisplayRegion>, displayregionoffset: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn RequestSize(&mut self, framesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn TryShowAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppWindow, AppWindowCloseRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCloseRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindow {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindow";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IAppWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindow_Vtbl {
        unsafe extern "system" fn Content<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DispatcherQueue<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Frame<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVisible<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PersistedStateId<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPersistedStateId<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPersistedStateId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Presenter<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Title<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TitleBar<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UIContext<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WindowingEnvironment<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CloseAsync<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPlacement<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDisplayRegions<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestMoveToDisplayRegion<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveToDisplayRegion(&*(&displayregion as *const <DisplayRegion as ::windows::core::Abi>::Abi as *const <DisplayRegion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveAdjacentToCurrentView<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveAdjacentToCurrentView().into()
        }
        unsafe extern "system" fn RequestMoveAdjacentToWindow<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveAdjacentToWindow(&*(&anchorwindow as *const <AppWindow as ::windows::core::Abi>::Abi as *const <AppWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToWindowContent<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, anchorwindow: ::windows::core::RawPtr, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToWindowContent(&*(&anchorwindow as *const <AppWindow as ::windows::core::Abi>::Abi as *const <AppWindow as ::windows::core::DefaultType>::DefaultType), &*(&contentoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToCurrentViewContent<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToCurrentViewContent(&*(&contentoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestMoveRelativeToDisplayRegion<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayregion: ::windows::core::RawPtr, displayregionoffset: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestMoveRelativeToDisplayRegion(&*(&displayregion as *const <DisplayRegion as ::windows::core::Abi>::Abi as *const <DisplayRegion as ::windows::core::DefaultType>::DefaultType), &*(&displayregionoffset as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestSize<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framesize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestSize(&*(&framesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryShowAsync<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChanged<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseRequested<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCloseRequested<Impl: IAppWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCloseRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindow, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
            Frame: Frame::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            PersistedStateId: PersistedStateId::<Impl, IMPL_OFFSET>,
            SetPersistedStateId: SetPersistedStateId::<Impl, IMPL_OFFSET>,
            Presenter: Presenter::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            TitleBar: TitleBar::<Impl, IMPL_OFFSET>,
            UIContext: UIContext::<Impl, IMPL_OFFSET>,
            WindowingEnvironment: WindowingEnvironment::<Impl, IMPL_OFFSET>,
            CloseAsync: CloseAsync::<Impl, IMPL_OFFSET>,
            GetPlacement: GetPlacement::<Impl, IMPL_OFFSET>,
            GetDisplayRegions: GetDisplayRegions::<Impl, IMPL_OFFSET>,
            RequestMoveToDisplayRegion: RequestMoveToDisplayRegion::<Impl, IMPL_OFFSET>,
            RequestMoveAdjacentToCurrentView: RequestMoveAdjacentToCurrentView::<Impl, IMPL_OFFSET>,
            RequestMoveAdjacentToWindow: RequestMoveAdjacentToWindow::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToWindowContent: RequestMoveRelativeToWindowContent::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToCurrentViewContent: RequestMoveRelativeToCurrentViewContent::<Impl, IMPL_OFFSET>,
            RequestMoveRelativeToDisplayRegion: RequestMoveRelativeToDisplayRegion::<Impl, IMPL_OFFSET>,
            RequestSize: RequestSize::<Impl, IMPL_OFFSET>,
            TryShowAsync: TryShowAsync::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseRequested: CloseRequested::<Impl, IMPL_OFFSET>,
            RemoveCloseRequested: RemoveCloseRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowChangedEventArgs_Impl: Sized {
    fn DidAvailableWindowPresentationsChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidDisplayRegionsChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidFrameChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidSizeChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidTitleBarChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidVisibilityChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidWindowingEnvironmentChange(&mut self) -> ::windows::core::Result<bool>;
    fn DidWindowPresentationChange(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowChangedEventArgs_Vtbl {
        unsafe extern "system" fn DidAvailableWindowPresentationsChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidDisplayRegionsChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidFrameChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidSizeChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidTitleBarChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidVisibilityChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidWindowingEnvironmentChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DidWindowPresentationChange<Impl: IAppWindowChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowChangedEventArgs, BASE_OFFSET>(),
            DidAvailableWindowPresentationsChange: DidAvailableWindowPresentationsChange::<Impl, IMPL_OFFSET>,
            DidDisplayRegionsChange: DidDisplayRegionsChange::<Impl, IMPL_OFFSET>,
            DidFrameChange: DidFrameChange::<Impl, IMPL_OFFSET>,
            DidSizeChange: DidSizeChange::<Impl, IMPL_OFFSET>,
            DidTitleBarChange: DidTitleBarChange::<Impl, IMPL_OFFSET>,
            DidVisibilityChange: DidVisibilityChange::<Impl, IMPL_OFFSET>,
            DidWindowingEnvironmentChange: DidWindowingEnvironmentChange::<Impl, IMPL_OFFSET>,
            DidWindowPresentationChange: DidWindowPresentationChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowCloseRequestedEventArgs_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowCloseRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowCloseRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowCloseRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowCloseRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowCloseRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Cancel<Impl: IAppWindowCloseRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IAppWindowCloseRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IAppWindowCloseRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowCloseRequestedEventArgs, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowCloseRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowClosedEventArgs_Impl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<AppWindowClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowClosedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowClosedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowClosedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowClosedEventArgs_Vtbl {
        unsafe extern "system" fn Reason<Impl: IAppWindowClosedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowClosedReason) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowClosedEventArgs, BASE_OFFSET>(), Reason: Reason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IAppWindowFrame_Impl: Sized {
    fn DragRegionVisuals(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Composition::IVisualElement>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowFrame {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowFrame";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IAppWindowFrame_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowFrame_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowFrame_Vtbl {
        unsafe extern "system" fn DragRegionVisuals<Impl: IAppWindowFrame_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowFrame, BASE_OFFSET>(),
            DragRegionVisuals: DragRegionVisuals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowFrameStyle_Impl: Sized {
    fn GetFrameStyle(&mut self) -> ::windows::core::Result<AppWindowFrameStyle>;
    fn SetFrameStyle(&mut self, framestyle: AppWindowFrameStyle) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowFrameStyle {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowFrameStyle";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowFrameStyle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowFrameStyle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowFrameStyle_Vtbl {
        unsafe extern "system" fn GetFrameStyle<Impl: IAppWindowFrameStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowFrameStyle) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFrameStyle<Impl: IAppWindowFrameStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framestyle: AppWindowFrameStyle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrameStyle(framestyle).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowFrameStyle, BASE_OFFSET>(),
            GetFrameStyle: GetFrameStyle::<Impl, IMPL_OFFSET>,
            SetFrameStyle: SetFrameStyle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowFrameStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowPlacement_Impl: Sized {
    fn DisplayRegion(&mut self) -> ::windows::core::Result<DisplayRegion>;
    fn Offset(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowPlacement {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPlacement";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowPlacement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPlacement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPlacement_Vtbl {
        unsafe extern "system" fn DisplayRegion<Impl: IAppWindowPlacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Offset<Impl: IAppWindowPlacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IAppWindowPlacement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowPlacement, BASE_OFFSET>(),
            DisplayRegion: DisplayRegion::<Impl, IMPL_OFFSET>,
            Offset: Offset::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPlacement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfiguration_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<AppWindowPresentationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresentationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresentationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresentationConfiguration_Vtbl {
        unsafe extern "system" fn Kind<Impl: IAppWindowPresentationConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowPresentationKind) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowPresentationConfiguration, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresentationConfigurationFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresentationConfigurationFactory {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresentationConfigurationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresentationConfigurationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresentationConfigurationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresentationConfigurationFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowPresentationConfigurationFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresentationConfigurationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowPresenter_Impl: Sized {
    fn GetConfiguration(&mut self) -> ::windows::core::Result<AppWindowPresentationConfiguration>;
    fn IsPresentationSupported(&mut self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
    fn RequestPresentation(&mut self, configuration: &::core::option::Option<AppWindowPresentationConfiguration>) -> ::windows::core::Result<bool>;
    fn RequestPresentationByKind(&mut self, presentationkind: AppWindowPresentationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowPresenter {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowPresenter_Vtbl {
        unsafe extern "system" fn GetConfiguration<Impl: IAppWindowPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPresentationSupported<Impl: IAppWindowPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestPresentation<Impl: IAppWindowPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestPresentationByKind<Impl: IAppWindowPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationkind: AppWindowPresentationKind, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowPresenter, BASE_OFFSET>(),
            GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET>,
            IsPresentationSupported: IsPresentationSupported::<Impl, IMPL_OFFSET>,
            RequestPresentation: RequestPresentation::<Impl, IMPL_OFFSET>,
            RequestPresentationByKind: RequestPresentationByKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowStatics_Impl: Sized {
    fn TryCreateAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppWindow>>;
    fn ClearAllPersistedState(&mut self) -> ::windows::core::Result<()>;
    fn ClearPersistedState(&mut self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowStatics_Vtbl {
        unsafe extern "system" fn TryCreateAsync<Impl: IAppWindowStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearAllPersistedState<Impl: IAppWindowStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAllPersistedState().into()
        }
        unsafe extern "system" fn ClearPersistedState<Impl: IAppWindowStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPersistedState(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowStatics, BASE_OFFSET>(),
            TryCreateAsync: TryCreateAsync::<Impl, IMPL_OFFSET>,
            ClearAllPersistedState: ClearAllPersistedState::<Impl, IMPL_OFFSET>,
            ClearPersistedState: ClearPersistedState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppWindowTitleBar_Impl: Sized {
    fn BackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonHoverForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonInactiveForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetButtonPressedForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ExtendsContentIntoTitleBar(&mut self) -> ::windows::core::Result<bool>;
    fn SetExtendsContentIntoTitleBar(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveBackgroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetInactiveForegroundColor(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn GetTitleBarOcclusions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppWindowTitleBarOcclusion>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowTitleBar {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBar";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppWindowTitleBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBar_Vtbl {
        unsafe extern "system" fn BackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonHoverForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonHoverForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonHoverForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonInactiveForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonInactiveForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ButtonPressedForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetButtonPressedForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetButtonPressedForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendsContentIntoTitleBar<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExtendsContentIntoTitleBar<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendsContentIntoTitleBar(value).into()
        }
        unsafe extern "system" fn ForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveBackgroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveBackgroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InactiveForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInactiveForegroundColor<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInactiveForegroundColor(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsVisible<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTitleBarOcclusions<Impl: IAppWindowTitleBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowTitleBar, BASE_OFFSET>(),
            BackgroundColor: BackgroundColor::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonBackgroundColor: ButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonBackgroundColor: SetButtonBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonForegroundColor: ButtonForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonForegroundColor: SetButtonForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverBackgroundColor: ButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverBackgroundColor: SetButtonHoverBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonHoverForegroundColor: ButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonHoverForegroundColor: SetButtonHoverForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveBackgroundColor: ButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveBackgroundColor: SetButtonInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonInactiveForegroundColor: ButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonInactiveForegroundColor: SetButtonInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedBackgroundColor: ButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedBackgroundColor: SetButtonPressedBackgroundColor::<Impl, IMPL_OFFSET>,
            ButtonPressedForegroundColor: ButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            SetButtonPressedForegroundColor: SetButtonPressedForegroundColor::<Impl, IMPL_OFFSET>,
            ExtendsContentIntoTitleBar: ExtendsContentIntoTitleBar::<Impl, IMPL_OFFSET>,
            SetExtendsContentIntoTitleBar: SetExtendsContentIntoTitleBar::<Impl, IMPL_OFFSET>,
            ForegroundColor: ForegroundColor::<Impl, IMPL_OFFSET>,
            SetForegroundColor: SetForegroundColor::<Impl, IMPL_OFFSET>,
            InactiveBackgroundColor: InactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveBackgroundColor: SetInactiveBackgroundColor::<Impl, IMPL_OFFSET>,
            InactiveForegroundColor: InactiveForegroundColor::<Impl, IMPL_OFFSET>,
            SetInactiveForegroundColor: SetInactiveForegroundColor::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            GetTitleBarOcclusions: GetTitleBarOcclusions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppWindowTitleBarOcclusion_Impl: Sized {
    fn OccludingRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppWindowTitleBarOcclusion {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBarOcclusion";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppWindowTitleBarOcclusion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBarOcclusion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBarOcclusion_Vtbl {
        unsafe extern "system" fn OccludingRect<Impl: IAppWindowTitleBarOcclusion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowTitleBarOcclusion, BASE_OFFSET>(),
            OccludingRect: OccludingRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBarOcclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppWindowTitleBarVisibility_Impl: Sized {
    fn GetPreferredVisibility(&mut self) -> ::windows::core::Result<AppWindowTitleBarVisibility>;
    fn SetPreferredVisibility(&mut self, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppWindowTitleBarVisibility {
    const NAME: &'static str = "Windows.UI.WindowManagement.IAppWindowTitleBarVisibility";
}
#[cfg(feature = "implement_exclusive")]
impl IAppWindowTitleBarVisibility_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppWindowTitleBarVisibility_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppWindowTitleBarVisibility_Vtbl {
        unsafe extern "system" fn GetPreferredVisibility<Impl: IAppWindowTitleBarVisibility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppWindowTitleBarVisibility) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredVisibility<Impl: IAppWindowTitleBarVisibility_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visibilitymode: AppWindowTitleBarVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredVisibility(visibilitymode).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppWindowTitleBarVisibility, BASE_OFFSET>(),
            GetPreferredVisibility: GetPreferredVisibility::<Impl, IMPL_OFFSET>,
            SetPreferredVisibility: SetPreferredVisibility::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBarVisibility as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompactOverlayPresentationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompactOverlayPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.ICompactOverlayPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl ICompactOverlayPresentationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompactOverlayPresentationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompactOverlayPresentationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompactOverlayPresentationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompactOverlayPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDefaultPresentationConfiguration_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDefaultPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IDefaultPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDefaultPresentationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultPresentationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultPresentationConfiguration_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDefaultPresentationConfiguration, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDisplayRegion_Impl: Sized {
    fn DisplayMonitorDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn WorkAreaOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn WorkAreaSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn WindowingEnvironment(&mut self) -> ::windows::core::Result<WindowingEnvironment>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DisplayRegion, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDisplayRegion {
    const NAME: &'static str = "Windows.UI.WindowManagement.IDisplayRegion";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDisplayRegion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayRegion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayRegion_Vtbl {
        unsafe extern "system" fn DisplayMonitorDeviceId<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVisible<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WorkAreaOffset<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WorkAreaSize<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WindowingEnvironment<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChanged<Impl: IDisplayRegion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayRegion, BASE_OFFSET>(),
            DisplayMonitorDeviceId: DisplayMonitorDeviceId::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            WorkAreaOffset: WorkAreaOffset::<Impl, IMPL_OFFSET>,
            WorkAreaSize: WorkAreaSize::<Impl, IMPL_OFFSET>,
            WindowingEnvironment: WindowingEnvironment::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayRegion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullScreenPresentationConfiguration_Impl: Sized {
    fn IsExclusive(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsExclusive(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFullScreenPresentationConfiguration {
    const NAME: &'static str = "Windows.UI.WindowManagement.IFullScreenPresentationConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IFullScreenPresentationConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullScreenPresentationConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFullScreenPresentationConfiguration_Vtbl {
        unsafe extern "system" fn IsExclusive<Impl: IFullScreenPresentationConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsExclusive<Impl: IFullScreenPresentationConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExclusive(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFullScreenPresentationConfiguration, BASE_OFFSET>(),
            IsExclusive: IsExclusive::<Impl, IMPL_OFFSET>,
            SetIsExclusive: SetIsExclusive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullScreenPresentationConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowServicesStatics_Impl: Sized {
    fn FindAllTopLevelWindowIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::WindowId>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowServicesStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowServicesStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowServicesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowServicesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowServicesStatics_Vtbl {
        unsafe extern "system" fn FindAllTopLevelWindowIds<Impl: IWindowServicesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowServicesStatics, BASE_OFFSET>(),
            FindAllTopLevelWindowIds: FindAllTopLevelWindowIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowServicesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowingEnvironment_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn Kind(&mut self) -> ::windows::core::Result<WindowingEnvironmentKind>;
    fn GetDisplayRegions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DisplayRegion>>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<WindowingEnvironment, WindowingEnvironmentChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowingEnvironment {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowingEnvironment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironment_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IWindowingEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IWindowingEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WindowingEnvironmentKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDisplayRegions<Impl: IWindowingEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Changed<Impl: IWindowingEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChanged<Impl: IWindowingEnvironment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowingEnvironment, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            GetDisplayRegions: GetDisplayRegions::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentAddedEventArgs_Impl: Sized {
    fn WindowingEnvironment(&mut self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentAddedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentAddedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentAddedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentAddedEventArgs_Vtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IWindowingEnvironmentAddedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowingEnvironmentAddedEventArgs, BASE_OFFSET>(),
            WindowingEnvironment: WindowingEnvironment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowingEnvironmentChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowingEnvironmentRemovedEventArgs_Impl: Sized {
    fn WindowingEnvironment(&mut self) -> ::windows::core::Result<WindowingEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowingEnvironmentRemovedEventArgs {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowingEnvironmentRemovedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentRemovedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentRemovedEventArgs_Vtbl {
        unsafe extern "system" fn WindowingEnvironment<Impl: IWindowingEnvironmentRemovedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowingEnvironmentRemovedEventArgs, BASE_OFFSET>(),
            WindowingEnvironment: WindowingEnvironment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWindowingEnvironmentStatics_Impl: Sized {
    fn FindAll(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
    fn FindAllWithKind(&mut self, kind: WindowingEnvironmentKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WindowingEnvironment>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowingEnvironmentStatics {
    const NAME: &'static str = "Windows.UI.WindowManagement.IWindowingEnvironmentStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWindowingEnvironmentStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowingEnvironmentStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowingEnvironmentStatics_Vtbl {
        unsafe extern "system" fn FindAll<Impl: IWindowingEnvironmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllWithKind<Impl: IWindowingEnvironmentStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: WindowingEnvironmentKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowingEnvironmentStatics, BASE_OFFSET>(),
            FindAll: FindAll::<Impl, IMPL_OFFSET>,
            FindAllWithKind: FindAllWithKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowingEnvironmentStatics as ::windows::core::Interface>::IID
    }
}
