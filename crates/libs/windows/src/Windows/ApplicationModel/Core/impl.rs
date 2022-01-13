#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppListEntryImpl: Sized {
    fn DisplayInfo(&mut self) -> ::windows::core::Result<super::AppDisplayInfo>;
    fn LaunchAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppListEntry, BASE_OFFSET>(),
            DisplayInfo: DisplayInfo::<Impl, IMPL_OFFSET>,
            LaunchAsync: LaunchAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry2Impl: Sized {
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppListEntry2, BASE_OFFSET>(), AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IAppListEntry3Impl: Sized {
    fn LaunchForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppListEntry3, BASE_OFFSET>(),
            LaunchForUserAsync: LaunchForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppListEntry4Impl: Sized {
    fn AppInfo(&mut self) -> ::windows::core::Result<super::AppInfo>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppListEntry4, BASE_OFFSET>(), AppInfo: AppInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppListEntry4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreApplicationImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suspending(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::SuspendingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn GetCurrentView(&mut self) -> ::windows::core::Result<CoreApplicationView>;
    fn Run(&mut self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<()>;
    fn RunWithActivationFactories(&mut self, activationfactorycallback: &::core::option::Option<super::super::Foundation::IGetActivationFactory>) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplication, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Suspending: Suspending::<Impl, IMPL_OFFSET>,
            RemoveSuspending: RemoveSuspending::<Impl, IMPL_OFFSET>,
            Resuming: Resuming::<Impl, IMPL_OFFSET>,
            RemoveResuming: RemoveResuming::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            GetCurrentView: GetCurrentView::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            RunWithActivationFactories: RunWithActivationFactories::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplication2Impl: Sized {
    fn BackgroundActivated(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LeavingBackground(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::LeavingBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<super::EnteredBackgroundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnablePrelaunch(&mut self, value: bool) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplication2, BASE_OFFSET>(),
            BackgroundActivated: BackgroundActivated::<Impl, IMPL_OFFSET>,
            RemoveBackgroundActivated: RemoveBackgroundActivated::<Impl, IMPL_OFFSET>,
            LeavingBackground: LeavingBackground::<Impl, IMPL_OFFSET>,
            RemoveLeavingBackground: RemoveLeavingBackground::<Impl, IMPL_OFFSET>,
            EnteredBackground: EnteredBackground::<Impl, IMPL_OFFSET>,
            RemoveEnteredBackground: RemoveEnteredBackground::<Impl, IMPL_OFFSET>,
            EnablePrelaunch: EnablePrelaunch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait ICoreApplication3Impl: Sized {
    fn RequestRestartAsync(&mut self, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
    fn RequestRestartForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRestartFailureReason>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplication3, BASE_OFFSET>(),
            RequestRestartAsync: RequestRestartAsync::<Impl, IMPL_OFFSET>,
            RequestRestartForUserAsync: RequestRestartForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationExitImpl: Sized {
    fn Exit(&mut self) -> ::windows::core::Result<()>;
    fn Exiting(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveExiting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationExit, BASE_OFFSET>(),
            Exit: Exit::<Impl, IMPL_OFFSET>,
            Exiting: Exiting::<Impl, IMPL_OFFSET>,
            RemoveExiting: RemoveExiting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationExit as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICoreApplicationUnhandledErrorImpl: Sized {
    fn UnhandledErrorDetected(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationUnhandledError, BASE_OFFSET>(),
            UnhandledErrorDetected: UnhandledErrorDetected::<Impl, IMPL_OFFSET>,
            RemoveUnhandledErrorDetected: RemoveUnhandledErrorDetected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUnhandledError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreApplicationUseCountImpl: Sized {
    fn IncrementApplicationUseCount(&mut self) -> ::windows::core::Result<()>;
    fn DecrementApplicationUseCount(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationUseCount, BASE_OFFSET>(),
            IncrementApplicationUseCount: IncrementApplicationUseCount::<Impl, IMPL_OFFSET>,
            DecrementApplicationUseCount: DecrementApplicationUseCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationUseCount as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreApplicationViewImpl: Sized {
    fn CoreWindow(&mut self) -> ::windows::core::Result<super::super::UI::Core::CoreWindow>;
    fn Activated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, super::Activation::IActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsMain(&mut self) -> ::windows::core::Result<bool>;
    fn IsHosted(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationView, BASE_OFFSET>(),
            CoreWindow: CoreWindow::<Impl, IMPL_OFFSET>,
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            IsMain: IsMain::<Impl, IMPL_OFFSET>,
            IsHosted: IsHosted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICoreApplicationView2Impl: Sized {
    fn Dispatcher(&mut self) -> ::windows::core::Result<super::super::UI::Core::CoreDispatcher>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationView2, BASE_OFFSET>(), Dispatcher: Dispatcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationView3Impl: Sized {
    fn IsComponent(&mut self) -> ::windows::core::Result<bool>;
    fn TitleBar(&mut self) -> ::windows::core::Result<CoreApplicationViewTitleBar>;
    fn HostedViewClosing(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationView, HostedViewClosingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHostedViewClosing(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationView3, BASE_OFFSET>(),
            IsComponent: IsComponent::<Impl, IMPL_OFFSET>,
            TitleBar: TitleBar::<Impl, IMPL_OFFSET>,
            HostedViewClosing: HostedViewClosing::<Impl, IMPL_OFFSET>,
            RemoveHostedViewClosing: RemoveHostedViewClosing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreApplicationView5Impl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationView5, BASE_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ICoreApplicationView6Impl: Sized {
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationView6, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationView6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreApplicationViewTitleBarImpl: Sized {
    fn SetExtendViewIntoTitleBar(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExtendViewIntoTitleBar(&mut self) -> ::windows::core::Result<bool>;
    fn SystemOverlayLeftInset(&mut self) -> ::windows::core::Result<f64>;
    fn SystemOverlayRightInset(&mut self) -> ::windows::core::Result<f64>;
    fn Height(&mut self) -> ::windows::core::Result<f64>;
    fn LayoutMetricsChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutMetricsChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn IsVisible(&mut self) -> ::windows::core::Result<bool>;
    fn IsVisibleChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreApplicationViewTitleBar, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsVisibleChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreApplicationViewTitleBar, BASE_OFFSET>(),
            SetExtendViewIntoTitleBar: SetExtendViewIntoTitleBar::<Impl, IMPL_OFFSET>,
            ExtendViewIntoTitleBar: ExtendViewIntoTitleBar::<Impl, IMPL_OFFSET>,
            SystemOverlayLeftInset: SystemOverlayLeftInset::<Impl, IMPL_OFFSET>,
            SystemOverlayRightInset: SystemOverlayRightInset::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            LayoutMetricsChanged: LayoutMetricsChanged::<Impl, IMPL_OFFSET>,
            RemoveLayoutMetricsChanged: RemoveLayoutMetricsChanged::<Impl, IMPL_OFFSET>,
            IsVisible: IsVisible::<Impl, IMPL_OFFSET>,
            IsVisibleChanged: IsVisibleChanged::<Impl, IMPL_OFFSET>,
            RemoveIsVisibleChanged: RemoveIsVisibleChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreApplicationViewTitleBar as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICoreImmersiveApplicationImpl: Sized {
    fn Views(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<CoreApplicationView>>;
    fn CreateNewView(&mut self, runtimetype: &::windows::core::HSTRING, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<CoreApplicationView>;
    fn MainView(&mut self) -> ::windows::core::Result<CoreApplicationView>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreImmersiveApplication, BASE_OFFSET>(),
            Views: Views::<Impl, IMPL_OFFSET>,
            CreateNewView: CreateNewView::<Impl, IMPL_OFFSET>,
            MainView: MainView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication2Impl: Sized {
    fn CreateNewViewFromMainView(&mut self) -> ::windows::core::Result<CoreApplicationView>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreImmersiveApplication2, BASE_OFFSET>(),
            CreateNewViewFromMainView: CreateNewViewFromMainView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreImmersiveApplication3Impl: Sized {
    fn CreateNewViewWithViewSource(&mut self, viewsource: &::core::option::Option<IFrameworkViewSource>) -> ::windows::core::Result<CoreApplicationView>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreImmersiveApplication3, BASE_OFFSET>(),
            CreateNewViewWithViewSource: CreateNewViewWithViewSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreImmersiveApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Core")]
pub trait IFrameworkViewImpl: Sized {
    fn Initialize(&mut self, applicationview: &::core::option::Option<CoreApplicationView>) -> ::windows::core::Result<()>;
    fn SetWindow(&mut self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<()>;
    fn Load(&mut self, entrypoint: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Run(&mut self) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkView, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            SetWindow: SetWindow::<Impl, IMPL_OFFSET>,
            Load: Load::<Impl, IMPL_OFFSET>,
            Run: Run::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkView as ::windows::core::Interface>::IID
    }
}
pub trait IFrameworkViewSourceImpl: Sized {
    fn CreateView(&mut self) -> ::windows::core::Result<IFrameworkView>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkViewSource, BASE_OFFSET>(), CreateView: CreateView::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkViewSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHostedViewClosingEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHostedViewClosingEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostedViewClosingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorImpl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn Propagate(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnhandledError, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            Propagate: Propagate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnhandledError as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledErrorDetectedEventArgsImpl: Sized {
    fn UnhandledError(&mut self) -> ::windows::core::Result<UnhandledError>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnhandledErrorDetectedEventArgs, BASE_OFFSET>(),
            UnhandledError: UnhandledError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnhandledErrorDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
