#[cfg(feature = "implement_exclusive")]
pub trait IAppActivationResult_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn AppResourceGroupInfo(&mut self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppActivationResult {
    const NAME: &'static str = "Windows.System.IAppActivationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAppActivationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppActivationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppActivationResult_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppActivationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppActivationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppActivationResult, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            AppResourceGroupInfo: AppResourceGroupInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppActivationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfo_Impl: Sized {
    fn AppInfo(&mut self) -> ::windows::core::Result<super::ApplicationModel::AppInfo>;
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo";
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl IAppDiagnosticInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfo_Vtbl {
        unsafe extern "system" fn AppInfo<Impl: IAppDiagnosticInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfo, BASE_OFFSET>(), AppInfo: AppInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfo2_Impl: Sized {
    fn GetResourceGroups(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>>;
    fn CreateResourceGroupWatcher(&mut self) -> ::windows::core::Result<AppResourceGroupInfoWatcher>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppDiagnosticInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfo2_Vtbl {
        unsafe extern "system" fn GetResourceGroups<Impl: IAppDiagnosticInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceGroupWatcher<Impl: IAppDiagnosticInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceGroupWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfo2, BASE_OFFSET>(),
            GetResourceGroups: GetResourceGroups::<Impl, IMPL_OFFSET>,
            CreateResourceGroupWatcher: CreateResourceGroupWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfo3_Impl: Sized {
    fn LaunchAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo3 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfo3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfo3_Vtbl {
        unsafe extern "system" fn LaunchAsync<Impl: IAppDiagnosticInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfo3, BASE_OFFSET>(), LaunchAsync: LaunchAsync::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoStatics_Impl: Sized {
    fn RequestInfoAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoStatics_Vtbl {
        unsafe extern "system" fn RequestInfoAsync<Impl: IAppDiagnosticInfoStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfoStatics, BASE_OFFSET>(),
            RequestInfoAsync: RequestInfoAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoStatics2_Impl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<AppDiagnosticInfoWatcher>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>;
    fn RequestInfoForPackageAsync(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppUserModelId(&mut self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoStatics2_Vtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IAppDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IAppDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForPackageAsync<Impl: IAppDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInfoForPackageAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForAppAsync<Impl: IAppDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInfoForAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForAppUserModelId<Impl: IAppDiagnosticInfoStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInfoForAppUserModelId(&*(&appusermodelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfoStatics2, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForPackageAsync: RequestInfoForPackageAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForAppAsync: RequestInfoForAppAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForAppUserModelId: RequestInfoForAppUserModelId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoWatcher_Impl: Sized {
    fn Added(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoWatcher_Vtbl {
        unsafe extern "system" fn Added<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppDiagnosticInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfoWatcher, BASE_OFFSET>(),
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoWatcherEventArgs_Impl: Sized {
    fn AppDiagnosticInfo(&mut self) -> ::windows::core::Result<AppDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoWatcherEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoWatcherEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoWatcherEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoWatcherEventArgs_Vtbl {
        unsafe extern "system" fn AppDiagnosticInfo<Impl: IAppDiagnosticInfoWatcherEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDiagnosticInfoWatcherEventArgs, BASE_OFFSET>(),
            AppDiagnosticInfo: AppDiagnosticInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoWatcherEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExecutionStateChangeResult_Impl: Sized {
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.IAppExecutionStateChangeResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExecutionStateChangeResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExecutionStateChangeResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExecutionStateChangeResult_Vtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppExecutionStateChangeResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExecutionStateChangeResult, BASE_OFFSET>(),
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExecutionStateChangeResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReport_Impl: Sized {
    fn PrivateCommitUsage(&mut self) -> ::windows::core::Result<u64>;
    fn PeakPrivateCommitUsage(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCommitLimit(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryReport {
    const NAME: &'static str = "Windows.System.IAppMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryReport_Vtbl {
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPrivateCommitUsage<Impl: IAppMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeakPrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitLimit<Impl: IAppMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCommitLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppMemoryReport, BASE_OFFSET>(),
            PrivateCommitUsage: PrivateCommitUsage::<Impl, IMPL_OFFSET>,
            PeakPrivateCommitUsage: PeakPrivateCommitUsage::<Impl, IMPL_OFFSET>,
            TotalCommitUsage: TotalCommitUsage::<Impl, IMPL_OFFSET>,
            TotalCommitLimit: TotalCommitLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReport2_Impl: Sized {
    fn ExpectedTotalCommitLimit(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryReport2 {
    const NAME: &'static str = "Windows.System.IAppMemoryReport2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryReport2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryReport2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryReport2_Vtbl {
        unsafe extern "system" fn ExpectedTotalCommitLimit<Impl: IAppMemoryReport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedTotalCommitLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppMemoryReport2, BASE_OFFSET>(),
            ExpectedTotalCommitLimit: ExpectedTotalCommitLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryReport2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryUsageLimitChangingEventArgs_Impl: Sized {
    fn OldLimit(&mut self) -> ::windows::core::Result<u64>;
    fn NewLimit(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.IAppMemoryUsageLimitChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryUsageLimitChangingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryUsageLimitChangingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryUsageLimitChangingEventArgs_Vtbl {
        unsafe extern "system" fn OldLimit<Impl: IAppMemoryUsageLimitChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewLimit<Impl: IAppMemoryUsageLimitChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppMemoryUsageLimitChangingEventArgs, BASE_OFFSET>(),
            OldLimit: OldLimit::<Impl, IMPL_OFFSET>,
            NewLimit: NewLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryUsageLimitChangingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupBackgroundTaskReport_Impl: Sized {
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trigger(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EntryPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupBackgroundTaskReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupBackgroundTaskReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupBackgroundTaskReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupBackgroundTaskReport_Vtbl {
        unsafe extern "system" fn TaskId<Impl: IAppResourceGroupBackgroundTaskReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAppResourceGroupBackgroundTaskReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trigger<Impl: IAppResourceGroupBackgroundTaskReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryPoint<Impl: IAppResourceGroupBackgroundTaskReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupBackgroundTaskReport, BASE_OFFSET>(),
            TaskId: TaskId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Trigger: Trigger::<Impl, IMPL_OFFSET>,
            EntryPoint: EntryPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupBackgroundTaskReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfo_Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsShared(&mut self) -> ::windows::core::Result<bool>;
    fn GetBackgroundTaskReports(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>;
    fn GetMemoryReport(&mut self) -> ::windows::core::Result<AppResourceGroupMemoryReport>;
    fn GetProcessDiagnosticInfos(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>;
    fn GetStateReport(&mut self) -> ::windows::core::Result<AppResourceGroupStateReport>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfo_Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsShared() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackgroundTaskReports<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackgroundTaskReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemoryReport<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessDiagnosticInfos<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateReport<Impl: IAppResourceGroupInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStateReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupInfo, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            IsShared: IsShared::<Impl, IMPL_OFFSET>,
            GetBackgroundTaskReports: GetBackgroundTaskReports::<Impl, IMPL_OFFSET>,
            GetMemoryReport: GetMemoryReport::<Impl, IMPL_OFFSET>,
            GetProcessDiagnosticInfos: GetProcessDiagnosticInfos::<Impl, IMPL_OFFSET>,
            GetStateReport: GetStateReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfo2_Impl: Sized {
    fn StartSuspendAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartResumeAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartTerminateAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo2 {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppResourceGroupInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfo2_Vtbl {
        unsafe extern "system" fn StartSuspendAsync<Impl: IAppResourceGroupInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartSuspendAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartResumeAsync<Impl: IAppResourceGroupInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTerminateAsync<Impl: IAppResourceGroupInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTerminateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupInfo2, BASE_OFFSET>(),
            StartSuspendAsync: StartSuspendAsync::<Impl, IMPL_OFFSET>,
            StartResumeAsync: StartResumeAsync::<Impl, IMPL_OFFSET>,
            StartTerminateAsync: StartTerminateAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcher_Impl: Sized {
    fn Added(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExecutionStateChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveExecutionStateChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcher_Vtbl {
        unsafe extern "system" fn Added<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecutionStateChanged<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutionStateChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveExecutionStateChanged<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveExecutionStateChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppResourceGroupInfoWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupInfoWatcher, BASE_OFFSET>(),
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
            ExecutionStateChanged: ExecutionStateChanged::<Impl, IMPL_OFFSET>,
            RemoveExecutionStateChanged: RemoveExecutionStateChanged::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcherEventArgs_Impl: Sized {
    fn AppDiagnosticInfos(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&mut self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcherEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcherEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcherEventArgs_Vtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupInfoWatcherEventArgs, BASE_OFFSET>(),
            AppDiagnosticInfos: AppDiagnosticInfos::<Impl, IMPL_OFFSET>,
            AppResourceGroupInfo: AppResourceGroupInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcherEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Impl: Sized {
    fn AppDiagnosticInfos(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&mut self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs, BASE_OFFSET>(),
            AppDiagnosticInfos: AppDiagnosticInfos::<Impl, IMPL_OFFSET>,
            AppResourceGroupInfo: AppResourceGroupInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupMemoryReport_Impl: Sized {
    fn CommitUsageLimit(&mut self) -> ::windows::core::Result<u64>;
    fn CommitUsageLevel(&mut self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn PrivateCommitUsage(&mut self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupMemoryReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupMemoryReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupMemoryReport_Vtbl {
        unsafe extern "system" fn CommitUsageLimit<Impl: IAppResourceGroupMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitUsageLevel<Impl: IAppResourceGroupMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppResourceGroupMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppResourceGroupMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupMemoryReport, BASE_OFFSET>(),
            CommitUsageLimit: CommitUsageLimit::<Impl, IMPL_OFFSET>,
            CommitUsageLevel: CommitUsageLevel::<Impl, IMPL_OFFSET>,
            PrivateCommitUsage: PrivateCommitUsage::<Impl, IMPL_OFFSET>,
            TotalCommitUsage: TotalCommitUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupStateReport_Impl: Sized {
    fn ExecutionState(&mut self) -> ::windows::core::Result<AppResourceGroupExecutionState>;
    fn EnergyQuotaState(&mut self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupStateReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupStateReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupStateReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupStateReport_Vtbl {
        unsafe extern "system" fn ExecutionState<Impl: IAppResourceGroupStateReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnergyQuotaState<Impl: IAppResourceGroupStateReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnergyQuotaState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppResourceGroupStateReport, BASE_OFFSET>(),
            ExecutionState: ExecutionState::<Impl, IMPL_OFFSET>,
            EnergyQuotaState: EnergyQuotaState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupStateReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHost_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHost {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHost";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHost_Vtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAppUriHandlerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerHost, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHost2_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHost2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHost2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHost2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHost2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHost2_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAppUriHandlerHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IAppUriHandlerHost2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerHost2, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHost2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHostFactory_Impl: Sized {
    fn CreateInstance(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerHost>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHostFactory {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHostFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHostFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHostFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHostFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAppUriHandlerHostFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerHostFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHostFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppUriHandlerRegistration_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&mut self) -> ::windows::core::Result<User>;
    fn GetAppAddedHostsAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>;
    fn SetAppAddedHostsAsync(&mut self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppUriHandlerRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistration_Vtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppAddedHostsAsync<Impl: IAppUriHandlerRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppAddedHostsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppAddedHostsAsync<Impl: IAppUriHandlerRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAppAddedHostsAsync(&*(&hosts as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistration, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            GetAppAddedHostsAsync: GetAppAddedHostsAsync::<Impl, IMPL_OFFSET>,
            SetAppAddedHostsAsync: SetAppAddedHostsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppUriHandlerRegistration2_Impl: Sized {
    fn GetAllHosts(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>>;
    fn UpdateHosts(&mut self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppUriHandlerRegistration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistration2_Vtbl {
        unsafe extern "system" fn GetAllHosts<Impl: IAppUriHandlerRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllHosts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateHosts<Impl: IAppUriHandlerRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateHosts(&*(&hosts as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistration2, BASE_OFFSET>(),
            GetAllHosts: GetAllHosts::<Impl, IMPL_OFFSET>,
            UpdateHosts: UpdateHosts::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManager_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<User>;
    fn TryGetRegistration(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManager_Vtbl {
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistrationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetRegistration<Impl: IAppUriHandlerRegistrationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetRegistration(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistrationManager, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            TryGetRegistration: TryGetRegistration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManager2_Impl: Sized {
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManager2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManager2_Vtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistrationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistrationManager2, BASE_OFFSET>(),
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForUser(&mut self, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManagerStatics {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppUriHandlerRegistrationManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IAppUriHandlerRegistrationManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistrationManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStatics2_Impl: Sized {
    fn GetForPackage(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForPackageForUser(&mut self, packagefamilyname: &::windows::core::HSTRING, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManagerStatics2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManagerStatics2_Vtbl {
        unsafe extern "system" fn GetForPackage<Impl: IAppUriHandlerRegistrationManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForPackage(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForPackageForUser<Impl: IAppUriHandlerRegistrationManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForPackageForUser(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUriHandlerRegistrationManagerStatics2, BASE_OFFSET>(),
            GetForPackage: GetForPackage::<Impl, IMPL_OFFSET>,
            GetForPackageForUser: GetForPackageForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDateTimeSettingsStatics_Impl: Sized {
    fn SetSystemDateTime(&mut self, utcdatetime: &super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDateTimeSettingsStatics {
    const NAME: &'static str = "Windows.System.IDateTimeSettingsStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDateTimeSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDateTimeSettingsStatics_Vtbl {
        unsafe extern "system" fn SetSystemDateTime<Impl: IDateTimeSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemDateTime(&*(&utcdatetime as *const <super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDateTimeSettingsStatics, BASE_OFFSET>(),
            SetSystemDateTime: SetSystemDateTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDateTimeSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueue_Impl: Sized {
    fn CreateTimer(&mut self) -> ::windows::core::Result<DispatcherQueueTimer>;
    fn TryEnqueue(&mut self, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn TryEnqueueWithPriority(&mut self, priority: DispatcherQueuePriority, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn ShutdownStarting(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownStarting(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShutdownCompleted(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownCompleted(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueue {
    const NAME: &'static str = "Windows.System.IDispatcherQueue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueue_Vtbl {
        unsafe extern "system" fn CreateTimer<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTimer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnqueue<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnqueue(&*(&callback as *const <DispatcherQueueHandler as ::windows::core::Abi>::Abi as *const <DispatcherQueueHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnqueueWithPriority<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryEnqueueWithPriority(priority, &*(&callback as *const <DispatcherQueueHandler as ::windows::core::Abi>::Abi as *const <DispatcherQueueHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownStarting<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownStarting(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShutdownStarting<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShutdownStarting(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShutdownCompleted<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShutdownCompleted<Impl: IDispatcherQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShutdownCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueue, BASE_OFFSET>(),
            CreateTimer: CreateTimer::<Impl, IMPL_OFFSET>,
            TryEnqueue: TryEnqueue::<Impl, IMPL_OFFSET>,
            TryEnqueueWithPriority: TryEnqueueWithPriority::<Impl, IMPL_OFFSET>,
            ShutdownStarting: ShutdownStarting::<Impl, IMPL_OFFSET>,
            RemoveShutdownStarting: RemoveShutdownStarting::<Impl, IMPL_OFFSET>,
            ShutdownCompleted: ShutdownCompleted::<Impl, IMPL_OFFSET>,
            RemoveShutdownCompleted: RemoveShutdownCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueue2_Impl: Sized {
    fn HasThreadAccess(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueue2 {
    const NAME: &'static str = "Windows.System.IDispatcherQueue2";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueue2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueue2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueue2_Vtbl {
        unsafe extern "system" fn HasThreadAccess<Impl: IDispatcherQueue2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasThreadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueue2, BASE_OFFSET>(),
            HasThreadAccess: HasThreadAccess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueController_Impl: Sized {
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<DispatcherQueue>;
    fn ShutdownQueueAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueController {
    const NAME: &'static str = "Windows.System.IDispatcherQueueController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueController_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueController_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: IDispatcherQueueController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShutdownQueueAsync<Impl: IDispatcherQueueController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownQueueAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueueController, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
            ShutdownQueueAsync: ShutdownQueueAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueControllerStatics_Impl: Sized {
    fn CreateOnDedicatedThread(&mut self) -> ::windows::core::Result<DispatcherQueueController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueControllerStatics {
    const NAME: &'static str = "Windows.System.IDispatcherQueueControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueControllerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueControllerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueControllerStatics_Vtbl {
        unsafe extern "system" fn CreateOnDedicatedThread<Impl: IDispatcherQueueControllerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOnDedicatedThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueueControllerStatics, BASE_OFFSET>(),
            CreateOnDedicatedThread: CreateOnDedicatedThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueShutdownStartingEventArgs_Impl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.IDispatcherQueueShutdownStartingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueShutdownStartingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueShutdownStartingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueShutdownStartingEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Impl: IDispatcherQueueShutdownStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueueShutdownStartingEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueShutdownStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueStatics_Impl: Sized {
    fn GetForCurrentThread(&mut self) -> ::windows::core::Result<DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueStatics {
    const NAME: &'static str = "Windows.System.IDispatcherQueueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentThread<Impl: IDispatcherQueueStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueueStatics, BASE_OFFSET>(),
            GetForCurrentThread: GetForCurrentThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueTimer_Impl: Sized {
    fn Interval(&mut self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetInterval(&mut self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsRunning(&mut self) -> ::windows::core::Result<bool>;
    fn IsRepeating(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRepeating(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Tick(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.IDispatcherQueueTimer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueTimer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueTimer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueTimer_Vtbl {
        unsafe extern "system" fn Interval<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRunning<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRepeating<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRepeating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRepeating<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRepeating(value).into()
        }
        unsafe extern "system" fn Start<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Tick<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tick(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTick<Impl: IDispatcherQueueTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTick(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherQueueTimer, BASE_OFFSET>(),
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            IsRunning: IsRunning::<Impl, IMPL_OFFSET>,
            IsRepeating: IsRepeating::<Impl, IMPL_OFFSET>,
            SetIsRepeating: SetIsRepeating::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Tick: Tick::<Impl, IMPL_OFFSET>,
            RemoveTick: RemoveTick::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueTimer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IFolderLauncherOptions_Impl: Sized {
    fn ItemsToSelect(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderLauncherOptions {
    const NAME: &'static str = "Windows.System.IFolderLauncherOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IFolderLauncherOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderLauncherOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderLauncherOptions_Vtbl {
        unsafe extern "system" fn ItemsToSelect<Impl: IFolderLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsToSelect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFolderLauncherOptions, BASE_OFFSET>(),
            ItemsToSelect: ItemsToSelect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStatics_Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccountName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GuestHost(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrincipalName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionInitiationProtocolUri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownUserPropertiesStatics {
    const NAME: &'static str = "Windows.System.IKnownUserPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownUserPropertiesStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUserPropertiesStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUserPropertiesStatics_Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GuestHost<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GuestHost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrincipalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionInitiationProtocolUri<Impl: IKnownUserPropertiesStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInitiationProtocolUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownUserPropertiesStatics, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            FirstName: FirstName::<Impl, IMPL_OFFSET>,
            LastName: LastName::<Impl, IMPL_OFFSET>,
            ProviderName: ProviderName::<Impl, IMPL_OFFSET>,
            AccountName: AccountName::<Impl, IMPL_OFFSET>,
            GuestHost: GuestHost::<Impl, IMPL_OFFSET>,
            PrincipalName: PrincipalName::<Impl, IMPL_OFFSET>,
            DomainName: DomainName::<Impl, IMPL_OFFSET>,
            SessionInitiationProtocolUri: SessionInitiationProtocolUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUserPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStatics2_Impl: Sized {
    fn AgeEnforcementRegion(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownUserPropertiesStatics2 {
    const NAME: &'static str = "Windows.System.IKnownUserPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownUserPropertiesStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUserPropertiesStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUserPropertiesStatics2_Vtbl {
        unsafe extern "system" fn AgeEnforcementRegion<Impl: IKnownUserPropertiesStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgeEnforcementRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownUserPropertiesStatics2, BASE_OFFSET>(),
            AgeEnforcementRegion: AgeEnforcementRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUserPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILaunchUriResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<LaunchUriStatus>;
    fn Result(&mut self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILaunchUriResult {
    const NAME: &'static str = "Windows.System.ILaunchUriResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILaunchUriResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchUriResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILaunchUriResult_Vtbl {
        unsafe extern "system" fn Status<Impl: ILaunchUriResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ILaunchUriResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILaunchUriResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Result: Result::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchUriResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILauncherOptions_Impl: Sized {
    fn TreatAsUntrusted(&mut self) -> ::windows::core::Result<bool>;
    fn SetTreatAsUntrusted(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayApplicationPicker(&mut self) -> ::windows::core::Result<bool>;
    fn SetDisplayApplicationPicker(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn UI(&mut self) -> ::windows::core::Result<LauncherUIOptions>;
    fn PreferredApplicationPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PreferredApplicationDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&mut self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&mut self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherOptions {
    const NAME: &'static str = "Windows.System.ILauncherOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILauncherOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions_Vtbl {
        unsafe extern "system" fn TreatAsUntrusted<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TreatAsUntrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreatAsUntrusted<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTreatAsUntrusted(value).into()
        }
        unsafe extern "system" fn DisplayApplicationPicker<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayApplicationPicker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayApplicationPicker<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayApplicationPicker(value).into()
        }
        unsafe extern "system" fn UI<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredApplicationPackageFamilyName<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredApplicationPackageFamilyName<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredApplicationDisplayName<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredApplicationDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredApplicationDisplayName<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FallbackUri<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackUri<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: ILauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherOptions, BASE_OFFSET>(),
            TreatAsUntrusted: TreatAsUntrusted::<Impl, IMPL_OFFSET>,
            SetTreatAsUntrusted: SetTreatAsUntrusted::<Impl, IMPL_OFFSET>,
            DisplayApplicationPicker: DisplayApplicationPicker::<Impl, IMPL_OFFSET>,
            SetDisplayApplicationPicker: SetDisplayApplicationPicker::<Impl, IMPL_OFFSET>,
            UI: UI::<Impl, IMPL_OFFSET>,
            PreferredApplicationPackageFamilyName: PreferredApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetPreferredApplicationPackageFamilyName: SetPreferredApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            PreferredApplicationDisplayName: PreferredApplicationDisplayName::<Impl, IMPL_OFFSET>,
            SetPreferredApplicationDisplayName: SetPreferredApplicationDisplayName::<Impl, IMPL_OFFSET>,
            FallbackUri: FallbackUri::<Impl, IMPL_OFFSET>,
            SetFallbackUri: SetFallbackUri::<Impl, IMPL_OFFSET>,
            ContentType: ContentType::<Impl, IMPL_OFFSET>,
            SetContentType: SetContentType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
pub trait ILauncherOptions2_Impl: Sized {
    fn TargetApplicationPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetApplicationPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NeighboringFilesQuery(&mut self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult>;
    fn SetNeighboringFilesQuery(&mut self, value: &::core::option::Option<super::Storage::Search::StorageFileQueryResult>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherOptions2 {
    const NAME: &'static str = "Windows.System.ILauncherOptions2";
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
impl ILauncherOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions2_Vtbl {
        unsafe extern "system" fn TargetApplicationPackageFamilyName<Impl: ILauncherOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetApplicationPackageFamilyName<Impl: ILauncherOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NeighboringFilesQuery<Impl: ILauncherOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeighboringFilesQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeighboringFilesQuery<Impl: ILauncherOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNeighboringFilesQuery(&*(&value as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::Abi>::Abi as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherOptions2, BASE_OFFSET>(),
            TargetApplicationPackageFamilyName: TargetApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetTargetApplicationPackageFamilyName: SetTargetApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            NeighboringFilesQuery: NeighboringFilesQuery::<Impl, IMPL_OFFSET>,
            SetNeighboringFilesQuery: SetNeighboringFilesQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions3_Impl: Sized {
    fn IgnoreAppUriHandlers(&mut self) -> ::windows::core::Result<bool>;
    fn SetIgnoreAppUriHandlers(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions3 {
    const NAME: &'static str = "Windows.System.ILauncherOptions3";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptions3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions3_Vtbl {
        unsafe extern "system" fn IgnoreAppUriHandlers<Impl: ILauncherOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnoreAppUriHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreAppUriHandlers<Impl: ILauncherOptions3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreAppUriHandlers(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherOptions3, BASE_OFFSET>(),
            IgnoreAppUriHandlers: IgnoreAppUriHandlers::<Impl, IMPL_OFFSET>,
            SetIgnoreAppUriHandlers: SetIgnoreAppUriHandlers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions4_Impl: Sized {
    fn LimitPickerToCurrentAppAndAppUriHandlers(&mut self) -> ::windows::core::Result<bool>;
    fn SetLimitPickerToCurrentAppAndAppUriHandlers(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions4 {
    const NAME: &'static str = "Windows.System.ILauncherOptions4";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptions4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions4_Vtbl {
        unsafe extern "system" fn LimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LimitPickerToCurrentAppAndAppUriHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLimitPickerToCurrentAppAndAppUriHandlers(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherOptions4, BASE_OFFSET>(),
            LimitPickerToCurrentAppAndAppUriHandlers: LimitPickerToCurrentAppAndAppUriHandlers::<Impl, IMPL_OFFSET>,
            SetLimitPickerToCurrentAppAndAppUriHandlers: SetLimitPickerToCurrentAppAndAppUriHandlers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStatics_Impl: Sized {
    fn LaunchFileAsync(&mut self, file: &::core::option::Option<super::Storage::IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFileWithOptionsAsync(&mut self, file: &::core::option::Option<super::Storage::IStorageFile>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriWithOptionsAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics {
    const NAME: &'static str = "Windows.System.ILauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics_Vtbl {
        unsafe extern "system" fn LaunchFileAsync<Impl: ILauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFileAsync(&*(&file as *const <super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFileWithOptionsAsync<Impl: ILauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFileWithOptionsAsync(&*(&file as *const <super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriAsync<Impl: ILauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: ILauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithOptionsAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherStatics, BASE_OFFSET>(),
            LaunchFileAsync: LaunchFileAsync::<Impl, IMPL_OFFSET>,
            LaunchFileWithOptionsAsync: LaunchFileWithOptionsAsync::<Impl, IMPL_OFFSET>,
            LaunchUriAsync: LaunchUriAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithOptionsAsync: LaunchUriWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStatics2_Impl: Sized {
    fn LaunchUriForResultsAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriWithDataAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn QueryUriSupportAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryUriSupportWithPackageFamilyNameAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportAsync(&mut self, file: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportWithPackageFamilyNameAsync(&mut self, file: &::core::option::Option<super::Storage::StorageFile>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindUriSchemeHandlersAsync(&mut self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindUriSchemeHandlersWithLaunchUriTypeAsync(&mut self, scheme: &::windows::core::HSTRING, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindFileHandlersAsync(&mut self, extension: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics2 {
    const NAME: &'static str = "Windows.System.ILauncherStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics2_Vtbl {
        unsafe extern "system" fn LaunchUriForResultsAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForResultsWithDataAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsWithDataAsync(
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdata as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithDataAsync(
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdata as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryUriSupportAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryUriSupportAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryUriSupportWithPackageFamilyNameAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype, &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFileSupportAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryFileSupportAsync(&*(&file as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFileSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryFileSupportWithPackageFamilyNameAsync(&*(&file as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUriSchemeHandlersAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUriSchemeHandlersAsync(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUriSchemeHandlersWithLaunchUriTypeAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUriSchemeHandlersWithLaunchUriTypeAsync(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFileHandlersAsync<Impl: ILauncherStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFileHandlersAsync(&*(&extension as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherStatics2, BASE_OFFSET>(),
            LaunchUriForResultsAsync: LaunchUriForResultsAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsWithDataAsync: LaunchUriForResultsWithDataAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithDataAsync: LaunchUriWithDataAsync::<Impl, IMPL_OFFSET>,
            QueryUriSupportAsync: QueryUriSupportAsync::<Impl, IMPL_OFFSET>,
            QueryUriSupportWithPackageFamilyNameAsync: QueryUriSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            QueryFileSupportAsync: QueryFileSupportAsync::<Impl, IMPL_OFFSET>,
            QueryFileSupportWithPackageFamilyNameAsync: QueryFileSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            FindUriSchemeHandlersAsync: FindUriSchemeHandlersAsync::<Impl, IMPL_OFFSET>,
            FindUriSchemeHandlersWithLaunchUriTypeAsync: FindUriSchemeHandlersWithLaunchUriTypeAsync::<Impl, IMPL_OFFSET>,
            FindFileHandlersAsync: FindFileHandlersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStatics3_Impl: Sized {
    fn LaunchFolderAsync(&mut self, folder: &::core::option::Option<super::Storage::IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderWithOptionsAsync(&mut self, folder: &::core::option::Option<super::Storage::IStorageFolder>, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics3 {
    const NAME: &'static str = "Windows.System.ILauncherStatics3";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics3_Vtbl {
        unsafe extern "system" fn LaunchFolderAsync<Impl: ILauncherStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderAsync(&*(&folder as *const <super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderWithOptionsAsync<Impl: ILauncherStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderWithOptionsAsync(&*(&folder as *const <super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherStatics3, BASE_OFFSET>(),
            LaunchFolderAsync: LaunchFolderAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderWithOptionsAsync: LaunchFolderWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILauncherStatics4_Impl: Sized {
    fn QueryAppUriSupportAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryAppUriSupportWithPackageFamilyNameAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindAppUriHandlersAsync(&mut self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn LaunchUriForUserAsync(&mut self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithOptionsForUserAsync(&mut self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithDataForUserAsync(&mut self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriForResultsForUserAsync(&mut self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataForUserAsync(&mut self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics4 {
    const NAME: &'static str = "Windows.System.ILauncherStatics4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILauncherStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics4_Vtbl {
        unsafe extern "system" fn QueryAppUriSupportAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAppUriSupportAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAppUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryAppUriSupportWithPackageFamilyNameAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppUriHandlersAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppUriHandlersAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForUserAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsForUserAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithOptionsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithDataForUserAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithDataForUserAsync(
                &*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdata as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForResultsForUserAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForResultsWithDataForUserAsync<Impl: ILauncherStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsWithDataForUserAsync(
                &*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdata as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherStatics4, BASE_OFFSET>(),
            QueryAppUriSupportAsync: QueryAppUriSupportAsync::<Impl, IMPL_OFFSET>,
            QueryAppUriSupportWithPackageFamilyNameAsync: QueryAppUriSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            FindAppUriHandlersAsync: FindAppUriHandlersAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForUserAsync: LaunchUriForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithOptionsForUserAsync: LaunchUriWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithDataForUserAsync: LaunchUriWithDataForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsForUserAsync: LaunchUriForResultsForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsWithDataForUserAsync: LaunchUriForResultsWithDataForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILauncherStatics5_Impl: Sized {
    fn LaunchFolderPathAsync(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsAsync(&mut self, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathForUserAsync(&mut self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsForUserAsync(&mut self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics5 {
    const NAME: &'static str = "Windows.System.ILauncherStatics5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILauncherStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics5_Vtbl {
        unsafe extern "system" fn LaunchFolderPathAsync<Impl: ILauncherStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathWithOptionsAsync<Impl: ILauncherStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathWithOptionsAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathForUserAsync<Impl: ILauncherStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathWithOptionsForUserAsync<Impl: ILauncherStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathWithOptionsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherStatics5, BASE_OFFSET>(),
            LaunchFolderPathAsync: LaunchFolderPathAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathWithOptionsAsync: LaunchFolderPathWithOptionsAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathForUserAsync: LaunchFolderPathForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathWithOptionsForUserAsync: LaunchFolderPathWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ILauncherUIOptions_Impl: Sized {
    fn InvocationPoint(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>>;
    fn SetInvocationPoint(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn SelectionRect(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>>;
    fn SetSelectionRect(&mut self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn PreferredPlacement(&mut self) -> ::windows::core::Result<super::UI::Popups::Placement>;
    fn SetPreferredPlacement(&mut self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherUIOptions {
    const NAME: &'static str = "Windows.System.ILauncherUIOptions";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ILauncherUIOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherUIOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherUIOptions_Vtbl {
        unsafe extern "system" fn InvocationPoint<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvocationPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInvocationPoint<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInvocationPoint(&*(&value as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionRect<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionRect<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionRect(&*(&value as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredPlacement<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredPlacement<Impl: ILauncherUIOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredPlacement(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherUIOptions, BASE_OFFSET>(),
            InvocationPoint: InvocationPoint::<Impl, IMPL_OFFSET>,
            SetInvocationPoint: SetInvocationPoint::<Impl, IMPL_OFFSET>,
            SelectionRect: SelectionRect::<Impl, IMPL_OFFSET>,
            SetSelectionRect: SetSelectionRect::<Impl, IMPL_OFFSET>,
            PreferredPlacement: PreferredPlacement::<Impl, IMPL_OFFSET>,
            SetPreferredPlacement: SetPreferredPlacement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherUIOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_ViewManagement")]
pub trait ILauncherViewOptions_Impl: Sized {
    fn DesiredRemainingView(&mut self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&mut self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows::core::RuntimeName for ILauncherViewOptions {
    const NAME: &'static str = "Windows.System.ILauncherViewOptions";
}
#[cfg(feature = "UI_ViewManagement")]
impl ILauncherViewOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherViewOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherViewOptions_Vtbl {
        unsafe extern "system" fn DesiredRemainingView<Impl: ILauncherViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredRemainingView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRemainingView<Impl: ILauncherViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredRemainingView(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILauncherViewOptions, BASE_OFFSET>(),
            DesiredRemainingView: DesiredRemainingView::<Impl, IMPL_OFFSET>,
            SetDesiredRemainingView: SetDesiredRemainingView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherViewOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMemoryManagerStatics_Impl: Sized {
    fn AppMemoryUsage(&mut self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLimit(&mut self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLevel(&mut self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn AppMemoryUsageIncreased(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageIncreased(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageDecreased(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageDecreased(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageLimitChanging(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageLimitChanging(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMemoryManagerStatics {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMemoryManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics_Vtbl {
        unsafe extern "system" fn AppMemoryUsage<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageLimit<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageLevel<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageIncreased<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageIncreased(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageIncreased<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageIncreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageDecreased<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageDecreased(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageDecreased<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageDecreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageLimitChanging<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLimitChanging(&*(&handler as *const <super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageLimitChanging<Impl: IMemoryManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageLimitChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryManagerStatics, BASE_OFFSET>(),
            AppMemoryUsage: AppMemoryUsage::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLimit: AppMemoryUsageLimit::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLevel: AppMemoryUsageLevel::<Impl, IMPL_OFFSET>,
            AppMemoryUsageIncreased: AppMemoryUsageIncreased::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageIncreased: RemoveAppMemoryUsageIncreased::<Impl, IMPL_OFFSET>,
            AppMemoryUsageDecreased: AppMemoryUsageDecreased::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageDecreased: RemoveAppMemoryUsageDecreased::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLimitChanging: AppMemoryUsageLimitChanging::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageLimitChanging: RemoveAppMemoryUsageLimitChanging::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics2_Impl: Sized {
    fn GetAppMemoryReport(&mut self) -> ::windows::core::Result<AppMemoryReport>;
    fn GetProcessMemoryReport(&mut self) -> ::windows::core::Result<ProcessMemoryReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics2 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics2_Vtbl {
        unsafe extern "system" fn GetAppMemoryReport<Impl: IMemoryManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessMemoryReport<Impl: IMemoryManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryManagerStatics2, BASE_OFFSET>(),
            GetAppMemoryReport: GetAppMemoryReport::<Impl, IMPL_OFFSET>,
            GetProcessMemoryReport: GetProcessMemoryReport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics3_Impl: Sized {
    fn TrySetAppMemoryUsageLimit(&mut self, value: u64) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics3 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics3_Vtbl {
        unsafe extern "system" fn TrySetAppMemoryUsageLimit<Impl: IMemoryManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetAppMemoryUsageLimit(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryManagerStatics3, BASE_OFFSET>(),
            TrySetAppMemoryUsageLimit: TrySetAppMemoryUsageLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics4_Impl: Sized {
    fn ExpectedAppMemoryUsageLimit(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics4 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics4_Vtbl {
        unsafe extern "system" fn ExpectedAppMemoryUsageLimit<Impl: IMemoryManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedAppMemoryUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryManagerStatics4, BASE_OFFSET>(),
            ExpectedAppMemoryUsageLimit: ExpectedAppMemoryUsageLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IProcessLauncherOptions_Impl: Sized {
    fn StandardInput(&mut self) -> ::windows::core::Result<super::Storage::Streams::IInputStream>;
    fn SetStandardInput(&mut self, value: &::core::option::Option<super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn StandardOutput(&mut self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardOutput(&mut self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn StandardError(&mut self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardError(&mut self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetWorkingDirectory(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.IProcessLauncherOptions";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IProcessLauncherOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherOptions_Vtbl {
        unsafe extern "system" fn StandardInput<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardInput<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardInput(&*(&value as *const <super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardOutput<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardOutput<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardOutput(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardError<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StandardError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardError<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardError(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IProcessLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessLauncherOptions, BASE_OFFSET>(),
            StandardInput: StandardInput::<Impl, IMPL_OFFSET>,
            SetStandardInput: SetStandardInput::<Impl, IMPL_OFFSET>,
            StandardOutput: StandardOutput::<Impl, IMPL_OFFSET>,
            SetStandardOutput: SetStandardOutput::<Impl, IMPL_OFFSET>,
            StandardError: StandardError::<Impl, IMPL_OFFSET>,
            SetStandardError: SetStandardError::<Impl, IMPL_OFFSET>,
            WorkingDirectory: WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherResult_Impl: Sized {
    fn ExitCode(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessLauncherResult {
    const NAME: &'static str = "Windows.System.IProcessLauncherResult";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessLauncherResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherResult_Vtbl {
        unsafe extern "system" fn ExitCode<Impl: IProcessLauncherResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessLauncherResult, BASE_OFFSET>(), ExitCode: ExitCode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessLauncherStatics_Impl: Sized {
    fn RunToCompletionAsync(&mut self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
    fn RunToCompletionAsyncWithOptions(&mut self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING, options: &::core::option::Option<ProcessLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessLauncherStatics {
    const NAME: &'static str = "Windows.System.IProcessLauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessLauncherStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherStatics_Vtbl {
        unsafe extern "system" fn RunToCompletionAsync<Impl: IProcessLauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunToCompletionAsync(&*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunToCompletionAsyncWithOptions<Impl: IProcessLauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunToCompletionAsyncWithOptions(&*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <ProcessLauncherOptions as ::windows::core::Abi>::Abi as *const <ProcessLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessLauncherStatics, BASE_OFFSET>(),
            RunToCompletionAsync: RunToCompletionAsync::<Impl, IMPL_OFFSET>,
            RunToCompletionAsyncWithOptions: RunToCompletionAsyncWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryReport_Impl: Sized {
    fn PrivateWorkingSetUsage(&mut self) -> ::windows::core::Result<u64>;
    fn TotalWorkingSetUsage(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryReport {
    const NAME: &'static str = "Windows.System.IProcessMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryReport_Vtbl {
        unsafe extern "system" fn PrivateWorkingSetUsage<Impl: IProcessMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrivateWorkingSetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWorkingSetUsage<Impl: IProcessMemoryReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalWorkingSetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessMemoryReport, BASE_OFFSET>(),
            PrivateWorkingSetUsage: PrivateWorkingSetUsage::<Impl, IMPL_OFFSET>,
            TotalWorkingSetUsage: TotalWorkingSetUsage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProtocolForResultsOperation_Impl: Sized {
    fn ReportCompleted(&mut self, data: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.IProtocolForResultsOperation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProtocolForResultsOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolForResultsOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtocolForResultsOperation_Vtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IProtocolForResultsOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&data as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtocolForResultsOperation, BASE_OFFSET>(),
            ReportCompleted: ReportCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolForResultsOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteLauncherOptions_Impl: Sized {
    fn FallbackUri(&mut self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&mut self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PreferredAppIds(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.IRemoteLauncherOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteLauncherOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteLauncherOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteLauncherOptions_Vtbl {
        unsafe extern "system" fn FallbackUri<Impl: IRemoteLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackUri<Impl: IRemoteLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredAppIds<Impl: IRemoteLauncherOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredAppIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteLauncherOptions, BASE_OFFSET>(),
            FallbackUri: FallbackUri::<Impl, IMPL_OFFSET>,
            SetFallbackUri: SetFallbackUri::<Impl, IMPL_OFFSET>,
            PreferredAppIds: PreferredAppIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
pub trait IRemoteLauncherStatics_Impl: Sized {
    fn LaunchUriAsync(&mut self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithOptionsAsync(&mut self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithDataAsync(&mut self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteLauncherStatics {
    const NAME: &'static str = "Windows.System.IRemoteLauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl IRemoteLauncherStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteLauncherStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteLauncherStatics_Vtbl {
        unsafe extern "system" fn LaunchUriAsync<Impl: IRemoteLauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriAsync(&*(&remotesystemconnectionrequest as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: IRemoteLauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithOptionsAsync(
                &*(&remotesystemconnectionrequest as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <RemoteLauncherOptions as ::windows::core::Abi>::Abi as *const <RemoteLauncherOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: IRemoteLauncherStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithDataAsync(
                &*(&remotesystemconnectionrequest as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <RemoteLauncherOptions as ::windows::core::Abi>::Abi as *const <RemoteLauncherOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&inputdata as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteLauncherStatics, BASE_OFFSET>(),
            LaunchUriAsync: LaunchUriAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithOptionsAsync: LaunchUriWithOptionsAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithDataAsync: LaunchUriWithDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShutdownManagerStatics_Impl: Sized {
    fn BeginShutdown(&mut self, shutdownkind: ShutdownKind, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CancelShutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShutdownManagerStatics {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShutdownManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShutdownManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShutdownManagerStatics_Vtbl {
        unsafe extern "system" fn BeginShutdown<Impl: IShutdownManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginShutdown(shutdownkind, &*(&timeout as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelShutdown<Impl: IShutdownManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelShutdown().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShutdownManagerStatics, BASE_OFFSET>(),
            BeginShutdown: BeginShutdown::<Impl, IMPL_OFFSET>,
            CancelShutdown: CancelShutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShutdownManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShutdownManagerStatics2_Impl: Sized + IShutdownManagerStatics_Impl {
    fn IsPowerStateSupported(&mut self, powerstate: PowerState) -> ::windows::core::Result<bool>;
    fn EnterPowerState(&mut self, powerstate: PowerState) -> ::windows::core::Result<()>;
    fn EnterPowerStateWithTimeSpan(&mut self, powerstate: PowerState, wakeupafter: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShutdownManagerStatics2 {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShutdownManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShutdownManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShutdownManagerStatics2_Vtbl {
        unsafe extern "system" fn IsPowerStateSupported<Impl: IShutdownManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPowerStateSupported(powerstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterPowerState<Impl: IShutdownManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnterPowerState(powerstate).into()
        }
        unsafe extern "system" fn EnterPowerStateWithTimeSpan<Impl: IShutdownManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnterPowerStateWithTimeSpan(powerstate, &*(&wakeupafter as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IShutdownManagerStatics2, BASE_OFFSET>(),
            IsPowerStateSupported: IsPowerStateSupported::<Impl, IMPL_OFFSET>,
            EnterPowerState: EnterPowerState::<Impl, IMPL_OFFSET>,
            EnterPowerStateWithTimeSpan: EnterPowerStateWithTimeSpan::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShutdownManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimeZoneSettingsStatics_Impl: Sized {
    fn CurrentTimeZoneDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedTimeZoneDisplayNames(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CanChangeTimeZone(&mut self) -> ::windows::core::Result<bool>;
    fn ChangeTimeZoneByDisplayName(&mut self, timezonedisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimeZoneSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeZoneSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeZoneSettingsStatics_Vtbl {
        unsafe extern "system" fn CurrentTimeZoneDisplayName<Impl: ITimeZoneSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTimeZoneDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTimeZoneDisplayNames<Impl: ITimeZoneSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTimeZoneDisplayNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeTimeZone<Impl: ITimeZoneSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanChangeTimeZone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeTimeZoneByDisplayName<Impl: ITimeZoneSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeTimeZoneByDisplayName(&*(&timezonedisplayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimeZoneSettingsStatics, BASE_OFFSET>(),
            CurrentTimeZoneDisplayName: CurrentTimeZoneDisplayName::<Impl, IMPL_OFFSET>,
            SupportedTimeZoneDisplayNames: SupportedTimeZoneDisplayNames::<Impl, IMPL_OFFSET>,
            CanChangeTimeZone: CanChangeTimeZone::<Impl, IMPL_OFFSET>,
            ChangeTimeZoneByDisplayName: ChangeTimeZoneByDisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeZoneSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimeZoneSettingsStatics2_Impl: Sized {
    fn AutoUpdateTimeZoneAsync(&mut self, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics2 {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimeZoneSettingsStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeZoneSettingsStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeZoneSettingsStatics2_Vtbl {
        unsafe extern "system" fn AutoUpdateTimeZoneAsync<Impl: ITimeZoneSettingsStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoUpdateTimeZoneAsync(&*(&timeout as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimeZoneSettingsStatics2, BASE_OFFSET>(),
            AutoUpdateTimeZoneAsync: AutoUpdateTimeZoneAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeZoneSettingsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUser_Impl: Sized {
    fn NonRoamableId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AuthenticationStatus(&mut self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn Type(&mut self) -> ::windows::core::Result<UserType>;
    fn GetPropertyAsync(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetPropertiesAsync(&mut self, values: &::core::option::Option<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>;
    fn GetPictureAsync(&mut self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUser {
    const NAME: &'static str = "Windows.System.IUser";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUser_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUser_Vtbl {
        unsafe extern "system" fn NonRoamableId<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonRoamableId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationStatus<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAsync<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyAsync(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync(&*(&values as *const <super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPictureAsync<Impl: IUser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPictureAsync(desiredsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUser, BASE_OFFSET>(),
            NonRoamableId: NonRoamableId::<Impl, IMPL_OFFSET>,
            AuthenticationStatus: AuthenticationStatus::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            GetPropertyAsync: GetPropertyAsync::<Impl, IMPL_OFFSET>,
            GetPropertiesAsync: GetPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetPictureAsync: GetPictureAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUser2_Impl: Sized {
    fn CheckUserAgeConsentGroupAsync(&mut self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUser2 {
    const NAME: &'static str = "Windows.System.IUser2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUser2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUser2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUser2_Vtbl {
        unsafe extern "system" fn CheckUserAgeConsentGroupAsync<Impl: IUser2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckUserAgeConsentGroupAsync(consentgroup) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUser2, BASE_OFFSET>(),
            CheckUserAgeConsentGroupAsync: CheckUserAgeConsentGroupAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangeDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.IUserAuthenticationStatusChangeDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IUserAuthenticationStatusChangeDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserAuthenticationStatusChangeDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserAuthenticationStatusChangeDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IUserAuthenticationStatusChangeDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserAuthenticationStatusChangeDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserAuthenticationStatusChangeDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangingEventArgs_Impl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral>;
    fn User(&mut self) -> ::windows::core::Result<User>;
    fn NewStatus(&mut self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn CurrentStatus(&mut self) -> ::windows::core::Result<UserAuthenticationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.IUserAuthenticationStatusChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserAuthenticationStatusChangingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserAuthenticationStatusChangingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserAuthenticationStatusChangingEventArgs_Vtbl {
        unsafe extern "system" fn GetDeferral<Impl: IUserAuthenticationStatusChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IUserAuthenticationStatusChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStatus<Impl: IUserAuthenticationStatusChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStatus<Impl: IUserAuthenticationStatusChangingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserAuthenticationStatusChangingEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            NewStatus: NewStatus::<Impl, IMPL_OFFSET>,
            CurrentStatus: CurrentStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserAuthenticationStatusChangingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserChangedEventArgs_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserChangedEventArgs {
    const NAME: &'static str = "Windows.System.IUserChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserChangedEventArgs_Vtbl {
        unsafe extern "system" fn User<Impl: IUserChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserChangedEventArgs, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserChangedEventArgs2_Impl: Sized {
    fn ChangedPropertyKinds(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserChangedEventArgs2 {
    const NAME: &'static str = "Windows.System.IUserChangedEventArgs2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserChangedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserChangedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserChangedEventArgs2_Vtbl {
        unsafe extern "system" fn ChangedPropertyKinds<Impl: IUserChangedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangedPropertyKinds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserChangedEventArgs2, BASE_OFFSET>(),
            ChangedPropertyKinds: ChangedPropertyKinds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDeviceAssociationChangedEventArgs_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewUser(&mut self) -> ::windows::core::Result<User>;
    fn OldUser(&mut self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.IUserDeviceAssociationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDeviceAssociationChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDeviceAssociationChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDeviceAssociationChangedEventArgs_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IUserDeviceAssociationChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewUser<Impl: IUserDeviceAssociationChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldUser<Impl: IUserDeviceAssociationChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDeviceAssociationChangedEventArgs, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            NewUser: NewUser::<Impl, IMPL_OFFSET>,
            OldUser: OldUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDeviceAssociationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDeviceAssociationStatics_Impl: Sized {
    fn FindUserFromDeviceId(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
    fn UserDeviceAssociationChanged(&mut self, handler: &::core::option::Option<super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUserDeviceAssociationChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDeviceAssociationStatics {
    const NAME: &'static str = "Windows.System.IUserDeviceAssociationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDeviceAssociationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDeviceAssociationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDeviceAssociationStatics_Vtbl {
        unsafe extern "system" fn FindUserFromDeviceId<Impl: IUserDeviceAssociationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUserFromDeviceId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDeviceAssociationChanged<Impl: IUserDeviceAssociationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDeviceAssociationChanged(&*(&handler as *const <super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserDeviceAssociationChanged<Impl: IUserDeviceAssociationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserDeviceAssociationChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDeviceAssociationStatics, BASE_OFFSET>(),
            FindUserFromDeviceId: FindUserFromDeviceId::<Impl, IMPL_OFFSET>,
            UserDeviceAssociationChanged: UserDeviceAssociationChanged::<Impl, IMPL_OFFSET>,
            RemoveUserDeviceAssociationChanged: RemoveUserDeviceAssociationChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDeviceAssociationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserPicker_Impl: Sized {
    fn AllowGuestAccounts(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowGuestAccounts(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SuggestedSelectedUser(&mut self) -> ::windows::core::Result<User>;
    fn SetSuggestedSelectedUser(&mut self, value: &::core::option::Option<User>) -> ::windows::core::Result<()>;
    fn PickSingleUserAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserPicker {
    const NAME: &'static str = "Windows.System.IUserPicker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserPicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserPicker_Vtbl {
        unsafe extern "system" fn AllowGuestAccounts<Impl: IUserPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowGuestAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowGuestAccounts<Impl: IUserPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowGuestAccounts(value).into()
        }
        unsafe extern "system" fn SuggestedSelectedUser<Impl: IUserPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedSelectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedSelectedUser<Impl: IUserPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedSelectedUser(&*(&value as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSingleUserAsync<Impl: IUserPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PickSingleUserAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserPicker, BASE_OFFSET>(),
            AllowGuestAccounts: AllowGuestAccounts::<Impl, IMPL_OFFSET>,
            SetAllowGuestAccounts: SetAllowGuestAccounts::<Impl, IMPL_OFFSET>,
            SuggestedSelectedUser: SuggestedSelectedUser::<Impl, IMPL_OFFSET>,
            SetSuggestedSelectedUser: SetSuggestedSelectedUser::<Impl, IMPL_OFFSET>,
            PickSingleUserAsync: PickSingleUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserPickerStatics_Impl: Sized {
    fn IsSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserPickerStatics {
    const NAME: &'static str = "Windows.System.IUserPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserPickerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserPickerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserPickerStatics_Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IUserPickerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserPickerStatics, BASE_OFFSET>(), IsSupported: IsSupported::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserPickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserStatics_Impl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<UserWatcher>;
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByType(&mut self, r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByTypeAndStatus(&mut self, r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn GetFromId(&mut self, nonroamableid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserStatics {
    const NAME: &'static str = "Windows.System.IUserStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserStatics_Vtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IUserStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IUserStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncByType<Impl: IUserStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncByType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncByTypeAndStatus<Impl: IUserStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncByTypeAndStatus(r#type, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromId<Impl: IUserStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonroamableid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromId(&*(&nonroamableid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserStatics, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            FindAllAsyncByType: FindAllAsyncByType::<Impl, IMPL_OFFSET>,
            FindAllAsyncByTypeAndStatus: FindAllAsyncByTypeAndStatus::<Impl, IMPL_OFFSET>,
            GetFromId: GetFromId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserStatics2_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserStatics2 {
    const NAME: &'static str = "Windows.System.IUserStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserStatics2_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserStatics2, BASE_OFFSET>(), GetDefault: GetDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserWatcher_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<UserWatcherStatus>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Added(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanging(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanging(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserWatcher {
    const NAME: &'static str = "Windows.System.IUserWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserWatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserWatcher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserWatcher_Vtbl {
        unsafe extern "system" fn Status<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Added<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanged<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatusChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationStatusChanged<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanging<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatusChanging(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationStatusChanging<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IUserWatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserWatcher, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            AuthenticationStatusChanged: AuthenticationStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveAuthenticationStatusChanged: RemoveAuthenticationStatusChanged::<Impl, IMPL_OFFSET>,
            AuthenticationStatusChanging: AuthenticationStatusChanging::<Impl, IMPL_OFFSET>,
            RemoveAuthenticationStatusChanging: RemoveAuthenticationStatusChanging::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped: Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped: RemoveStopped::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserWatcher as ::windows::core::Interface>::IID
    }
}
