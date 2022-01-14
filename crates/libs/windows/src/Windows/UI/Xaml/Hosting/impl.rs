#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppExitedEventArgs_Impl: Sized {
    fn ExitCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesignerAppExitedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesignerAppExitedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDesignerAppExitedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignerAppExitedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignerAppExitedEventArgs_Vtbl {
        unsafe extern "system" fn ExitCode<Impl: IDesignerAppExitedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignerAppExitedEventArgs, BASE_OFFSET>(), ExitCode: ExitCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignerAppExitedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDesignerAppManager_Impl: Sized {
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DesignerAppExited(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDesignerAppExited(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateNewViewAsync(&mut self, initialviewstate: DesignerAppViewState, initialviewsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DesignerAppView>>;
    fn LoadObjectIntoAppAsync(&mut self, dllname: &::windows::core::HSTRING, classid: &::windows::core::GUID, initializationdata: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDesignerAppManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesignerAppManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDesignerAppManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignerAppManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignerAppManager_Vtbl {
        unsafe extern "system" fn AppUserModelId<Impl: IDesignerAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesignerAppExited<Impl: IDesignerAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesignerAppExited(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DesignerAppManager, DesignerAppExitedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDesignerAppExited<Impl: IDesignerAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDesignerAppExited(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateNewViewAsync<Impl: IDesignerAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, initialviewstate: DesignerAppViewState, initialviewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewViewAsync(initialviewstate, &*(&initialviewsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadObjectIntoAppAsync<Impl: IDesignerAppManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dllname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, classid: ::windows::core::GUID, initializationdata: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadObjectIntoAppAsync(
                &*(&dllname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&classid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&initializationdata as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignerAppManager, BASE_OFFSET>(),
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            DesignerAppExited: DesignerAppExited::<Impl, IMPL_OFFSET>,
            RemoveDesignerAppExited: RemoveDesignerAppExited::<Impl, IMPL_OFFSET>,
            CreateNewViewAsync: CreateNewViewAsync::<Impl, IMPL_OFFSET>,
            LoadObjectIntoAppAsync: LoadObjectIntoAppAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignerAppManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignerAppManagerFactory_Impl: Sized {
    fn Create(&mut self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<DesignerAppManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesignerAppManagerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesignerAppManagerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDesignerAppManagerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignerAppManagerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignerAppManagerFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IDesignerAppManagerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&appusermodelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignerAppManagerFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignerAppManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDesignerAppView_Impl: Sized {
    fn ApplicationViewId(&mut self) -> ::windows::core::Result<i32>;
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ViewState(&mut self) -> ::windows::core::Result<DesignerAppViewState>;
    fn ViewSize(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Size>;
    fn UpdateViewAsync(&mut self, viewstate: DesignerAppViewState, viewsize: &super::super::super::Foundation::Size) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDesignerAppView {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesignerAppView";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDesignerAppView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignerAppView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignerAppView_Vtbl {
        unsafe extern "system" fn ApplicationViewId<Impl: IDesignerAppView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationViewId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppUserModelId<Impl: IDesignerAppView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewState<Impl: IDesignerAppView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DesignerAppViewState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewSize<Impl: IDesignerAppView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateViewAsync<Impl: IDesignerAppView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewstate: DesignerAppViewState, viewsize: super::super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateViewAsync(viewstate, &*(&viewsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignerAppView, BASE_OFFSET>(),
            ApplicationViewId: ApplicationViewId::<Impl, IMPL_OFFSET>,
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            ViewState: ViewState::<Impl, IMPL_OFFSET>,
            ViewSize: ViewSize::<Impl, IMPL_OFFSET>,
            UpdateViewAsync: UpdateViewAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignerAppView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDesktopWindowXamlSource_Impl: Sized {
    fn Content(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn SetContent(&mut self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn HasFocus(&mut self) -> ::windows::core::Result<bool>;
    fn TakeFocusRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveTakeFocusRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NavigateFocus(&mut self, request: &::core::option::Option<XamlSourceFocusNavigationRequest>) -> ::windows::core::Result<XamlSourceFocusNavigationResult>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDesktopWindowXamlSource {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSource";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDesktopWindowXamlSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSource_Vtbl {
        unsafe extern "system" fn Content<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContent<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HasFocus<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeFocusRequested<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeFocusRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceTakeFocusRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTakeFocusRequested<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTakeFocusRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GotFocus<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DesktopWindowXamlSource, DesktopWindowXamlSourceGotFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NavigateFocus<Impl: IDesktopWindowXamlSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateFocus(&*(&request as *const <XamlSourceFocusNavigationRequest as ::windows::core::Abi>::Abi as *const <XamlSourceFocusNavigationRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesktopWindowXamlSource, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            HasFocus: HasFocus::<Impl, IMPL_OFFSET>,
            TakeFocusRequested: TakeFocusRequested::<Impl, IMPL_OFFSET>,
            RemoveTakeFocusRequested: RemoveTakeFocusRequested::<Impl, IMPL_OFFSET>,
            GotFocus: GotFocus::<Impl, IMPL_OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Impl, IMPL_OFFSET>,
            NavigateFocus: NavigateFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DesktopWindowXamlSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesktopWindowXamlSourceFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDesktopWindowXamlSourceFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSourceFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDesktopWindowXamlSourceFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesktopWindowXamlSourceFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceGotFocusEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesktopWindowXamlSourceGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceGotFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceGotFocusEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSourceGotFocusEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: IDesktopWindowXamlSourceGotFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesktopWindowXamlSourceGotFocusEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceGotFocusEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesktopWindowXamlSourceTakeFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IDesktopWindowXamlSourceTakeFocusRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: IDesktopWindowXamlSourceTakeFocusRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesktopWindowXamlSourceTakeFocusRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceTakeFocusRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementCompositionPreview_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementCompositionPreview {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IElementCompositionPreview";
}
#[cfg(feature = "implement_exclusive")]
impl IElementCompositionPreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementCompositionPreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementCompositionPreview_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IElementCompositionPreview, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementCompositionPreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IElementCompositionPreviewStatics_Impl: Sized {
    fn GetElementVisual(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::Visual>;
    fn GetElementChildVisual(&mut self, element: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::Visual>;
    fn SetElementChildVisual(&mut self, element: &::core::option::Option<super::UIElement>, visual: &::core::option::Option<super::super::Composition::Visual>) -> ::windows::core::Result<()>;
    fn GetScrollViewerManipulationPropertySet(&mut self, scrollviewer: &::core::option::Option<super::Controls::ScrollViewer>) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IElementCompositionPreviewStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IElementCompositionPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementCompositionPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementCompositionPreviewStatics_Vtbl {
        unsafe extern "system" fn GetElementVisual<Impl: IElementCompositionPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElementVisual(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementChildVisual<Impl: IElementCompositionPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElementChildVisual(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementChildVisual<Impl: IElementCompositionPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementChildVisual(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&visual as *const <super::super::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::super::Composition::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetScrollViewerManipulationPropertySet<Impl: IElementCompositionPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scrollviewer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScrollViewerManipulationPropertySet(&*(&scrollviewer as *const <super::Controls::ScrollViewer as ::windows::core::Abi>::Abi as *const <super::Controls::ScrollViewer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementCompositionPreviewStatics, BASE_OFFSET>(),
            GetElementVisual: GetElementVisual::<Impl, IMPL_OFFSET>,
            GetElementChildVisual: GetElementChildVisual::<Impl, IMPL_OFFSET>,
            SetElementChildVisual: SetElementChildVisual::<Impl, IMPL_OFFSET>,
            GetScrollViewerManipulationPropertySet: GetScrollViewerManipulationPropertySet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementCompositionPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IElementCompositionPreviewStatics2_Impl: Sized {
    fn SetImplicitShowAnimation(&mut self, element: &::core::option::Option<super::UIElement>, animation: &::core::option::Option<super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn SetImplicitHideAnimation(&mut self, element: &::core::option::Option<super::UIElement>, animation: &::core::option::Option<super::super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn SetIsTranslationEnabled(&mut self, element: &::core::option::Option<super::UIElement>, value: bool) -> ::windows::core::Result<()>;
    fn GetPointerPositionPropertySet(&mut self, targetelement: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Composition::CompositionPropertySet>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IElementCompositionPreviewStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics2";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IElementCompositionPreviewStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementCompositionPreviewStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementCompositionPreviewStatics2_Vtbl {
        unsafe extern "system" fn SetImplicitShowAnimation<Impl: IElementCompositionPreviewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplicitShowAnimation(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <super::super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetImplicitHideAnimation<Impl: IElementCompositionPreviewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImplicitHideAnimation(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&animation as *const <super::super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIsTranslationEnabled<Impl: IElementCompositionPreviewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTranslationEnabled(&*(&element as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn GetPointerPositionPropertySet<Impl: IElementCompositionPreviewStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPointerPositionPropertySet(&*(&targetelement as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementCompositionPreviewStatics2, BASE_OFFSET>(),
            SetImplicitShowAnimation: SetImplicitShowAnimation::<Impl, IMPL_OFFSET>,
            SetImplicitHideAnimation: SetImplicitHideAnimation::<Impl, IMPL_OFFSET>,
            SetIsTranslationEnabled: SetIsTranslationEnabled::<Impl, IMPL_OFFSET>,
            GetPointerPositionPropertySet: GetPointerPositionPropertySet::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementCompositionPreviewStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
pub trait IElementCompositionPreviewStatics3_Impl: Sized {
    fn SetAppWindowContent(&mut self, appwindow: &::core::option::Option<super::super::WindowManagement::AppWindow>, xamlcontent: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn GetAppWindowContent(&mut self, appwindow: &::core::option::Option<super::super::WindowManagement::AppWindow>) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IElementCompositionPreviewStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IElementCompositionPreviewStatics3";
}
#[cfg(all(feature = "UI_WindowManagement", feature = "implement_exclusive"))]
impl IElementCompositionPreviewStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementCompositionPreviewStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementCompositionPreviewStatics3_Vtbl {
        unsafe extern "system" fn SetAppWindowContent<Impl: IElementCompositionPreviewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, xamlcontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppWindowContent(&*(&appwindow as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType), &*(&xamlcontent as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAppWindowContent<Impl: IElementCompositionPreviewStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppWindowContent(&*(&appwindow as *const <super::super::WindowManagement::AppWindow as ::windows::core::Abi>::Abi as *const <super::super::WindowManagement::AppWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementCompositionPreviewStatics3, BASE_OFFSET>(),
            SetAppWindowContent: SetAppWindowContent::<Impl, IMPL_OFFSET>,
            GetAppWindowContent: GetAppWindowContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementCompositionPreviewStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsXamlManager_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowsXamlManager {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IWindowsXamlManager";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowsXamlManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsXamlManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsXamlManager_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsXamlManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsXamlManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowsXamlManagerStatics_Impl: Sized {
    fn InitializeForCurrentThread(&mut self) -> ::windows::core::Result<WindowsXamlManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowsXamlManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IWindowsXamlManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowsXamlManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsXamlManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsXamlManagerStatics_Vtbl {
        unsafe extern "system" fn InitializeForCurrentThread<Impl: IWindowsXamlManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeForCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowsXamlManagerStatics, BASE_OFFSET>(),
            InitializeForCurrentThread: InitializeForCurrentThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsXamlManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IXamlSourceFocusNavigationRequest_Impl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<XamlSourceFocusNavigationReason>;
    fn HintRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlSourceFocusNavigationRequest {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IXamlSourceFocusNavigationRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlSourceFocusNavigationRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlSourceFocusNavigationRequest_Vtbl {
        unsafe extern "system" fn Reason<Impl: IXamlSourceFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XamlSourceFocusNavigationReason) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HintRect<Impl: IXamlSourceFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HintRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IXamlSourceFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlSourceFocusNavigationRequest, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
            HintRect: HintRect::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlSourceFocusNavigationRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IXamlSourceFocusNavigationRequestFactory_Impl: Sized {
    fn CreateInstance(&mut self, reason: XamlSourceFocusNavigationReason) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
    fn CreateInstanceWithHintRect(&mut self, reason: XamlSourceFocusNavigationReason, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
    fn CreateInstanceWithHintRectAndCorrelationId(&mut self, reason: XamlSourceFocusNavigationReason, hintrect: &super::super::super::Foundation::Rect, correlationid: &::windows::core::GUID) -> ::windows::core::Result<XamlSourceFocusNavigationRequest>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlSourceFocusNavigationRequestFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationRequestFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IXamlSourceFocusNavigationRequestFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlSourceFocusNavigationRequestFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlSourceFocusNavigationRequestFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlSourceFocusNavigationRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(reason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithHintRect<Impl: IXamlSourceFocusNavigationRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHintRect(reason, &*(&hintrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithHintRectAndCorrelationId<Impl: IXamlSourceFocusNavigationRequestFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reason: XamlSourceFocusNavigationReason, hintrect: super::super::super::Foundation::Rect, correlationid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHintRectAndCorrelationId(reason, &*(&hintrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&correlationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlSourceFocusNavigationRequestFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithHintRect: CreateInstanceWithHintRect::<Impl, IMPL_OFFSET>,
            CreateInstanceWithHintRectAndCorrelationId: CreateInstanceWithHintRectAndCorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlSourceFocusNavigationRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationResult_Impl: Sized {
    fn WasFocusMoved(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlSourceFocusNavigationResult {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlSourceFocusNavigationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlSourceFocusNavigationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlSourceFocusNavigationResult_Vtbl {
        unsafe extern "system" fn WasFocusMoved<Impl: IXamlSourceFocusNavigationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasFocusMoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlSourceFocusNavigationResult, BASE_OFFSET>(),
            WasFocusMoved: WasFocusMoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlSourceFocusNavigationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlSourceFocusNavigationResultFactory_Impl: Sized {
    fn CreateInstance(&mut self, focusmoved: bool) -> ::windows::core::Result<XamlSourceFocusNavigationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlSourceFocusNavigationResultFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlSourceFocusNavigationResultFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlSourceFocusNavigationResultFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlSourceFocusNavigationResultFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlSourceFocusNavigationResultFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlSourceFocusNavigationResultFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusmoved: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(focusmoved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlSourceFocusNavigationResultFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlSourceFocusNavigationResultFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUIPresenter_Impl: Sized {
    fn RootElement(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn SetRootElement(&mut self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ThemeKey(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetThemeKey(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ThemeResourcesXaml(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetThemeResourcesXaml(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetSize(&mut self, width: i32, height: i32) -> ::windows::core::Result<()>;
    fn Render(&mut self) -> ::windows::core::Result<()>;
    fn Present(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUIPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUIPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenter_Vtbl {
        unsafe extern "system" fn RootElement<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RootElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootElement<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRootElement(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ThemeKey<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThemeKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThemeKey<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThemeKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ThemeResourcesXaml<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThemeResourcesXaml() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThemeResourcesXaml<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetThemeResourcesXaml(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetSize<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: i32, height: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSize(width, height).into()
        }
        unsafe extern "system" fn Render<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Render().into()
        }
        unsafe extern "system" fn Present<Impl: IXamlUIPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenter, BASE_OFFSET>(),
            RootElement: RootElement::<Impl, IMPL_OFFSET>,
            SetRootElement: SetRootElement::<Impl, IMPL_OFFSET>,
            ThemeKey: ThemeKey::<Impl, IMPL_OFFSET>,
            SetThemeKey: SetThemeKey::<Impl, IMPL_OFFSET>,
            ThemeResourcesXaml: ThemeResourcesXaml::<Impl, IMPL_OFFSET>,
            SetThemeResourcesXaml: SetThemeResourcesXaml::<Impl, IMPL_OFFSET>,
            SetSize: SetSize::<Impl, IMPL_OFFSET>,
            Render: Render::<Impl, IMPL_OFFSET>,
            Present: Present::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenter as ::windows::core::Interface>::IID
    }
}
pub trait IXamlUIPresenterHost_Impl: Sized {
    fn ResolveFileResource(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IXamlUIPresenterHost {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost";
}
impl IXamlUIPresenterHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenterHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenterHost_Vtbl {
        unsafe extern "system" fn ResolveFileResource<Impl: IXamlUIPresenterHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveFileResource(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost, BASE_OFFSET>(),
            ResolveFileResource: ResolveFileResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost as ::windows::core::Interface>::IID
    }
}
pub trait IXamlUIPresenterHost2_Impl: Sized {
    fn GetGenericXamlFilePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IXamlUIPresenterHost2 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost2";
}
impl IXamlUIPresenterHost2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenterHost2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenterHost2_Vtbl {
        unsafe extern "system" fn GetGenericXamlFilePath<Impl: IXamlUIPresenterHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenericXamlFilePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost2, BASE_OFFSET>(),
            GetGenericXamlFilePath: GetGenericXamlFilePath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost2 as ::windows::core::Interface>::IID
    }
}
pub trait IXamlUIPresenterHost3_Impl: Sized {
    fn ResolveDictionaryResource(&mut self, dictionary: &::core::option::Option<super::ResourceDictionary>, dictionarykey: &::core::option::Option<::windows::core::IInspectable>, suggestedvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IXamlUIPresenterHost3 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterHost3";
}
impl IXamlUIPresenterHost3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenterHost3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenterHost3_Vtbl {
        unsafe extern "system" fn ResolveDictionaryResource<Impl: IXamlUIPresenterHost3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dictionary: ::windows::core::RawPtr, dictionarykey: *mut ::core::ffi::c_void, suggestedvalue: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveDictionaryResource(
                &*(&dictionary as *const <super::ResourceDictionary as ::windows::core::Abi>::Abi as *const <super::ResourceDictionary as ::windows::core::DefaultType>::DefaultType),
                &*(&dictionarykey as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&suggestedvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenterHost3, BASE_OFFSET>(),
            ResolveDictionaryResource: ResolveDictionaryResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenterHost3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUIPresenterStatics_Impl: Sized {
    fn CompleteTimelinesAutomatically(&mut self) -> ::windows::core::Result<bool>;
    fn SetCompleteTimelinesAutomatically(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SetHost(&mut self, host: &::core::option::Option<IXamlUIPresenterHost>) -> ::windows::core::Result<()>;
    fn NotifyWindowSizeChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUIPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUIPresenterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenterStatics_Vtbl {
        unsafe extern "system" fn CompleteTimelinesAutomatically<Impl: IXamlUIPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompleteTimelinesAutomatically() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleteTimelinesAutomatically<Impl: IXamlUIPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleteTimelinesAutomatically(value).into()
        }
        unsafe extern "system" fn SetHost<Impl: IXamlUIPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHost(&*(&host as *const <IXamlUIPresenterHost as ::windows::core::Abi>::Abi as *const <IXamlUIPresenterHost as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyWindowSizeChanged<Impl: IXamlUIPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyWindowSizeChanged().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenterStatics, BASE_OFFSET>(),
            CompleteTimelinesAutomatically: CompleteTimelinesAutomatically::<Impl, IMPL_OFFSET>,
            SetCompleteTimelinesAutomatically: SetCompleteTimelinesAutomatically::<Impl, IMPL_OFFSET>,
            SetHost: SetHost::<Impl, IMPL_OFFSET>,
            NotifyWindowSizeChanged: NotifyWindowSizeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IXamlUIPresenterStatics2_Impl: Sized {
    fn GetFlyoutPlacementTargetInfo(&mut self, placementtarget: &::core::option::Option<super::FrameworkElement>, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: &mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: &mut bool) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn GetFlyoutPlacement(&mut self, placementtargetbounds: &super::super::super::Foundation::Rect, controlsize: &super::super::super::Foundation::Size, mincontrolsize: &super::super::super::Foundation::Size, containerrect: &super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: &mut super::Controls::Primitives::FlyoutPlacementMode) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlUIPresenterStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Hosting.IXamlUIPresenterStatics2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IXamlUIPresenterStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUIPresenterStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUIPresenterStatics2_Vtbl {
        unsafe extern "system" fn GetFlyoutPlacementTargetInfo<Impl: IXamlUIPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, preferredplacement: super::Controls::Primitives::FlyoutPlacementMode, targetpreferredplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: *mut bool, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlyoutPlacementTargetInfo(&*(&placementtarget as *const <super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), preferredplacement, ::core::mem::transmute_copy(&targetpreferredplacement), ::core::mem::transmute_copy(&allowfallbacks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlyoutPlacement<Impl: IXamlUIPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtargetbounds: super::super::super::Foundation::Rect, controlsize: super::super::super::Foundation::Size, mincontrolsize: super::super::super::Foundation::Size, containerrect: super::super::super::Foundation::Rect, targetpreferredplacement: super::Controls::Primitives::FlyoutPlacementMode, allowfallbacks: bool, chosenplacement: *mut super::Controls::Primitives::FlyoutPlacementMode, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlyoutPlacement(
                &*(&placementtargetbounds as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                &*(&controlsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType),
                &*(&mincontrolsize as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType),
                &*(&containerrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                targetpreferredplacement,
                allowfallbacks,
                ::core::mem::transmute_copy(&chosenplacement),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUIPresenterStatics2, BASE_OFFSET>(),
            GetFlyoutPlacementTargetInfo: GetFlyoutPlacementTargetInfo::<Impl, IMPL_OFFSET>,
            GetFlyoutPlacement: GetFlyoutPlacement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUIPresenterStatics2 as ::windows::core::Interface>::IID
    }
}
