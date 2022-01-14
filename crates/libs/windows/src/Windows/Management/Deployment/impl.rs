#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAddPackageOptions_Impl: Sized {
    fn DependencyPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&mut self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&mut self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&mut self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&mut self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&mut self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&mut self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RetainFilesOnFailure(&mut self) -> ::windows::core::Result<bool>;
    fn SetRetainFilesOnFailure(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&mut self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&mut self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAddPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IAddPackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAddPackageOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddPackageOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddPackageOptions_Vtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetVolume<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetVolume<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OptionalPackageUris<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RelatedPackageUris<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExternalLocationUri<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExternalLocationUri<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StubPackageOption<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStubPackageOption<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStubPackageOption(value).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeveloperMode<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceAppShutdown<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceAppShutdown<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceTargetAppShutdown<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceTargetAppShutdown<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceTargetAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallAllResources<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn RequiredContentGroupOnly<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequiredContentGroupOnly<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredContentGroupOnly(value).into()
        }
        unsafe extern "system" fn RetainFilesOnFailure<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRetainFilesOnFailure<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetainFilesOnFailure(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStageInPlace<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowUnsigned<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUnsigned(value).into()
        }
        unsafe extern "system" fn DeferRegistrationWhenPackagesAreInUse<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeferRegistrationWhenPackagesAreInUse<Impl: IAddPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IAppInstallerManager_Impl: Sized {
    fn SetAutoUpdateSettings(&mut self, packagefamilyname: &::windows::core::HSTRING, appinstallerinfo: &::core::option::Option<AutoUpdateSettingsOptions>) -> ::windows::core::Result<()>;
    fn ClearAutoUpdateSettings(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseAutoUpdatesUntil(&mut self, packagefamilyname: &::windows::core::HSTRING, datetime: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallerManager {
    const NAME: &'static str = "Windows.Management.Deployment.IAppInstallerManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallerManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerManager_Vtbl {
        unsafe extern "system" fn SetAutoUpdateSettings<Impl: IAppInstallerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, appinstallerinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoUpdateSettings(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&appinstallerinfo as *const <AutoUpdateSettingsOptions as ::windows::core::Abi>::Abi as *const <AutoUpdateSettingsOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearAutoUpdateSettings<Impl: IAppInstallerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearAutoUpdateSettings(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseAutoUpdatesUntil<Impl: IAppInstallerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, datetime: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
pub trait IAppInstallerManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<AppInstallerManager>;
    fn GetForSystem(&mut self) -> ::windows::core::Result<AppInstallerManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallerManagerStatics {
    const NAME: &'static str = "Windows.Management.Deployment.IAppInstallerManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallerManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallerManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallerManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: IAppInstallerManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForSystem<Impl: IAppInstallerManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutoUpdateSettingsOptions_Impl: Sized {
    fn Version(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::PackageVersion>;
    fn SetVersion(&mut self, value: &super::super::ApplicationModel::PackageVersion) -> ::windows::core::Result<()>;
    fn AppInstallerUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetAppInstallerUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn OnLaunch(&mut self) -> ::windows::core::Result<bool>;
    fn SetOnLaunch(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn HoursBetweenUpdateChecks(&mut self) -> ::windows::core::Result<u32>;
    fn SetHoursBetweenUpdateChecks(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ShowPrompt(&mut self) -> ::windows::core::Result<bool>;
    fn SetShowPrompt(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateBlocksActivation(&mut self) -> ::windows::core::Result<bool>;
    fn SetUpdateBlocksActivation(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutomaticBackgroundTask(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutomaticBackgroundTask(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAutoRepairEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAutoRepairEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RepairUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn DependencyPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn OptionalPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoUpdateSettingsOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IAutoUpdateSettingsOptions";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutoUpdateSettingsOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoUpdateSettingsOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoUpdateSettingsOptions_Vtbl {
        unsafe extern "system" fn Version<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVersion<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::PackageVersion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(&*(&value as *const <super::super::ApplicationModel::PackageVersion as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::PackageVersion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppInstallerUri<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppInstallerUri<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppInstallerUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLaunch<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOnLaunch<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOnLaunch(value).into()
        }
        unsafe extern "system" fn HoursBetweenUpdateChecks<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHoursBetweenUpdateChecks<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHoursBetweenUpdateChecks(value).into()
        }
        unsafe extern "system" fn ShowPrompt<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShowPrompt<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowPrompt(value).into()
        }
        unsafe extern "system" fn UpdateBlocksActivation<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUpdateBlocksActivation<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateBlocksActivation(value).into()
        }
        unsafe extern "system" fn AutomaticBackgroundTask<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomaticBackgroundTask<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticBackgroundTask(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn IsAutoRepairEnabled<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsAutoRepairEnabled<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAutoRepairEnabled(value).into()
        }
        unsafe extern "system" fn UpdateUris<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RepairUris<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DependencyPackageUris<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OptionalPackageUris<Impl: IAutoUpdateSettingsOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IAutoUpdateSettingsOptionsStatics_Impl: Sized {
    fn CreateFromAppInstallerInfo(&mut self, appinstallerinfo: &::core::option::Option<super::super::ApplicationModel::AppInstallerInfo>) -> ::windows::core::Result<AutoUpdateSettingsOptions>;
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoUpdateSettingsOptionsStatics {
    const NAME: &'static str = "Windows.Management.Deployment.IAutoUpdateSettingsOptionsStatics";
}
#[cfg(all(feature = "ApplicationModel", feature = "implement_exclusive"))]
impl IAutoUpdateSettingsOptionsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoUpdateSettingsOptionsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoUpdateSettingsOptionsStatics_Vtbl {
        unsafe extern "system" fn CreateFromAppInstallerInfo<Impl: IAutoUpdateSettingsOptionsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICreateSharedPackageContainerOptions_Impl: Sized {
    fn Members(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn ForceAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CreateCollisionOption(&mut self) -> ::windows::core::Result<SharedPackageContainerCreationCollisionOptions>;
    fn SetCreateCollisionOption(&mut self, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICreateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.ICreateSharedPackageContainerOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ICreateSharedPackageContainerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateSharedPackageContainerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateSharedPackageContainerOptions_Vtbl {
        unsafe extern "system" fn Members<Impl: ICreateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ForceAppShutdown<Impl: ICreateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceAppShutdown<Impl: ICreateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn CreateCollisionOption<Impl: ICreateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCreateCollisionOption<Impl: ICreateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SharedPackageContainerCreationCollisionOptions) -> ::windows::core::HRESULT {
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
pub trait ICreateSharedPackageContainerResult_Impl: Sized {
    fn Container(&mut self) -> ::windows::core::Result<SharedPackageContainer>;
    fn Status(&mut self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICreateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.ICreateSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl ICreateSharedPackageContainerResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateSharedPackageContainerResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateSharedPackageContainerResult_Vtbl {
        unsafe extern "system" fn Container<Impl: ICreateSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: ICreateSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: ICreateSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
pub trait IDeleteSharedPackageContainerOptions_Impl: Sized {
    fn ForceAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllUsers(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllUsers(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeleteSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IDeleteSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IDeleteSharedPackageContainerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteSharedPackageContainerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeleteSharedPackageContainerOptions_Vtbl {
        unsafe extern "system" fn ForceAppShutdown<Impl: IDeleteSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceAppShutdown<Impl: IDeleteSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn AllUsers<Impl: IDeleteSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllUsers<Impl: IDeleteSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IDeleteSharedPackageContainerResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeleteSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.IDeleteSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeleteSharedPackageContainerResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteSharedPackageContainerResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeleteSharedPackageContainerResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IDeleteSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IDeleteSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
pub trait IDeploymentResult_Impl: Sized {
    fn ErrorText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActivityId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ExtendedErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeploymentResult {
    const NAME: &'static str = "Windows.Management.Deployment.IDeploymentResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDeploymentResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeploymentResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeploymentResult_Vtbl {
        unsafe extern "system" fn ErrorText<Impl: IDeploymentResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivityId<Impl: IDeploymentResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedErrorCode<Impl: IDeploymentResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
pub trait IDeploymentResult2_Impl: Sized {
    fn IsRegistered(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeploymentResult2 {
    const NAME: &'static str = "Windows.Management.Deployment.IDeploymentResult2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeploymentResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeploymentResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeploymentResult2_Vtbl {
        unsafe extern "system" fn IsRegistered<Impl: IDeploymentResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IFindSharedPackageContainerOptions_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFindSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IFindSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFindSharedPackageContainerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindSharedPackageContainerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindSharedPackageContainerOptions_Vtbl {
        unsafe extern "system" fn Name<Impl: IFindSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetName<Impl: IFindSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IFindSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPackageFamilyName<Impl: IFindSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IPackageAllUserProvisioningOptions_Impl: Sized {
    fn OptionalPackageFamilyNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProjectionOrderPackageFamilyNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageAllUserProvisioningOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageAllUserProvisioningOptions";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageAllUserProvisioningOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageAllUserProvisioningOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageAllUserProvisioningOptions_Vtbl {
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IPackageAllUserProvisioningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProjectionOrderPackageFamilyNames<Impl: IPackageAllUserProvisioningOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager_Impl: Sized {
    fn AddPackageAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn UpdatePackageAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageAsync(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageAsync(&mut self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&mut self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&mut self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&mut self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindUsers(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageUserInformation>>;
    fn SetPackageState(&mut self, packagefullname: &::windows::core::HSTRING, packagestate: PackageState) -> ::windows::core::Result<()>;
    fn FindPackageByPackageFullName(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
    fn CleanupPackageForUserAsync(&mut self, packagename: &::windows::core::HSTRING, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesByPackageFamilyName(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&mut self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&mut self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::ApplicationModel::Package>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager_Vtbl {
        unsafe extern "system" fn AddPackageAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdatePackageAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePackageAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterPackageAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackages<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityId<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByNamePublisher<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisher<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindUsers<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPackageState<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagestate: PackageState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageState(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), packagestate).into()
        }
        unsafe extern "system" fn FindPackageByPackageFullName<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CleanupPackageForUserAsync<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByPackageFamilyName<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyName<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackageByUserSecurityIdPackageFullName<Impl: IPackageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager10_Impl: Sized {
    fn ProvisionPackageForAllUsersWithOptionsAsync(&mut self, mainpackagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<PackageAllUserProvisioningOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager10 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager10";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManager10_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager10_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager10_Vtbl {
        unsafe extern "system" fn ProvisionPackageForAllUsersWithOptionsAsync<Impl: IPackageManager10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager2_Impl: Sized {
    fn RemovePackageWithOptionsAsync(&mut self, packagefullname: &::windows::core::HSTRING, removaloptions: RemovalOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageWithOptionsAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFullNameAsync(&mut self, mainpackagefullname: &::windows::core::HSTRING, dependencypackagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackagesWithPackageTypes(&mut self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackageTypes(&mut self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&mut self, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::ApplicationModel::Package>>;
    fn StageUserDataAsync(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager2 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager2_Vtbl {
        unsafe extern "system" fn RemovePackageWithOptionsAsync<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, removaloptions: RemovalOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageWithOptionsAsync<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterPackageByFullNameAsync<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefullnames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByNamePublisherWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByPackageFamilyNameWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StageUserDataAsync<Impl: IPackageManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager3_Impl: Sized {
    fn AddPackageVolumeAsync(&mut self, packagestorepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PackageVolume>>;
    fn AddPackageToVolumeAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn ClearPackageStatus(&mut self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn RegisterPackageWithAppDataVolumeAsync(&mut self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn FindPackageVolumeByName(&mut self, volumename: &::windows::core::HSTRING) -> ::windows::core::Result<PackageVolume>;
    fn FindPackageVolumes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<PackageVolume>>;
    fn GetDefaultPackageVolume(&mut self) -> ::windows::core::Result<PackageVolume>;
    fn MovePackageToVolumeAsync(&mut self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RemovePackageVolumeAsync(&mut self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetDefaultPackageVolume(&mut self, volume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn SetPackageStatus(&mut self, packagefullname: &::windows::core::HSTRING, status: PackageStatus) -> ::windows::core::Result<()>;
    fn SetPackageVolumeOfflineAsync(&mut self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageVolumeOnlineAsync(&mut self, packagevolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StageUserDataWithOptionsAsync(&mut self, packagefullname: &::windows::core::HSTRING, deploymentoptions: DeploymentOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager3 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager3_Vtbl {
        unsafe extern "system" fn AddPackageVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagestorepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddPackageToVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearPackageStatus<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearPackageStatus(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), status).into()
        }
        unsafe extern "system" fn RegisterPackageWithAppDataVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackageVolumeByName<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volumename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackageVolumes<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDefaultPackageVolume<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MovePackageToVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePackageVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDefaultPackageVolume<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultPackageVolume(&*(&volume as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPackageStatus<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, status: PackageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageStatus(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), status).into()
        }
        unsafe extern "system" fn SetPackageVolumeOfflineAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPackageVolumeOnlineAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagevolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageToVolumeAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StageUserDataWithOptionsAsync<Impl: IPackageManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deploymentoptions: DeploymentOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager4_Impl: Sized {
    fn GetPackageVolumesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PackageVolume>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager4 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager4";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager4_Vtbl {
        unsafe extern "system" fn GetPackageVolumesAsync<Impl: IPackageManager4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager5_Impl: Sized {
    fn AddPackageToVolumeAndOptionalPackagesAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndOptionalPackagesAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, externalpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByFamilyNameAndOptionalPackagesAsync(&mut self, mainpackagefamilyname: &::windows::core::HSTRING, dependencypackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, deploymentoptions: DeploymentOptions, appdatavolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn DebugSettings(&mut self) -> ::windows::core::Result<PackageManagerDebugSettings>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager5 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager5";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager5_Vtbl {
        unsafe extern "system" fn AddPackageToVolumeAndOptionalPackagesAsync<Impl: IPackageManager5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageToVolumeAndOptionalPackagesAsync<Impl: IPackageManager5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, externalpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterPackageByFamilyNameAndOptionalPackagesAsync<Impl: IPackageManager5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainpackagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dependencypackagefamilynames: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, appdatavolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DebugSettings<Impl: IPackageManager5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager6_Impl: Sized {
    fn ProvisionPackageForAllUsersAsync(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageByAppInstallerFileAsync(&mut self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageByAppInstallerFileAsync(&mut self, appinstallerfileuri: &::core::option::Option<super::super::Foundation::Uri>, options: AddPackageByAppInstallerOptions, targetvolume: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn AddPackageToVolumeAndRelatedSetAsync(
        &mut self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageToVolumeAndRelatedSetAsync(
        &mut self,
        packageuri: &::core::option::Option<super::super::Foundation::Uri>,
        dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        options: DeploymentOptions,
        targetvolume: &::core::option::Option<PackageVolume>,
        optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
        packageuristoinstall: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
        relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RequestAddPackageAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, dependencypackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>, deploymentoptions: DeploymentOptions, targetvolume: &::core::option::Option<PackageVolume>, optionalpackagefamilynames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, relatedpackageuris: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager6 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager6";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager6_Vtbl {
        unsafe extern "system" fn ProvisionPackageForAllUsersAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddPackageByAppInstallerFileAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAddPackageByAppInstallerFileAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appinstallerfileuri: ::windows::core::RawPtr, options: AddPackageByAppInstallerOptions, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddPackageToVolumeAndRelatedSetAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageToVolumeAndRelatedSetAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, options: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAddPackageAsync<Impl: IPackageManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager7_Impl: Sized {
    fn RequestAddPackageAndRelatedSetAsync(
        &mut self,
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
impl IPackageManager7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager7_Vtbl {
        unsafe extern "system" fn RequestAddPackageAndRelatedSetAsync<Impl: IPackageManager7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, dependencypackageuris: ::windows::core::RawPtr, deploymentoptions: DeploymentOptions, targetvolume: ::windows::core::RawPtr, optionalpackagefamilynames: ::windows::core::RawPtr, relatedpackageuris: ::windows::core::RawPtr, packageuristoinstall: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager8_Impl: Sized {
    fn DeprovisionPackageForAllUsersAsync(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager8 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager8";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManager8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager8_Vtbl {
        unsafe extern "system" fn DeprovisionPackageForAllUsersAsync<Impl: IPackageManager8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageManager9_Impl: Sized {
    fn FindProvisionedPackages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn AddPackageByUriAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<AddPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn StagePackageByUriAsync(&mut self, packageuri: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<StagePackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackageByUriAsync(&mut self, manifesturi: &::core::option::Option<super::super::Foundation::Uri>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn RegisterPackagesByFullNameAsync(&mut self, packagefullnames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, options: &::core::option::Option<RegisterPackageOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>>;
    fn SetPackageStubPreference(&mut self, packagefamilyname: &::windows::core::HSTRING, usestub: PackageStubPreference) -> ::windows::core::Result<()>;
    fn GetPackageStubPreference(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<PackageStubPreference>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManager9 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManager9";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageManager9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManager9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManager9_Vtbl {
        unsafe extern "system" fn FindProvisionedPackages<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddPackageByUriAsync<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StagePackageByUriAsync<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packageuri: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterPackageByUriAsync<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, manifesturi: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterPackagesByFullNameAsync<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullnames: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPackageStubPreference<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usestub: PackageStubPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageStubPreference(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usestub).into()
        }
        unsafe extern "system" fn GetPackageStubPreference<Impl: IPackageManager9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut PackageStubPreference) -> ::windows::core::HRESULT {
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
pub trait IPackageManagerDebugSettings_Impl: Sized {
    fn SetContentGroupStateAsync(&mut self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetContentGroupStateWithPercentageAsync(&mut self, package: &::core::option::Option<super::super::ApplicationModel::Package>, contentgroupname: &::windows::core::HSTRING, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageManagerDebugSettings {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageManagerDebugSettings";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageManagerDebugSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageManagerDebugSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageManagerDebugSettings_Vtbl {
        unsafe extern "system" fn SetContentGroupStateAsync<Impl: IPackageManagerDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentGroupStateWithPercentageAsync<Impl: IPackageManagerDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, package: ::windows::core::RawPtr, contentgroupname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, state: super::super::ApplicationModel::PackageContentGroupState, completionpercentage: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageUserInformation_Impl: Sized {
    fn UserSecurityId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallState(&mut self) -> ::windows::core::Result<PackageInstallState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPackageUserInformation {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageUserInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IPackageUserInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageUserInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageUserInformation_Vtbl {
        unsafe extern "system" fn UserSecurityId<Impl: IPackageUserInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstallState<Impl: IPackageUserInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PackageInstallState) -> ::windows::core::HRESULT {
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
pub trait IPackageVolume_Impl: Sized {
    fn IsOffline(&mut self) -> ::windows::core::Result<bool>;
    fn IsSystemVolume(&mut self) -> ::windows::core::Result<bool>;
    fn MountPoint(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageStorePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportsHardLinks(&mut self) -> ::windows::core::Result<bool>;
    fn FindPackages(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisher(&mut self, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyName(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesWithPackageTypes(&mut self, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByNamePublisherWithPackagesTypes(&mut self, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByPackageFamilyNameWithPackageTypes(&mut self, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByPackageFullName(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityId(&mut self, usersecurityid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisher(&mut self, usersecurityid: &::windows::core::HSTRING, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyName(&mut self, usersecurityid: &::windows::core::HSTRING, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdWithPackageTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagename: &::windows::core::HSTRING, packagepublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&mut self, usersecurityid: &::windows::core::HSTRING, packagetypes: PackageTypes, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
    fn FindPackageByUserSecurityIdPackageFullName(&mut self, usersecurityid: &::windows::core::HSTRING, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::ApplicationModel::Package>>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageVolume {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageVolume";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IPackageVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageVolume_Vtbl {
        unsafe extern "system" fn IsOffline<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSystemVolume<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MountPoint<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PackageStorePath<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportsHardLinks<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackages<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByNamePublisher<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByPackageFamilyName<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesWithPackageTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByNamePublisherWithPackagesTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByPackageFamilyNameWithPackageTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackageByPackageFullName<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityId<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisher<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyName<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdWithPackageTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagepublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagetypes: PackageTypes, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindPackageByUserSecurityIdPackageFullName<Impl: IPackageVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersecurityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPackageVolume2_Impl: Sized {
    fn IsFullTrustPackageSupported(&mut self) -> ::windows::core::Result<bool>;
    fn IsAppxInstallSupported(&mut self) -> ::windows::core::Result<bool>;
    fn GetAvailableSpaceAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u64>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPackageVolume2 {
    const NAME: &'static str = "Windows.Management.Deployment.IPackageVolume2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPackageVolume2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPackageVolume2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPackageVolume2_Vtbl {
        unsafe extern "system" fn IsFullTrustPackageSupported<Impl: IPackageVolume2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAppxInstallSupported<Impl: IPackageVolume2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAvailableSpaceAsync<Impl: IPackageVolume2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRegisterPackageOptions_Impl: Sized {
    fn DependencyPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn AppDataVolume(&mut self) -> ::windows::core::Result<PackageVolume>;
    fn SetAppDataVolume(&mut self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ExternalLocationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn DeveloperMode(&mut self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceTargetAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceTargetAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&mut self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&mut self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DeferRegistrationWhenPackagesAreInUse(&mut self) -> ::windows::core::Result<bool>;
    fn SetDeferRegistrationWhenPackagesAreInUse(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRegisterPackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IRegisterPackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRegisterPackageOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRegisterPackageOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRegisterPackageOptions_Vtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppDataVolume<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppDataVolume<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppDataVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExternalLocationUri<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExternalLocationUri<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeveloperMode<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceAppShutdown<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceAppShutdown<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceTargetAppShutdown<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceTargetAppShutdown<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceTargetAppShutdown(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallAllResources<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStageInPlace<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowUnsigned<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowUnsigned(value).into()
        }
        unsafe extern "system" fn DeferRegistrationWhenPackagesAreInUse<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeferRegistrationWhenPackagesAreInUse<Impl: IRegisterPackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ISharedPackageContainer_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetMembers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainerMember>>;
    fn RemovePackageFamily(&mut self, packagefamilyname: &::windows::core::HSTRING, options: &::core::option::Option<UpdateSharedPackageContainerOptions>) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
    fn ResetData(&mut self) -> ::windows::core::Result<UpdateSharedPackageContainerResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISharedPackageContainer {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISharedPackageContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainer_Vtbl {
        unsafe extern "system" fn Name<Impl: ISharedPackageContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: ISharedPackageContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMembers<Impl: ISharedPackageContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemovePackageFamily<Impl: ISharedPackageContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResetData<Impl: ISharedPackageContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISharedPackageContainerManager_Impl: Sized {
    fn CreateContainer(&mut self, name: &::windows::core::HSTRING, options: &::core::option::Option<CreateSharedPackageContainerOptions>) -> ::windows::core::Result<CreateSharedPackageContainerResult>;
    fn DeleteContainer(&mut self, id: &::windows::core::HSTRING, options: &::core::option::Option<DeleteSharedPackageContainerOptions>) -> ::windows::core::Result<DeleteSharedPackageContainerResult>;
    fn GetContainer(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainer>;
    fn FindContainers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
    fn FindContainersWithOptions(&mut self, options: &::core::option::Option<FindSharedPackageContainerOptions>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SharedPackageContainer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISharedPackageContainerManager {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerManager";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISharedPackageContainerManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerManager_Vtbl {
        unsafe extern "system" fn CreateContainer<Impl: ISharedPackageContainerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteContainer<Impl: ISharedPackageContainerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetContainer<Impl: ISharedPackageContainerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindContainers<Impl: ISharedPackageContainerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindContainersWithOptions<Impl: ISharedPackageContainerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISharedPackageContainerManagerStatics_Impl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForUser(&mut self, usersid: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerManager>;
    fn GetForProvisioning(&mut self) -> ::windows::core::Result<SharedPackageContainerManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerManagerStatics {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerManagerStatics_Vtbl {
        unsafe extern "system" fn GetDefault<Impl: ISharedPackageContainerManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForUser<Impl: ISharedPackageContainerManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usersid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetForProvisioning<Impl: ISharedPackageContainerManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISharedPackageContainerMember_Impl: Sized {
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerMember {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerMember";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerMember_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerMember_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerMember_Vtbl {
        unsafe extern "system" fn PackageFamilyName<Impl: ISharedPackageContainerMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ISharedPackageContainerMemberFactory_Impl: Sized {
    fn CreateInstance(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<SharedPackageContainerMember>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISharedPackageContainerMemberFactory {
    const NAME: &'static str = "Windows.Management.Deployment.ISharedPackageContainerMemberFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISharedPackageContainerMemberFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPackageContainerMemberFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPackageContainerMemberFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISharedPackageContainerMemberFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IStagePackageOptions_Impl: Sized {
    fn DependencyPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn TargetVolume(&mut self) -> ::windows::core::Result<PackageVolume>;
    fn SetTargetVolume(&mut self, value: &::core::option::Option<PackageVolume>) -> ::windows::core::Result<()>;
    fn OptionalPackageFamilyNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn OptionalPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn RelatedPackageUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn ExternalLocationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetExternalLocationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn StubPackageOption(&mut self) -> ::windows::core::Result<StubPackageOption>;
    fn SetStubPackageOption(&mut self, value: StubPackageOption) -> ::windows::core::Result<()>;
    fn DeveloperMode(&mut self) -> ::windows::core::Result<bool>;
    fn SetDeveloperMode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ForceUpdateFromAnyVersion(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceUpdateFromAnyVersion(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InstallAllResources(&mut self) -> ::windows::core::Result<bool>;
    fn SetInstallAllResources(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RequiredContentGroupOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequiredContentGroupOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StageInPlace(&mut self) -> ::windows::core::Result<bool>;
    fn SetStageInPlace(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowUnsigned(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowUnsigned(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStagePackageOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IStagePackageOptions";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStagePackageOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStagePackageOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStagePackageOptions_Vtbl {
        unsafe extern "system" fn DependencyPackageUris<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetVolume<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetVolume<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <PackageVolume as ::windows::core::Abi>::Abi as *const <PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionalPackageFamilyNames<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OptionalPackageUris<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RelatedPackageUris<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExternalLocationUri<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExternalLocationUri<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExternalLocationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StubPackageOption<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StubPackageOption) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStubPackageOption<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StubPackageOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStubPackageOption(value).into()
        }
        unsafe extern "system" fn DeveloperMode<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeveloperMode<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeveloperMode(value).into()
        }
        unsafe extern "system" fn ForceUpdateFromAnyVersion<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUpdateFromAnyVersion<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUpdateFromAnyVersion(value).into()
        }
        unsafe extern "system" fn InstallAllResources<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallAllResources<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallAllResources(value).into()
        }
        unsafe extern "system" fn RequiredContentGroupOnly<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequiredContentGroupOnly<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiredContentGroupOnly(value).into()
        }
        unsafe extern "system" fn StageInPlace<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStageInPlace<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageInPlace(value).into()
        }
        unsafe extern "system" fn AllowUnsigned<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowUnsigned<Impl: IStagePackageOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IUpdateSharedPackageContainerOptions_Impl: Sized {
    fn ForceAppShutdown(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceAppShutdown(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn RequirePackagesPresent(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequirePackagesPresent(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUpdateSharedPackageContainerOptions {
    const NAME: &'static str = "Windows.Management.Deployment.IUpdateSharedPackageContainerOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IUpdateSharedPackageContainerOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSharedPackageContainerOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSharedPackageContainerOptions_Vtbl {
        unsafe extern "system" fn ForceAppShutdown<Impl: IUpdateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceAppShutdown<Impl: IUpdateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceAppShutdown(value).into()
        }
        unsafe extern "system" fn RequirePackagesPresent<Impl: IUpdateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequirePackagesPresent<Impl: IUpdateSharedPackageContainerOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IUpdateSharedPackageContainerResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SharedPackageContainerOperationStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUpdateSharedPackageContainerResult {
    const NAME: &'static str = "Windows.Management.Deployment.IUpdateSharedPackageContainerResult";
}
#[cfg(feature = "implement_exclusive")]
impl IUpdateSharedPackageContainerResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUpdateSharedPackageContainerResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUpdateSharedPackageContainerResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IUpdateSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SharedPackageContainerOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IUpdateSharedPackageContainerResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
