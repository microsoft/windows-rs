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
    pub const fn new<Impl: IAppActivationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppActivationResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppActivationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppActivationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppActivationResult>, base.5, ExtendedError::<Impl, OFFSET>, AppResourceGroupInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoImpl: Sized {
    fn AppInfo(&self) -> ::windows::core::Result<super::ApplicationModel::AppInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoVtbl {
    pub const fn new<Impl: IAppDiagnosticInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfoVtbl {
        unsafe extern "system" fn AppInfo<Impl: IAppDiagnosticInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo>, base.5, AppInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfo2Impl: Sized {
    fn GetResourceGroups(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupInfo>>;
    fn CreateResourceGroupWatcher(&self) -> ::windows::core::Result<AppResourceGroupInfoWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfo2Vtbl {
    pub const fn new<Impl: IAppDiagnosticInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfo2Vtbl {
        unsafe extern "system" fn GetResourceGroups<Impl: IAppDiagnosticInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResourceGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResourceGroupWatcher<Impl: IAppDiagnosticInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResourceGroupWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo2>, base.5, GetResourceGroups::<Impl, OFFSET>, CreateResourceGroupWatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfo3Impl: Sized {
    fn LaunchAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppActivationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfo3 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfo3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfo3Vtbl {
    pub const fn new<Impl: IAppDiagnosticInfo3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfo3Vtbl {
        unsafe extern "system" fn LaunchAsync<Impl: IAppDiagnosticInfo3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfo3>, base.5, LaunchAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoStaticsImpl: Sized {
    fn RequestInfoAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoStaticsVtbl {
    pub const fn new<Impl: IAppDiagnosticInfoStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfoStaticsVtbl {
        unsafe extern "system" fn RequestInfoAsync<Impl: IAppDiagnosticInfoStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoStatics>, base.5, RequestInfoAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppDiagnosticInfoStatics2Impl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<AppDiagnosticInfoWatcher>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<DiagnosticAccessStatus>>;
    fn RequestInfoForPackageAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
    fn RequestInfoForAppUserModelId(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppDiagnosticInfo>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoStatics2 {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoStatics2Vtbl {
    pub const fn new<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfoStatics2Vtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForPackageAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestInfoForPackageAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForAppAsync<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestInfoForAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInfoForAppUserModelId<Impl: IAppDiagnosticInfoStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestInfoForAppUserModelId(&*(&appusermodelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoStatics2>, base.5, CreateWatcher::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>, RequestInfoForPackageAsync::<Impl, OFFSET>, RequestInfoForAppAsync::<Impl, OFFSET>, RequestInfoForAppUserModelId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppDiagnosticInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppDiagnosticInfoWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IAppDiagnosticInfoWatcherVtbl {
    pub const fn new<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfoWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppDiagnosticInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppDiagnosticInfoWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppDiagnosticInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoWatcher>, base.5, Added::<Impl, OFFSET>, RemoveAdded::<Impl, OFFSET>, Removed::<Impl, OFFSET>, RemoveRemoved::<Impl, OFFSET>, EnumerationCompleted::<Impl, OFFSET>, RemoveEnumerationCompleted::<Impl, OFFSET>, Stopped::<Impl, OFFSET>, RemoveStopped::<Impl, OFFSET>, Status::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppDiagnosticInfoWatcherEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppDiagnosticInfoWatcherEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfo<Impl: IAppDiagnosticInfoWatcherEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppDiagnosticInfoWatcherEventArgs>, base.5, AppDiagnosticInfo::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppExecutionStateChangeResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppExecutionStateChangeResultVtbl {
        unsafe extern "system" fn ExtendedError<Impl: IAppExecutionStateChangeResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppExecutionStateChangeResult>, base.5, ExtendedError::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppMemoryReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppMemoryReportVtbl {
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeakPrivateCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeakPrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitLimit<Impl: IAppMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCommitLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppMemoryReport>, base.5, PrivateCommitUsage::<Impl, OFFSET>, PeakPrivateCommitUsage::<Impl, OFFSET>, TotalCommitUsage::<Impl, OFFSET>, TotalCommitLimit::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppMemoryReport2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppMemoryReport2Vtbl {
        unsafe extern "system" fn ExpectedTotalCommitLimit<Impl: IAppMemoryReport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpectedTotalCommitLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppMemoryReport2>, base.5, ExpectedTotalCommitLimit::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppMemoryUsageLimitChangingEventArgsVtbl {
        unsafe extern "system" fn OldLimit<Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewLimit<Impl: IAppMemoryUsageLimitChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppMemoryUsageLimitChangingEventArgs>, base.5, OldLimit::<Impl, OFFSET>, NewLimit::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupBackgroundTaskReportVtbl {
        unsafe extern "system" fn TaskId<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Trigger<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EntryPoint<Impl: IAppResourceGroupBackgroundTaskReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EntryPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupBackgroundTaskReport>, base.5, TaskId::<Impl, OFFSET>, Name::<Impl, OFFSET>, Trigger::<Impl, OFFSET>, EntryPoint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn IsShared(&self) -> ::windows::core::Result<bool>;
    fn GetBackgroundTaskReports(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppResourceGroupBackgroundTaskReport>>;
    fn GetMemoryReport(&self) -> ::windows::core::Result<AppResourceGroupMemoryReport>;
    fn GetProcessDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<Diagnostics::ProcessDiagnosticInfo>>;
    fn GetStateReport(&self) -> ::windows::core::Result<AppResourceGroupStateReport>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupInfoVtbl {
    pub const fn new<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupInfoVtbl {
        unsafe extern "system" fn InstanceId<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShared<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsShared() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackgroundTaskReports<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBackgroundTaskReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMemoryReport<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessDiagnosticInfos<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProcessDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateReport<Impl: IAppResourceGroupInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStateReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfo>, base.5, InstanceId::<Impl, OFFSET>, IsShared::<Impl, OFFSET>, GetBackgroundTaskReports::<Impl, OFFSET>, GetMemoryReport::<Impl, OFFSET>, GetProcessDiagnosticInfos::<Impl, OFFSET>, GetStateReport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfo2Impl: Sized {
    fn StartSuspendAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartResumeAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
    fn StartTerminateAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AppExecutionStateChangeResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupInfo2 {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupInfo2Vtbl {
    pub const fn new<Impl: IAppResourceGroupInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupInfo2Vtbl {
        unsafe extern "system" fn StartSuspendAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartSuspendAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartResumeAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartResumeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartTerminateAsync<Impl: IAppResourceGroupInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartTerminateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfo2>, base.5, StartSuspendAsync::<Impl, OFFSET>, StartResumeAsync::<Impl, OFFSET>, StartTerminateAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcher {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupInfoWatcherVtbl {
    pub const fn new<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupInfoWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecutionStateChanged<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecutionStateChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveExecutionStateChanged<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveExecutionStateChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupInfoWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAppResourceGroupInfoWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcher>,
            base.5,
            Added::<Impl, OFFSET>,
            RemoveAdded::<Impl, OFFSET>,
            Removed::<Impl, OFFSET>,
            RemoveRemoved::<Impl, OFFSET>,
            EnumerationCompleted::<Impl, OFFSET>,
            RemoveEnumerationCompleted::<Impl, OFFSET>,
            Stopped::<Impl, OFFSET>,
            RemoveStopped::<Impl, OFFSET>,
            ExecutionStateChanged::<Impl, OFFSET>,
            RemoveExecutionStateChanged::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoWatcherEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupInfoWatcherEventArgsVtbl {
    pub const fn new<Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupInfoWatcherEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcherEventArgs>, base.5, AppDiagnosticInfos::<Impl, OFFSET>, AppResourceGroupInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl: Sized {
    fn AppDiagnosticInfos(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<AppDiagnosticInfo>>;
    fn AppResourceGroupInfo(&self) -> ::windows::core::Result<AppResourceGroupInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    const NAME: &'static str = "Windows.System.IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl {
    pub const fn new<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl {
        unsafe extern "system" fn AppDiagnosticInfos<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppDiagnosticInfos() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppResourceGroupInfo<Impl: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppResourceGroupInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs>, base.5, AppDiagnosticInfos::<Impl, OFFSET>, AppResourceGroupInfo::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupMemoryReportVtbl {
        unsafe extern "system" fn CommitUsageLimit<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitUsageLevel<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateCommitUsage<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalCommitUsage<Impl: IAppResourceGroupMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalCommitUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupMemoryReport>, base.5, CommitUsageLimit::<Impl, OFFSET>, CommitUsageLevel::<Impl, OFFSET>, PrivateCommitUsage::<Impl, OFFSET>, TotalCommitUsage::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppResourceGroupStateReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppResourceGroupStateReportVtbl {
        unsafe extern "system" fn ExecutionState<Impl: IAppResourceGroupStateReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupExecutionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecutionState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnergyQuotaState<Impl: IAppResourceGroupStateReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppResourceGroupEnergyQuotaState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnergyQuotaState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppResourceGroupStateReport>, base.5, ExecutionState::<Impl, OFFSET>, EnergyQuotaState::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerHostImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerHostVtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAppUriHandlerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHost>, base.5, Name::<Impl, OFFSET>, SetName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerHost2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerHost2Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IAppUriHandlerHost2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IAppUriHandlerHost2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHost2>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerHostFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerHostFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAppUriHandlerHostFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerHostFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistrationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn User(&self) -> ::windows::core::Result<User>;
    fn GetAppAddedHostsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<AppUriHandlerHost>>>;
    fn SetAppAddedHostsAsync(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistrationVtbl {
    pub const fn new<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistrationVtbl {
        unsafe extern "system" fn Name<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppAddedHostsAsync<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAppAddedHostsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppAddedHostsAsync<Impl: IAppUriHandlerRegistrationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAppAddedHostsAsync(&*(&hosts as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistration>, base.5, Name::<Impl, OFFSET>, User::<Impl, OFFSET>, GetAppAddedHostsAsync::<Impl, OFFSET>, SetAppAddedHostsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUriHandlerRegistration2Impl: Sized {
    fn GetAllHosts(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppUriHandlerHost>>;
    fn UpdateHosts(&self, hosts: &::core::option::Option<super::Foundation::Collections::IIterable<AppUriHandlerHost>>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUriHandlerRegistration2 {
    const NAME: &'static str = "Windows.System.IAppUriHandlerRegistration2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUriHandlerRegistration2Vtbl {
    pub const fn new<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistration2Vtbl {
        unsafe extern "system" fn GetAllHosts<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllHosts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateHosts<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hosts: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateHosts(&*(&hosts as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<AppUriHandlerHost> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistration2>, base.5, GetAllHosts::<Impl, OFFSET>, UpdateHosts::<Impl, OFFSET>, PackageFamilyName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerRegistrationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistrationManagerVtbl {
        unsafe extern "system" fn User<Impl: IAppUriHandlerRegistrationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetRegistration<Impl: IAppUriHandlerRegistrationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetRegistration(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManager>, base.5, User::<Impl, OFFSET>, TryGetRegistration::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerRegistrationManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistrationManager2Vtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: IAppUriHandlerRegistrationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManager2>, base.5, PackageFamilyName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistrationManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IAppUriHandlerRegistrationManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManagerStatics>, base.5, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppUriHandlerRegistrationManagerStatics2Vtbl {
        unsafe extern "system" fn GetForPackage<Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForPackage(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForPackageForUser<Impl: IAppUriHandlerRegistrationManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForPackageForUser(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppUriHandlerRegistrationManagerStatics2>, base.5, GetForPackage::<Impl, OFFSET>, GetForPackageForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDateTimeSettingsStaticsImpl: Sized {
    fn SetSystemDateTime(&self, utcdatetime: &super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDateTimeSettingsStatics {
    const NAME: &'static str = "Windows.System.IDateTimeSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDateTimeSettingsStaticsVtbl {
    pub const fn new<Impl: IDateTimeSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDateTimeSettingsStaticsVtbl {
        unsafe extern "system" fn SetSystemDateTime<Impl: IDateTimeSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, utcdatetime: super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSystemDateTime(&*(&utcdatetime as *const <super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDateTimeSettingsStatics>, base.5, SetSystemDateTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueImpl: Sized {
    fn CreateTimer(&self) -> ::windows::core::Result<DispatcherQueueTimer>;
    fn TryEnqueue(&self, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: &::core::option::Option<DispatcherQueueHandler>) -> ::windows::core::Result<bool>;
    fn ShutdownStarting(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownStarting(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShutdownCompleted(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveShutdownCompleted(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueue {
    const NAME: &'static str = "Windows.System.IDispatcherQueue";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueVtbl {
    pub const fn new<Impl: IDispatcherQueueImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueVtbl {
        unsafe extern "system" fn CreateTimer<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTimer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnqueue<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryEnqueue(&*(&callback as *const <DispatcherQueueHandler as ::windows::core::Abi>::Abi as *const <DispatcherQueueHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryEnqueueWithPriority<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: DispatcherQueuePriority, callback: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryEnqueueWithPriority(priority, &*(&callback as *const <DispatcherQueueHandler as ::windows::core::Abi>::Abi as *const <DispatcherQueueHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownStarting<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownStarting(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShutdownStarting<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveShutdownStarting(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShutdownCompleted<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueue, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShutdownCompleted<Impl: IDispatcherQueueImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveShutdownCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueue>, base.5, CreateTimer::<Impl, OFFSET>, TryEnqueue::<Impl, OFFSET>, TryEnqueueWithPriority::<Impl, OFFSET>, ShutdownStarting::<Impl, OFFSET>, RemoveShutdownStarting::<Impl, OFFSET>, ShutdownCompleted::<Impl, OFFSET>, RemoveShutdownCompleted::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDispatcherQueue2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueue2Vtbl {
        unsafe extern "system" fn HasThreadAccess<Impl: IDispatcherQueue2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasThreadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueue2>, base.5, HasThreadAccess::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueControllerImpl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<DispatcherQueue>;
    fn ShutdownQueueAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueController {
    const NAME: &'static str = "Windows.System.IDispatcherQueueController";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueControllerVtbl {
    pub const fn new<Impl: IDispatcherQueueControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueControllerVtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: IDispatcherQueueControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownQueueAsync<Impl: IDispatcherQueueControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShutdownQueueAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueueController>, base.5, DispatcherQueue::<Impl, OFFSET>, ShutdownQueueAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDispatcherQueueControllerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueControllerStaticsVtbl {
        unsafe extern "system" fn CreateOnDedicatedThread<Impl: IDispatcherQueueControllerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateOnDedicatedThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueueControllerStatics>, base.5, CreateOnDedicatedThread::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherQueueShutdownStartingEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueShutdownStartingEventArgs {
    const NAME: &'static str = "Windows.System.IDispatcherQueueShutdownStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueShutdownStartingEventArgsVtbl {
    pub const fn new<Impl: IDispatcherQueueShutdownStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueShutdownStartingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IDispatcherQueueShutdownStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueueShutdownStartingEventArgs>, base.5, GetDeferral::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDispatcherQueueStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueStaticsVtbl {
        unsafe extern "system" fn GetForCurrentThread<Impl: IDispatcherQueueStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueueStatics>, base.5, GetForCurrentThread::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherQueueTimer {
    const NAME: &'static str = "Windows.System.IDispatcherQueueTimer";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherQueueTimerVtbl {
    pub const fn new<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherQueueTimerVtbl {
        unsafe extern "system" fn Interval<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterval(&*(&value as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsRunning<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRunning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRepeating<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRepeating() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRepeating<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsRepeating(value).into()
        }
        unsafe extern "system" fn Start<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Tick<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tick(&*(&handler as *const <super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<DispatcherQueueTimer, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTick<Impl: IDispatcherQueueTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTick(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherQueueTimer>, base.5, Interval::<Impl, OFFSET>, SetInterval::<Impl, OFFSET>, IsRunning::<Impl, OFFSET>, IsRepeating::<Impl, OFFSET>, SetIsRepeating::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Tick::<Impl, OFFSET>, RemoveTick::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFolderLauncherOptionsImpl: Sized {
    fn ItemsToSelect(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<super::Storage::IStorageItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFolderLauncherOptions {
    const NAME: &'static str = "Windows.System.IFolderLauncherOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFolderLauncherOptionsVtbl {
    pub const fn new<Impl: IFolderLauncherOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFolderLauncherOptionsVtbl {
        unsafe extern "system" fn ItemsToSelect<Impl: IFolderLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsToSelect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFolderLauncherOptions>, base.5, ItemsToSelect::<Impl, OFFSET>)
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
    pub const fn new<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownUserPropertiesStaticsVtbl {
        unsafe extern "system" fn DisplayName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FirstName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccountName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GuestHost<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GuestHost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrincipalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainName<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionInitiationProtocolUri<Impl: IKnownUserPropertiesStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionInitiationProtocolUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownUserPropertiesStatics>, base.5, DisplayName::<Impl, OFFSET>, FirstName::<Impl, OFFSET>, LastName::<Impl, OFFSET>, ProviderName::<Impl, OFFSET>, AccountName::<Impl, OFFSET>, GuestHost::<Impl, OFFSET>, PrincipalName::<Impl, OFFSET>, DomainName::<Impl, OFFSET>, SessionInitiationProtocolUri::<Impl, OFFSET>)
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
    pub const fn new<Impl: IKnownUserPropertiesStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKnownUserPropertiesStatics2Vtbl {
        unsafe extern "system" fn AgeEnforcementRegion<Impl: IKnownUserPropertiesStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AgeEnforcementRegion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKnownUserPropertiesStatics2>, base.5, AgeEnforcementRegion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILaunchUriResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LaunchUriStatus>;
    fn Result(&self) -> ::windows::core::Result<super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILaunchUriResult {
    const NAME: &'static str = "Windows.System.ILaunchUriResult";
}
#[cfg(feature = "implement_exclusive")]
impl ILaunchUriResultVtbl {
    pub const fn new<Impl: ILaunchUriResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILaunchUriResultVtbl {
        unsafe extern "system" fn Status<Impl: ILaunchUriResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LaunchUriStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Result<Impl: ILaunchUriResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILaunchUriResult>, base.5, Status::<Impl, OFFSET>, Result::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions {
    const NAME: &'static str = "Windows.System.ILauncherOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptionsVtbl {
    pub const fn new<Impl: ILauncherOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherOptionsVtbl {
        unsafe extern "system" fn TreatAsUntrusted<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TreatAsUntrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTreatAsUntrusted<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTreatAsUntrusted(value).into()
        }
        unsafe extern "system" fn DisplayApplicationPicker<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayApplicationPicker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayApplicationPicker<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayApplicationPicker(value).into()
        }
        unsafe extern "system" fn UI<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferredApplicationPackageFamilyName<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredApplicationPackageFamilyName<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredApplicationDisplayName<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredApplicationDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredApplicationDisplayName<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredApplicationDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FallbackUri<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackUri<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentType<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentType<Impl: ILauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentType(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILauncherOptions>,
            base.5,
            TreatAsUntrusted::<Impl, OFFSET>,
            SetTreatAsUntrusted::<Impl, OFFSET>,
            DisplayApplicationPicker::<Impl, OFFSET>,
            SetDisplayApplicationPicker::<Impl, OFFSET>,
            UI::<Impl, OFFSET>,
            PreferredApplicationPackageFamilyName::<Impl, OFFSET>,
            SetPreferredApplicationPackageFamilyName::<Impl, OFFSET>,
            PreferredApplicationDisplayName::<Impl, OFFSET>,
            SetPreferredApplicationDisplayName::<Impl, OFFSET>,
            FallbackUri::<Impl, OFFSET>,
            SetFallbackUri::<Impl, OFFSET>,
            ContentType::<Impl, OFFSET>,
            SetContentType::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherOptions2Impl: Sized {
    fn TargetApplicationPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTargetApplicationPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::Storage::Search::StorageFileQueryResult>;
    fn SetNeighboringFilesQuery(&self, value: &::core::option::Option<super::Storage::Search::StorageFileQueryResult>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherOptions2 {
    const NAME: &'static str = "Windows.System.ILauncherOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherOptions2Vtbl {
    pub const fn new<Impl: ILauncherOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherOptions2Vtbl {
        unsafe extern "system" fn TargetApplicationPackageFamilyName<Impl: ILauncherOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetApplicationPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetApplicationPackageFamilyName<Impl: ILauncherOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetApplicationPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NeighboringFilesQuery<Impl: ILauncherOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NeighboringFilesQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNeighboringFilesQuery<Impl: ILauncherOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNeighboringFilesQuery(&*(&value as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::Abi>::Abi as *const <super::Storage::Search::StorageFileQueryResult as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherOptions2>, base.5, TargetApplicationPackageFamilyName::<Impl, OFFSET>, SetTargetApplicationPackageFamilyName::<Impl, OFFSET>, NeighboringFilesQuery::<Impl, OFFSET>, SetNeighboringFilesQuery::<Impl, OFFSET>)
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
    pub const fn new<Impl: ILauncherOptions3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherOptions3Vtbl {
        unsafe extern "system" fn IgnoreAppUriHandlers<Impl: ILauncherOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IgnoreAppUriHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIgnoreAppUriHandlers<Impl: ILauncherOptions3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIgnoreAppUriHandlers(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherOptions3>, base.5, IgnoreAppUriHandlers::<Impl, OFFSET>, SetIgnoreAppUriHandlers::<Impl, OFFSET>)
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
    pub const fn new<Impl: ILauncherOptions4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherOptions4Vtbl {
        unsafe extern "system" fn LimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LimitPickerToCurrentAppAndAppUriHandlers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLimitPickerToCurrentAppAndAppUriHandlers<Impl: ILauncherOptions4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLimitPickerToCurrentAppAndAppUriHandlers(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherOptions4>, base.5, LimitPickerToCurrentAppAndAppUriHandlers::<Impl, OFFSET>, SetLimitPickerToCurrentAppAndAppUriHandlers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStaticsImpl: Sized {
    fn LaunchFileAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFileWithOptionsAsync(&self, file: &::core::option::Option<super::Storage::IStorageFile>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchUriWithOptionsAsync(&self, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<LauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherStatics {
    const NAME: &'static str = "Windows.System.ILauncherStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherStaticsVtbl {
    pub const fn new<Impl: ILauncherStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherStaticsVtbl {
        unsafe extern "system" fn LaunchFileAsync<Impl: ILauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFileAsync(&*(&file as *const <super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFileWithOptionsAsync<Impl: ILauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFileWithOptionsAsync(&*(&file as *const <super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriAsync<Impl: ILauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: ILauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithOptionsAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherStatics>, base.5, LaunchFileAsync::<Impl, OFFSET>, LaunchFileWithOptionsAsync::<Impl, OFFSET>, LaunchUriAsync::<Impl, OFFSET>, LaunchUriWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherStatics2 {
    const NAME: &'static str = "Windows.System.ILauncherStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherStatics2Vtbl {
    pub const fn new<Impl: ILauncherStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherStatics2Vtbl {
        unsafe extern "system" fn LaunchUriForResultsAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForResultsWithDataAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn QueryUriSupportAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryUriSupportAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, launchquerysupporttype: LaunchQuerySupportType, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryUriSupportWithPackageFamilyNameAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype, &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFileSupportAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryFileSupportAsync(&*(&file as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryFileSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryFileSupportWithPackageFamilyNameAsync(&*(&file as *const <super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUriSchemeHandlersAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindUriSchemeHandlersAsync(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUriSchemeHandlersWithLaunchUriTypeAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scheme: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, launchquerysupporttype: LaunchQuerySupportType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindUriSchemeHandlersWithLaunchUriTypeAsync(&*(&scheme as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), launchquerysupporttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFileHandlersAsync<Impl: ILauncherStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILauncherStatics2>,
            base.5,
            LaunchUriForResultsAsync::<Impl, OFFSET>,
            LaunchUriForResultsWithDataAsync::<Impl, OFFSET>,
            LaunchUriWithDataAsync::<Impl, OFFSET>,
            QueryUriSupportAsync::<Impl, OFFSET>,
            QueryUriSupportWithPackageFamilyNameAsync::<Impl, OFFSET>,
            QueryFileSupportAsync::<Impl, OFFSET>,
            QueryFileSupportWithPackageFamilyNameAsync::<Impl, OFFSET>,
            FindUriSchemeHandlersAsync::<Impl, OFFSET>,
            FindUriSchemeHandlersWithLaunchUriTypeAsync::<Impl, OFFSET>,
            FindFileHandlersAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics3Impl: Sized {
    fn LaunchFolderAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderWithOptionsAsync(&self, folder: &::core::option::Option<super::Storage::IStorageFolder>, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherStatics3 {
    const NAME: &'static str = "Windows.System.ILauncherStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherStatics3Vtbl {
    pub const fn new<Impl: ILauncherStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherStatics3Vtbl {
        unsafe extern "system" fn LaunchFolderAsync<Impl: ILauncherStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderAsync(&*(&folder as *const <super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderWithOptionsAsync<Impl: ILauncherStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderWithOptionsAsync(&*(&folder as *const <super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherStatics3>, base.5, LaunchFolderAsync::<Impl, OFFSET>, LaunchFolderWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherStatics4 {
    const NAME: &'static str = "Windows.System.ILauncherStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherStatics4Vtbl {
    pub const fn new<Impl: ILauncherStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherStatics4Vtbl {
        unsafe extern "system" fn QueryAppUriSupportAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryAppUriSupportAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryAppUriSupportWithPackageFamilyNameAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QueryAppUriSupportWithPackageFamilyNameAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAppUriHandlersAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAppUriHandlersAsync(&*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriWithOptionsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithDataForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn LaunchUriForResultsForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriForResultsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <LauncherOptions as ::windows::core::Abi>::Abi as *const <LauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriForResultsWithDataForUserAsync<Impl: ILauncherStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILauncherStatics4>,
            base.5,
            QueryAppUriSupportAsync::<Impl, OFFSET>,
            QueryAppUriSupportWithPackageFamilyNameAsync::<Impl, OFFSET>,
            FindAppUriHandlersAsync::<Impl, OFFSET>,
            LaunchUriForUserAsync::<Impl, OFFSET>,
            LaunchUriWithOptionsForUserAsync::<Impl, OFFSET>,
            LaunchUriWithDataForUserAsync::<Impl, OFFSET>,
            LaunchUriForResultsForUserAsync::<Impl, OFFSET>,
            LaunchUriForResultsWithDataForUserAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherStatics5Impl: Sized {
    fn LaunchFolderPathAsync(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsAsync(&self, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
    fn LaunchFolderPathWithOptionsForUserAsync(&self, user: &::core::option::Option<User>, path: &::windows::core::HSTRING, options: &::core::option::Option<FolderLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherStatics5 {
    const NAME: &'static str = "Windows.System.ILauncherStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherStatics5Vtbl {
    pub const fn new<Impl: ILauncherStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherStatics5Vtbl {
        unsafe extern "system" fn LaunchFolderPathAsync<Impl: ILauncherStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathWithOptionsAsync<Impl: ILauncherStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathWithOptionsAsync(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathForUserAsync<Impl: ILauncherStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFolderPathWithOptionsForUserAsync<Impl: ILauncherStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFolderPathWithOptionsForUserAsync(&*(&user as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType), &*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <FolderLauncherOptions as ::windows::core::Abi>::Abi as *const <FolderLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherStatics5>, base.5, LaunchFolderPathAsync::<Impl, OFFSET>, LaunchFolderPathWithOptionsAsync::<Impl, OFFSET>, LaunchFolderPathForUserAsync::<Impl, OFFSET>, LaunchFolderPathWithOptionsForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILauncherUIOptionsImpl: Sized {
    fn InvocationPoint(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Point>>;
    fn SetInvocationPoint(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn SelectionRect(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::Rect>>;
    fn SetSelectionRect(&self, value: &::core::option::Option<super::Foundation::IReference<super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn PreferredPlacement(&self) -> ::windows::core::Result<super::UI::Popups::Placement>;
    fn SetPreferredPlacement(&self, value: super::UI::Popups::Placement) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILauncherUIOptions {
    const NAME: &'static str = "Windows.System.ILauncherUIOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ILauncherUIOptionsVtbl {
    pub const fn new<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherUIOptionsVtbl {
        unsafe extern "system" fn InvocationPoint<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InvocationPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInvocationPoint<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInvocationPoint(&*(&value as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionRect<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionRect<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionRect(&*(&value as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::Foundation::IReference<super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredPlacement<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredPlacement<Impl: ILauncherUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::UI::Popups::Placement) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPreferredPlacement(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherUIOptions>, base.5, InvocationPoint::<Impl, OFFSET>, SetInvocationPoint::<Impl, OFFSET>, SelectionRect::<Impl, OFFSET>, SetSelectionRect::<Impl, OFFSET>, PreferredPlacement::<Impl, OFFSET>, SetPreferredPlacement::<Impl, OFFSET>)
    }
}
pub trait ILauncherViewOptionsImpl: Sized {
    fn DesiredRemainingView(&self) -> ::windows::core::Result<super::UI::ViewManagement::ViewSizePreference>;
    fn SetDesiredRemainingView(&self, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILauncherViewOptions {
    const NAME: &'static str = "Windows.System.ILauncherViewOptions";
}
impl ILauncherViewOptionsVtbl {
    pub const fn new<Impl: ILauncherViewOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILauncherViewOptionsVtbl {
        unsafe extern "system" fn DesiredRemainingView<Impl: ILauncherViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredRemainingView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRemainingView<Impl: ILauncherViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::UI::ViewManagement::ViewSizePreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredRemainingView(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILauncherViewOptions>, base.5, DesiredRemainingView::<Impl, OFFSET>, SetDesiredRemainingView::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryManagerStatics {
    const NAME: &'static str = "Windows.System.IMemoryManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryManagerStaticsVtbl {
    pub const fn new<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMemoryManagerStaticsVtbl {
        unsafe extern "system" fn AppMemoryUsage<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageLimit<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageLevel<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AppMemoryUsageLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMemoryUsageIncreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageIncreased(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageIncreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageIncreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageDecreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageDecreased(&*(&handler as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageDecreased<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageDecreased(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppMemoryUsageLimitChanging<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppMemoryUsageLimitChanging(&*(&handler as *const <super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAppMemoryUsageLimitChanging<Impl: IMemoryManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAppMemoryUsageLimitChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics>, base.5, AppMemoryUsage::<Impl, OFFSET>, AppMemoryUsageLimit::<Impl, OFFSET>, AppMemoryUsageLevel::<Impl, OFFSET>, AppMemoryUsageIncreased::<Impl, OFFSET>, RemoveAppMemoryUsageIncreased::<Impl, OFFSET>, AppMemoryUsageDecreased::<Impl, OFFSET>, RemoveAppMemoryUsageDecreased::<Impl, OFFSET>, AppMemoryUsageLimitChanging::<Impl, OFFSET>, RemoveAppMemoryUsageLimitChanging::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMemoryManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMemoryManagerStatics2Vtbl {
        unsafe extern "system" fn GetAppMemoryReport<Impl: IMemoryManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAppMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessMemoryReport<Impl: IMemoryManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProcessMemoryReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics2>, base.5, GetAppMemoryReport::<Impl, OFFSET>, GetProcessMemoryReport::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMemoryManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMemoryManagerStatics3Vtbl {
        unsafe extern "system" fn TrySetAppMemoryUsageLimit<Impl: IMemoryManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetAppMemoryUsageLimit(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics3>, base.5, TrySetAppMemoryUsageLimit::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMemoryManagerStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMemoryManagerStatics4Vtbl {
        unsafe extern "system" fn ExpectedAppMemoryUsageLimit<Impl: IMemoryManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpectedAppMemoryUsageLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMemoryManagerStatics4>, base.5, ExpectedAppMemoryUsageLimit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessLauncherOptions {
    const NAME: &'static str = "Windows.System.IProcessLauncherOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessLauncherOptionsVtbl {
    pub const fn new<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessLauncherOptionsVtbl {
        unsafe extern "system" fn StandardInput<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StandardInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardInput<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStandardInput(&*(&value as *const <super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardOutput<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StandardOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardOutput<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStandardOutput(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StandardError<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StandardError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStandardError<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStandardError(&*(&value as *const <super::Storage::Streams::IOutputStream as ::windows::core::Abi>::Abi as *const <super::Storage::Streams::IOutputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IProcessLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessLauncherOptions>, base.5, StandardInput::<Impl, OFFSET>, SetStandardInput::<Impl, OFFSET>, StandardOutput::<Impl, OFFSET>, SetStandardOutput::<Impl, OFFSET>, StandardError::<Impl, OFFSET>, SetStandardError::<Impl, OFFSET>, WorkingDirectory::<Impl, OFFSET>, SetWorkingDirectory::<Impl, OFFSET>)
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
    pub const fn new<Impl: IProcessLauncherResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessLauncherResultVtbl {
        unsafe extern "system" fn ExitCode<Impl: IProcessLauncherResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessLauncherResult>, base.5, ExitCode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessLauncherStaticsImpl: Sized {
    fn RunToCompletionAsync(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
    fn RunToCompletionAsyncWithOptions(&self, filename: &::windows::core::HSTRING, args: &::windows::core::HSTRING, options: &::core::option::Option<ProcessLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<ProcessLauncherResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessLauncherStatics {
    const NAME: &'static str = "Windows.System.IProcessLauncherStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessLauncherStaticsVtbl {
    pub const fn new<Impl: IProcessLauncherStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessLauncherStaticsVtbl {
        unsafe extern "system" fn RunToCompletionAsync<Impl: IProcessLauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunToCompletionAsync(&*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunToCompletionAsyncWithOptions<Impl: IProcessLauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, args: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunToCompletionAsyncWithOptions(&*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <ProcessLauncherOptions as ::windows::core::Abi>::Abi as *const <ProcessLauncherOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessLauncherStatics>, base.5, RunToCompletionAsync::<Impl, OFFSET>, RunToCompletionAsyncWithOptions::<Impl, OFFSET>)
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
    pub const fn new<Impl: IProcessMemoryReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessMemoryReportVtbl {
        unsafe extern "system" fn PrivateWorkingSetUsage<Impl: IProcessMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrivateWorkingSetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalWorkingSetUsage<Impl: IProcessMemoryReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TotalWorkingSetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessMemoryReport>, base.5, PrivateWorkingSetUsage::<Impl, OFFSET>, TotalWorkingSetUsage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtocolForResultsOperationImpl: Sized {
    fn ReportCompleted(&self, data: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtocolForResultsOperation {
    const NAME: &'static str = "Windows.System.IProtocolForResultsOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IProtocolForResultsOperationVtbl {
    pub const fn new<Impl: IProtocolForResultsOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProtocolForResultsOperationVtbl {
        unsafe extern "system" fn ReportCompleted<Impl: IProtocolForResultsOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReportCompleted(&*(&data as *const <super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProtocolForResultsOperation>, base.5, ReportCompleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteLauncherOptionsImpl: Sized {
    fn FallbackUri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn SetFallbackUri(&self, value: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PreferredAppIds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteLauncherOptions {
    const NAME: &'static str = "Windows.System.IRemoteLauncherOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteLauncherOptionsVtbl {
    pub const fn new<Impl: IRemoteLauncherOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRemoteLauncherOptionsVtbl {
        unsafe extern "system" fn FallbackUri<Impl: IRemoteLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FallbackUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFallbackUri<Impl: IRemoteLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFallbackUri(&*(&value as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreferredAppIds<Impl: IRemoteLauncherOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreferredAppIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRemoteLauncherOptions>, base.5, FallbackUri::<Impl, OFFSET>, SetFallbackUri::<Impl, OFFSET>, PreferredAppIds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteLauncherStaticsImpl: Sized {
    fn LaunchUriAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithOptionsAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
    fn LaunchUriWithDataAsync(&self, remotesystemconnectionrequest: &::core::option::Option<RemoteSystems::RemoteSystemConnectionRequest>, uri: &::core::option::Option<super::Foundation::Uri>, options: &::core::option::Option<RemoteLauncherOptions>, inputdata: &::core::option::Option<super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<RemoteLaunchUriStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteLauncherStatics {
    const NAME: &'static str = "Windows.System.IRemoteLauncherStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteLauncherStaticsVtbl {
    pub const fn new<Impl: IRemoteLauncherStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRemoteLauncherStaticsVtbl {
        unsafe extern "system" fn LaunchUriAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchUriAsync(&*(&remotesystemconnectionrequest as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType), &*(&uri as *const <super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUriWithOptionsAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn LaunchUriWithDataAsync<Impl: IRemoteLauncherStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, uri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, inputdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRemoteLauncherStatics>, base.5, LaunchUriAsync::<Impl, OFFSET>, LaunchUriWithOptionsAsync::<Impl, OFFSET>, LaunchUriWithDataAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShutdownManagerStaticsImpl: Sized {
    fn BeginShutdown(&self, shutdownkind: ShutdownKind, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CancelShutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShutdownManagerStatics {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IShutdownManagerStaticsVtbl {
    pub const fn new<Impl: IShutdownManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShutdownManagerStaticsVtbl {
        unsafe extern "system" fn BeginShutdown<Impl: IShutdownManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shutdownkind: ShutdownKind, timeout: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).BeginShutdown(shutdownkind, &*(&timeout as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelShutdown<Impl: IShutdownManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CancelShutdown().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShutdownManagerStatics>, base.5, BeginShutdown::<Impl, OFFSET>, CancelShutdown::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IShutdownManagerStatics2Impl: Sized + IShutdownManagerStaticsImpl {
    fn IsPowerStateSupported(&self, powerstate: PowerState) -> ::windows::core::Result<bool>;
    fn EnterPowerState(&self, powerstate: PowerState) -> ::windows::core::Result<()>;
    fn EnterPowerStateWithTimeSpan(&self, powerstate: PowerState, wakeupafter: &super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IShutdownManagerStatics2 {
    const NAME: &'static str = "Windows.System.IShutdownManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IShutdownManagerStatics2Vtbl {
    pub const fn new<Impl: IShutdownManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShutdownManagerStatics2Vtbl {
        unsafe extern "system" fn IsPowerStateSupported<Impl: IShutdownManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPowerStateSupported(powerstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnterPowerState<Impl: IShutdownManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, powerstate: PowerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnterPowerState(powerstate).into()
        }
        unsafe extern "system" fn EnterPowerStateWithTimeSpan<Impl: IShutdownManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, powerstate: PowerState, wakeupafter: super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).EnterPowerStateWithTimeSpan(powerstate, &*(&wakeupafter as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShutdownManagerStatics2>, base.5, IsPowerStateSupported::<Impl, OFFSET>, EnterPowerState::<Impl, OFFSET>, EnterPowerStateWithTimeSpan::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeZoneSettingsStaticsImpl: Sized {
    fn CurrentTimeZoneDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedTimeZoneDisplayNames(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn CanChangeTimeZone(&self) -> ::windows::core::Result<bool>;
    fn ChangeTimeZoneByDisplayName(&self, timezonedisplayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeZoneSettingsStaticsVtbl {
    pub const fn new<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimeZoneSettingsStaticsVtbl {
        unsafe extern "system" fn CurrentTimeZoneDisplayName<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentTimeZoneDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTimeZoneDisplayNames<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedTimeZoneDisplayNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanChangeTimeZone<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanChangeTimeZone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeTimeZoneByDisplayName<Impl: ITimeZoneSettingsStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timezonedisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ChangeTimeZoneByDisplayName(&*(&timezonedisplayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimeZoneSettingsStatics>, base.5, CurrentTimeZoneDisplayName::<Impl, OFFSET>, SupportedTimeZoneDisplayNames::<Impl, OFFSET>, CanChangeTimeZone::<Impl, OFFSET>, ChangeTimeZoneByDisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeZoneSettingsStatics2Impl: Sized {
    fn AutoUpdateTimeZoneAsync(&self, timeout: &super::Foundation::TimeSpan) -> ::windows::core::Result<super::Foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimeZoneSettingsStatics2 {
    const NAME: &'static str = "Windows.System.ITimeZoneSettingsStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ITimeZoneSettingsStatics2Vtbl {
    pub const fn new<Impl: ITimeZoneSettingsStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimeZoneSettingsStatics2Vtbl {
        unsafe extern "system" fn AutoUpdateTimeZoneAsync<Impl: ITimeZoneSettingsStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoUpdateTimeZoneAsync(&*(&timeout as *const <super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimeZoneSettingsStatics2>, base.5, AutoUpdateTimeZoneAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserImpl: Sized {
    fn NonRoamableId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AuthenticationStatus(&self) -> ::windows::core::Result<UserAuthenticationStatus>;
    fn Type(&self) -> ::windows::core::Result<UserType>;
    fn GetPropertyAsync(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetPropertiesAsync(&self, values: &::core::option::Option<super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IPropertySet>>;
    fn GetPictureAsync(&self, desiredsize: UserPictureSize) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Storage::Streams::IRandomAccessStreamReference>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUser {
    const NAME: &'static str = "Windows.System.IUser";
}
#[cfg(feature = "implement_exclusive")]
impl IUserVtbl {
    pub const fn new<Impl: IUserImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserVtbl {
        unsafe extern "system" fn NonRoamableId<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NonRoamableId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationStatus<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyAsync<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertyAsync(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertiesAsync<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, values: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropertiesAsync(&*(&values as *const <super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPictureAsync<Impl: IUserImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desiredsize: UserPictureSize, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPictureAsync(desiredsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUser>, base.5, NonRoamableId::<Impl, OFFSET>, AuthenticationStatus::<Impl, OFFSET>, Type::<Impl, OFFSET>, GetPropertyAsync::<Impl, OFFSET>, GetPropertiesAsync::<Impl, OFFSET>, GetPictureAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUser2Impl: Sized {
    fn CheckUserAgeConsentGroupAsync(&self, consentgroup: UserAgeConsentGroup) -> ::windows::core::Result<super::Foundation::IAsyncOperation<UserAgeConsentResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUser2 {
    const NAME: &'static str = "Windows.System.IUser2";
}
#[cfg(feature = "implement_exclusive")]
impl IUser2Vtbl {
    pub const fn new<Impl: IUser2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUser2Vtbl {
        unsafe extern "system" fn CheckUserAgeConsentGroupAsync<Impl: IUser2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, consentgroup: UserAgeConsentGroup, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckUserAgeConsentGroupAsync(consentgroup) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUser2>, base.5, CheckUserAgeConsentGroupAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserAuthenticationStatusChangeDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserAuthenticationStatusChangeDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IUserAuthenticationStatusChangeDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserAuthenticationStatusChangeDeferral>, base.5, Complete::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserAuthenticationStatusChangingEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStatus<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStatus<Impl: IUserAuthenticationStatusChangingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserAuthenticationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserAuthenticationStatusChangingEventArgs>, base.5, GetDeferral::<Impl, OFFSET>, User::<Impl, OFFSET>, NewStatus::<Impl, OFFSET>, CurrentStatus::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserChangedEventArgsVtbl {
        unsafe extern "system" fn User<Impl: IUserChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserChangedEventArgs>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserChangedEventArgs2Impl: Sized {
    fn ChangedPropertyKinds(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<UserWatcherUpdateKind>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserChangedEventArgs2 {
    const NAME: &'static str = "Windows.System.IUserChangedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserChangedEventArgs2Vtbl {
    pub const fn new<Impl: IUserChangedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserChangedEventArgs2Vtbl {
        unsafe extern "system" fn ChangedPropertyKinds<Impl: IUserChangedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChangedPropertyKinds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserChangedEventArgs2>, base.5, ChangedPropertyKinds::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDeviceAssociationChangedEventArgsVtbl {
        unsafe extern "system" fn DeviceId<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewUser<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldUser<Impl: IUserDeviceAssociationChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDeviceAssociationChangedEventArgs>, base.5, DeviceId::<Impl, OFFSET>, NewUser::<Impl, OFFSET>, OldUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDeviceAssociationStaticsImpl: Sized {
    fn FindUserFromDeviceId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
    fn UserDeviceAssociationChanged(&self, handler: &::core::option::Option<super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemoveUserDeviceAssociationChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDeviceAssociationStatics {
    const NAME: &'static str = "Windows.System.IUserDeviceAssociationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDeviceAssociationStaticsVtbl {
    pub const fn new<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserDeviceAssociationStaticsVtbl {
        unsafe extern "system" fn FindUserFromDeviceId<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindUserFromDeviceId(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDeviceAssociationChanged<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserDeviceAssociationChanged(&*(&handler as *const <super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::EventHandler<UserDeviceAssociationChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserDeviceAssociationChanged<Impl: IUserDeviceAssociationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUserDeviceAssociationChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserDeviceAssociationStatics>, base.5, FindUserFromDeviceId::<Impl, OFFSET>, UserDeviceAssociationChanged::<Impl, OFFSET>, RemoveUserDeviceAssociationChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserPickerImpl: Sized {
    fn AllowGuestAccounts(&self) -> ::windows::core::Result<bool>;
    fn SetAllowGuestAccounts(&self, value: bool) -> ::windows::core::Result<()>;
    fn SuggestedSelectedUser(&self) -> ::windows::core::Result<User>;
    fn SetSuggestedSelectedUser(&self, value: &::core::option::Option<User>) -> ::windows::core::Result<()>;
    fn PickSingleUserAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<User>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserPicker {
    const NAME: &'static str = "Windows.System.IUserPicker";
}
#[cfg(feature = "implement_exclusive")]
impl IUserPickerVtbl {
    pub const fn new<Impl: IUserPickerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserPickerVtbl {
        unsafe extern "system" fn AllowGuestAccounts<Impl: IUserPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowGuestAccounts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowGuestAccounts<Impl: IUserPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowGuestAccounts(value).into()
        }
        unsafe extern "system" fn SuggestedSelectedUser<Impl: IUserPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuggestedSelectedUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuggestedSelectedUser<Impl: IUserPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSuggestedSelectedUser(&*(&value as *const <User as ::windows::core::Abi>::Abi as *const <User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PickSingleUserAsync<Impl: IUserPickerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PickSingleUserAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserPicker>, base.5, AllowGuestAccounts::<Impl, OFFSET>, SetAllowGuestAccounts::<Impl, OFFSET>, SuggestedSelectedUser::<Impl, OFFSET>, SetSuggestedSelectedUser::<Impl, OFFSET>, PickSingleUserAsync::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserPickerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserPickerStaticsVtbl {
        unsafe extern "system" fn IsSupported<Impl: IUserPickerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserPickerStatics>, base.5, IsSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<UserWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByType(&self, r#type: UserType) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn FindAllAsyncByTypeAndStatus(&self, r#type: UserType, status: UserAuthenticationStatus) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<User>>>;
    fn GetFromId(&self, nonroamableid: &::windows::core::HSTRING) -> ::windows::core::Result<User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserStatics {
    const NAME: &'static str = "Windows.System.IUserStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserStaticsVtbl {
    pub const fn new<Impl: IUserStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IUserStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsync<Impl: IUserStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncByType<Impl: IUserStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: UserType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncByType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllAsyncByTypeAndStatus<Impl: IUserStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: UserType, status: UserAuthenticationStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindAllAsyncByTypeAndStatus(r#type, status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromId<Impl: IUserStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nonroamableid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFromId(&*(&nonroamableid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserStatics>, base.5, CreateWatcher::<Impl, OFFSET>, FindAllAsync::<Impl, OFFSET>, FindAllAsyncByType::<Impl, OFFSET>, FindAllAsyncByTypeAndStatus::<Impl, OFFSET>, GetFromId::<Impl, OFFSET>)
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
    pub const fn new<Impl: IUserStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserStatics2Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserStatics2>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserWatcher {
    const NAME: &'static str = "Windows.System.IUserWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IUserWatcherVtbl {
    pub const fn new<Impl: IUserWatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserWatcherVtbl {
        unsafe extern "system" fn Status<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UserWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Added<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanged<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatusChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationStatusChanged<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AuthenticationStatusChanging<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AuthenticationStatusChanging(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationStatusChanging<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStatusChanging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<UserWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IUserWatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUserWatcher>,
            base.5,
            Status::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Added::<Impl, OFFSET>,
            RemoveAdded::<Impl, OFFSET>,
            Removed::<Impl, OFFSET>,
            RemoveRemoved::<Impl, OFFSET>,
            Updated::<Impl, OFFSET>,
            RemoveUpdated::<Impl, OFFSET>,
            AuthenticationStatusChanged::<Impl, OFFSET>,
            RemoveAuthenticationStatusChanged::<Impl, OFFSET>,
            AuthenticationStatusChanging::<Impl, OFFSET>,
            RemoveAuthenticationStatusChanging::<Impl, OFFSET>,
            EnumerationCompleted::<Impl, OFFSET>,
            RemoveEnumerationCompleted::<Impl, OFFSET>,
            Stopped::<Impl, OFFSET>,
            RemoveStopped::<Impl, OFFSET>,
        )
    }
}
