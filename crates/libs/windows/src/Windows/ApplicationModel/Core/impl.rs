#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppListEntryImpl: Sized {
    fn DisplayInfo(&self) -> ::windows::core::Result<super::AppDisplayInfo>;
    fn LaunchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppListEntry {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IAppListEntry";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppListEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppListEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppListEntryVtbl {
        unsafe extern "system" fn DisplayInfo<Impl: IAppListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchAsync<Impl: IAppListEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppListEntry>, ::windows::core::GetTrustLevel, DisplayInfo::<Impl, IMPL_OFFSET>, LaunchAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry2Impl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppListEntry2 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IAppListEntry2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppListEntry2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppListEntry2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppListEntry2Vtbl {
        unsafe extern "system" fn AppUserModelId<Impl: IAppListEntry2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppListEntry2>, ::windows::core::GetTrustLevel, AppUserModelId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IAppListEntry3Impl: Sized {
    fn LaunchForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppListEntry3 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IAppListEntry3";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IAppListEntry3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppListEntry3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppListEntry3Vtbl {
        unsafe extern "system" fn LaunchForUserAsync<Impl: IAppListEntry3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppListEntry3>, ::windows::core::GetTrustLevel, LaunchForUserAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry4Impl: Sized {
    fn AppInfo(&self) -> ::windows::core::Result<super::AppInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppListEntry4 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IAppListEntry4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppListEntry4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppListEntry4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppListEntry4Vtbl {
        unsafe extern "system" fn AppInfo<Impl: IAppListEntry4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppListEntry4>, ::windows::core::GetTrustLevel, AppInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreApplicationImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suspending(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::SuspendingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn GetCurrentView(&self) -> ::windows::core::Result<CoreApplicationView>;
    fn Run(&self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<()>;
    fn RunWithActivationFactories(&self, activationfactorycallback: &::core::option::Option<super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplication";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationVtbl {
        unsafe extern "system" fn Id<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Suspending<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspending(&*(&handler as *const <super::super::Foundation::EventHandler<super::SuspendingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<super::SuspendingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspending<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Resuming<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resuming(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResuming<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResuming(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentView<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewsource: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run(&*(&viewsource as *const <IFrameworkViewSource as ::windows::core::Abi>::Abi as *const <IFrameworkViewSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RunWithActivationFactories<Impl: ICoreApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationfactorycallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunWithActivationFactories(&*(&activationfactorycallback as *const <super::super::Foundation::IGetActivationFactory as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IGetActivationFactory as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreApplication>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, IMPL_OFFSET>,
            Suspending::<Impl, IMPL_OFFSET>,
            RemoveSuspending::<Impl, IMPL_OFFSET>,
            Resuming::<Impl, IMPL_OFFSET>,
            RemoveResuming::<Impl, IMPL_OFFSET>,
            Properties::<Impl, IMPL_OFFSET>,
            GetCurrentView::<Impl, IMPL_OFFSET>,
            Run::<Impl, IMPL_OFFSET>,
            RunWithActivationFactories::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplication2Impl: Sized {
    fn BackgroundActivated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LeavingBackground(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplication2 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplication2";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplication2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplication2Vtbl {
        unsafe extern "system" fn BackgroundActivated<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackgroundActivated(&*(&handler as *const <super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackgroundActivated<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackgroundActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LeavingBackground<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeavingBackground(&*(&handler as *const <super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLeavingBackground<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLeavingBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnteredBackground<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnteredBackground(&*(&handler as *const <super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnteredBackground<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnteredBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnablePrelaunch<Impl: ICoreApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePrelaunch(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreApplication2>,
            ::windows::core::GetTrustLevel,
            BackgroundActivated::<Impl, IMPL_OFFSET>,
            RemoveBackgroundActivated::<Impl, IMPL_OFFSET>,
            LeavingBackground::<Impl, IMPL_OFFSET>,
            RemoveLeavingBackground::<Impl, IMPL_OFFSET>,
            EnteredBackground::<Impl, IMPL_OFFSET>,
            RemoveEnteredBackground::<Impl, IMPL_OFFSET>,
            EnablePrelaunch::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait ICoreApplication3Impl: Sized {
    fn RequestRestartAsync(&self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplication3 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplication3";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ICoreApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplication3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplication3Vtbl {
        unsafe extern "system" fn RequestRestartAsync<Impl: ICoreApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRestartAsync(&*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRestartForUserAsync<Impl: ICoreApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRestartForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&launcharguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplication3>, ::windows::core::GetTrustLevel, RequestRestartAsync::<Impl, IMPL_OFFSET>, RequestRestartForUserAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationExitImpl: Sized {
    fn Exit(&self) -> ::windows::core::Result<()>;
    fn Exiting(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveExiting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationExit {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationExit";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreApplicationExitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationExitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationExitVtbl {
        unsafe extern "system" fn Exit<Impl: ICoreApplicationExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Exit().into()
        }
        unsafe extern "system" fn Exiting<Impl: ICoreApplicationExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exiting(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveExiting<Impl: ICoreApplicationExitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveExiting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationExit>, ::windows::core::GetTrustLevel, Exit::<Impl, IMPL_OFFSET>, Exiting::<Impl, IMPL_OFFSET>, RemoveExiting::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationExit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICoreApplicationUnhandledErrorImpl: Sized {
    fn UnhandledErrorDetected(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreApplicationUnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationUnhandledError";
}
#[cfg(feature = "Foundation")]
impl ICoreApplicationUnhandledErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationUnhandledErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationUnhandledErrorVtbl {
        unsafe extern "system" fn UnhandledErrorDetected<Impl: ICoreApplicationUnhandledErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnhandledErrorDetected(&*(&handler as *const <super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledErrorDetected<Impl: ICoreApplicationUnhandledErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnhandledErrorDetected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationUnhandledError>, ::windows::core::GetTrustLevel, UnhandledErrorDetected::<Impl, IMPL_OFFSET>, RemoveUnhandledErrorDetected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUnhandledError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationUseCountImpl: Sized {
    fn IncrementApplicationUseCount(&self) -> ::windows::core::Result<()>;
    fn DecrementApplicationUseCount(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreApplicationUseCount {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationUseCount";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreApplicationUseCountVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationUseCountImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationUseCountVtbl {
        unsafe extern "system" fn IncrementApplicationUseCount<Impl: ICoreApplicationUseCountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IncrementApplicationUseCount().into()
        }
        unsafe extern "system" fn DecrementApplicationUseCount<Impl: ICoreApplicationUseCountImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecrementApplicationUseCount().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationUseCount>, ::windows::core::GetTrustLevel, IncrementApplicationUseCount::<Impl, IMPL_OFFSET>, DecrementApplicationUseCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUseCount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreApplicationViewImpl: Sized {
    fn CoreWindow(&self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsMain(&self) -> ::windows::core::Result<bool>;
    fn IsHosted(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationView";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ICoreApplicationViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationViewVtbl {
        unsafe extern "system" fn CoreWindow<Impl: ICoreApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoreWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activated<Impl: ICoreApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: ICoreApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsMain<Impl: ICoreApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHosted<Impl: ICoreApplicationViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHosted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationView>, ::windows::core::GetTrustLevel, CoreWindow::<Impl, IMPL_OFFSET>, Activated::<Impl, IMPL_OFFSET>, RemoveActivated::<Impl, IMPL_OFFSET>, IsMain::<Impl, IMPL_OFFSET>, IsHosted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreApplicationView2Impl: Sized {
    fn Dispatcher(&self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationView2 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationView2";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ICoreApplicationView2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationView2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationView2Vtbl {
        unsafe extern "system" fn Dispatcher<Impl: ICoreApplicationView2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationView2>, ::windows::core::GetTrustLevel, Dispatcher::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationView3Impl: Sized {
    fn IsComponent(&self) -> ::windows::core::Result<bool>;
    fn TitleBar(&self) -> ::windows::core::Result<CoreApplicationViewTitleBar>;
    fn HostedViewClosing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHostedViewClosing(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationView3 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationView3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreApplicationView3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationView3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationView3Vtbl {
        unsafe extern "system" fn IsComponent<Impl: ICoreApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComponent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleBar<Impl: ICoreApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HostedViewClosing<Impl: ICoreApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostedViewClosing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHostedViewClosing<Impl: ICoreApplicationView3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHostedViewClosing(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationView3>, ::windows::core::GetTrustLevel, IsComponent::<Impl, IMPL_OFFSET>, TitleBar::<Impl, IMPL_OFFSET>, HostedViewClosing::<Impl, IMPL_OFFSET>, RemoveHostedViewClosing::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreApplicationView5Impl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationView5 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationView5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreApplicationView5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationView5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationView5Vtbl {
        unsafe extern "system" fn Properties<Impl: ICoreApplicationView5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationView5>, ::windows::core::GetTrustLevel, Properties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ICoreApplicationView6Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationView6 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationView6";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ICoreApplicationView6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationView6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationView6Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICoreApplicationView6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreApplicationView6>, ::windows::core::GetTrustLevel, DispatcherQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationViewTitleBarImpl: Sized {
    fn SetExtendViewIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExtendViewIntoTitleBar(&self) -> ::windows::core::Result<bool>;
    fn SystemOverlayLeftInset(&self) -> ::windows::core::Result<f64>;
    fn SystemOverlayRightInset(&self) -> ::windows::core::Result<f64>;
    fn Height(&self) -> ::windows::core::Result<f64>;
    fn LayoutMetricsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutMetricsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn IsVisibleChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsVisibleChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreApplicationViewTitleBar {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreApplicationViewTitleBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreApplicationViewTitleBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreApplicationViewTitleBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreApplicationViewTitleBarVtbl {
        unsafe extern "system" fn SetExtendViewIntoTitleBar<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendViewIntoTitleBar(value).into()
        }
        unsafe extern "system" fn ExtendViewIntoTitleBar<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendViewIntoTitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemOverlayLeftInset<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemOverlayLeftInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemOverlayRightInset<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemOverlayRightInset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Height<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LayoutMetricsChanged<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayoutMetricsChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLayoutMetricsChanged<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLayoutMetricsChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsVisible<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVisibleChanged<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVisibleChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsVisibleChanged<Impl: ICoreApplicationViewTitleBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsVisibleChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICoreApplicationViewTitleBar>,
            ::windows::core::GetTrustLevel,
            SetExtendViewIntoTitleBar::<Impl, IMPL_OFFSET>,
            ExtendViewIntoTitleBar::<Impl, IMPL_OFFSET>,
            SystemOverlayLeftInset::<Impl, IMPL_OFFSET>,
            SystemOverlayRightInset::<Impl, IMPL_OFFSET>,
            Height::<Impl, IMPL_OFFSET>,
            LayoutMetricsChanged::<Impl, IMPL_OFFSET>,
            RemoveLayoutMetricsChanged::<Impl, IMPL_OFFSET>,
            IsVisible::<Impl, IMPL_OFFSET>,
            IsVisibleChanged::<Impl, IMPL_OFFSET>,
            RemoveIsVisibleChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationViewTitleBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreImmersiveApplicationImpl: Sized {
    fn Views(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>;
    fn CreateNewView(&self, runtimetype: &::windows::core::HSTRING, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<CoreApplicationView>;
    fn MainView(&self) -> ::windows::core::Result<CoreApplicationView>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreImmersiveApplication {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreImmersiveApplication";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICoreImmersiveApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreImmersiveApplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreImmersiveApplicationVtbl {
        unsafe extern "system" fn Views<Impl: ICoreImmersiveApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Views() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateNewView<Impl: ICoreImmersiveApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, runtimetype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewView(&*(&runtimetype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&entrypoint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MainView<Impl: ICoreImmersiveApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MainView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreImmersiveApplication>, ::windows::core::GetTrustLevel, Views::<Impl, IMPL_OFFSET>, CreateNewView::<Impl, IMPL_OFFSET>, MainView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication2Impl: Sized {
    fn CreateNewViewFromMainView(&self) -> ::windows::core::Result<CoreApplicationView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreImmersiveApplication2 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreImmersiveApplication2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreImmersiveApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreImmersiveApplication2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreImmersiveApplication2Vtbl {
        unsafe extern "system" fn CreateNewViewFromMainView<Impl: ICoreImmersiveApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewViewFromMainView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreImmersiveApplication2>, ::windows::core::GetTrustLevel, CreateNewViewFromMainView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication3Impl: Sized {
    fn CreateNewViewWithViewSource(&self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<CoreApplicationView>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreImmersiveApplication3 {
    const NAME: &'static str = "Windows.ApplicationModel.Core.ICoreImmersiveApplication3";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreImmersiveApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreImmersiveApplication3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreImmersiveApplication3Vtbl {
        unsafe extern "system" fn CreateNewViewWithViewSource<Impl: ICoreImmersiveApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewsource: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewViewWithViewSource(&*(&viewsource as *const <IFrameworkViewSource as ::windows::core::Abi>::Abi as *const <IFrameworkViewSource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreImmersiveApplication3>, ::windows::core::GetTrustLevel, CreateNewViewWithViewSource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Core")]
pub trait IFrameworkViewImpl: Sized {
    fn Initialize(&self, applicationview: &::core::option::Option<CoreApplicationView>) -> ::windows::core::Result<()>;
    fn SetWindow(&self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<()>;
    fn Load(&self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Run(&self) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Core")]
impl ::windows::core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkView";
}
#[cfg(feature = "UI_Core")]
impl IFrameworkViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkViewVtbl {
        unsafe extern "system" fn Initialize<Impl: IFrameworkViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationview: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(&*(&applicationview as *const <CoreApplicationView as ::windows::core::Abi>::Abi as *const <CoreApplicationView as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetWindow<Impl: IFrameworkViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWindow(&*(&window as *const <super::super::UI::Core::CoreWindow as ::windows::core::Abi>::Abi as *const <super::super::UI::Core::CoreWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Load<Impl: IFrameworkViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, entrypoint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Load(&*(&entrypoint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Run<Impl: IFrameworkViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run().into()
        }
        unsafe extern "system" fn Uninitialize<Impl: IFrameworkViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFrameworkView>, ::windows::core::GetTrustLevel, Initialize::<Impl, IMPL_OFFSET>, SetWindow::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Run::<Impl, IMPL_OFFSET>, Uninitialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkView as ::windows::core::Interface>::IID
    }
}
pub trait IFrameworkViewSourceImpl: Sized {
    fn CreateView(&self) -> ::windows::core::Result<IFrameworkView>;
}
impl ::windows::core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IFrameworkViewSource";
}
impl IFrameworkViewSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkViewSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkViewSourceVtbl {
        unsafe extern "system" fn CreateView<Impl: IFrameworkViewSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFrameworkViewSource>, ::windows::core::GetTrustLevel, CreateView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkViewSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHostedViewClosingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHostedViewClosingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IHostedViewClosingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHostedViewClosingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostedViewClosingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostedViewClosingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IHostedViewClosingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHostedViewClosingEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostedViewClosingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn Propagate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnhandledError {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IUnhandledError";
}
#[cfg(feature = "implement_exclusive")]
impl IUnhandledErrorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnhandledErrorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnhandledErrorVtbl {
        unsafe extern "system" fn Handled<Impl: IUnhandledErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Propagate<Impl: IUnhandledErrorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Propagate().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUnhandledError>, ::windows::core::GetTrustLevel, Handled::<Impl, IMPL_OFFSET>, Propagate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnhandledError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorDetectedEventArgsImpl: Sized {
    fn UnhandledError(&self) -> ::windows::core::Result<UnhandledError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnhandledErrorDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Core.IUnhandledErrorDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUnhandledErrorDetectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnhandledErrorDetectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnhandledErrorDetectedEventArgsVtbl {
        unsafe extern "system" fn UnhandledError<Impl: IUnhandledErrorDetectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnhandledError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUnhandledErrorDetectedEventArgs>, ::windows::core::GetTrustLevel, UnhandledError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnhandledErrorDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
