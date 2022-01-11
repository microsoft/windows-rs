#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IFrameNavigationOptionsImpl: Sized {
    fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitionInfoOverride(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
    fn SetTransitionInfoOverride(&self, value: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameNavigationOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IFrameNavigationOptions";
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IFrameNavigationOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameNavigationOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameNavigationOptionsVtbl {
        unsafe extern "system" fn IsNavigationStackEnabled<Impl: IFrameNavigationOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNavigationStackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsNavigationStackEnabled<Impl: IFrameNavigationOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsNavigationStackEnabled(value).into()
        }
        unsafe extern "system" fn TransitionInfoOverride<Impl: IFrameNavigationOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionInfoOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitionInfoOverride<Impl: IFrameNavigationOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitionInfoOverride(&*(&value as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameNavigationOptions, BASE_OFFSET>(),
            IsNavigationStackEnabled: IsNavigationStackEnabled::<Impl, IMPL_OFFSET>,
            SetIsNavigationStackEnabled: SetIsNavigationStackEnabled::<Impl, IMPL_OFFSET>,
            TransitionInfoOverride: TransitionInfoOverride::<Impl, IMPL_OFFSET>,
            SetTransitionInfoOverride: SetTransitionInfoOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameNavigationOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameNavigationOptionsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameNavigationOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameNavigationOptionsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IFrameNavigationOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameNavigationOptionsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameNavigationOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameNavigationOptionsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameNavigationOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameNavigationOptionsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameNavigationOptionsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait INavigatingCancelEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigatingCancelEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl INavigatingCancelEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigatingCancelEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigatingCancelEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: INavigatingCancelEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: INavigatingCancelEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn NavigationMode<Impl: INavigatingCancelEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigatingCancelEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigatingCancelEventArgs, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
            NavigationMode: NavigationMode::<Impl, IMPL_OFFSET>,
            SourcePageType: SourcePageType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigatingCancelEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait INavigatingCancelEventArgs2Impl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigatingCancelEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2";
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl INavigatingCancelEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigatingCancelEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigatingCancelEventArgs2Vtbl {
        unsafe extern "system" fn Parameter<Impl: INavigatingCancelEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationTransitionInfo<Impl: INavigatingCancelEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigatingCancelEventArgs2, BASE_OFFSET>(),
            Parameter: Parameter::<Impl, IMPL_OFFSET>,
            NavigationTransitionInfo: NavigationTransitionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigatingCancelEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait INavigationEventArgsImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigationEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl INavigationEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationEventArgsVtbl {
        unsafe extern "system" fn Content<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Parameter<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationMode<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: INavigationEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationEventArgs, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            Parameter: Parameter::<Impl, IMPL_OFFSET>,
            SourcePageType: SourcePageType::<Impl, IMPL_OFFSET>,
            NavigationMode: NavigationMode::<Impl, IMPL_OFFSET>,
            Uri: Uri::<Impl, IMPL_OFFSET>,
            SetUri: SetUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait INavigationEventArgs2Impl: Sized {
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigationEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationEventArgs2";
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl INavigationEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationEventArgs2Vtbl {
        unsafe extern "system" fn NavigationTransitionInfo<Impl: INavigationEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationEventArgs2, BASE_OFFSET>(),
            NavigationTransitionInfo: NavigationTransitionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait INavigationFailedEventArgsImpl: Sized {
    fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigationFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationFailedEventArgs";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl INavigationFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationFailedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationFailedEventArgsVtbl {
        unsafe extern "system" fn Exception<Impl: INavigationFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exception() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: INavigationFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: INavigationFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigationFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationFailedEventArgs, BASE_OFFSET>(),
            Exception: Exception::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            SourcePageType: SourcePageType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IPageStackEntryImpl: Sized {
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPageStackEntry {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IPageStackEntry";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IPageStackEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPageStackEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPageStackEntryVtbl {
        unsafe extern "system" fn SourcePageType<Impl: IPageStackEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parameter<Impl: IPageStackEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationTransitionInfo<Impl: IPageStackEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPageStackEntry, BASE_OFFSET>(),
            SourcePageType: SourcePageType::<Impl, IMPL_OFFSET>,
            Parameter: Parameter::<Impl, IMPL_OFFSET>,
            NavigationTransitionInfo: NavigationTransitionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPageStackEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IPageStackEntryFactoryImpl: Sized {
    fn CreateInstance(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, navigationtransitioninfo: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<PageStackEntry>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPageStackEntryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IPageStackEntryFactory";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IPageStackEntryFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPageStackEntryFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPageStackEntryFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPageStackEntryFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, navigationtransitioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&sourcepagetype as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&navigationtransitioninfo as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPageStackEntryFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPageStackEntryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryStaticsImpl: Sized {
    fn SourcePageTypeProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPageStackEntryStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IPageStackEntryStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPageStackEntryStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPageStackEntryStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPageStackEntryStaticsVtbl {
        unsafe extern "system" fn SourcePageTypeProperty<Impl: IPageStackEntryStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePageTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPageStackEntryStatics, BASE_OFFSET>(),
            SourcePageTypeProperty: SourcePageTypeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPageStackEntryStatics as ::windows::core::Interface>::IID
    }
}
