#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IAppDisplayInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogo(&mut self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppDisplayInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            GetLogo: GetLogo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDisplayInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfoImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayInfo(&mut self) -> ::windows::core::Result<AppDisplayInfo>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            DisplayInfo: DisplayInfo::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo2Impl: Sized {
    fn Package(&mut self) -> ::windows::core::Result<Package>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInfo2, BASE_OFFSET>(), Package: Package::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo3Impl: Sized {
    fn ExecutionContext(&mut self) -> ::windows::core::Result<AppExecutionContext>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInfo3, BASE_OFFSET>(), ExecutionContext: ExecutionContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInfo4Impl: Sized {
    fn SupportedFileExtensions(&mut self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInfo4, BASE_OFFSET>(),
            SupportedFileExtensions: SupportedFileExtensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfo4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppInfoStaticsImpl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelId(&mut self, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
    fn GetFromAppUserModelIdForUser(&mut self, user: &::core::option::Option<super::System::User>, appusermodelid: &::windows::core::HSTRING) -> ::windows::core::Result<AppInfo>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInfoStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            GetFromAppUserModelId: GetFromAppUserModelId::<Impl, IMPL_OFFSET>,
            GetFromAppUserModelIdForUser: GetFromAppUserModelIdForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallerInfoImpl: Sized {
    fn Uri(&mut self) -> ::windows::core::Result<super::Foundation::Uri>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallerInfo, BASE_OFFSET>(), Uri: Uri::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallerInfo2Impl: Sized {
    fn OnLaunch(&mut self) -> ::windows::core::Result<bool>;
    fn HoursBetweenUpdateChecks(&mut self) -> ::windows::core::Result<u32>;
    fn ShowPrompt(&mut self) -> ::windows::core::Result<bool>;
    fn UpdateBlocksActivation(&mut self) -> ::windows::core::Result<bool>;
    fn AutomaticBackgroundTask(&mut self) -> ::windows::core::Result<bool>;
    fn ForceUpdateFromAnyVersion(&mut self) -> ::windows::core::Result<bool>;
    fn IsAutoRepairEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn Version(&mut self) -> ::windows::core::Result<PackageVersion>;
    fn LastChecked(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn PausedUntil(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
    fn UpdateUris(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn RepairUris(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn DependencyPackageUris(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn OptionalPackageUris(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<super::Foundation::Uri>>;
    fn PolicySource(&mut self) -> ::windows::core::Result<AppInstallerPolicySource>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallerInfo2, BASE_OFFSET>(),
            OnLaunch: OnLaunch::<Impl, IMPL_OFFSET>,
            HoursBetweenUpdateChecks: HoursBetweenUpdateChecks::<Impl, IMPL_OFFSET>,
            ShowPrompt: ShowPrompt::<Impl, IMPL_OFFSET>,
            UpdateBlocksActivation: UpdateBlocksActivation::<Impl, IMPL_OFFSET>,
            AutomaticBackgroundTask: AutomaticBackgroundTask::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion: ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            IsAutoRepairEnabled: IsAutoRepairEnabled::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            LastChecked: LastChecked::<Impl, IMPL_OFFSET>,
            PausedUntil: PausedUntil::<Impl, IMPL_OFFSET>,
            UpdateUris: UpdateUris::<Impl, IMPL_OFFSET>,
            RepairUris: RepairUris::<Impl, IMPL_OFFSET>,
            DependencyPackageUris: DependencyPackageUris::<Impl, IMPL_OFFSET>,
            OptionalPackageUris: OptionalPackageUris::<Impl, IMPL_OFFSET>,
            PolicySource: PolicySource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstanceImpl: Sized {
    fn Key(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsCurrentInstance(&mut self) -> ::windows::core::Result<bool>;
    fn RedirectActivationTo(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstance, BASE_OFFSET>(),
            Key: Key::<Impl, IMPL_OFFSET>,
            IsCurrentInstance: IsCurrentInstance::<Impl, IMPL_OFFSET>,
            RedirectActivationTo: RedirectActivationTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstanceStaticsImpl: Sized {
    fn RecommendedInstance(&mut self) -> ::windows::core::Result<AppInstance>;
    fn GetActivatedEventArgs(&mut self) -> ::windows::core::Result<Activation::IActivatedEventArgs>;
    fn FindOrRegisterInstanceForKey(&mut self, key: &::windows::core::HSTRING) -> ::windows::core::Result<AppInstance>;
    fn Unregister(&mut self) -> ::windows::core::Result<()>;
    fn GetInstances(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVector<AppInstance>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstanceStatics, BASE_OFFSET>(),
            RecommendedInstance: RecommendedInstance::<Impl, IMPL_OFFSET>,
            GetActivatedEventArgs: GetActivatedEventArgs::<Impl, IMPL_OFFSET>,
            FindOrRegisterInstanceForKey: FindOrRegisterInstanceForKey::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
            GetInstances: GetInstances::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstanceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraApplicationManagerStaticsImpl: Sized {
    fn ShowInstalledApplicationsUI(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICameraApplicationManagerStatics, BASE_OFFSET>(),
            ShowInstalledApplicationsUI: ShowInstalledApplicationsUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraApplicationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStaticsImpl: Sized {
    fn DesignModeEnabled(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignModeStatics, BASE_OFFSET>(),
            DesignModeEnabled: DesignModeEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignModeStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDesignModeStatics2Impl: Sized {
    fn DesignMode2Enabled(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDesignModeStatics2, BASE_OFFSET>(),
            DesignMode2Enabled: DesignMode2Enabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesignModeStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IEnteredBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::Foundation::Deferral>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEnteredBackgroundEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFullTrustProcessLaunchResultImpl: Sized {
    fn LaunchResult(&mut self) -> ::windows::core::Result<FullTrustLaunchResult>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFullTrustProcessLaunchResult, BASE_OFFSET>(),
            LaunchResult: LaunchResult::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLaunchResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFullTrustProcessLauncherStaticsImpl: Sized {
    fn LaunchFullTrustProcessForCurrentAppAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForCurrentAppWithParametersAsync(&mut self, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppAsync(&mut self, fulltrustpackagerelativeappid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
    fn LaunchFullTrustProcessForAppWithParametersAsync(&mut self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, parametergroupid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncAction>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFullTrustProcessLauncherStatics, BASE_OFFSET>(),
            LaunchFullTrustProcessForCurrentAppAsync: LaunchFullTrustProcessForCurrentAppAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForCurrentAppWithParametersAsync: LaunchFullTrustProcessForCurrentAppWithParametersAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForAppAsync: LaunchFullTrustProcessForAppAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForAppWithParametersAsync: LaunchFullTrustProcessForAppWithParametersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLauncherStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFullTrustProcessLauncherStatics2Impl: Sized {
    fn LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(&mut self, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
    fn LaunchFullTrustProcessForAppWithArgumentsAsync(&mut self, fulltrustpackagerelativeappid: &::windows::core::HSTRING, commandline: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<FullTrustProcessLaunchResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFullTrustProcessLauncherStatics2, BASE_OFFSET>(),
            LaunchFullTrustProcessForCurrentAppWithArgumentsAsync: LaunchFullTrustProcessForCurrentAppWithArgumentsAsync::<Impl, IMPL_OFFSET>,
            LaunchFullTrustProcessForAppWithArgumentsAsync: LaunchFullTrustProcessForAppWithArgumentsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullTrustProcessLauncherStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ILeavingBackgroundEventArgsImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::Foundation::Deferral>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILeavingBackgroundEventArgs, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILimitedAccessFeatureRequestResultImpl: Sized {
    fn FeatureId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&mut self) -> ::windows::core::Result<LimitedAccessFeatureStatus>;
    fn EstimatedRemovalDate(&mut self) -> ::windows::core::Result<super::Foundation::IReference<super::Foundation::DateTime>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILimitedAccessFeatureRequestResult, BASE_OFFSET>(),
            FeatureId: FeatureId::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            EstimatedRemovalDate: EstimatedRemovalDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILimitedAccessFeatureRequestResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILimitedAccessFeaturesStaticsImpl: Sized {
    fn TryUnlockFeature(&mut self, featureid: &::windows::core::HSTRING, token: &::windows::core::HSTRING, attestation: &::windows::core::HSTRING) -> ::windows::core::Result<LimitedAccessFeatureRequestResult>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILimitedAccessFeaturesStatics, BASE_OFFSET>(),
            TryUnlockFeature: TryUnlockFeature::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILimitedAccessFeaturesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IPackageImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<PackageId>;
    fn InstalledLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn IsFramework(&mut self) -> ::windows::core::Result<bool>;
    fn Dependencies(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            InstalledLocation: InstalledLocation::<Impl, IMPL_OFFSET>,
            IsFramework: IsFramework::<Impl, IMPL_OFFSET>,
            Dependencies: Dependencies::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage2Impl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Logo(&mut self) -> ::windows::core::Result<super::Foundation::Uri>;
    fn IsResourcePackage(&mut self) -> ::windows::core::Result<bool>;
    fn IsBundle(&mut self) -> ::windows::core::Result<bool>;
    fn IsDevelopmentMode(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage2, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            PublisherDisplayName: PublisherDisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Logo: Logo::<Impl, IMPL_OFFSET>,
            IsResourcePackage: IsResourcePackage::<Impl, IMPL_OFFSET>,
            IsBundle: IsBundle::<Impl, IMPL_OFFSET>,
            IsDevelopmentMode: IsDevelopmentMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackage3Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<PackageStatus>;
    fn InstalledDate(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetAppListEntriesAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<Core::AppListEntry>>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage3, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            InstalledDate: InstalledDate::<Impl, IMPL_OFFSET>,
            GetAppListEntriesAsync: GetAppListEntriesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage4Impl: Sized {
    fn SignatureKind(&mut self) -> ::windows::core::Result<PackageSignatureKind>;
    fn IsOptional(&mut self) -> ::windows::core::Result<bool>;
    fn VerifyContentIntegrityAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage4, BASE_OFFSET>(),
            SignatureKind: SignatureKind::<Impl, IMPL_OFFSET>,
            IsOptional: IsOptional::<Impl, IMPL_OFFSET>,
            VerifyContentIntegrityAsync: VerifyContentIntegrityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackage5Impl: Sized {
    fn GetContentGroupsAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn GetContentGroupAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageContentGroup>>;
    fn StageContentGroupsAsync(&mut self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn StageContentGroupsWithPriorityAsync(&mut self, names: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, movetoheadofqueue: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVector<PackageContentGroup>>>;
    fn SetInUseAsync(&mut self, inuse: bool) -> ::windows::core::Result<super::Foundation::IAsyncOperation<bool>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage5, BASE_OFFSET>(),
            GetContentGroupsAsync: GetContentGroupsAsync::<Impl, IMPL_OFFSET>,
            GetContentGroupAsync: GetContentGroupAsync::<Impl, IMPL_OFFSET>,
            StageContentGroupsAsync: StageContentGroupsAsync::<Impl, IMPL_OFFSET>,
            StageContentGroupsWithPriorityAsync: StageContentGroupsWithPriorityAsync::<Impl, IMPL_OFFSET>,
            SetInUseAsync: SetInUseAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackage6Impl: Sized {
    fn GetAppInstallerInfo(&mut self) -> ::windows::core::Result<AppInstallerInfo>;
    fn CheckUpdateAvailabilityAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageUpdateAvailabilityResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage6, BASE_OFFSET>(),
            GetAppInstallerInfo: GetAppInstallerInfo::<Impl, IMPL_OFFSET>,
            CheckUpdateAvailabilityAsync: CheckUpdateAvailabilityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IPackage7Impl: Sized {
    fn MutableLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn EffectiveLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage7, BASE_OFFSET>(),
            MutableLocation: MutableLocation::<Impl, IMPL_OFFSET>,
            EffectiveLocation: EffectiveLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IPackage8Impl: Sized {
    fn EffectiveExternalLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn MachineExternalLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn UserExternalLocation(&mut self) -> ::windows::core::Result<super::Storage::StorageFolder>;
    fn InstalledPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MutablePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectivePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EffectiveExternalPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MachineExternalPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserExternalPath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLogoAsRandomAccessStreamReference(&mut self, size: &super::Foundation::Size) -> ::windows::core::Result<super::Storage::Streams::RandomAccessStreamReference>;
    fn GetAppListEntries(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Core::AppListEntry>>;
    fn IsStub(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackage8, BASE_OFFSET>(),
            EffectiveExternalLocation: EffectiveExternalLocation::<Impl, IMPL_OFFSET>,
            MachineExternalLocation: MachineExternalLocation::<Impl, IMPL_OFFSET>,
            UserExternalLocation: UserExternalLocation::<Impl, IMPL_OFFSET>,
            InstalledPath: InstalledPath::<Impl, IMPL_OFFSET>,
            MutablePath: MutablePath::<Impl, IMPL_OFFSET>,
            EffectivePath: EffectivePath::<Impl, IMPL_OFFSET>,
            EffectiveExternalPath: EffectiveExternalPath::<Impl, IMPL_OFFSET>,
            MachineExternalPath: MachineExternalPath::<Impl, IMPL_OFFSET>,
            UserExternalPath: UserExternalPath::<Impl, IMPL_OFFSET>,
            GetLogoAsRandomAccessStreamReference: GetLogoAsRandomAccessStreamReference::<Impl, IMPL_OFFSET>,
            GetAppListEntries: GetAppListEntries::<Impl, IMPL_OFFSET>,
            IsStub: IsStub::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackage8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageCatalogImpl: Sized {
    fn PackageStaging(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStaging(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageInstalling(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageInstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageInstalling(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdating(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUpdatingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdating(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUninstalling(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageUninstallingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageUninstalling(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageStatusChanged(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageStatusChangedEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageStatusChanged(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalog, BASE_OFFSET>(),
            PackageStaging: PackageStaging::<Impl, IMPL_OFFSET>,
            RemovePackageStaging: RemovePackageStaging::<Impl, IMPL_OFFSET>,
            PackageInstalling: PackageInstalling::<Impl, IMPL_OFFSET>,
            RemovePackageInstalling: RemovePackageInstalling::<Impl, IMPL_OFFSET>,
            PackageUpdating: PackageUpdating::<Impl, IMPL_OFFSET>,
            RemovePackageUpdating: RemovePackageUpdating::<Impl, IMPL_OFFSET>,
            PackageUninstalling: PackageUninstalling::<Impl, IMPL_OFFSET>,
            RemovePackageUninstalling: RemovePackageUninstalling::<Impl, IMPL_OFFSET>,
            PackageStatusChanged: PackageStatusChanged::<Impl, IMPL_OFFSET>,
            RemovePackageStatusChanged: RemovePackageStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageCatalog2Impl: Sized {
    fn PackageContentGroupStaging(&mut self, handler: &::core::option::Option<super::Foundation::TypedEventHandler<PackageCatalog, PackageContentGroupStagingEventArgs>>) -> ::windows::core::Result<super::Foundation::EventRegistrationToken>;
    fn RemovePackageContentGroupStaging(&mut self, token: &super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddOptionalPackageAsync(&mut self, optionalpackagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogAddOptionalPackageResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalog2, BASE_OFFSET>(),
            PackageContentGroupStaging: PackageContentGroupStaging::<Impl, IMPL_OFFSET>,
            RemovePackageContentGroupStaging: RemovePackageContentGroupStaging::<Impl, IMPL_OFFSET>,
            AddOptionalPackageAsync: AddOptionalPackageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalog3Impl: Sized {
    fn RemoveOptionalPackagesAsync(&mut self, optionalpackagefamilynames: &::core::option::Option<super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveOptionalPackagesResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalog3, BASE_OFFSET>(),
            RemoveOptionalPackagesAsync: RemoveOptionalPackagesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalog4Impl: Sized {
    fn AddResourcePackageAsync(&mut self, resourcepackagefamilyname: &::windows::core::HSTRING, resourceid: &::windows::core::HSTRING, options: AddResourcePackageOptions) -> ::windows::core::Result<super::Foundation::IAsyncOperationWithProgress<PackageCatalogAddResourcePackageResult, PackageInstallProgress>>;
    fn RemoveResourcePackagesAsync(&mut self, resourcepackages: &::core::option::Option<super::Foundation::Collections::IIterable<Package>>) -> ::windows::core::Result<super::Foundation::IAsyncOperation<PackageCatalogRemoveResourcePackagesResult>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalog4, BASE_OFFSET>(),
            AddResourcePackageAsync: AddResourcePackageAsync::<Impl, IMPL_OFFSET>,
            RemoveResourcePackagesAsync: RemoveResourcePackagesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalog4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddOptionalPackageResultImpl: Sized {
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalogAddOptionalPackageResult, BASE_OFFSET>(),
            Package: Package::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogAddOptionalPackageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogAddResourcePackageResultImpl: Sized {
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalogAddResourcePackageResult, BASE_OFFSET>(),
            Package: Package::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogAddResourcePackageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalogRemoveOptionalPackagesResultImpl: Sized {
    fn PackagesRemoved(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalogRemoveOptionalPackagesResult, BASE_OFFSET>(),
            PackagesRemoved: PackagesRemoved::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogRemoveOptionalPackagesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageCatalogRemoveResourcePackagesResultImpl: Sized {
    fn PackagesRemoved(&mut self) -> ::windows::core::Result<super::Foundation::Collections::IVectorView<Package>>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalogRemoveResourcePackagesResult, BASE_OFFSET>(),
            PackagesRemoved: PackagesRemoved::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogRemoveResourcePackagesResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageCatalogStaticsImpl: Sized {
    fn OpenForCurrentPackage(&mut self) -> ::windows::core::Result<PackageCatalog>;
    fn OpenForCurrentUser(&mut self) -> ::windows::core::Result<PackageCatalog>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageCatalogStatics, BASE_OFFSET>(),
            OpenForCurrentPackage: OpenForCurrentPackage::<Impl, IMPL_OFFSET>,
            OpenForCurrentUser: OpenForCurrentUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageCatalogStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupImpl: Sized {
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&mut self) -> ::windows::core::Result<PackageContentGroupState>;
    fn IsRequired(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageContentGroup, BASE_OFFSET>(),
            Package: Package::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            IsRequired: IsRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStagingEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn Progress(&mut self) -> ::windows::core::Result<f64>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ContentGroupName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsContentGroupRequired(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageContentGroupStagingEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            ContentGroupName: ContentGroupName::<Impl, IMPL_OFFSET>,
            IsContentGroupRequired: IsContentGroupRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroupStagingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageContentGroupStaticsImpl: Sized {
    fn RequiredGroupName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageContentGroupStatics, BASE_OFFSET>(),
            RequiredGroupName: RequiredGroupName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageContentGroupStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IPackageIdImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Version(&mut self) -> ::windows::core::Result<PackageVersion>;
    fn Architecture(&mut self) -> ::windows::core::Result<super::System::ProcessorArchitecture>;
    fn ResourceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Publisher(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublisherId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FullName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageId, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            Architecture: Architecture::<Impl, IMPL_OFFSET>,
            ResourceId: ResourceId::<Impl, IMPL_OFFSET>,
            Publisher: Publisher::<Impl, IMPL_OFFSET>,
            PublisherId: PublisherId::<Impl, IMPL_OFFSET>,
            FullName: FullName::<Impl, IMPL_OFFSET>,
            FamilyName: FamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageIdWithMetadataImpl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Author(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageIdWithMetadata, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            Author: Author::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageIdWithMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageInstallingEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn Progress(&mut self) -> ::windows::core::Result<f64>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageInstallingEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageInstallingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStagingEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn Progress(&mut self) -> ::windows::core::Result<f64>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageStagingEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStagingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStaticsImpl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<Package>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageStatics, BASE_OFFSET>(), Current: Current::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusImpl: Sized {
    fn VerifyIsOK(&mut self) -> ::windows::core::Result<bool>;
    fn NotAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn PackageOffline(&mut self) -> ::windows::core::Result<bool>;
    fn DataOffline(&mut self) -> ::windows::core::Result<bool>;
    fn Disabled(&mut self) -> ::windows::core::Result<bool>;
    fn NeedsRemediation(&mut self) -> ::windows::core::Result<bool>;
    fn LicenseIssue(&mut self) -> ::windows::core::Result<bool>;
    fn Modified(&mut self) -> ::windows::core::Result<bool>;
    fn Tampered(&mut self) -> ::windows::core::Result<bool>;
    fn DependencyIssue(&mut self) -> ::windows::core::Result<bool>;
    fn Servicing(&mut self) -> ::windows::core::Result<bool>;
    fn DeploymentInProgress(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageStatus, BASE_OFFSET>(),
            VerifyIsOK: VerifyIsOK::<Impl, IMPL_OFFSET>,
            NotAvailable: NotAvailable::<Impl, IMPL_OFFSET>,
            PackageOffline: PackageOffline::<Impl, IMPL_OFFSET>,
            DataOffline: DataOffline::<Impl, IMPL_OFFSET>,
            Disabled: Disabled::<Impl, IMPL_OFFSET>,
            NeedsRemediation: NeedsRemediation::<Impl, IMPL_OFFSET>,
            LicenseIssue: LicenseIssue::<Impl, IMPL_OFFSET>,
            Modified: Modified::<Impl, IMPL_OFFSET>,
            Tampered: Tampered::<Impl, IMPL_OFFSET>,
            DependencyIssue: DependencyIssue::<Impl, IMPL_OFFSET>,
            Servicing: Servicing::<Impl, IMPL_OFFSET>,
            DeploymentInProgress: DeploymentInProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatus2Impl: Sized {
    fn IsPartiallyStaged(&mut self) -> ::windows::core::Result<bool>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageStatus2, BASE_OFFSET>(),
            IsPartiallyStaged: IsPartiallyStaged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageStatusChangedEventArgsImpl: Sized {
    fn Package(&mut self) -> ::windows::core::Result<Package>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageStatusChangedEventArgs, BASE_OFFSET>(), Package: Package::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUninstallingEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Package(&mut self) -> ::windows::core::Result<Package>;
    fn Progress(&mut self) -> ::windows::core::Result<f64>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageUninstallingEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUninstallingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdateAvailabilityResultImpl: Sized {
    fn Availability(&mut self) -> ::windows::core::Result<PackageUpdateAvailability>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageUpdateAvailabilityResult, BASE_OFFSET>(),
            Availability: Availability::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUpdateAvailabilityResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUpdatingEventArgsImpl: Sized {
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SourcePackage(&mut self) -> ::windows::core::Result<Package>;
    fn TargetPackage(&mut self) -> ::windows::core::Result<Package>;
    fn Progress(&mut self) -> ::windows::core::Result<f64>;
    fn IsComplete(&mut self) -> ::windows::core::Result<bool>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageUpdatingEventArgs, BASE_OFFSET>(),
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            SourcePackage: SourcePackage::<Impl, IMPL_OFFSET>,
            TargetPackage: TargetPackage::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            IsComplete: IsComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageWithMetadataImpl: Sized {
    fn InstallDate(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
    fn GetThumbnailToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Launch(&mut self, parameters: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageWithMetadata, BASE_OFFSET>(),
            InstallDate: InstallDate::<Impl, IMPL_OFFSET>,
            GetThumbnailToken: GetThumbnailToken::<Impl, IMPL_OFFSET>,
            Launch: Launch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageWithMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStartupTaskImpl: Sized {
    fn RequestEnableAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTaskState>>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<StartupTaskState>;
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStartupTask, BASE_OFFSET>(),
            RequestEnableAsync: RequestEnableAsync::<Impl, IMPL_OFFSET>,
            Disable: Disable::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            TaskId: TaskId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStartupTaskStaticsImpl: Sized {
    fn GetForCurrentPackageAsync(&mut self) -> ::windows::core::Result<super::Foundation::IAsyncOperation<super::Foundation::Collections::IVectorView<StartupTask>>>;
    fn GetAsync(&mut self, taskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::Foundation::IAsyncOperation<StartupTask>>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStartupTaskStatics, BASE_OFFSET>(),
            GetForCurrentPackageAsync: GetForCurrentPackageAsync::<Impl, IMPL_OFFSET>,
            GetAsync: GetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStartupTaskStatics as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingDeferral as ::windows::core::Interface>::IID
    }
}
pub trait ISuspendingEventArgsImpl: Sized {
    fn SuspendingOperation(&mut self) -> ::windows::core::Result<SuspendingOperation>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingEventArgs, BASE_OFFSET>(),
            SuspendingOperation: SuspendingOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ISuspendingOperationImpl: Sized {
    fn GetDeferral(&mut self) -> ::windows::core::Result<SuspendingDeferral>;
    fn Deadline(&mut self) -> ::windows::core::Result<super::Foundation::DateTime>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISuspendingOperation, BASE_OFFSET>(),
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            Deadline: Deadline::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISuspendingOperation as ::windows::core::Interface>::IID
    }
}
