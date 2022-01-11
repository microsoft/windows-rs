#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAddPackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn RetainFilesOnFailure(&self) -> ::windows::core::Result<bool>;
    fn SetRetainFilesOnFailure(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IAddPackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAddPackageOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddPackageOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddPackageOptionsVtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetVolume<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetVolume<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalPackageFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionalPackageUris<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RelatedPackageUris<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelatedPackageUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalLocationUri<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalLocationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExternalLocationUri<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StubPackageOption<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StubPackageOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStubPackageOption<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStubPackageOption(value).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeveloperMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeveloperMode<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceAppShutdown<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceAppShutdown<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceTargetAppShutdown<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceTargetAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceTargetAppShutdown<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceTargetAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallAllResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallAllResources<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn RequiredContentGroupOnly<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredContentGroupOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequiredContentGroupOnly<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredContentGroupOnly(value).into()
        }
        unsafe extern "system" fn RetainFilesOnFailure<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetainFilesOnFailure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRetainFilesOnFailure<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetainFilesOnFailure(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageInPlace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageInPlace<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowUnsigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowUnsigned<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUnsigned(value).into()
        }
        unsafe extern "system" fn DeferRegistrationWhenPackagesAreInUse<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeferRegistrationWhenPackagesAreInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeferRegistrationWhenPackagesAreInUse<Impl: IAddPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeferRegistrationWhenPackagesAreInUse(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAddPackageOptions, BASE_OFFSET>(),
            DependencyPackageUris: DependencyPackageUris::<Impl, IMPL_OFFSET>,
            TargetVolume: TargetVolume::<Impl, IMPL_OFFSET>,
            SetTargetVolume: SetTargetVolume::<Impl, IMPL_OFFSET>,
            OptionalPackageFamilyNames: OptionalPackageFamilyNames::<Impl, IMPL_OFFSET>,
            OptionalPackageUris: OptionalPackageUris::<Impl, IMPL_OFFSET>,
            RelatedPackageUris: RelatedPackageUris::<Impl, IMPL_OFFSET>,
            ExternalLocationUri: ExternalLocationUri::<Impl, IMPL_OFFSET>,
            SetExternalLocationUri: SetExternalLocationUri::<Impl, IMPL_OFFSET>,
            StubPackageOption: StubPackageOption::<Impl, IMPL_OFFSET>,
            SetStubPackageOption: SetStubPackageOption::<Impl, IMPL_OFFSET>,
            DeveloperMode: DeveloperMode::<Impl, IMPL_OFFSET>,
            SetDeveloperMode: SetDeveloperMode::<Impl, IMPL_OFFSET>,
            ForceAppShutdown: ForceAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceAppShutdown: SetForceAppShutdown::<Impl, IMPL_OFFSET>,
            ForceTargetAppShutdown: ForceTargetAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceTargetAppShutdown: SetForceTargetAppShutdown::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion: ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            SetForceUpdateFromAnyVersion: SetForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            InstallAllResources: InstallAllResources::<Impl, IMPL_OFFSET>,
            SetInstallAllResources: SetInstallAllResources::<Impl, IMPL_OFFSET>,
            RequiredContentGroupOnly: RequiredContentGroupOnly::<Impl, IMPL_OFFSET>,
            SetRequiredContentGroupOnly: SetRequiredContentGroupOnly::<Impl, IMPL_OFFSET>,
            RetainFilesOnFailure: RetainFilesOnFailure::<Impl, IMPL_OFFSET>,
            SetRetainFilesOnFailure: SetRetainFilesOnFailure::<Impl, IMPL_OFFSET>,
            StageInPlace: StageInPlace::<Impl, IMPL_OFFSET>,
            SetStageInPlace: SetStageInPlace::<Impl, IMPL_OFFSET>,
            AllowUnsigned: AllowUnsigned::<Impl, IMPL_OFFSET>,
            SetAllowUnsigned: SetAllowUnsigned::<Impl, IMPL_OFFSET>,
            DeferRegistrationWhenPackagesAreInUse: DeferRegistrationWhenPackagesAreInUse::<Impl, IMPL_OFFSET>,
            SetDeferRegistrationWhenPackagesAreInUse: SetDeferRegistrationWhenPackagesAreInUse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddPackageOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallerManagerImpl: Sized {
    fn SetAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING, appinstallerinfo: &::core::option::Option<AutoUpdateSettingsOptions>) -> ::windows::core::Result<()>;
    fn ClearAutoUpdateSettings(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseAutoUpdatesUntil(&self, packagefamilyname: &::windows::core::HSTRING, datetime: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.IAppInstallerManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallerManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerManagerVtbl {
        unsafe extern "system" fn SetAutoUpdateSettings<Impl: IAppInstallerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appinstallerinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoUpdateSettings(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&appinstallerinfo as *const <AutoUpdateSettingsOptions as ::windows::core::Abi>::Abi as *const <AutoUpdateSettingsOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAutoUpdateSettings<Impl: IAppInstallerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAutoUpdateSettings(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseAutoUpdatesUntil<Impl: IAppInstallerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datetime: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseAutoUpdatesUntil(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&datetime as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallerManager, BASE_OFFSET>(),
            SetAutoUpdateSettings: SetAutoUpdateSettings::<Impl, IMPL_OFFSET>,
            ClearAutoUpdateSettings: ClearAutoUpdateSettings::<Impl, IMPL_OFFSET>,
            PauseAutoUpdatesUntil: PauseAutoUpdatesUntil::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallerManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppInstallerManager>;
    fn GetForSystem(&self) -> ::windows::core::Result<AppInstallerManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallerManagerStatics {
    const NAME: &'static str = "Windows.Management.Deployment.IAppInstallerManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallerManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppInstallerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForSystem<Impl: IAppInstallerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallerManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForSystem: GetForSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallerManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutoUpdateSettingsOptionsImpl: Sized {
    fn Version(&self) -> ::windows::core::Result<super::super::ApplicationModel::PackageVersion>;
    fn SetVersion(&self, value: &super::super::ApplicationModel::PackageVersion) -> ::windows::core::Result<()>;
    fn AppInstallerUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetAppInstallerUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OnLaunch(&self) -> ::windows::core::Result<bool>;
    fn SetOnLaunch(&self, value: bool) -> ::windows::core::Result<()>;
    fn HoursBetweenUpdateChecks(&self) -> ::windows::core::Result<u32>;
    fn SetHoursBetweenUpdateChecks(&self, value: u32) -> ::windows::core::Result<()>;
    fn ShowPrompt(&self) -> ::windows::core::Result<bool>;
    fn SetShowPrompt(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateBlocksActivation(&self) -> ::windows::core::Result<bool>;
    fn SetUpdateBlocksActivation(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticBackgroundTask(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticBackgroundTask(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAutoRepairEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoRepairEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RepairUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IAutoUpdateSettingsOptions";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutoUpdateSettingsOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoUpdateSettingsOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoUpdateSettingsOptionsVtbl {
        unsafe extern "system" fn Version<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVersion<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(&*(&value as *const <super::super::ApplicationModel::PackageVersion as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::PackageVersion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppInstallerUri<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInstallerUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppInstallerUri<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppInstallerUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLaunch<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOnLaunch<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOnLaunch(value).into()
        }
        unsafe extern "system" fn HoursBetweenUpdateChecks<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHoursBetweenUpdateChecks<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoursBetweenUpdateChecks(value).into()
        }
        unsafe extern "system" fn ShowPrompt<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShowPrompt<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowPrompt(value).into()
        }
        unsafe extern "system" fn UpdateBlocksActivation<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUpdateBlocksActivation<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateBlocksActivation(value).into()
        }
        unsafe extern "system" fn AutomaticBackgroundTask<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomaticBackgroundTask<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticBackgroundTask(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn IsAutoRepairEnabled<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsAutoRepairEnabled<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAutoRepairEnabled(value).into()
        }
        unsafe extern "system" fn UpdateUris<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RepairUris<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DependencyPackageUris<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OptionalPackageUris<Impl: IAutoUpdateSettingsOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoUpdateSettingsOptions, BASE_OFFSET>(),
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            AppInstallerUri: AppInstallerUri::<Impl, IMPL_OFFSET>,
            SetAppInstallerUri: SetAppInstallerUri::<Impl, IMPL_OFFSET>,
            OnLaunch: OnLaunch::<Impl, IMPL_OFFSET>,
            SetOnLaunch: SetOnLaunch::<Impl, IMPL_OFFSET>,
            HoursBetweenUpdateChecks: HoursBetweenUpdateChecks::<Impl, IMPL_OFFSET>,
            SetHoursBetweenUpdateChecks: SetHoursBetweenUpdateChecks::<Impl, IMPL_OFFSET>,
            ShowPrompt: ShowPrompt::<Impl, IMPL_OFFSET>,
            SetShowPrompt: SetShowPrompt::<Impl, IMPL_OFFSET>,
            UpdateBlocksActivation: UpdateBlocksActivation::<Impl, IMPL_OFFSET>,
            SetUpdateBlocksActivation: SetUpdateBlocksActivation::<Impl, IMPL_OFFSET>,
            AutomaticBackgroundTask: AutomaticBackgroundTask::<Impl, IMPL_OFFSET>,
            SetAutomaticBackgroundTask: SetAutomaticBackgroundTask::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion: ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            SetForceUpdateFromAnyVersion: SetForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            IsAutoRepairEnabled: IsAutoRepairEnabled::<Impl, IMPL_OFFSET>,
            SetIsAutoRepairEnabled: SetIsAutoRepairEnabled::<Impl, IMPL_OFFSET>,
            UpdateUris: UpdateUris::<Impl, IMPL_OFFSET>,
            RepairUris: RepairUris::<Impl, IMPL_OFFSET>,
            DependencyPackageUris: DependencyPackageUris::<Impl, IMPL_OFFSET>,
            OptionalPackageUris: OptionalPackageUris::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoUpdateSettingsOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
pub trait IAutoUpdateSettingsOptionsStaticsImpl: Sized {
    fn CreateFromAppInstallerInfo(&self, appinstallerinfo: &::core::option::Option<super::super::ApplicationModel::AppInstallerInfo>) -> ::windows::core::Result<AutoUpdateSettingsOptions>;
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoUpdateSettingsOptionsStatics {
    const NAME: &'static str = "Windows.Management.Deployment.IAutoUpdateSettingsOptionsStatics";
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl IAutoUpdateSettingsOptionsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoUpdateSettingsOptionsStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoUpdateSettingsOptionsStaticsVtbl {
        unsafe extern "system" fn CreateFromAppInstallerInfo<Impl: IAutoUpdateSettingsOptionsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromAppInstallerInfo(&*(&appinstallerinfo as *const <super::super::ApplicationModel::AppInstallerInfo as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::AppInstallerInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoUpdateSettingsOptionsStatics, BASE_OFFSET>(),
            CreateFromAppInstallerInfo: CreateFromAppInstallerInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoUpdateSettingsOptionsStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ICreateSharedPackageContainerOptionsImpl: Sized {
    fn Members(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn CreateCollisionOption(&self) -> ::windows::core::Result<SharedPackageContainerCreationCollisionOptions>;
    fn SetCreateCollisionOption(&self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.ICreateSharedPackageContainerOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICreateSharedPackageContainerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateSharedPackageContainerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateSharedPackageContainerOptionsVtbl {
        unsafe extern "system" fn Members<Impl: ICreateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceAppShutdown<Impl: ICreateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceAppShutdown<Impl: ICreateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn CreateCollisionOption<Impl: ICreateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCollisionOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateCollisionOption<Impl: ICreateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateCollisionOption(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICreateSharedPackageContainerOptions, BASE_OFFSET>(),
            Members: Members::<Impl, IMPL_OFFSET>,
            ForceAppShutdown: ForceAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceAppShutdown: SetForceAppShutdown::<Impl, IMPL_OFFSET>,
            CreateCollisionOption: CreateCollisionOption::<Impl, IMPL_OFFSET>,
            SetCreateCollisionOption: SetCreateCollisionOption::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateSharedPackageContainerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICreateSharedPackageContainerResultImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<SharedPackageContainer>;
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.ICreateSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateSharedPackageContainerResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateSharedPackageContainerResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateSharedPackageContainerResultVtbl {
        unsafe extern "system" fn Container<Impl: ICreateSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: ICreateSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: ICreateSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICreateSharedPackageContainerResult, BASE_OFFSET>(),
            Container: Container::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateSharedPackageContainerResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeleteSharedPackageContainerOptionsImpl: Sized {
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllUsers(&self) -> ::windows::core::Result<bool>;
    fn SetAllUsers(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IDeleteSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IDeleteSharedPackageContainerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteSharedPackageContainerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeleteSharedPackageContainerOptionsVtbl {
        unsafe extern "system" fn ForceAppShutdown<Impl: IDeleteSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceAppShutdown<Impl: IDeleteSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn AllUsers<Impl: IDeleteSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllUsers<Impl: IDeleteSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllUsers(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeleteSharedPackageContainerOptions, BASE_OFFSET>(),
            ForceAppShutdown: ForceAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceAppShutdown: SetForceAppShutdown::<Impl, IMPL_OFFSET>,
            AllUsers: AllUsers::<Impl, IMPL_OFFSET>,
            SetAllUsers: SetAllUsers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeleteSharedPackageContainerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeleteSharedPackageContainerResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.IDeleteSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeleteSharedPackageContainerResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteSharedPackageContainerResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeleteSharedPackageContainerResultVtbl {
        unsafe extern "system" fn Status<Impl: IDeleteSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IDeleteSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeleteSharedPackageContainerResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeleteSharedPackageContainerResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeploymentResultImpl: Sized {
    fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExtendedErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.IDeploymentResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeploymentResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeploymentResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeploymentResultVtbl {
        unsafe extern "system" fn ErrorText<Impl: IDeploymentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: IDeploymentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDeploymentResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeploymentResult, BASE_OFFSET>(),
            ErrorText: ErrorText::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
            ExtendedErrorCode: ExtendedErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeploymentResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeploymentResult2Impl: Sized {
    fn IsRegistered(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeploymentResult2 {
    const NAME: &'static str = "Windows.Management.Deployment.IDeploymentResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeploymentResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeploymentResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeploymentResult2Vtbl {
        unsafe extern "system" fn IsRegistered<Impl: IDeploymentResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRegistered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeploymentResult2, BASE_OFFSET>(), IsRegistered: IsRegistered::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeploymentResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFindSharedPackageContainerOptionsImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IFindSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFindSharedPackageContainerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindSharedPackageContainerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindSharedPackageContainerOptionsVtbl {
        unsafe extern "system" fn Name<Impl: IFindSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IFindSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IFindSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPackageFamilyName<Impl: IFindSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFindSharedPackageContainerOptions, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
            SetPackageFamilyName: SetPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindSharedPackageContainerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageAllUserProvisioningOptionsImpl: Sized {
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProjectionOrderPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageAllUserProvisioningOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageAllUserProvisioningOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageAllUserProvisioningOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageAllUserProvisioningOptionsVtbl {
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IPackageAllUserProvisioningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalPackageFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionOrderPackageFamilyNames<Impl: IPackageAllUserProvisioningOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionOrderPackageFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageAllUserProvisioningOptions, BASE_OFFSET>(),
            OptionalPackageFamilyNames: OptionalPackageFamilyNames::<Impl, IMPL_OFFSET>,
            ProjectionOrderPackageFamilyNames: ProjectionOrderPackageFamilyNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageAllUserProvisioningOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManagerImpl: Sized {
    fn AddPackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn UpdatePackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindUsers(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>>;
    fn SetPackageState(&self, packagefullname: &::windows::core::HSTRING, packagestate: PackageState) -> ::windows::core::Result<()>;
    fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn CleanupPackageForUserAsync(&self, packagename: &::windows::core::HSTRING, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManagerVtbl {
        unsafe extern "system" fn AddPackageAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdatePackageAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdatePackageAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePackageAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackageAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageAsync(&*(&manifesturi as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackages<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityId<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityId(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByNamePublisher<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByNamePublisher(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisher<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdNamePublisher(
                &*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindUsers<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindUsers(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPackageState<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagestate: PackageState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageState(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagestate).into()
        }
        unsafe extern "system" fn FindPackageByPackageFullName<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageByPackageFullName(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CleanupPackageForUserAsync<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CleanupPackageForUserAsync(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByPackageFamilyName<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByPackageFamilyName(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyName<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdPackageFamilyName(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackageByUserSecurityIdPackageFullName<Impl: IPackageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageByUserSecurityIdPackageFullName(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager, BASE_OFFSET>(),
            AddPackageAsync: AddPackageAsync::<Impl, IMPL_OFFSET>,
            UpdatePackageAsync: UpdatePackageAsync::<Impl, IMPL_OFFSET>,
            RemovePackageAsync: RemovePackageAsync::<Impl, IMPL_OFFSET>,
            StagePackageAsync: StagePackageAsync::<Impl, IMPL_OFFSET>,
            RegisterPackageAsync: RegisterPackageAsync::<Impl, IMPL_OFFSET>,
            FindPackages: FindPackages::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityId: FindPackagesByUserSecurityId::<Impl, IMPL_OFFSET>,
            FindPackagesByNamePublisher: FindPackagesByNamePublisher::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdNamePublisher: FindPackagesByUserSecurityIdNamePublisher::<Impl, IMPL_OFFSET>,
            FindUsers: FindUsers::<Impl, IMPL_OFFSET>,
            SetPackageState: SetPackageState::<Impl, IMPL_OFFSET>,
            FindPackageByPackageFullName: FindPackageByPackageFullName::<Impl, IMPL_OFFSET>,
            CleanupPackageForUserAsync: CleanupPackageForUserAsync::<Impl, IMPL_OFFSET>,
            FindPackagesByPackageFamilyName: FindPackagesByPackageFamilyName::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdPackageFamilyName: FindPackagesByUserSecurityIdPackageFamilyName::<Impl, IMPL_OFFSET>,
            FindPackageByUserSecurityIdPackageFullName: FindPackageByUserSecurityIdPackageFullName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageManager10Impl: Sized {
    fn ProvisionPackageForAllUsersWithOptionsAsync(&self, mainpackagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<PackageAllUserProvisioningOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager10 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager10";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManager10Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager10Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager10Vtbl {
        unsafe extern "system" fn ProvisionPackageForAllUsersWithOptionsAsync<Impl: IPackageManager10Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionPackageForAllUsersWithOptionsAsync(&*(&mainpackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <PackageAllUserProvisioningOptions as ::windows::core::Abi>::Abi as *const <PackageAllUserProvisioningOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager10, BASE_OFFSET>(),
            ProvisionPackageForAllUsersWithOptionsAsync: ProvisionPackageForAllUsersWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager10 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager2Impl: Sized {
    fn RemovePackageWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, removaloptions: RemovalOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageWithOptionsAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFullNameAsync(&self, mainpackagefullname: &::windows::core::HSTRING, dependencypackagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackageTypes(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn StageUserDataAsync(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager2 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager2Vtbl {
        unsafe extern "system" fn RemovePackageWithOptionsAsync<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, removaloptions: RemovalOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePackageWithOptionsAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), removaloptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageWithOptionsAsync<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageWithOptionsAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackageByFullNameAsync<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefullnames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageByFullNameAsync(&*(&mainpackagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&dependencypackagefullnames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesWithPackageTypes(packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdWithPackageTypes(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByNamePublisherWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByNamePublisherWithPackageTypes(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(
                &*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                packagetypes,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByPackageFamilyNameWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByPackageFamilyNameWithPackageTypes(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageUserDataAsync<Impl: IPackageManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageUserDataAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager2, BASE_OFFSET>(),
            RemovePackageWithOptionsAsync: RemovePackageWithOptionsAsync::<Impl, IMPL_OFFSET>,
            StagePackageWithOptionsAsync: StagePackageWithOptionsAsync::<Impl, IMPL_OFFSET>,
            RegisterPackageByFullNameAsync: RegisterPackageByFullNameAsync::<Impl, IMPL_OFFSET>,
            FindPackagesWithPackageTypes: FindPackagesWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdWithPackageTypes: FindPackagesByUserSecurityIdWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByNamePublisherWithPackageTypes: FindPackagesByNamePublisherWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: FindPackagesByUserSecurityIdNamePublisherWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByPackageFamilyNameWithPackageTypes: FindPackagesByPackageFamilyNameWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes: FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes::<Impl, IMPL_OFFSET>,
            StageUserDataAsync: StageUserDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager3Impl: Sized {
    fn AddPackageVolumeAsync(&self, packagestorepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PackageVolume>>;
    fn AddPackageToVolumeAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn ClearPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn RegisterPackageWithAppDataVolumeAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackageVolumeByName(&self, volumename: &::windows::core::HSTRING) -> ::windows::core::Result<PackageVolume>;
    fn FindPackageVolumes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageVolume>>;
    fn GetDefaultPackageVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn MovePackageToVolumeAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageVolumeAsync(&self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetDefaultPackageVolume(&self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn SetPackageStatus(&self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn SetPackageVolumeOfflineAsync(&self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageVolumeOnlineAsync(&self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StageUserDataWithOptionsAsync(&self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager3 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager3Vtbl {
        unsafe extern "system" fn AddPackageVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestorepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageVolumeAsync(&*(&packagestorepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackageToVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageToVolumeAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearPackageStatus<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPackageStatus(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), status).into()
        }
        unsafe extern "system" fn RegisterPackageWithAppDataVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageWithAppDataVolumeAsync(
                &*(&manifesturi as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&appdatavolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackageVolumeByName<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageVolumeByName(&*(&volumename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackageVolumes<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageVolumes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultPackageVolume<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultPackageVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MovePackageToVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MovePackageToVolumeAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), deploymentoptions, &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePackageVolumeAsync(&*(&volume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultPackageVolume<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultPackageVolume(&*(&volume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPackageStatus<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageStatus(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), status).into()
        }
        unsafe extern "system" fn SetPackageVolumeOfflineAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPackageVolumeOfflineAsync(&*(&packagevolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPackageVolumeOnlineAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPackageVolumeOnlineAsync(&*(&packagevolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageToVolumeAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageToVolumeAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageUserDataWithOptionsAsync<Impl: IPackageManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageUserDataWithOptionsAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), deploymentoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager3, BASE_OFFSET>(),
            AddPackageVolumeAsync: AddPackageVolumeAsync::<Impl, IMPL_OFFSET>,
            AddPackageToVolumeAsync: AddPackageToVolumeAsync::<Impl, IMPL_OFFSET>,
            ClearPackageStatus: ClearPackageStatus::<Impl, IMPL_OFFSET>,
            RegisterPackageWithAppDataVolumeAsync: RegisterPackageWithAppDataVolumeAsync::<Impl, IMPL_OFFSET>,
            FindPackageVolumeByName: FindPackageVolumeByName::<Impl, IMPL_OFFSET>,
            FindPackageVolumes: FindPackageVolumes::<Impl, IMPL_OFFSET>,
            GetDefaultPackageVolume: GetDefaultPackageVolume::<Impl, IMPL_OFFSET>,
            MovePackageToVolumeAsync: MovePackageToVolumeAsync::<Impl, IMPL_OFFSET>,
            RemovePackageVolumeAsync: RemovePackageVolumeAsync::<Impl, IMPL_OFFSET>,
            SetDefaultPackageVolume: SetDefaultPackageVolume::<Impl, IMPL_OFFSET>,
            SetPackageStatus: SetPackageStatus::<Impl, IMPL_OFFSET>,
            SetPackageVolumeOfflineAsync: SetPackageVolumeOfflineAsync::<Impl, IMPL_OFFSET>,
            SetPackageVolumeOnlineAsync: SetPackageVolumeOnlineAsync::<Impl, IMPL_OFFSET>,
            StagePackageToVolumeAsync: StagePackageToVolumeAsync::<Impl, IMPL_OFFSET>,
            StageUserDataWithOptionsAsync: StageUserDataWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager4Impl: Sized {
    fn GetPackageVolumesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager4 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager4Vtbl {
        unsafe extern "system" fn GetPackageVolumesAsync<Impl: IPackageManager4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageVolumesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager4, BASE_OFFSET>(),
            GetPackageVolumesAsync: GetPackageVolumesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager5Impl: Sized {
    fn AddPackageToVolumeAndOptionalPackagesAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndOptionalPackagesAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFamilyNameAndOptionalPackagesAsync(&self, mainpackagefamilyname: &::windows::core::HSTRING, dependencypackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn DebugSettings(&self) -> ::windows::core::Result<PackageManagerDebugSettings>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager5 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager5";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager5Vtbl {
        unsafe extern "system" fn AddPackageToVolumeAndOptionalPackagesAsync<Impl: IPackageManager5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageToVolumeAndOptionalPackagesAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&externalpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageToVolumeAndOptionalPackagesAsync<Impl: IPackageManager5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageToVolumeAndOptionalPackagesAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&externalpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<Impl: IPackageManager5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefamilynames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageByFamilyNameAndOptionalPackagesAsync(
                &*(&mainpackagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&appdatavolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DebugSettings<Impl: IPackageManager5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DebugSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager5, BASE_OFFSET>(),
            AddPackageToVolumeAndOptionalPackagesAsync: AddPackageToVolumeAndOptionalPackagesAsync::<Impl, IMPL_OFFSET>,
            StagePackageToVolumeAndOptionalPackagesAsync: StagePackageToVolumeAndOptionalPackagesAsync::<Impl, IMPL_OFFSET>,
            RegisterPackageByFamilyNameAndOptionalPackagesAsync: RegisterPackageByFamilyNameAndOptionalPackagesAsync::<Impl, IMPL_OFFSET>,
            DebugSettings: DebugSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager6Impl: Sized {
    fn ProvisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageByAppInstallerFileAsync(&self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageToVolumeAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager6 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager6";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager6Vtbl {
        unsafe extern "system" fn ProvisionPackageForAllUsersAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisionPackageForAllUsersAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackageByAppInstallerFileAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageByAppInstallerFileAsync(&*(&appinstallerfileuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), options, &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddPackageByAppInstallerFileAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddPackageByAppInstallerFileAsync(&*(&appinstallerfileuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), options, &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackageToVolumeAndRelatedSetAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageToVolumeAndRelatedSetAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                options,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&packageuristoinstall as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                &*(&relatedpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageToVolumeAndRelatedSetAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageToVolumeAndRelatedSetAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                options,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&packageuristoinstall as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                &*(&relatedpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAddPackageAsync<Impl: IPackageManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddPackageAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&relatedpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager6, BASE_OFFSET>(),
            ProvisionPackageForAllUsersAsync: ProvisionPackageForAllUsersAsync::<Impl, IMPL_OFFSET>,
            AddPackageByAppInstallerFileAsync: AddPackageByAppInstallerFileAsync::<Impl, IMPL_OFFSET>,
            RequestAddPackageByAppInstallerFileAsync: RequestAddPackageByAppInstallerFileAsync::<Impl, IMPL_OFFSET>,
            AddPackageToVolumeAndRelatedSetAsync: AddPackageToVolumeAndRelatedSetAsync::<Impl, IMPL_OFFSET>,
            StagePackageToVolumeAndRelatedSetAsync: StagePackageToVolumeAndRelatedSetAsync::<Impl, IMPL_OFFSET>,
            RequestAddPackageAsync: RequestAddPackageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager7Impl: Sized {
    fn RequestAddPackageAndRelatedSetAsync(
        &self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        deploymentoptions: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager7 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager7";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager7Vtbl {
        unsafe extern "system" fn RequestAddPackageAndRelatedSetAsync<Impl: IPackageManager7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAddPackageAndRelatedSetAsync(
                &*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                &*(&dependencypackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                deploymentoptions,
                &*(&targetvolume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType),
                &*(&optionalpackagefamilynames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType),
                &*(&relatedpackageuris as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
                &*(&packageuristoinstall as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager7, BASE_OFFSET>(),
            RequestAddPackageAndRelatedSetAsync: RequestAddPackageAndRelatedSetAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageManager8Impl: Sized {
    fn DeprovisionPackageForAllUsersAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager8 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager8";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManager8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager8Vtbl {
        unsafe extern "system" fn DeprovisionPackageForAllUsersAsync<Impl: IPackageManager8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeprovisionPackageForAllUsersAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager8, BASE_OFFSET>(),
            DeprovisionPackageForAllUsersAsync: DeprovisionPackageForAllUsersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageManager9Impl: Sized {
    fn FindProvisionedPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn AddPackageByUriAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<AddPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageByUriAsync(&self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<StagePackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByUriAsync(&self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackagesByFullNameAsync(&self, packagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING, usestub: PackageStubPreference) -> ::windows::core::Result<()>;
    fn GetPackageStubPreference(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<PackageStubPreference>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager9 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager9";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager9Vtbl {
        unsafe extern "system" fn FindProvisionedPackages<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindProvisionedPackages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPackageByUriAsync<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPackageByUriAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <AddPackageOptions as ::windows::core::Abi>::Abi as *const <AddPackageOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StagePackageByUriAsync<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StagePackageByUriAsync(&*(&packageuri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <StagePackageOptions as ::windows::core::Abi>::Abi as *const <StagePackageOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackageByUriAsync<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackageByUriAsync(&*(&manifesturi as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <RegisterPackageOptions as ::windows::core::Abi>::Abi as *const <RegisterPackageOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPackagesByFullNameAsync<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullnames: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPackagesByFullNameAsync(&*(&packagefullnames as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <RegisterPackageOptions as ::windows::core::Abi>::Abi as *const <RegisterPackageOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPackageStubPreference<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usestub: PackageStubPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageStubPreference(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usestub).into()
        }
        unsafe extern "system" fn GetPackageStubPreference<Impl: IPackageManager9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut PackageStubPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPackageStubPreference(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManager9, BASE_OFFSET>(),
            FindProvisionedPackages: FindProvisionedPackages::<Impl, IMPL_OFFSET>,
            AddPackageByUriAsync: AddPackageByUriAsync::<Impl, IMPL_OFFSET>,
            StagePackageByUriAsync: StagePackageByUriAsync::<Impl, IMPL_OFFSET>,
            RegisterPackageByUriAsync: RegisterPackageByUriAsync::<Impl, IMPL_OFFSET>,
            RegisterPackagesByFullNameAsync: RegisterPackagesByFullNameAsync::<Impl, IMPL_OFFSET>,
            SetPackageStubPreference: SetPackageStubPreference::<Impl, IMPL_OFFSET>,
            GetPackageStubPreference: GetPackageStubPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManager9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageManagerDebugSettingsImpl: Sized {
    fn SetContentGroupStateAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetContentGroupStateWithPercentageAsync(&self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManagerDebugSettings";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManagerDebugSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManagerDebugSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManagerDebugSettingsVtbl {
        unsafe extern "system" fn SetContentGroupStateAsync<Impl: IPackageManagerDebugSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContentGroupStateAsync(&*(&package as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType), &*(&contentgroupname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentGroupStateWithPercentageAsync<Impl: IPackageManagerDebugSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContentGroupStateWithPercentageAsync(&*(&package as *const <super::super::ApplicationModel::Package as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Package as ::windows::core::DefaultType>::DefaultType), &*(&contentgroupname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), state, completionpercentage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageManagerDebugSettings, BASE_OFFSET>(),
            SetContentGroupStateAsync: SetContentGroupStateAsync::<Impl, IMPL_OFFSET>,
            SetContentGroupStateWithPercentageAsync: SetContentGroupStateWithPercentageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageManagerDebugSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPackageUserInformationImpl: Sized {
    fn UserSecurityId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallState(&self) -> ::windows::core::Result<PackageInstallState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageUserInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageUserInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUserInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageUserInformationVtbl {
        unsafe extern "system" fn UserSecurityId<Impl: IPackageUserInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSecurityId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallState<Impl: IPackageUserInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageInstallState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageUserInformation, BASE_OFFSET>(),
            UserSecurityId: UserSecurityId::<Impl, IMPL_OFFSET>,
            InstallState: InstallState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageUserInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IPackageVolumeImpl: Sized {
    fn IsOffline(&self) -> ::windows::core::Result<bool>;
    fn IsSystemVolume(&self) -> ::windows::core::Result<bool>;
    fn MountPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageStorePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportsHardLinks(&self) -> ::windows::core::Result<bool>;
    fn FindPackages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyName(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesWithPackageTypes(&self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackagesTypes(&self, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByPackageFullName(&self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageVolume";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageVolumeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageVolumeVtbl {
        unsafe extern "system" fn IsOffline<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffline() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemVolume<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MountPoint<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MountPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PackageStorePath<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageStorePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsHardLinks<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsHardLinks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackages<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByNamePublisher<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByNamePublisher(&*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByPackageFamilyName<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByPackageFamilyName(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesWithPackageTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesWithPackageTypes(packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByNamePublisherWithPackagesTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByNamePublisherWithPackagesTypes(packagetypes, &*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByPackageFamilyNameWithPackageTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByPackageFamilyNameWithPackageTypes(packagetypes, &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackageByPackageFullName<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageByPackageFullName(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityId<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityId(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisher<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdNamePublisher(
                &*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyName<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdPackageFamilyName(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdWithPackageTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdWithPackageTypes(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(
                &*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                packagetypes,
                &*(&packagename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packagepublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagetypes, &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindPackageByUserSecurityIdPackageFullName<Impl: IPackageVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindPackageByUserSecurityIdPackageFullName(&*(&usersecurityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageVolume, BASE_OFFSET>(),
            IsOffline: IsOffline::<Impl, IMPL_OFFSET>,
            IsSystemVolume: IsSystemVolume::<Impl, IMPL_OFFSET>,
            MountPoint: MountPoint::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            PackageStorePath: PackageStorePath::<Impl, IMPL_OFFSET>,
            SupportsHardLinks: SupportsHardLinks::<Impl, IMPL_OFFSET>,
            FindPackages: FindPackages::<Impl, IMPL_OFFSET>,
            FindPackagesByNamePublisher: FindPackagesByNamePublisher::<Impl, IMPL_OFFSET>,
            FindPackagesByPackageFamilyName: FindPackagesByPackageFamilyName::<Impl, IMPL_OFFSET>,
            FindPackagesWithPackageTypes: FindPackagesWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByNamePublisherWithPackagesTypes: FindPackagesByNamePublisherWithPackagesTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByPackageFamilyNameWithPackageTypes: FindPackagesByPackageFamilyNameWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackageByPackageFullName: FindPackageByPackageFullName::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityId: FindPackagesByUserSecurityId::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdNamePublisher: FindPackagesByUserSecurityIdNamePublisher::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdPackageFamilyName: FindPackagesByUserSecurityIdPackageFamilyName::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdWithPackageTypes: FindPackagesByUserSecurityIdWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdNamePublisherWithPackageTypes: FindPackagesByUserSecurityIdNamePublisherWithPackageTypes::<Impl, IMPL_OFFSET>,
            FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes: FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes::<Impl, IMPL_OFFSET>,
            FindPackageByUserSecurityIdPackageFullName: FindPackageByUserSecurityIdPackageFullName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPackageVolume2Impl: Sized {
    fn IsFullTrustPackageSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAppxInstallSupported(&self) -> ::windows::core::Result<bool>;
    fn GetAvailableSpaceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageVolume2 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageVolume2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageVolume2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageVolume2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageVolume2Vtbl {
        unsafe extern "system" fn IsFullTrustPackageSupported<Impl: IPackageVolume2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFullTrustPackageSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAppxInstallSupported<Impl: IPackageVolume2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAppxInstallSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableSpaceAsync<Impl: IPackageVolume2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableSpaceAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPackageVolume2, BASE_OFFSET>(),
            IsFullTrustPackageSupported: IsFullTrustPackageSupported::<Impl, IMPL_OFFSET>,
            IsAppxInstallSupported: IsAppxInstallSupported::<Impl, IMPL_OFFSET>,
            GetAvailableSpaceAsync: GetAvailableSpaceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPackageVolume2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRegisterPackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn AppDataVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetAppDataVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IRegisterPackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRegisterPackageOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterPackageOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisterPackageOptionsVtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppDataVolume<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppDataVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppDataVolume<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppDataVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalPackageFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalLocationUri<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalLocationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExternalLocationUri<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeveloperMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeveloperMode<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceAppShutdown<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceAppShutdown<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceTargetAppShutdown<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceTargetAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceTargetAppShutdown<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceTargetAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallAllResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallAllResources<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageInPlace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageInPlace<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowUnsigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowUnsigned<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUnsigned(value).into()
        }
        unsafe extern "system" fn DeferRegistrationWhenPackagesAreInUse<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeferRegistrationWhenPackagesAreInUse() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeferRegistrationWhenPackagesAreInUse<Impl: IRegisterPackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeferRegistrationWhenPackagesAreInUse(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRegisterPackageOptions, BASE_OFFSET>(),
            DependencyPackageUris: DependencyPackageUris::<Impl, IMPL_OFFSET>,
            AppDataVolume: AppDataVolume::<Impl, IMPL_OFFSET>,
            SetAppDataVolume: SetAppDataVolume::<Impl, IMPL_OFFSET>,
            OptionalPackageFamilyNames: OptionalPackageFamilyNames::<Impl, IMPL_OFFSET>,
            ExternalLocationUri: ExternalLocationUri::<Impl, IMPL_OFFSET>,
            SetExternalLocationUri: SetExternalLocationUri::<Impl, IMPL_OFFSET>,
            DeveloperMode: DeveloperMode::<Impl, IMPL_OFFSET>,
            SetDeveloperMode: SetDeveloperMode::<Impl, IMPL_OFFSET>,
            ForceAppShutdown: ForceAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceAppShutdown: SetForceAppShutdown::<Impl, IMPL_OFFSET>,
            ForceTargetAppShutdown: ForceTargetAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceTargetAppShutdown: SetForceTargetAppShutdown::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion: ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            SetForceUpdateFromAnyVersion: SetForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            InstallAllResources: InstallAllResources::<Impl, IMPL_OFFSET>,
            SetInstallAllResources: SetInstallAllResources::<Impl, IMPL_OFFSET>,
            StageInPlace: StageInPlace::<Impl, IMPL_OFFSET>,
            SetStageInPlace: SetStageInPlace::<Impl, IMPL_OFFSET>,
            AllowUnsigned: AllowUnsigned::<Impl, IMPL_OFFSET>,
            SetAllowUnsigned: SetAllowUnsigned::<Impl, IMPL_OFFSET>,
            DeferRegistrationWhenPackagesAreInUse: DeferRegistrationWhenPackagesAreInUse::<Impl, IMPL_OFFSET>,
            SetDeferRegistrationWhenPackagesAreInUse: SetDeferRegistrationWhenPackagesAreInUse::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRegisterPackageOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISharedPackageContainerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetMembers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn RemovePackageFamily(&self, packagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<UpdateSharedPackageContainerOptions>) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
    fn ResetData(&self) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISharedPackageContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerVtbl {
        unsafe extern "system" fn Name<Impl: ISharedPackageContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: ISharedPackageContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMembers<Impl: ISharedPackageContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageFamily<Impl: ISharedPackageContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovePackageFamily(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <UpdateSharedPackageContainerOptions as ::windows::core::Abi>::Abi as *const <UpdateSharedPackageContainerOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetData<Impl: ISharedPackageContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedPackageContainer, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            GetMembers: GetMembers::<Impl, IMPL_OFFSET>,
            RemovePackageFamily: RemovePackageFamily::<Impl, IMPL_OFFSET>,
            ResetData: ResetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPackageContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISharedPackageContainerManagerImpl: Sized {
    fn CreateContainer(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<CreateSharedPackageContainerOptions>) -> ::windows::core::Result<CreateSharedPackageContainerResult>;
    fn DeleteContainer(&self, id: &::windows::core::HSTRING, options: &::core::option::Option<DeleteSharedPackageContainerOptions>) -> ::windows::core::Result<DeleteSharedPackageContainerResult>;
    fn GetContainer(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainer>;
    fn FindContainers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
    fn FindContainersWithOptions(&self, options: &::core::option::Option<FindSharedPackageContainerOptions>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISharedPackageContainerManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerManagerVtbl {
        unsafe extern "system" fn CreateContainer<Impl: ISharedPackageContainerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateContainer(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <CreateSharedPackageContainerOptions as ::windows::core::Abi>::Abi as *const <CreateSharedPackageContainerOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteContainer<Impl: ISharedPackageContainerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteContainer(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <DeleteSharedPackageContainerOptions as ::windows::core::Abi>::Abi as *const <DeleteSharedPackageContainerOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainer<Impl: ISharedPackageContainerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainer(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContainers<Impl: ISharedPackageContainerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContainers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindContainersWithOptions<Impl: ISharedPackageContainerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindContainersWithOptions(&*(&options as *const <FindSharedPackageContainerOptions as ::windows::core::Abi>::Abi as *const <FindSharedPackageContainerOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedPackageContainerManager, BASE_OFFSET>(),
            CreateContainer: CreateContainer::<Impl, IMPL_OFFSET>,
            DeleteContainer: DeleteContainer::<Impl, IMPL_OFFSET>,
            GetContainer: GetContainer::<Impl, IMPL_OFFSET>,
            FindContainers: FindContainers::<Impl, IMPL_OFFSET>,
            FindContainersWithOptions: FindContainersWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPackageContainerManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForUser(&self, usersid: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForProvisioning(&self) -> ::windows::core::Result<SharedPackageContainerManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerManagerStatics {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: ISharedPackageContainerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: ISharedPackageContainerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&usersid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForProvisioning<Impl: ISharedPackageContainerManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForProvisioning() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedPackageContainerManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
            GetForProvisioning: GetForProvisioning::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPackageContainerManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerMemberImpl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerMember";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerMemberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerMemberImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerMemberVtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: ISharedPackageContainerMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedPackageContainerMember, BASE_OFFSET>(),
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPackageContainerMember as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedPackageContainerMemberFactoryImpl: Sized {
    fn CreateInstance(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerMember>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerMemberFactory {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerMemberFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerMemberFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerMemberFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerMemberFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISharedPackageContainerMemberFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISharedPackageContainerMemberFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPackageContainerMemberFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStagePackageOptionsImpl: Sized {
    fn DependencyPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IStagePackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStagePackageOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStagePackageOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStagePackageOptionsVtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetVolume<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetVolume<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionalPackageFamilyNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OptionalPackageUris<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RelatedPackageUris<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RelatedPackageUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExternalLocationUri<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExternalLocationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExternalLocationUri<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StubPackageOption<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StubPackageOption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStubPackageOption<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStubPackageOption(value).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeveloperMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeveloperMode<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallAllResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallAllResources<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn RequiredContentGroupOnly<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiredContentGroupOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequiredContentGroupOnly<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredContentGroupOnly(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageInPlace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageInPlace<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowUnsigned() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowUnsigned<Impl: IStagePackageOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUnsigned(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStagePackageOptions, BASE_OFFSET>(),
            DependencyPackageUris: DependencyPackageUris::<Impl, IMPL_OFFSET>,
            TargetVolume: TargetVolume::<Impl, IMPL_OFFSET>,
            SetTargetVolume: SetTargetVolume::<Impl, IMPL_OFFSET>,
            OptionalPackageFamilyNames: OptionalPackageFamilyNames::<Impl, IMPL_OFFSET>,
            OptionalPackageUris: OptionalPackageUris::<Impl, IMPL_OFFSET>,
            RelatedPackageUris: RelatedPackageUris::<Impl, IMPL_OFFSET>,
            ExternalLocationUri: ExternalLocationUri::<Impl, IMPL_OFFSET>,
            SetExternalLocationUri: SetExternalLocationUri::<Impl, IMPL_OFFSET>,
            StubPackageOption: StubPackageOption::<Impl, IMPL_OFFSET>,
            SetStubPackageOption: SetStubPackageOption::<Impl, IMPL_OFFSET>,
            DeveloperMode: DeveloperMode::<Impl, IMPL_OFFSET>,
            SetDeveloperMode: SetDeveloperMode::<Impl, IMPL_OFFSET>,
            ForceUpdateFromAnyVersion: ForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            SetForceUpdateFromAnyVersion: SetForceUpdateFromAnyVersion::<Impl, IMPL_OFFSET>,
            InstallAllResources: InstallAllResources::<Impl, IMPL_OFFSET>,
            SetInstallAllResources: SetInstallAllResources::<Impl, IMPL_OFFSET>,
            RequiredContentGroupOnly: RequiredContentGroupOnly::<Impl, IMPL_OFFSET>,
            SetRequiredContentGroupOnly: SetRequiredContentGroupOnly::<Impl, IMPL_OFFSET>,
            StageInPlace: StageInPlace::<Impl, IMPL_OFFSET>,
            SetStageInPlace: SetStageInPlace::<Impl, IMPL_OFFSET>,
            AllowUnsigned: AllowUnsigned::<Impl, IMPL_OFFSET>,
            SetAllowUnsigned: SetAllowUnsigned::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStagePackageOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUpdateSharedPackageContainerOptionsImpl: Sized {
    fn ForceAppShutdown(&self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&self, value: bool) -> ::windows::core::Result<()>;
    fn RequirePackagesPresent(&self) -> ::windows::core::Result<bool>;
    fn SetRequirePackagesPresent(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IUpdateSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IUpdateSharedPackageContainerOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSharedPackageContainerOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSharedPackageContainerOptionsVtbl {
        unsafe extern "system" fn ForceAppShutdown<Impl: IUpdateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceAppShutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceAppShutdown<Impl: IUpdateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn RequirePackagesPresent<Impl: IUpdateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequirePackagesPresent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequirePackagesPresent<Impl: IUpdateSharedPackageContainerOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequirePackagesPresent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUpdateSharedPackageContainerOptions, BASE_OFFSET>(),
            ForceAppShutdown: ForceAppShutdown::<Impl, IMPL_OFFSET>,
            SetForceAppShutdown: SetForceAppShutdown::<Impl, IMPL_OFFSET>,
            RequirePackagesPresent: RequirePackagesPresent::<Impl, IMPL_OFFSET>,
            SetRequirePackagesPresent: SetRequirePackagesPresent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSharedPackageContainerOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUpdateSharedPackageContainerResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.IUpdateSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl IUpdateSharedPackageContainerResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSharedPackageContainerResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSharedPackageContainerResultVtbl {
        unsafe extern "system" fn Status<Impl: IUpdateSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IUpdateSharedPackageContainerResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUpdateSharedPackageContainerResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUpdateSharedPackageContainerResult as ::windows::core::Interface>::IID
    }
}
