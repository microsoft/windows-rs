#[cfg(feature = "implement_exclusive")]
pub trait IAppActivationResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppActivationResult {
    const NAME: &'static str = "Windows.System.IAppActivationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAppActivationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppActivationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppActivationResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppActivationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppActivationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppActivationResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>, AppResourceGroupInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppActivationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoImpl: Sized {
    fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo>;
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo";
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoVtbl {
        unsafe extern "system" fn AppInfo<Impl: IAppDiagnosticInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo>, ::windows::core::GetTrustLevel, AppInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfo2Impl: Sized {
    fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>>;
    fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppDiagnosticInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfo2Vtbl {
        unsafe extern "system" fn GetResourceGroups<Impl: IAppDiagnosticInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateResourceGroupWatcher<Impl: IAppDiagnosticInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo2>, ::windows::core::GetTrustLevel, GetResourceGroups::<Impl, IMPL_OFFSET>, CreateResourceGroupWatcher::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfo3Impl: Sized {
    fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo3 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfo3Vtbl {
        unsafe extern "system" fn LaunchAsync<Impl: IAppDiagnosticInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo3>, ::windows::core::GetTrustLevel, LaunchAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoStaticsImpl: Sized {
    fn RequestInfoAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn RequestInfoAsync<Impl: IAppDiagnosticInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoStatics>, ::windows::core::GetTrustLevel, RequestInfoAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoStatics2Impl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcher>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>;
    fn RequestInfoForPackageAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppUserModelId(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestInfoForPackageAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestInfoForAppAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestInfoForAppUserModelId<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoStatics2>,
            ::windows::core::GetTrustLevel,
            CreateWatcher::<Impl, IMPL_OFFSET>,
            RequestAccessAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForPackageAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForAppAsync::<Impl, IMPL_OFFSET>,
            RequestInfoForAppUserModelId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppDiagnosticInfoWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppDiagnosticInfoWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAdded<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemoved<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopped<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoWatcher>,
            ::windows::core::GetTrustLevel,
            Added::<Impl, IMPL_OFFSET>,
            RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoWatcherEventArgsImpl: Sized {
    fn AppDiagnosticInfo(&self) -> ::windows::core::Result<AppDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoWatcherEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoWatcherEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDiagnosticInfoWatcherEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDiagnosticInfoWatcherEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfo<Impl: IAppDiagnosticInfoWatcherEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoWatcherEventArgs>, ::windows::core::GetTrustLevel, AppDiagnosticInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDiagnosticInfoWatcherEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExecutionStateChangeResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExecutionStateChangeResult {
    const NAME: &'static str = "Windows.System.IAppExecutionStateChangeResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExecutionStateChangeResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExecutionStateChangeResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExecutionStateChangeResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppExecutionStateChangeResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppExecutionStateChangeResult>, ::windows::core::GetTrustLevel, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExecutionStateChangeResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReportImpl: Sized {
    fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn PeakPrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryReport {
    const NAME: &'static str = "Windows.System.IAppMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryReportVtbl {
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeakPrivateCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TotalCommitLimit<Impl: IAppMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppMemoryReport>, ::windows::core::GetTrustLevel, PrivateCommitUsage::<Impl, IMPL_OFFSET>, PeakPrivateCommitUsage::<Impl, IMPL_OFFSET>, TotalCommitUsage::<Impl, IMPL_OFFSET>, TotalCommitLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryReport2Impl: Sized {
    fn ExpectedTotalCommitLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryReport2 {
    const NAME: &'static str = "Windows.System.IAppMemoryReport2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryReport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryReport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryReport2Vtbl {
        unsafe extern "system" fn ExpectedTotalCommitLimit<Impl: IAppMemoryReport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppMemoryReport2>, ::windows::core::GetTrustLevel, ExpectedTotalCommitLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryReport2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppMemoryUsageLimitChangingEventArgsImpl: Sized {
    fn OldLimit(&self) -> ::windows::core::Result<u64>;
    fn NewLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppMemoryUsageLimitChangingEventArgs {
    const NAME: &'static str = "Windows.System.IAppMemoryUsageLimitChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppMemoryUsageLimitChangingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppMemoryUsageLimitChangingEventArgsVtbl {
        unsafe extern "system" fn OldLimit<Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewLimit<Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppMemoryUsageLimitChangingEventArgs>, ::windows::core::GetTrustLevel, OldLimit::<Impl, IMPL_OFFSET>, NewLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppMemoryUsageLimitChangingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupBackgroundTaskReportImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Trigger(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupBackgroundTaskReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupBackgroundTaskReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupBackgroundTaskReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupBackgroundTaskReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupBackgroundTaskReportVtbl {
        unsafe extern "system" fn TaskId<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Trigger<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EntryPoint<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupBackgroundTaskReport>, ::windows::core::GetTrustLevel, TaskId::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, Trigger::<Impl, IMPL_OFFSET>, EntryPoint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupBackgroundTaskReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsShared(&self) -> ::windows::core::Result<bool>;
    fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>;
    fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport>;
    fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>;
    fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoVtbl {
        unsafe extern "system" fn InstanceId<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsShared<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBackgroundTaskReports<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMemoryReport<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProcessDiagnosticInfos<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetStateReport<Impl: IAppResourceGroupInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfo>,
            ::windows::core::GetTrustLevel,
            InstanceId::<Impl, IMPL_OFFSET>,
            IsShared::<Impl, IMPL_OFFSET>,
            GetBackgroundTaskReports::<Impl, IMPL_OFFSET>,
            GetMemoryReport::<Impl, IMPL_OFFSET>,
            GetProcessDiagnosticInfos::<Impl, IMPL_OFFSET>,
            GetStateReport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfo2Impl: Sized {
    fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo2 {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppResourceGroupInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfo2Vtbl {
        unsafe extern "system" fn StartSuspendAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartResumeAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartTerminateAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfo2>, ::windows::core::GetTrustLevel, StartSuspendAsync::<Impl, IMPL_OFFSET>, StartResumeAsync::<Impl, IMPL_OFFSET>, StartTerminateAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExecutionStateChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveExecutionStateChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAdded<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemoved<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopped<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecutionStateChanged<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveExecutionStateChanged<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveExecutionStateChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcher>,
            ::windows::core::GetTrustLevel,
            Added::<Impl, IMPL_OFFSET>,
            RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
            ExecutionStateChanged::<Impl, IMPL_OFFSET>,
            RemoveExecutionStateChanged::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcherEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcherEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcherEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcherEventArgs>, ::windows::core::GetTrustLevel, AppDiagnosticInfos::<Impl, IMPL_OFFSET>, AppResourceGroupInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcherEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs>, ::windows::core::GetTrustLevel, AppDiagnosticInfos::<Impl, IMPL_OFFSET>, AppResourceGroupInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupMemoryReportImpl: Sized {
    fn CommitUsageLimit(&self) -> ::windows::core::Result<u64>;
    fn CommitUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn PrivateCommitUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalCommitUsage(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupMemoryReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupMemoryReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupMemoryReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupMemoryReportVtbl {
        unsafe extern "system" fn CommitUsageLimit<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommitUsageLevel<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupMemoryReport>, ::windows::core::GetTrustLevel, CommitUsageLimit::<Impl, IMPL_OFFSET>, CommitUsageLevel::<Impl, IMPL_OFFSET>, PrivateCommitUsage::<Impl, IMPL_OFFSET>, TotalCommitUsage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupStateReportImpl: Sized {
    fn ExecutionState(&self) -> ::windows::core::Result<AppResourceGroupExecutionState>;
    fn EnergyQuotaState(&self) -> ::windows::core::Result<AppResourceGroupEnergyQuotaState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupStateReport {
    const NAME: &'static str = "Windows.System.IAppResourceGroupStateReport";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupStateReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppResourceGroupStateReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppResourceGroupStateReportVtbl {
        unsafe extern "system" fn ExecutionState<Impl: IAppResourceGroupStateReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnergyQuotaState<Impl: IAppResourceGroupStateReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppResourceGroupStateReport>, ::windows::core::GetTrustLevel, ExecutionState::<Impl, IMPL_OFFSET>, EnergyQuotaState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppResourceGroupStateReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHostImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHost {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHost";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHostVtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IAppUriHandlerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHost>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHost2Impl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHost2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHost2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHost2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHost2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHost2Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAppUriHandlerHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IAppUriHandlerHost2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHost2>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, SetIsEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHost2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerHostFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerHost>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerHostFactory {
    const NAME: &'static str = "Windows.System.IAppUriHandlerHostFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerHostFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerHostFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerHostFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAppUriHandlerHostFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHostFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerHostFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppUriHandlerRegistrationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<User>;
    fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>;
    fn SetAppAddedHostsAsync(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppUriHandlerRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationVtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAppAddedHostsAsync<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppAddedHostsAsync<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistration>, ::windows::core::GetTrustLevel, Name::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>, GetAppAddedHostsAsync::<Impl, IMPL_OFFSET>, SetAppAddedHostsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppUriHandlerRegistration2Impl: Sized {
    fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>>;
    fn UpdateHosts(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppUriHandlerRegistration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistration2Vtbl {
        unsafe extern "system" fn GetAllHosts<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateHosts<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateHosts(&*(&hosts as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistration2>, ::windows::core::GetTrustLevel, GetAllHosts::<Impl, IMPL_OFFSET>, UpdateHosts::<Impl, IMPL_OFFSET>, PackageFamilyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerImpl: Sized {
    fn User(&self) -> ::windows::core::Result<User>;
    fn TryGetRegistration(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManager {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManagerVtbl {
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistrationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryGetRegistration<Impl: IAppUriHandlerRegistrationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManager>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>, TryGetRegistration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManager2Impl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManager2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManager2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManager2Vtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistrationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManager2>, ::windows::core::GetTrustLevel, PackageFamilyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForUser(&self, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManagerStatics {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManagerStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>, GetForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationManagerStatics2Impl: Sized {
    fn GetForPackage(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
    fn GetForPackageForUser(&self, packagefamilyname: &::windows::core::HSTRING, user: &::core::option::Option<User>) -> ::windows::core::Result<AppUriHandlerRegistrationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistrationManagerStatics2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistrationManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUriHandlerRegistrationManagerStatics2Vtbl {
        unsafe extern "system" fn GetForPackage<Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForPackageForUser<Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManagerStatics2>, ::windows::core::GetTrustLevel, GetForPackage::<Impl, IMPL_OFFSET>, GetForPackageForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUriHandlerRegistrationManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDateTimeSettingsStaticsImpl: Sized {
    fn SetSystemDateTime(&self, utcdatetime: &super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDateTimeSettingsStatics {
    const NAME: &'static str = "Windows.System.IDateTimeSettingsStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDateTimeSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDateTimeSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDateTimeSettingsStaticsVtbl {
        unsafe extern "system" fn SetSystemDateTime<Impl: IDateTimeSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemDateTime(&*(&utcdatetime as *const <super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDateTimeSettingsStatics>, ::windows::core::GetTrustLevel, SetSystemDateTime::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDateTimeSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueImpl: Sized {
    fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer>;
    fn TryEnqueue(&self, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn ShutdownStarting(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownStarting(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShutdownCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueue {
    const NAME: &'static str = "Windows.System.IDispatcherQueue";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueVtbl {
        unsafe extern "system" fn CreateTimer<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryEnqueue<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryEnqueueWithPriority<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShutdownStarting<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShutdownStarting<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShutdownStarting(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShutdownCompleted<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveShutdownCompleted<Impl: IDispatcherQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShutdownCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDispatcherQueue>,
            ::windows::core::GetTrustLevel,
            CreateTimer::<Impl, IMPL_OFFSET>,
            TryEnqueue::<Impl, IMPL_OFFSET>,
            TryEnqueueWithPriority::<Impl, IMPL_OFFSET>,
            ShutdownStarting::<Impl, IMPL_OFFSET>,
            RemoveShutdownStarting::<Impl, IMPL_OFFSET>,
            ShutdownCompleted::<Impl, IMPL_OFFSET>,
            RemoveShutdownCompleted::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueue as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueue2Impl: Sized {
    fn HasThreadAccess(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueue2 {
    const NAME: &'static str = "Windows.System.IDispatcherQueue2";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueue2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueue2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueue2Vtbl {
        unsafe extern "system" fn HasThreadAccess<Impl: IDispatcherQueue2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatcherQueue2>, ::windows::core::GetTrustLevel, HasThreadAccess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueue2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueControllerImpl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue>;
    fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueController {
    const NAME: &'static str = "Windows.System.IDispatcherQueueController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueControllerVtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: IDispatcherQueueControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShutdownQueueAsync<Impl: IDispatcherQueueControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatcherQueueController>, ::windows::core::GetTrustLevel, DispatcherQueue::<Impl, IMPL_OFFSET>, ShutdownQueueAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueControllerStaticsImpl: Sized {
    fn CreateOnDedicatedThread(&self) -> ::windows::core::Result<DispatcherQueueController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueControllerStatics {
    const NAME: &'static str = "Windows.System.IDispatcherQueueControllerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueControllerStaticsVtbl {
        unsafe extern "system" fn CreateOnDedicatedThread<Impl: IDispatcherQueueControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatcherQueueControllerStatics>, ::windows::core::GetTrustLevel, CreateOnDedicatedThread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueShutdownStartingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.IDispatcherQueueShutdownStartingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueShutdownStartingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueShutdownStartingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueShutdownStartingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IDispatcherQueueShutdownStartingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatcherQueueShutdownStartingEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueShutdownStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueStaticsImpl: Sized {
    fn GetForCurrentThread(&self) -> ::windows::core::Result<DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueStatics {
    const NAME: &'static str = "Windows.System.IDispatcherQueueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueStaticsVtbl {
        unsafe extern "system" fn GetForCurrentThread<Impl: IDispatcherQueueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispatcherQueueStatics>, ::windows::core::GetTrustLevel, GetForCurrentThread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherQueueTimerImpl: Sized {
    fn Interval(&self) -> ::windows::core::Result<super::Foundation::TimeSpan>;
    fn SetInterval(&self, value: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsRunning(&self) -> ::windows::core::Result<bool>;
    fn IsRepeating(&self) -> ::windows::core::Result<bool>;
    fn SetIsRepeating(&self, value: bool) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Tick(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.IDispatcherQueueTimer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherQueueTimerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherQueueTimerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherQueueTimerVtbl {
        unsafe extern "system" fn Interval<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInterval<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRunning<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRepeating<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsRepeating<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRepeating(value).into()
        }
        unsafe extern "system" fn Start<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Tick<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveTick<Impl: IDispatcherQueueTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTick(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDispatcherQueueTimer>,
            ::windows::core::GetTrustLevel,
            Interval::<Impl, IMPL_OFFSET>,
            SetInterval::<Impl, IMPL_OFFSET>,
            IsRunning::<Impl, IMPL_OFFSET>,
            IsRepeating::<Impl, IMPL_OFFSET>,
            SetIsRepeating::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Tick::<Impl, IMPL_OFFSET>,
            RemoveTick::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherQueueTimer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IFolderLauncherOptionsImpl: Sized {
    fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFolderLauncherOptions {
    const NAME: &'static str = "Windows.System.IFolderLauncherOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IFolderLauncherOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFolderLauncherOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFolderLauncherOptionsVtbl {
        unsafe extern "system" fn ItemsToSelect<Impl: IFolderLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFolderLauncherOptions>, ::windows::core::GetTrustLevel, ItemsToSelect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFolderLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStaticsImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GuestHost(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrincipalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DomainName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionInitiationProtocolUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownUserPropertiesStatics {
    const NAME: &'static str = "Windows.System.IKnownUserPropertiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownUserPropertiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUserPropertiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUserPropertiesStaticsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FirstName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LastName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccountName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GuestHost<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PrincipalName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DomainName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionInitiationProtocolUri<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IKnownUserPropertiesStatics>,
            ::windows::core::GetTrustLevel,
            DisplayName::<Impl, IMPL_OFFSET>,
            FirstName::<Impl, IMPL_OFFSET>,
            LastName::<Impl, IMPL_OFFSET>,
            ProviderName::<Impl, IMPL_OFFSET>,
            AccountName::<Impl, IMPL_OFFSET>,
            GuestHost::<Impl, IMPL_OFFSET>,
            PrincipalName::<Impl, IMPL_OFFSET>,
            DomainName::<Impl, IMPL_OFFSET>,
            SessionInitiationProtocolUri::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUserPropertiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownUserPropertiesStatics2Impl: Sized {
    fn AgeEnforcementRegion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownUserPropertiesStatics2 {
    const NAME: &'static str = "Windows.System.IKnownUserPropertiesStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownUserPropertiesStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownUserPropertiesStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownUserPropertiesStatics2Vtbl {
        unsafe extern "system" fn AgeEnforcementRegion<Impl: IKnownUserPropertiesStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownUserPropertiesStatics2>, ::windows::core::GetTrustLevel, AgeEnforcementRegion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownUserPropertiesStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILaunchUriResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LaunchUriStatus>;
    fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILaunchUriResult {
    const NAME: &'static str = "Windows.System.ILaunchUriResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILaunchUriResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILaunchUriResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILaunchUriResultVtbl {
        unsafe extern "system" fn Status<Impl: ILaunchUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Result<Impl: ILaunchUriResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILaunchUriResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Result::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILaunchUriResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILauncherOptionsImpl: Sized {
    fn TreatAsUntrusted(&self) -> ::windows::core::Result<bool>;
    fn SetTreatAsUntrusted(&self, value: bool) -> ::windows::core::Result<()>;
    fn DisplayApplicationPicker(&self) -> ::windows::core::Result<bool>;
    fn SetDisplayApplicationPicker(&self, value: bool) -> ::windows::core::Result<()>;
    fn UI(&self) -> ::windows::core::Result<LauncherUIOptions>;
    fn PreferredApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PreferredApplicationDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPreferredApplicationDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherOptions {
    const NAME: &'static str = "Windows.System.ILauncherOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILauncherOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptionsVtbl {
        unsafe extern "system" fn TreatAsUntrusted<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTreatAsUntrusted<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTreatAsUntrusted(value).into()
        }
        unsafe extern "system" fn DisplayApplicationPicker<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayApplicationPicker<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayApplicationPicker(value).into()
        }
        unsafe extern "system" fn UI<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PreferredApplicationPackageFamilyName<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredApplicationPackageFamilyName<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredApplicationDisplayName<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredApplicationDisplayName<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FallbackUri<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFallbackUri<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentType<Impl: ILauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherOptions>,
            ::windows::core::GetTrustLevel,
            TreatAsUntrusted::<Impl, IMPL_OFFSET>,
            SetTreatAsUntrusted::<Impl, IMPL_OFFSET>,
            DisplayApplicationPicker::<Impl, IMPL_OFFSET>,
            SetDisplayApplicationPicker::<Impl, IMPL_OFFSET>,
            UI::<Impl, IMPL_OFFSET>,
            PreferredApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetPreferredApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            PreferredApplicationDisplayName::<Impl, IMPL_OFFSET>,
            SetPreferredApplicationDisplayName::<Impl, IMPL_OFFSET>,
            FallbackUri::<Impl, IMPL_OFFSET>,
            SetFallbackUri::<Impl, IMPL_OFFSET>,
            ContentType::<Impl, IMPL_OFFSET>,
            SetContentType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
pub trait ILauncherOptions2Impl: Sized {
    fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult>;
    fn SetNeighboringFilesQuery(&self, value: &::core::option::Option<super::Storage::Search::StorageFileQueryResult>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherOptions2 {
    const NAME: &'static str = "Windows.System.ILauncherOptions2";
}
#[cfg(all(feature = "Storage_Search", feature = "implement_exclusive"))]
impl ILauncherOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions2Vtbl {
        unsafe extern "system" fn TargetApplicationPackageFamilyName<Impl: ILauncherOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetApplicationPackageFamilyName<Impl: ILauncherOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NeighboringFilesQuery<Impl: ILauncherOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNeighboringFilesQuery<Impl: ILauncherOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNeighboringFilesQuery(&*(&value as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::Abi>::Abi as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherOptions2>,
            ::windows::core::GetTrustLevel,
            TargetApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetTargetApplicationPackageFamilyName::<Impl, IMPL_OFFSET>,
            NeighboringFilesQuery::<Impl, IMPL_OFFSET>,
            SetNeighboringFilesQuery::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions3Impl: Sized {
    fn IgnoreAppUriHandlers(&self) -> ::windows::core::Result<bool>;
    fn SetIgnoreAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions3 {
    const NAME: &'static str = "Windows.System.ILauncherOptions3";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptions3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions3Vtbl {
        unsafe extern "system" fn IgnoreAppUriHandlers<Impl: ILauncherOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIgnoreAppUriHandlers<Impl: ILauncherOptions3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIgnoreAppUriHandlers(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILauncherOptions3>, ::windows::core::GetTrustLevel, IgnoreAppUriHandlers::<Impl, IMPL_OFFSET>, SetIgnoreAppUriHandlers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions4Impl: Sized {
    fn LimitPickerToCurrentAppAndAppUriHandlers(&self) -> ::windows::core::Result<bool>;
    fn SetLimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions4 {
    const NAME: &'static str = "Windows.System.ILauncherOptions4";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptions4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherOptions4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherOptions4Vtbl {
        unsafe extern "system" fn LimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLimitPickerToCurrentAppAndAppUriHandlers(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILauncherOptions4>, ::windows::core::GetTrustLevel, LimitPickerToCurrentAppAndAppUriHandlers::<Impl, IMPL_OFFSET>, SetLimitPickerToCurrentAppAndAppUriHandlers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherOptions4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStaticsImpl: Sized {
    fn LaunchFileAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFileWithOptionsAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriWithOptionsAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics {
    const NAME: &'static str = "Windows.System.ILauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStaticsVtbl {
        unsafe extern "system" fn LaunchFileAsync<Impl: ILauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchFileWithOptionsAsync<Impl: ILauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriAsync<Impl: ILauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: ILauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILauncherStatics>, ::windows::core::GetTrustLevel, LaunchFileAsync::<Impl, IMPL_OFFSET>, LaunchFileWithOptionsAsync::<Impl, IMPL_OFFSET>, LaunchUriAsync::<Impl, IMPL_OFFSET>, LaunchUriWithOptionsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStatics2Impl: Sized {
    fn LaunchUriForResultsAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriWithDataAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn QueryUriSupportAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryUriSupportWithPackageFamilyNameAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportAsync(&self, file: &::core::option::Option<super::Storage::StorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryFileSupportWithPackageFamilyNameAsync(&self, file: &::core::option::Option<super::Storage::StorageFile>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindUriSchemeHandlersAsync(&self, scheme: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindUriSchemeHandlersWithLaunchUriTypeAsync(&self, scheme: &::windows::core::HSTRING, launchquerysupporttype: LaunchQuerySupportType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn FindFileHandlersAsync(&self, extension: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics2 {
    const NAME: &'static str = "Windows.System.ILauncherStatics2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics2Vtbl {
        unsafe extern "system" fn LaunchUriForResultsAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriForResultsWithDataAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QueryUriSupportAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QueryUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QueryFileSupportAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QueryFileSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindUriSchemeHandlersAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindUriSchemeHandlersWithLaunchUriTypeAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindFileHandlersAsync<Impl: ILauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherStatics2>,
            ::windows::core::GetTrustLevel,
            LaunchUriForResultsAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsWithDataAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithDataAsync::<Impl, IMPL_OFFSET>,
            QueryUriSupportAsync::<Impl, IMPL_OFFSET>,
            QueryUriSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            QueryFileSupportAsync::<Impl, IMPL_OFFSET>,
            QueryFileSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            FindUriSchemeHandlersAsync::<Impl, IMPL_OFFSET>,
            FindUriSchemeHandlersWithLaunchUriTypeAsync::<Impl, IMPL_OFFSET>,
            FindFileHandlersAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
pub trait ILauncherStatics3Impl: Sized {
    fn LaunchFolderAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderWithOptionsAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics3 {
    const NAME: &'static str = "Windows.System.ILauncherStatics3";
}
#[cfg(all(feature = "Foundation", feature = "Storage", feature = "implement_exclusive"))]
impl ILauncherStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics3Vtbl {
        unsafe extern "system" fn LaunchFolderAsync<Impl: ILauncherStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchFolderWithOptionsAsync<Impl: ILauncherStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILauncherStatics3>, ::windows::core::GetTrustLevel, LaunchFolderAsync::<Impl, IMPL_OFFSET>, LaunchFolderWithOptionsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ILauncherStatics4Impl: Sized {
    fn QueryAppUriSupportAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn QueryAppUriSupportWithPackageFamilyNameAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchQuerySupportStatus>>;
    fn FindAppUriHandlersAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<super::ApplicationModel::AppInfo>>>;
    fn LaunchUriForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithOptionsForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriWithDataForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriStatus>>;
    fn LaunchUriForResultsForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
    fn LaunchUriForResultsWithDataForUserAsync(&self, user: &::core::option::Option<User>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<LaunchUriResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics4 {
    const NAME: &'static str = "Windows.System.ILauncherStatics4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILauncherStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics4Vtbl {
        unsafe extern "system" fn QueryAppUriSupportAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn QueryAppUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAppUriHandlersAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithOptionsForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithDataForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriForResultsForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriForResultsWithDataForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherStatics4>,
            ::windows::core::GetTrustLevel,
            QueryAppUriSupportAsync::<Impl, IMPL_OFFSET>,
            QueryAppUriSupportWithPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            FindAppUriHandlersAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriWithDataForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchUriForResultsWithDataForUserAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILauncherStatics5Impl: Sized {
    fn LaunchFolderPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsAsync(&self, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherStatics5 {
    const NAME: &'static str = "Windows.System.ILauncherStatics5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILauncherStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherStatics5Vtbl {
        unsafe extern "system" fn LaunchFolderPathAsync<Impl: ILauncherStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchFolderPathWithOptionsAsync<Impl: ILauncherStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchFolderPathForUserAsync<Impl: ILauncherStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchFolderPathWithOptionsForUserAsync<Impl: ILauncherStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherStatics5>,
            ::windows::core::GetTrustLevel,
            LaunchFolderPathAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathWithOptionsAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathForUserAsync::<Impl, IMPL_OFFSET>,
            LaunchFolderPathWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ILauncherUIOptionsImpl: Sized {
    fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>>;
    fn SetInvocationPoint(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>>;
    fn SetSelectionRect(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement>;
    fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILauncherUIOptions {
    const NAME: &'static str = "Windows.System.ILauncherUIOptions";
}
#[cfg(all(feature = "Foundation", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ILauncherUIOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherUIOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherUIOptionsVtbl {
        unsafe extern "system" fn InvocationPoint<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInvocationPoint<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInvocationPoint(&*(&value as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionRect<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionRect<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionRect(&*(&value as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredPlacement<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPreferredPlacement<Impl: ILauncherUIOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredPlacement(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ILauncherUIOptions>,
            ::windows::core::GetTrustLevel,
            InvocationPoint::<Impl, IMPL_OFFSET>,
            SetInvocationPoint::<Impl, IMPL_OFFSET>,
            SelectionRect::<Impl, IMPL_OFFSET>,
            SetSelectionRect::<Impl, IMPL_OFFSET>,
            PreferredPlacement::<Impl, IMPL_OFFSET>,
            SetPreferredPlacement::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherUIOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_ViewManagement")]
pub trait ILauncherViewOptionsImpl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_ViewManagement")]
impl ::windows::core::RuntimeName for ILauncherViewOptions {
    const NAME: &'static str = "Windows.System.ILauncherViewOptions";
}
#[cfg(feature = "UI_ViewManagement")]
impl ILauncherViewOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILauncherViewOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILauncherViewOptionsVtbl {
        unsafe extern "system" fn DesiredRemainingView<Impl: ILauncherViewOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredRemainingView<Impl: ILauncherViewOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredRemainingView(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILauncherViewOptions>, ::windows::core::GetTrustLevel, DesiredRemainingView::<Impl, IMPL_OFFSET>, SetDesiredRemainingView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILauncherViewOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMemoryManagerStaticsImpl: Sized {
    fn AppMemoryUsage(&self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLimit(&self) -> ::windows::core::Result<u64>;
    fn AppMemoryUsageLevel(&self) -> ::windows::core::Result<AppMemoryUsageLevel>;
    fn AppMemoryUsageIncreased(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageIncreased(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageDecreased(&self, handler: &::core::option::Option<super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageDecreased(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AppMemoryUsageLimitChanging(&self, handler: &::core::option::Option<super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAppMemoryUsageLimitChanging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMemoryManagerStatics {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IMemoryManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStaticsVtbl {
        unsafe extern "system" fn AppMemoryUsage<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppMemoryUsageLimit<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppMemoryUsageLevel<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppMemoryUsageIncreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAppMemoryUsageIncreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageIncreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageDecreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAppMemoryUsageDecreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageDecreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageLimitChanging<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAppMemoryUsageLimitChanging<Impl: IMemoryManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageLimitChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics>,
            ::windows::core::GetTrustLevel,
            AppMemoryUsage::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLimit::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLevel::<Impl, IMPL_OFFSET>,
            AppMemoryUsageIncreased::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageIncreased::<Impl, IMPL_OFFSET>,
            AppMemoryUsageDecreased::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageDecreased::<Impl, IMPL_OFFSET>,
            AppMemoryUsageLimitChanging::<Impl, IMPL_OFFSET>,
            RemoveAppMemoryUsageLimitChanging::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics2Impl: Sized {
    fn GetAppMemoryReport(&self) -> ::windows::core::Result<AppMemoryReport>;
    fn GetProcessMemoryReport(&self) -> ::windows::core::Result<ProcessMemoryReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics2 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics2Vtbl {
        unsafe extern "system" fn GetAppMemoryReport<Impl: IMemoryManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProcessMemoryReport<Impl: IMemoryManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics2>, ::windows::core::GetTrustLevel, GetAppMemoryReport::<Impl, IMPL_OFFSET>, GetProcessMemoryReport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics3Impl: Sized {
    fn TrySetAppMemoryUsageLimit(&self, value: u64) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics3 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics3Vtbl {
        unsafe extern "system" fn TrySetAppMemoryUsageLimit<Impl: IMemoryManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics3>, ::windows::core::GetTrustLevel, TrySetAppMemoryUsageLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryManagerStatics4Impl: Sized {
    fn ExpectedAppMemoryUsageLimit(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics4 {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryManagerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryManagerStatics4Vtbl {
        unsafe extern "system" fn ExpectedAppMemoryUsageLimit<Impl: IMemoryManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics4>, ::windows::core::GetTrustLevel, ExpectedAppMemoryUsageLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IProcessLauncherOptionsImpl: Sized {
    fn StandardInput(&self) -> ::windows::core::Result<super::Storage::Streams::IInputStream>;
    fn SetStandardInput(&self, value: &::core::option::Option<super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn StandardOutput(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardOutput(&self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn StandardError(&self) -> ::windows::core::Result<super::Storage::Streams::IOutputStream>;
    fn SetStandardError(&self, value: &::core::option::Option<super::Storage::Streams::IOutputStream>) -> ::windows::core::Result<()>;
    fn WorkingDirectory(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetWorkingDirectory(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.IProcessLauncherOptions";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IProcessLauncherOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherOptionsVtbl {
        unsafe extern "system" fn StandardInput<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStandardInput<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardInput(&*(&value as *const <super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardOutput<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStandardOutput<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardOutput(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardError<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStandardError<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStandardError(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWorkingDirectory<Impl: IProcessLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IProcessLauncherOptions>,
            ::windows::core::GetTrustLevel,
            StandardInput::<Impl, IMPL_OFFSET>,
            SetStandardInput::<Impl, IMPL_OFFSET>,
            StandardOutput::<Impl, IMPL_OFFSET>,
            SetStandardOutput::<Impl, IMPL_OFFSET>,
            StandardError::<Impl, IMPL_OFFSET>,
            SetStandardError::<Impl, IMPL_OFFSET>,
            WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherResultImpl: Sized {
    fn ExitCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessLauncherResult {
    const NAME: &'static str = "Windows.System.IProcessLauncherResult";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessLauncherResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherResultVtbl {
        unsafe extern "system" fn ExitCode<Impl: IProcessLauncherResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessLauncherResult>, ::windows::core::GetTrustLevel, ExitCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IProcessLauncherStaticsImpl: Sized {
    fn RunToCompletionAsync(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
    fn RunToCompletionAsyncWithOptions(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING, options: &::core::option::Option<ProcessLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessLauncherStatics {
    const NAME: &'static str = "Windows.System.IProcessLauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IProcessLauncherStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLauncherStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLauncherStaticsVtbl {
        unsafe extern "system" fn RunToCompletionAsync<Impl: IProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RunToCompletionAsyncWithOptions<Impl: IProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessLauncherStatics>, ::windows::core::GetTrustLevel, RunToCompletionAsync::<Impl, IMPL_OFFSET>, RunToCompletionAsyncWithOptions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryReportImpl: Sized {
    fn PrivateWorkingSetUsage(&self) -> ::windows::core::Result<u64>;
    fn TotalWorkingSetUsage(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessMemoryReport {
    const NAME: &'static str = "Windows.System.IProcessMemoryReport";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessMemoryReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessMemoryReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessMemoryReportVtbl {
        unsafe extern "system" fn PrivateWorkingSetUsage<Impl: IProcessMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TotalWorkingSetUsage<Impl: IProcessMemoryReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessMemoryReport>, ::windows::core::GetTrustLevel, PrivateWorkingSetUsage::<Impl, IMPL_OFFSET>, TotalWorkingSetUsage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessMemoryReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IProtocolForResultsOperationImpl: Sized {
    fn ReportCompleted(&self, data: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.IProtocolForResultsOperation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IProtocolForResultsOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtocolForResultsOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtocolForResultsOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IProtocolForResultsOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&data as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtocolForResultsOperation>, ::windows::core::GetTrustLevel, ReportCompleted::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtocolForResultsOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteLauncherOptionsImpl: Sized {
    fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.IRemoteLauncherOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteLauncherOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteLauncherOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteLauncherOptionsVtbl {
        unsafe extern "system" fn FallbackUri<Impl: IRemoteLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFallbackUri<Impl: IRemoteLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredAppIds<Impl: IRemoteLauncherOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteLauncherOptions>, ::windows::core::GetTrustLevel, FallbackUri::<Impl, IMPL_OFFSET>, SetFallbackUri::<Impl, IMPL_OFFSET>, PreferredAppIds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteLauncherOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
pub trait IRemoteLauncherStaticsImpl: Sized {
    fn LaunchUriAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithOptionsAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithDataAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteLauncherStatics {
    const NAME: &'static str = "Windows.System.IRemoteLauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl IRemoteLauncherStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteLauncherStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteLauncherStaticsVtbl {
        unsafe extern "system" fn LaunchUriAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteLauncherStatics>, ::windows::core::GetTrustLevel, LaunchUriAsync::<Impl, IMPL_OFFSET>, LaunchUriWithOptionsAsync::<Impl, IMPL_OFFSET>, LaunchUriWithDataAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShutdownManagerStaticsImpl: Sized {
    fn BeginShutdown(&self, shutdownkind: ShutdownKind, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CancelShutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShutdownManagerStatics {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShutdownManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShutdownManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShutdownManagerStaticsVtbl {
        unsafe extern "system" fn BeginShutdown<Impl: IShutdownManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginShutdown(shutdownkind, &*(&timeout as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelShutdown<Impl: IShutdownManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelShutdown().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShutdownManagerStatics>, ::windows::core::GetTrustLevel, BeginShutdown::<Impl, IMPL_OFFSET>, CancelShutdown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShutdownManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IShutdownManagerStatics2Impl: Sized + IShutdownManagerStaticsImpl {
    fn IsPowerStateSupported(&self, powerstate: PowerState) -> ::windows::core::Result<bool>;
    fn EnterPowerState(&self, powerstate: PowerState) -> ::windows::core::Result<()>;
    fn EnterPowerStateWithTimeSpan(&self, powerstate: PowerState, wakeupafter: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IShutdownManagerStatics2 {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IShutdownManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShutdownManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShutdownManagerStatics2Vtbl {
        unsafe extern "system" fn IsPowerStateSupported<Impl: IShutdownManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnterPowerState<Impl: IShutdownManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnterPowerState(powerstate).into()
        }
        unsafe extern "system" fn EnterPowerStateWithTimeSpan<Impl: IShutdownManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnterPowerStateWithTimeSpan(powerstate, &*(&wakeupafter as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IShutdownManagerStatics2>, ::windows::core::GetTrustLevel, IsPowerStateSupported::<Impl, IMPL_OFFSET>, EnterPowerState::<Impl, IMPL_OFFSET>, EnterPowerStateWithTimeSpan::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShutdownManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ITimeZoneSettingsStaticsImpl: Sized {
    fn CurrentTimeZoneDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedTimeZoneDisplayNames(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CanChangeTimeZone(&self) -> ::windows::core::Result<bool>;
    fn ChangeTimeZoneByDisplayName(&self, timezonedisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ITimeZoneSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeZoneSettingsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeZoneSettingsStaticsVtbl {
        unsafe extern "system" fn CurrentTimeZoneDisplayName<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedTimeZoneDisplayNames<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanChangeTimeZone<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChangeTimeZoneByDisplayName<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangeTimeZoneByDisplayName(&*(&timezonedisplayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimeZoneSettingsStatics>, ::windows::core::GetTrustLevel, CurrentTimeZoneDisplayName::<Impl, IMPL_OFFSET>, SupportedTimeZoneDisplayNames::<Impl, IMPL_OFFSET>, CanChangeTimeZone::<Impl, IMPL_OFFSET>, ChangeTimeZoneByDisplayName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeZoneSettingsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITimeZoneSettingsStatics2Impl: Sized {
    fn AutoUpdateTimeZoneAsync(&self, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics2 {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITimeZoneSettingsStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeZoneSettingsStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeZoneSettingsStatics2Vtbl {
        unsafe extern "system" fn AutoUpdateTimeZoneAsync<Impl: ITimeZoneSettingsStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimeZoneSettingsStatics2>, ::windows::core::GetTrustLevel, AutoUpdateTimeZoneAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeZoneSettingsStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserImpl: Sized {
    fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn Type(&self) -> ::windows::core::Result<UserType>;
    fn GetPropertyAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetPropertiesAsync(&self, values: &::core::option::Option<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>;
    fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUser {
    const NAME: &'static str = "Windows.System.IUser";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserVtbl {
        unsafe extern "system" fn NonRoamableId<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticationStatus<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPropertyAsync<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPropertiesAsync<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPictureAsync<Impl: IUserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUser>,
            ::windows::core::GetTrustLevel,
            NonRoamableId::<Impl, IMPL_OFFSET>,
            AuthenticationStatus::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            GetPropertyAsync::<Impl, IMPL_OFFSET>,
            GetPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetPictureAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUser as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUser2Impl: Sized {
    fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUser2 {
    const NAME: &'static str = "Windows.System.IUser2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUser2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUser2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUser2Vtbl {
        unsafe extern "system" fn CheckUserAgeConsentGroupAsync<Impl: IUser2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUser2>, ::windows::core::GetTrustLevel, CheckUserAgeConsentGroupAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUser2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangeDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserAuthenticationStatusChangeDeferral {
    const NAME: &'static str = "Windows.System.IUserAuthenticationStatusChangeDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IUserAuthenticationStatusChangeDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserAuthenticationStatusChangeDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserAuthenticationStatusChangeDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IUserAuthenticationStatusChangeDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserAuthenticationStatusChangeDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserAuthenticationStatusChangeDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserAuthenticationStatusChangingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<UserAuthenticationStatusChangeDeferral>;
    fn User(&self) -> ::windows::core::Result<User>;
    fn NewStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn CurrentStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserAuthenticationStatusChangingEventArgs {
    const NAME: &'static str = "Windows.System.IUserAuthenticationStatusChangingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserAuthenticationStatusChangingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserAuthenticationStatusChangingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserAuthenticationStatusChangingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn User<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewStatus<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentStatus<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserAuthenticationStatusChangingEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>, User::<Impl, IMPL_OFFSET>, NewStatus::<Impl, IMPL_OFFSET>, CurrentStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserAuthenticationStatusChangingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserChangedEventArgsImpl: Sized {
    fn User(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserChangedEventArgs {
    const NAME: &'static str = "Windows.System.IUserChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserChangedEventArgsVtbl {
        unsafe extern "system" fn User<Impl: IUserChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserChangedEventArgs>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserChangedEventArgs2Impl: Sized {
    fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserChangedEventArgs2 {
    const NAME: &'static str = "Windows.System.IUserChangedEventArgs2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserChangedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserChangedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangedPropertyKinds<Impl: IUserChangedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserChangedEventArgs2>, ::windows::core::GetTrustLevel, ChangedPropertyKinds::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserChangedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDeviceAssociationChangedEventArgsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewUser(&self) -> ::windows::core::Result<User>;
    fn OldUser(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDeviceAssociationChangedEventArgs {
    const NAME: &'static str = "Windows.System.IUserDeviceAssociationChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDeviceAssociationChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDeviceAssociationChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDeviceAssociationChangedEventArgsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewUser<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldUser<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDeviceAssociationChangedEventArgs>, ::windows::core::GetTrustLevel, DeviceId::<Impl, IMPL_OFFSET>, NewUser::<Impl, IMPL_OFFSET>, OldUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDeviceAssociationChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDeviceAssociationStaticsImpl: Sized {
    fn FindUserFromDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
    fn UserDeviceAssociationChanged(&self, handler: &::core::option::Option<super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUserDeviceAssociationChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDeviceAssociationStatics {
    const NAME: &'static str = "Windows.System.IUserDeviceAssociationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDeviceAssociationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDeviceAssociationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDeviceAssociationStaticsVtbl {
        unsafe extern "system" fn FindUserFromDeviceId<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UserDeviceAssociationChanged<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUserDeviceAssociationChanged<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserDeviceAssociationChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDeviceAssociationStatics>, ::windows::core::GetTrustLevel, FindUserFromDeviceId::<Impl, IMPL_OFFSET>, UserDeviceAssociationChanged::<Impl, IMPL_OFFSET>, RemoveUserDeviceAssociationChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDeviceAssociationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserPickerImpl: Sized {
    fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool>;
    fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User>;
    fn SetSuggestedSelectedUser(&self, value: &::core::option::Option<User>) -> ::windows::core::Result<()>;
    fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserPicker {
    const NAME: &'static str = "Windows.System.IUserPicker";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserPickerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserPickerVtbl {
        unsafe extern "system" fn AllowGuestAccounts<Impl: IUserPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowGuestAccounts<Impl: IUserPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowGuestAccounts(value).into()
        }
        unsafe extern "system" fn SuggestedSelectedUser<Impl: IUserPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSuggestedSelectedUser<Impl: IUserPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSuggestedSelectedUser(&*(&value as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSingleUserAsync<Impl: IUserPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserPicker>,
            ::windows::core::GetTrustLevel,
            AllowGuestAccounts::<Impl, IMPL_OFFSET>,
            SetAllowGuestAccounts::<Impl, IMPL_OFFSET>,
            SuggestedSelectedUser::<Impl, IMPL_OFFSET>,
            SetSuggestedSelectedUser::<Impl, IMPL_OFFSET>,
            PickSingleUserAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserPickerStaticsImpl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserPickerStatics {
    const NAME: &'static str = "Windows.System.IUserPickerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserPickerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserPickerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserPickerStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IUserPickerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserPickerStatics>, ::windows::core::GetTrustLevel, IsSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserPickerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<UserWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByType(&self, r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByTypeAndStatus(&self, r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn GetFromId(&self, nonroamableid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserStatics {
    const NAME: &'static str = "Windows.System.IUserStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IUserStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAsync<Impl: IUserStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAsyncByType<Impl: IUserStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllAsyncByTypeAndStatus<Impl: IUserStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFromId<Impl: IUserStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nonroamableid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserStatics>, ::windows::core::GetTrustLevel, CreateWatcher::<Impl, IMPL_OFFSET>, FindAllAsync::<Impl, IMPL_OFFSET>, FindAllAsyncByType::<Impl, IMPL_OFFSET>, FindAllAsyncByTypeAndStatus::<Impl, IMPL_OFFSET>, GetFromId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserStatics2Impl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserStatics2 {
    const NAME: &'static str = "Windows.System.IUserStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserStatics2Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserStatics2>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserWatcherImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<UserWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Added(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationStatusChanging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStatusChanging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserWatcher {
    const NAME: &'static str = "Windows.System.IUserWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserWatcherVtbl {
        unsafe extern "system" fn Status<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Start<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Added<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAdded<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRemoved<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUpdated<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanged<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAuthenticationStatusChanged<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanging<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAuthenticationStatusChanging<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStopped<Impl: IUserWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IUserWatcher>,
            ::windows::core::GetTrustLevel,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Added::<Impl, IMPL_OFFSET>,
            RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved::<Impl, IMPL_OFFSET>,
            Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated::<Impl, IMPL_OFFSET>,
            AuthenticationStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveAuthenticationStatusChanged::<Impl, IMPL_OFFSET>,
            AuthenticationStatusChanging::<Impl, IMPL_OFFSET>,
            RemoveAuthenticationStatusChanging::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserWatcher as ::windows::core::Interface>::IID
    }
}
