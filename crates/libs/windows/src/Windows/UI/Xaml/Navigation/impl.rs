#[cfg(feature = "implement_exclusive")]
pub trait IFrameNavigationOptionsImpl: Sized {
    fn IsNavigationStackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsNavigationStackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TransitionInfoOverride(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
    fn SetTransitionInfoOverride(&self, value: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameNavigationOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IFrameNavigationOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameNavigationOptionsVtbl {
    pub const fn new<Impl: IFrameNavigationOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameNavigationOptionsVtbl {
        unsafe extern "system" fn IsNavigationStackEnabled<Impl: IFrameNavigationOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsNavigationStackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsNavigationStackEnabled<Impl: IFrameNavigationOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsNavigationStackEnabled(value).into()
        }
        unsafe extern "system" fn TransitionInfoOverride<Impl: IFrameNavigationOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitionInfoOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitionInfoOverride<Impl: IFrameNavigationOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransitionInfoOverride(&*(&value as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::Abi>::Abi as *const <super::Media::Animation::NavigationTransitionInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameNavigationOptions>, base.5, IsNavigationStackEnabled::<Impl, OFFSET>, SetIsNavigationStackEnabled::<Impl, OFFSET>, TransitionInfoOverride::<Impl, OFFSET>, SetTransitionInfoOverride::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFrameNavigationOptionsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameNavigationOptionsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameNavigationOptionsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameNavigationOptionsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigatingCancelEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigatingCancelEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INavigatingCancelEventArgsVtbl {
    pub const fn new<Impl: INavigatingCancelEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigatingCancelEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: INavigatingCancelEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: INavigatingCancelEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn NavigationMode<Impl: INavigatingCancelEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigatingCancelEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigatingCancelEventArgs>, base.5, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>, NavigationMode::<Impl, OFFSET>, SourcePageType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigatingCancelEventArgs2Impl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigatingCancelEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigatingCancelEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl INavigatingCancelEventArgs2Vtbl {
    pub const fn new<Impl: INavigatingCancelEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigatingCancelEventArgs2Vtbl {
        unsafe extern "system" fn Parameter<Impl: INavigatingCancelEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationTransitionInfo<Impl: INavigatingCancelEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigatingCancelEventArgs2>, base.5, Parameter::<Impl, OFFSET>, NavigationTransitionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationEventArgsImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn NavigationMode(&self) -> ::windows::core::Result<NavigationMode>;
    fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationEventArgsVtbl {
    pub const fn new<Impl: INavigationEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationEventArgsVtbl {
        unsafe extern "system" fn Content<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parameter<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationMode<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uri<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUri<Impl: INavigationEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUri(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationEventArgs>, base.5, Content::<Impl, OFFSET>, Parameter::<Impl, OFFSET>, SourcePageType::<Impl, OFFSET>, NavigationMode::<Impl, OFFSET>, Uri::<Impl, OFFSET>, SetUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationEventArgs2Impl: Sized {
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationEventArgs2Vtbl {
    pub const fn new<Impl: INavigationEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationEventArgs2Vtbl {
        unsafe extern "system" fn NavigationTransitionInfo<Impl: INavigationEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationEventArgs2>, base.5, NavigationTransitionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationFailedEventArgsImpl: Sized {
    fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.INavigationFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationFailedEventArgsVtbl {
    pub const fn new<Impl: INavigationFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationFailedEventArgsVtbl {
        unsafe extern "system" fn Exception<Impl: INavigationFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exception() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: INavigationFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: INavigationFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn SourcePageType<Impl: INavigationFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationFailedEventArgs>, base.5, Exception::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, SourcePageType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryImpl: Sized {
    fn SourcePageType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NavigationTransitionInfo(&self) -> ::windows::core::Result<super::Media::Animation::NavigationTransitionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPageStackEntry {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IPageStackEntry";
}
#[cfg(feature = "implement_exclusive")]
impl IPageStackEntryVtbl {
    pub const fn new<Impl: IPageStackEntryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPageStackEntryVtbl {
        unsafe extern "system" fn SourcePageType<Impl: IPageStackEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parameter<Impl: IPageStackEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavigationTransitionInfo<Impl: IPageStackEntryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigationTransitionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPageStackEntry>, base.5, SourcePageType::<Impl, OFFSET>, Parameter::<Impl, OFFSET>, NavigationTransitionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPageStackEntryFactoryImpl: Sized {
    fn CreateInstance(&self, sourcepagetype: &super::Interop::TypeName, parameter: &::core::option::Option<::windows::core::IInspectable>, navigationtransitioninfo: &::core::option::Option<super::Media::Animation::NavigationTransitionInfo>) -> ::windows::core::Result<PageStackEntry>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPageStackEntryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Navigation.IPageStackEntryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPageStackEntryFactoryVtbl {
    pub const fn new<Impl: IPageStackEntryFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPageStackEntryFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPageStackEntryFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcepagetype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, navigationtransitioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPageStackEntryFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPageStackEntryStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPageStackEntryStaticsVtbl {
        unsafe extern "system" fn SourcePageTypeProperty<Impl: IPageStackEntryStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SourcePageTypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPageStackEntryStatics>, base.5, SourcePageTypeProperty::<Impl, OFFSET>)
    }
}
