#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppDisplayInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogo(&self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppDisplayInfo {
    const NAME: &'static str = "Windows.ApplicationModel.IAppDisplayInfo";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IAppDisplayInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDisplayInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDisplayInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IAppDisplayInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IAppDisplayInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogo<Impl: IAppDisplayInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogo(&*(&size as *const <super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDisplayInfo>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, Description::<Impl, IMPL_OFFSET>, GetLogo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDisplayInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayInfo(&self) -> ::windows::core::Result<AppDisplayInfo>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInfo {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInfoVtbl {
        unsafe extern "system" fn Id<Impl: IAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppUserModelId<Impl: IAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayInfo<Impl: IAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PackageFamilyName<Impl: IAppInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInfo>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>, AppUserModelId::<Impl, IMPL_OFFSET>, DisplayInfo::<Impl, IMPL_OFFSET>, PackageFamilyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo2Impl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInfo2Vtbl {
        unsafe extern "system" fn Package<Impl: IAppInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInfo2>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo3Impl: Sized {
    fn ExecutionContext(&self) -> ::windows::core::Result<AppExecutionContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInfo3 {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInfo3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInfo3Vtbl {
        unsafe extern "system" fn ExecutionContext<Impl: IAppInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppExecutionContext) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutionContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInfo3>, ::windows::core::GetTrustLevel, ExecutionContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo4Impl: Sized {
    fn SupportedFileExtensions(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInfo4 {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInfo4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInfo4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInfo4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInfo4Vtbl {
        unsafe extern "system" fn SupportedFileExtensions<Impl: IAppInfo4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFileExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInfo4>, ::windows::core::GetTrustLevel, SupportedFileExtensions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppInfoStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelId(&self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelIdForUser(&self, user: &::core::option::Option<super::System::User>, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInfoStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInfoStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInfoStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IAppInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromAppUserModelId<Impl: IAppInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromAppUserModelId(&*(&appusermodelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromAppUserModelIdForUser<Impl: IAppInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, appusermodelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFromAppUserModelIdForUser(&*(&user as *const <super::System::User as ::windows::core::Abi>::Abi as *const <super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&appusermodelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInfoStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>, GetFromAppUserModelId::<Impl, IMPL_OFFSET>, GetFromAppUserModelIdForUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallerInfoImpl: Sized {
    fn Uri(&self) -> ::windows::core::Result<super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallerInfo {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInstallerInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallerInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerInfoVtbl {
        unsafe extern "system" fn Uri<Impl: IAppInstallerInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallerInfo>, ::windows::core::GetTrustLevel, Uri::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallerInfo2Impl: Sized {
    fn OnLaunch(&self) -> ::windows::core::Result<bool>;
    fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32>;
    fn ShowPrompt(&self) -> ::windows::core::Result<bool>;
    fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool>;
    fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool>;
    fn Version(&self) -> ::windows::core::Result<PackageVersion>;
    fn LastChecked(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PausedUntil(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
    fn UpdateUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn RepairUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn PolicySource(&self) -> ::windows::core::Result<AppInstallerPolicySource>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallerInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInstallerInfo2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallerInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerInfo2Vtbl {
        unsafe extern "system" fn OnLaunch<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLaunch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoursBetweenUpdateChecks<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoursBetweenUpdateChecks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPrompt<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowPrompt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateBlocksActivation<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateBlocksActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutomaticBackgroundTask<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomaticBackgroundTask() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceUpdateFromAnyVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAutoRepairEnabled<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAutoRepairEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastChecked<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastChecked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PausedUntil<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PausedUntil() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateUris<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RepairUris<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepairUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DependencyPackageUris<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DependencyPackageUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionalPackageUris<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalPackageUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicySource<Impl: IAppInstallerInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallerPolicySource) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicySource() {
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
            ::windows::core::GetRuntimeClassName::<IAppInstallerInfo2>,
            ::windows::core::GetTrustLevel,
            OnLaunch::<Impl, IMPL_OFFSET>,
            HoursBetweenUpdateChecks::<Impl, IMPL_OFFSET>,
            ShowPrompt::<Impl, IMPL_OFFSET>,
            UpdateBlocksActivation::<Impl, IMPL_OFFSET>,
            AutomaticBackgroundTask::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            IsAutoRepairEnabled::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            LastChecked::<Impl, IMPL_OFFSET>,
            PausedUntil::<Impl, IMPL_OFFSET>,
            UpdateUris::<Impl, IMPL_OFFSET>,
            RepairUris::<Impl, IMPL_OFFSET>,
            DependencyPackageUris::<Impl, IMPL_OFFSET>,
            OptionalPackageUris::<Impl, IMPL_OFFSET>,
            PolicySource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstanceImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsCurrentInstance(&self) -> ::windows::core::Result<bool>;
    fn RedirectActivationTo(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstance {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInstance";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstanceVtbl {
        unsafe extern "system" fn Key<Impl: IAppInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentInstance<Impl: IAppInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedirectActivationTo<Impl: IAppInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RedirectActivationTo().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstance>, ::windows::core::GetTrustLevel, Key::<Impl, IMPL_OFFSET>, IsCurrentInstance::<Impl, IMPL_OFFSET>, RedirectActivationTo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstanceStaticsImpl: Sized {
    fn RecommendedInstance(&self) -> ::windows::core::Result<AppInstance>;
    fn GetActivatedEventArgs(&self) -> ::windows::core::Result<Activation::IActivatedEventArgs>;
    fn FindOrRegisterInstanceForKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<AppInstance>;
    fn Unregister(&self) -> ::windows::core::Result<()>;
    fn GetInstances(&self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppInstance>>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstanceStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IAppInstanceStatics";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstanceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstanceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstanceStaticsVtbl {
        unsafe extern "system" fn RecommendedInstance<Impl: IAppInstanceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecommendedInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivatedEventArgs<Impl: IAppInstanceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivatedEventArgs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindOrRegisterInstanceForKey<Impl: IAppInstanceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindOrRegisterInstanceForKey(&*(&key as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: IAppInstanceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister().into()
        }
        unsafe extern "system" fn GetInstances<Impl: IAppInstanceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstances() {
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
            ::windows::core::GetRuntimeClassName::<IAppInstanceStatics>,
            ::windows::core::GetTrustLevel,
            RecommendedInstance::<Impl, IMPL_OFFSET>,
            GetActivatedEventArgs::<Impl, IMPL_OFFSET>,
            FindOrRegisterInstanceForKey::<Impl, IMPL_OFFSET>,
            Unregister::<Impl, IMPL_OFFSET>,
            GetInstances::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstanceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraApplicationManagerStaticsImpl: Sized {
    fn ShowInstalledApplicationsUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraApplicationManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ICameraApplicationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraApplicationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraApplicationManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraApplicationManagerStaticsVtbl {
        unsafe extern "system" fn ShowInstalledApplicationsUI<Impl: ICameraApplicationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowInstalledApplicationsUI().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICameraApplicationManagerStatics>, ::windows::core::GetTrustLevel, ShowInstalledApplicationsUI::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraApplicationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStaticsImpl: Sized {
    fn DesignModeEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesignModeStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IDesignModeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDesignModeStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignModeStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignModeStaticsVtbl {
        unsafe extern "system" fn DesignModeEnabled<Impl: IDesignModeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesignModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDesignModeStatics>, ::windows::core::GetTrustLevel, DesignModeEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignModeStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStatics2Impl: Sized {
    fn DesignMode2Enabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesignModeStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.IDesignModeStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDesignModeStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesignModeStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesignModeStatics2Vtbl {
        unsafe extern "system" fn DesignMode2Enabled<Impl: IDesignModeStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesignMode2Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDesignModeStatics2>, ::windows::core::GetTrustLevel, DesignMode2Enabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignModeStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IEnteredBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IEnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IEnteredBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl IEnteredBackgroundEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnteredBackgroundEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnteredBackgroundEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: IEnteredBackgroundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnteredBackgroundEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullTrustProcessLaunchResultImpl: Sized {
    fn LaunchResult(&self) -> ::windows::core::Result<FullTrustLaunchResult>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFullTrustProcessLaunchResult {
    const NAME: &'static str = "Windows.ApplicationModel.IFullTrustProcessLaunchResult";
}
#[cfg(feature = "implement_exclusive")]
impl IFullTrustProcessLaunchResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullTrustProcessLaunchResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFullTrustProcessLaunchResultVtbl {
        unsafe extern "system" fn LaunchResult<Impl: IFullTrustProcessLaunchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FullTrustLaunchResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IFullTrustProcessLaunchResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFullTrustProcessLaunchResult>, ::windows::core::GetTrustLevel, LaunchResult::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLaunchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFullTrustProcessLauncherStaticsImpl: Sized {
    fn LaunchFullTrustProcessForCurrentAppAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForCurrentAppWithParametersAsync(&self, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppWithParametersAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFullTrustProcessLauncherStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IFullTrustProcessLauncherStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFullTrustProcessLauncherStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullTrustProcessLauncherStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFullTrustProcessLauncherStaticsVtbl {
        unsafe extern "system" fn LaunchFullTrustProcessForCurrentAppAsync<Impl: IFullTrustProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForCurrentAppAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFullTrustProcessForCurrentAppWithParametersAsync<Impl: IFullTrustProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parametergroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForCurrentAppWithParametersAsync(&*(&parametergroupid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFullTrustProcessForAppAsync<Impl: IFullTrustProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForAppAsync(&*(&fulltrustpackagerelativeappid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFullTrustProcessForAppWithParametersAsync<Impl: IFullTrustProcessLauncherStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, parametergroupid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForAppWithParametersAsync(&*(&fulltrustpackagerelativeappid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&parametergroupid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IFullTrustProcessLauncherStatics>,
            ::windows::core::GetTrustLevel,
            LaunchFullTrustProcessForCurrentAppAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForCurrentAppWithParametersAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForAppAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForAppWithParametersAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFullTrustProcessLauncherStatics2Impl: Sized {
    fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(&self, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
    fn LaunchFullTrustProcessForAppWithArgumentsAsync(&self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFullTrustProcessLauncherStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.IFullTrustProcessLauncherStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFullTrustProcessLauncherStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFullTrustProcessLauncherStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFullTrustProcessLauncherStatics2Vtbl {
        unsafe extern "system" fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync<Impl: IFullTrustProcessLauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandline: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(&*(&commandline as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFullTrustProcessForAppWithArgumentsAsync<Impl: IFullTrustProcessLauncherStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fulltrustpackagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, commandline: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchFullTrustProcessForAppWithArgumentsAsync(&*(&fulltrustpackagerelativeappid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&commandline as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFullTrustProcessLauncherStatics2>, ::windows::core::GetTrustLevel, LaunchFullTrustProcessForCurrentAppWithArgumentsAsync::<Impl, IMPL_OFFSET>, LaunchFullTrustProcessForAppWithArgumentsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLauncherStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ILeavingBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ILeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ILeavingBackgroundEventArgs";
}
#[cfg(feature = "Foundation")]
impl ILeavingBackgroundEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILeavingBackgroundEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILeavingBackgroundEventArgsVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ILeavingBackgroundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILeavingBackgroundEventArgs>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILimitedAccessFeatureRequestResultImpl: Sized {
    fn FeatureId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<LimitedAccessFeatureStatus>;
    fn EstimatedRemovalDate(&self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILimitedAccessFeatureRequestResult {
    const NAME: &'static str = "Windows.ApplicationModel.ILimitedAccessFeatureRequestResult";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILimitedAccessFeatureRequestResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILimitedAccessFeatureRequestResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILimitedAccessFeatureRequestResultVtbl {
        unsafe extern "system" fn FeatureId<Impl: ILimitedAccessFeatureRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FeatureId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ILimitedAccessFeatureRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LimitedAccessFeatureStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EstimatedRemovalDate<Impl: ILimitedAccessFeatureRequestResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EstimatedRemovalDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILimitedAccessFeatureRequestResult>, ::windows::core::GetTrustLevel, FeatureId::<Impl, IMPL_OFFSET>, Status::<Impl, IMPL_OFFSET>, EstimatedRemovalDate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILimitedAccessFeatureRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimitedAccessFeaturesStaticsImpl: Sized {
    fn TryUnlockFeature(&self, featureid: &::windows::core::HSTRING, token: &::windows::core::HSTRING, attestation: &::windows::core::HSTRING) -> ::windows::core::Result<LimitedAccessFeatureRequestResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILimitedAccessFeaturesStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ILimitedAccessFeaturesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILimitedAccessFeaturesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILimitedAccessFeaturesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILimitedAccessFeaturesStaticsVtbl {
        unsafe extern "system" fn TryUnlockFeature<Impl: ILimitedAccessFeaturesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, featureid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, token: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, attestation: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryUnlockFeature(
                &*(&featureid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&token as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&attestation as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILimitedAccessFeaturesStatics>, ::windows::core::GetTrustLevel, TryUnlockFeature::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILimitedAccessFeaturesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IPackageImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<PackageId>;
    fn InstalledLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn IsFramework(&self) -> ::windows::core::Result<bool>;
    fn Dependencies(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IPackageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageVtbl {
        unsafe extern "system" fn Id<Impl: IPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstalledLocation<Impl: IPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFramework<Impl: IPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFramework() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dependencies<Impl: IPackageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dependencies() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackage>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>, InstalledLocation::<Impl, IMPL_OFFSET>, IsFramework::<Impl, IMPL_OFFSET>, Dependencies::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage2Impl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Logo(&self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn IsResourcePackage(&self) -> ::windows::core::Result<bool>;
    fn IsBundle(&self) -> ::windows::core::Result<bool>;
    fn IsDevelopmentMode(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage2 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage2Vtbl {
        unsafe extern "system" fn DisplayName<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PublisherDisplayName<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Logo<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Logo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsResourcePackage<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsResourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBundle<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBundle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDevelopmentMode<Impl: IPackage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDevelopmentMode() {
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
            ::windows::core::GetRuntimeClassName::<IPackage2>,
            ::windows::core::GetTrustLevel,
            DisplayName::<Impl, IMPL_OFFSET>,
            PublisherDisplayName::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            Logo::<Impl, IMPL_OFFSET>,
            IsResourcePackage::<Impl, IMPL_OFFSET>,
            IsBundle::<Impl, IMPL_OFFSET>,
            IsDevelopmentMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackage3Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<PackageStatus>;
    fn InstalledDate(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetAppListEntriesAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage3 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage3";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage3Vtbl {
        unsafe extern "system" fn Status<Impl: IPackage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstalledDate<Impl: IPackage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppListEntriesAsync<Impl: IPackage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppListEntriesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackage3>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, InstalledDate::<Impl, IMPL_OFFSET>, GetAppListEntriesAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage4Impl: Sized {
    fn SignatureKind(&self) -> ::windows::core::Result<PackageSignatureKind>;
    fn IsOptional(&self) -> ::windows::core::Result<bool>;
    fn VerifyContentIntegrityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage4 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackage4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage4Vtbl {
        unsafe extern "system" fn SignatureKind<Impl: IPackage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageSignatureKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignatureKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOptional<Impl: IPackage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOptional() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyContentIntegrityAsync<Impl: IPackage4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyContentIntegrityAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackage4>, ::windows::core::GetTrustLevel, SignatureKind::<Impl, IMPL_OFFSET>, IsOptional::<Impl, IMPL_OFFSET>, VerifyContentIntegrityAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackage5Impl: Sized {
    fn GetContentGroupsAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn GetContentGroupAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>>;
    fn StageContentGroupsAsync(&self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn StageContentGroupsWithPriorityAsync(&self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, movetoheadofqueue: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn SetInUseAsync(&self, inuse: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage5 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage5";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackage5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage5Vtbl {
        unsafe extern "system" fn GetContentGroupsAsync<Impl: IPackage5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentGroupsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentGroupAsync<Impl: IPackage5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentGroupAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageContentGroupsAsync<Impl: IPackage5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageContentGroupsAsync(&*(&names as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageContentGroupsWithPriorityAsync<Impl: IPackage5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, names: ::windows::core::RawPtr, movetoheadofqueue: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageContentGroupsWithPriorityAsync(&*(&names as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), movetoheadofqueue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUseAsync<Impl: IPackage5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inuse: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInUseAsync(inuse) {
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
            ::windows::core::GetRuntimeClassName::<IPackage5>,
            ::windows::core::GetTrustLevel,
            GetContentGroupsAsync::<Impl, IMPL_OFFSET>,
            GetContentGroupAsync::<Impl, IMPL_OFFSET>,
            StageContentGroupsAsync::<Impl, IMPL_OFFSET>,
            StageContentGroupsWithPriorityAsync::<Impl, IMPL_OFFSET>,
            SetInUseAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage6Impl: Sized {
    fn GetAppInstallerInfo(&self) -> ::windows::core::Result<AppInstallerInfo>;
    fn CheckUpdateAvailabilityAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage6 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackage6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage6Vtbl {
        unsafe extern "system" fn GetAppInstallerInfo<Impl: IPackage6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppInstallerInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckUpdateAvailabilityAsync<Impl: IPackage6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckUpdateAvailabilityAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackage6>, ::windows::core::GetTrustLevel, GetAppInstallerInfo::<Impl, IMPL_OFFSET>, CheckUpdateAvailabilityAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IPackage7Impl: Sized {
    fn MutableLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn EffectiveLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage7 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage7";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IPackage7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage7Vtbl {
        unsafe extern "system" fn MutableLocation<Impl: IPackage7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MutableLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectiveLocation<Impl: IPackage7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackage7>, ::windows::core::GetTrustLevel, MutableLocation::<Impl, IMPL_OFFSET>, EffectiveLocation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPackage8Impl: Sized {
    fn EffectiveExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn MachineExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn UserExternalLocation(&self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn InstalledPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MutablePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectivePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectiveExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MachineExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserExternalPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogoAsRandomAccessStreamReference(&self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn GetAppListEntries(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>>;
    fn IsStub(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackage8 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackage8";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IPackage8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackage8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackage8Vtbl {
        unsafe extern "system" fn EffectiveExternalLocation<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveExternalLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MachineExternalLocation<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineExternalLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserExternalLocation<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserExternalLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledPath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MutablePath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MutablePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectivePath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectivePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectiveExternalPath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveExternalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MachineExternalPath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineExternalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserExternalPath<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserExternalPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogoAsRandomAccessStreamReference<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogoAsRandomAccessStreamReference(&*(&size as *const <super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAppListEntries<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAppListEntries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStub<Impl: IPackage8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStub() {
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
            ::windows::core::GetRuntimeClassName::<IPackage8>,
            ::windows::core::GetTrustLevel,
            EffectiveExternalLocation::<Impl, IMPL_OFFSET>,
            MachineExternalLocation::<Impl, IMPL_OFFSET>,
            UserExternalLocation::<Impl, IMPL_OFFSET>,
            InstalledPath::<Impl, IMPL_OFFSET>,
            MutablePath::<Impl, IMPL_OFFSET>,
            EffectivePath::<Impl, IMPL_OFFSET>,
            EffectiveExternalPath::<Impl, IMPL_OFFSET>,
            MachineExternalPath::<Impl, IMPL_OFFSET>,
            UserExternalPath::<Impl, IMPL_OFFSET>,
            GetLogoAsRandomAccessStreamReference::<Impl, IMPL_OFFSET>,
            GetAppListEntries::<Impl, IMPL_OFFSET>,
            IsStub::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageCatalogImpl: Sized {
    fn PackageStaging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStaging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageInstalling(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageInstalling(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdating(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdating(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUninstalling(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUninstalling(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageStatusChanged(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStatusChanged(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalog";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageCatalogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogVtbl {
        unsafe extern "system" fn PackageStaging<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageStaging(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageStaging<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageStaging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageInstalling<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageInstalling(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageInstalling<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageInstalling(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageUpdating<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageUpdating(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageUpdating<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageUpdating(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageUninstalling<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageUninstalling(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageUninstalling<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageUninstalling(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageStatusChanged<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageStatusChanged(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageStatusChanged<Impl: IPackageCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageStatusChanged(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IPackageCatalog>,
            ::windows::core::GetTrustLevel,
            PackageStaging::<Impl, IMPL_OFFSET>,
            RemovePackageStaging::<Impl, IMPL_OFFSET>,
            PackageInstalling::<Impl, IMPL_OFFSET>,
            RemovePackageInstalling::<Impl, IMPL_OFFSET>,
            PackageUpdating::<Impl, IMPL_OFFSET>,
            RemovePackageUpdating::<Impl, IMPL_OFFSET>,
            PackageUninstalling::<Impl, IMPL_OFFSET>,
            RemovePackageUninstalling::<Impl, IMPL_OFFSET>,
            PackageStatusChanged::<Impl, IMPL_OFFSET>,
            RemovePackageStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageCatalog2Impl: Sized {
    fn PackageContentGroupStaging(&self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageContentGroupStaging(&self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddOptionalPackageAsync(&self, optionalpackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalog2 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalog2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageCatalog2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalog2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalog2Vtbl {
        unsafe extern "system" fn PackageContentGroupStaging<Impl: IPackageCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageContentGroupStaging(&*(&handler as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs> as ::windows::core::Abi>::Abi as *const <super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageContentGroupStaging<Impl: IPackageCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageContentGroupStaging(&*(&token as *const <super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddOptionalPackageAsync<Impl: IPackageCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddOptionalPackageAsync(&*(&optionalpackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalog2>, ::windows::core::GetTrustLevel, PackageContentGroupStaging::<Impl, IMPL_OFFSET>, RemovePackageContentGroupStaging::<Impl, IMPL_OFFSET>, AddOptionalPackageAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalog3Impl: Sized {
    fn RemoveOptionalPackagesAsync(&self, optionalpackagefamilynames: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalog3 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalog3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageCatalog3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalog3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalog3Vtbl {
        unsafe extern "system" fn RemoveOptionalPackagesAsync<Impl: IPackageCatalog3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionalpackagefamilynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveOptionalPackagesAsync(&*(&optionalpackagefamilynames as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalog3>, ::windows::core::GetTrustLevel, RemoveOptionalPackagesAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalog4Impl: Sized {
    fn AddResourcePackageAsync(&self, resourcepackagefamilyname: &::windows::core::HSTRING, resourceid: &::windows::core::HSTRING, options: AddResourcePackageOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>>;
    fn RemoveResourcePackagesAsync(&self, resourcepackages: &::core::option::Option<super::Foundation::Collections::IIterable<Package>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalog4 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalog4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageCatalog4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalog4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalog4Vtbl {
        unsafe extern "system" fn AddResourcePackageAsync<Impl: IPackageCatalog4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcepackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, resourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: AddResourcePackageOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddResourcePackageAsync(&*(&resourcepackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&resourceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResourcePackagesAsync<Impl: IPackageCatalog4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourcepackages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveResourcePackagesAsync(&*(&resourcepackages as *const <super::Foundation::Collections::IIterable<Package> as ::windows::core::Abi>::Abi as *const <super::Foundation::Collections::IIterable<Package> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalog4>, ::windows::core::GetTrustLevel, AddResourcePackageAsync::<Impl, IMPL_OFFSET>, RemoveResourcePackagesAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddOptionalPackageResultImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageCatalogAddOptionalPackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageCatalogAddOptionalPackageResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogAddOptionalPackageResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogAddOptionalPackageResultVtbl {
        unsafe extern "system" fn Package<Impl: IPackageCatalogAddOptionalPackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IPackageCatalogAddOptionalPackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalogAddOptionalPackageResult>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogAddOptionalPackageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddResourcePackageResultImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageCatalogAddResourcePackageResult {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageCatalogAddResourcePackageResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogAddResourcePackageResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogAddResourcePackageResultVtbl {
        unsafe extern "system" fn Package<Impl: IPackageCatalogAddResourcePackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageCatalogAddResourcePackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IPackageCatalogAddResourcePackageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalogAddResourcePackageResult>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>, IsComplete::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogAddResourcePackageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalogRemoveOptionalPackagesResultImpl: Sized {
    fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalogRemoveOptionalPackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageCatalogRemoveOptionalPackagesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogRemoveOptionalPackagesResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogRemoveOptionalPackagesResultVtbl {
        unsafe extern "system" fn PackagesRemoved<Impl: IPackageCatalogRemoveOptionalPackagesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackagesRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IPackageCatalogRemoveOptionalPackagesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalogRemoveOptionalPackagesResult>, ::windows::core::GetTrustLevel, PackagesRemoved::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogRemoveOptionalPackagesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalogRemoveResourcePackagesResultImpl: Sized {
    fn PackagesRemoved(&self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageCatalogRemoveResourcePackagesResult {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageCatalogRemoveResourcePackagesResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogRemoveResourcePackagesResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogRemoveResourcePackagesResultVtbl {
        unsafe extern "system" fn PackagesRemoved<Impl: IPackageCatalogRemoveResourcePackagesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackagesRemoved() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IPackageCatalogRemoveResourcePackagesResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalogRemoveResourcePackagesResult>, ::windows::core::GetTrustLevel, PackagesRemoved::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogRemoveResourcePackagesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogStaticsImpl: Sized {
    fn OpenForCurrentPackage(&self) -> ::windows::core::Result<PackageCatalog>;
    fn OpenForCurrentUser(&self) -> ::windows::core::Result<PackageCatalog>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageCatalogStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageCatalogStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageCatalogStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageCatalogStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageCatalogStaticsVtbl {
        unsafe extern "system" fn OpenForCurrentPackage<Impl: IPackageCatalogStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenForCurrentPackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenForCurrentUser<Impl: IPackageCatalogStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenForCurrentUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageCatalogStatics>, ::windows::core::GetTrustLevel, OpenForCurrentPackage::<Impl, IMPL_OFFSET>, OpenForCurrentUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<PackageContentGroupState>;
    fn IsRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageContentGroup {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageContentGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageContentGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageContentGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageContentGroupVtbl {
        unsafe extern "system" fn Package<Impl: IPackageContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPackageContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn State<Impl: IPackageContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageContentGroupState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequired<Impl: IPackageContentGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageContentGroup>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>, IsRequired::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStagingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ContentGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsContentGroupRequired(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageContentGroupStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageContentGroupStagingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageContentGroupStagingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageContentGroupStagingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageContentGroupStagingEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentGroupName<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentGroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentGroupRequired<Impl: IPackageContentGroupStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentGroupRequired() {
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
            ::windows::core::GetRuntimeClassName::<IPackageContentGroupStagingEventArgs>,
            ::windows::core::GetTrustLevel,
            ActivityId::<Impl, IMPL_OFFSET>,
            Package::<Impl, IMPL_OFFSET>,
            Progress::<Impl, IMPL_OFFSET>,
            IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode::<Impl, IMPL_OFFSET>,
            ContentGroupName::<Impl, IMPL_OFFSET>,
            IsContentGroupRequired::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroupStagingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStaticsImpl: Sized {
    fn RequiredGroupName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageContentGroupStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageContentGroupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageContentGroupStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageContentGroupStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageContentGroupStaticsVtbl {
        unsafe extern "system" fn RequiredGroupName<Impl: IPackageContentGroupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredGroupName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageContentGroupStatics>, ::windows::core::GetTrustLevel, RequiredGroupName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IPackageIdImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&self) -> ::windows::core::Result<PackageVersion>;
    fn Architecture(&self) -> ::windows::core::Result<super::System::ProcessorArchitecture>;
    fn ResourceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageId {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageId";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IPackageIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageIdVtbl {
        unsafe extern "system" fn Name<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Version<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Architecture<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::System::ProcessorArchitecture) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Architecture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceId<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publisher<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Publisher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublisherId<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullName<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FamilyName<Impl: IPackageIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FamilyName() {
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
            ::windows::core::GetRuntimeClassName::<IPackageId>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, IMPL_OFFSET>,
            Version::<Impl, IMPL_OFFSET>,
            Architecture::<Impl, IMPL_OFFSET>,
            ResourceId::<Impl, IMPL_OFFSET>,
            Publisher::<Impl, IMPL_OFFSET>,
            PublisherId::<Impl, IMPL_OFFSET>,
            FullName::<Impl, IMPL_OFFSET>,
            FamilyName::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageIdWithMetadataImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageIdWithMetadata {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageIdWithMetadata";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageIdWithMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageIdWithMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageIdWithMetadataVtbl {
        unsafe extern "system" fn ProductId<Impl: IPackageIdWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Author<Impl: IPackageIdWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Author() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageIdWithMetadata>, ::windows::core::GetTrustLevel, ProductId::<Impl, IMPL_OFFSET>, Author::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageIdWithMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageInstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageInstallingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageInstallingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageInstallingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageInstallingEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IPackageInstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IPackageInstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IPackageInstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageInstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IPackageInstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageInstallingEventArgs>, ::windows::core::GetTrustLevel, ActivityId::<Impl, IMPL_OFFSET>, Package::<Impl, IMPL_OFFSET>, Progress::<Impl, IMPL_OFFSET>, IsComplete::<Impl, IMPL_OFFSET>, ErrorCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageInstallingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStagingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageStagingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageStagingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageStagingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageStagingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageStagingEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IPackageStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IPackageStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IPackageStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IPackageStagingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageStagingEventArgs>, ::windows::core::GetTrustLevel, ActivityId::<Impl, IMPL_OFFSET>, Package::<Impl, IMPL_OFFSET>, Progress::<Impl, IMPL_OFFSET>, IsComplete::<Impl, IMPL_OFFSET>, ErrorCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStagingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IPackageStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageStatics>, ::windows::core::GetTrustLevel, Current::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusImpl: Sized {
    fn VerifyIsOK(&self) -> ::windows::core::Result<bool>;
    fn NotAvailable(&self) -> ::windows::core::Result<bool>;
    fn PackageOffline(&self) -> ::windows::core::Result<bool>;
    fn DataOffline(&self) -> ::windows::core::Result<bool>;
    fn Disabled(&self) -> ::windows::core::Result<bool>;
    fn NeedsRemediation(&self) -> ::windows::core::Result<bool>;
    fn LicenseIssue(&self) -> ::windows::core::Result<bool>;
    fn Modified(&self) -> ::windows::core::Result<bool>;
    fn Tampered(&self) -> ::windows::core::Result<bool>;
    fn DependencyIssue(&self) -> ::windows::core::Result<bool>;
    fn Servicing(&self) -> ::windows::core::Result<bool>;
    fn DeploymentInProgress(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageStatus {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageStatusVtbl {
        unsafe extern "system" fn VerifyIsOK<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyIsOK() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotAvailable<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageOffline<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataOffline<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disabled<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NeedsRemediation<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NeedsRemediation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LicenseIssue<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LicenseIssue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modified<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modified() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tampered<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tampered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DependencyIssue<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DependencyIssue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Servicing<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Servicing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeploymentInProgress<Impl: IPackageStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeploymentInProgress() {
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
            ::windows::core::GetRuntimeClassName::<IPackageStatus>,
            ::windows::core::GetTrustLevel,
            VerifyIsOK::<Impl, IMPL_OFFSET>,
            NotAvailable::<Impl, IMPL_OFFSET>,
            PackageOffline::<Impl, IMPL_OFFSET>,
            DataOffline::<Impl, IMPL_OFFSET>,
            Disabled::<Impl, IMPL_OFFSET>,
            NeedsRemediation::<Impl, IMPL_OFFSET>,
            LicenseIssue::<Impl, IMPL_OFFSET>,
            Modified::<Impl, IMPL_OFFSET>,
            Tampered::<Impl, IMPL_OFFSET>,
            DependencyIssue::<Impl, IMPL_OFFSET>,
            Servicing::<Impl, IMPL_OFFSET>,
            DeploymentInProgress::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatus2Impl: Sized {
    fn IsPartiallyStaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageStatus2 {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageStatus2";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageStatus2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageStatus2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageStatus2Vtbl {
        unsafe extern "system" fn IsPartiallyStaged<Impl: IPackageStatus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPartiallyStaged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageStatus2>, ::windows::core::GetTrustLevel, IsPartiallyStaged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusChangedEventArgsImpl: Sized {
    fn Package(&self) -> ::windows::core::Result<Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageStatusChangedEventArgsVtbl {
        unsafe extern "system" fn Package<Impl: IPackageStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageStatusChangedEventArgs>, ::windows::core::GetTrustLevel, Package::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUninstallingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageUninstallingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageUninstallingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUninstallingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageUninstallingEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Package() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageUninstallingEventArgs>, ::windows::core::GetTrustLevel, ActivityId::<Impl, IMPL_OFFSET>, Package::<Impl, IMPL_OFFSET>, Progress::<Impl, IMPL_OFFSET>, IsComplete::<Impl, IMPL_OFFSET>, ErrorCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUninstallingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdateAvailabilityResultImpl: Sized {
    fn Availability(&self) -> ::windows::core::Result<PackageUpdateAvailability>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageUpdateAvailabilityResult {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageUpdateAvailabilityResult";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageUpdateAvailabilityResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUpdateAvailabilityResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageUpdateAvailabilityResultVtbl {
        unsafe extern "system" fn Availability<Impl: IPackageUpdateAvailabilityResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageUpdateAvailability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Availability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IPackageUpdateAvailabilityResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageUpdateAvailabilityResult>, ::windows::core::GetTrustLevel, Availability::<Impl, IMPL_OFFSET>, ExtendedError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUpdateAvailabilityResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdatingEventArgsImpl: Sized {
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SourcePackage(&self) -> ::windows::core::Result<Package>;
    fn TargetPackage(&self) -> ::windows::core::Result<Package>;
    fn Progress(&self) -> ::windows::core::Result<f64>;
    fn IsComplete(&self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageUpdatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageUpdatingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUpdatingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageUpdatingEventArgsVtbl {
        unsafe extern "system" fn ActivityId<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SourcePackage<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetPackage<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetPackage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsComplete<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
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
            ::windows::core::GetRuntimeClassName::<IPackageUpdatingEventArgs>,
            ::windows::core::GetTrustLevel,
            ActivityId::<Impl, IMPL_OFFSET>,
            SourcePackage::<Impl, IMPL_OFFSET>,
            TargetPackage::<Impl, IMPL_OFFSET>,
            Progress::<Impl, IMPL_OFFSET>,
            IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageWithMetadataImpl: Sized {
    fn InstallDate(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetThumbnailToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Launch(&self, parameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageWithMetadata {
    const NAME: &'static str = "Windows.ApplicationModel.IPackageWithMetadata";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageWithMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageWithMetadataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageWithMetadataVtbl {
        unsafe extern "system" fn InstallDate<Impl: IPackageWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThumbnailToken<Impl: IPackageWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThumbnailToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Launch<Impl: IPackageWithMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameters: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Launch(&*(&parameters as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPackageWithMetadata>, ::windows::core::GetTrustLevel, InstallDate::<Impl, IMPL_OFFSET>, GetThumbnailToken::<Impl, IMPL_OFFSET>, Launch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageWithMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStartupTaskImpl: Sized {
    fn RequestEnableAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTaskState>>;
    fn Disable(&self) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<StartupTaskState>;
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStartupTask {
    const NAME: &'static str = "Windows.ApplicationModel.IStartupTask";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStartupTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartupTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartupTaskVtbl {
        unsafe extern "system" fn RequestEnableAsync<Impl: IStartupTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestEnableAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IStartupTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn State<Impl: IStartupTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StartupTaskState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TaskId<Impl: IStartupTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStartupTask>, ::windows::core::GetTrustLevel, RequestEnableAsync::<Impl, IMPL_OFFSET>, Disable::<Impl, IMPL_OFFSET>, State::<Impl, IMPL_OFFSET>, TaskId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStartupTaskStaticsImpl: Sized {
    fn GetForCurrentPackageAsync(&self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>>;
    fn GetAsync(&self, taskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTask>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStartupTaskStatics {
    const NAME: &'static str = "Windows.ApplicationModel.IStartupTaskStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStartupTaskStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStartupTaskStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStartupTaskStaticsVtbl {
        unsafe extern "system" fn GetForCurrentPackageAsync<Impl: IStartupTaskStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentPackageAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsync<Impl: IStartupTaskStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsync(&*(&taskid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStartupTaskStatics>, ::windows::core::GetTrustLevel, GetForCurrentPackageAsync::<Impl, IMPL_OFFSET>, GetAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTaskStatics as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISuspendingDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingDeferral";
}
impl ISuspendingDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISuspendingDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: ISuspendingDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISuspendingDeferral>, ::windows::core::GetTrustLevel, Complete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingDeferral as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingEventArgsImpl: Sized {
    fn SuspendingOperation(&self) -> ::windows::core::Result<SuspendingOperation>;
}
impl ::windows::core::RuntimeName for ISuspendingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingEventArgs";
}
impl ISuspendingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISuspendingEventArgsVtbl {
        unsafe extern "system" fn SuspendingOperation<Impl: ISuspendingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendingOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISuspendingEventArgs>, ::windows::core::GetTrustLevel, SuspendingOperation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISuspendingOperationImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<SuspendingDeferral>;
    fn Deadline(&self) -> ::windows::core::Result<super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ISuspendingOperation {
    const NAME: &'static str = "Windows.ApplicationModel.ISuspendingOperation";
}
#[cfg(feature = "Foundation")]
impl ISuspendingOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISuspendingOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISuspendingOperationVtbl {
        unsafe extern "system" fn GetDeferral<Impl: ISuspendingOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Deadline<Impl: ISuspendingOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deadline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISuspendingOperation>, ::windows::core::GetTrustLevel, GetDeferral::<Impl, IMPL_OFFSET>, Deadline::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingOperation as ::windows::core::Interface>::IID
    }
}
