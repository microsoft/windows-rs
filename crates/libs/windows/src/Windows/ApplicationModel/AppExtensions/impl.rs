#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppExtensionImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
    fn AppInfo(&mut self) -> ::windows::core::Result<super::AppInfo>;
    fn GetExtensionPropertiesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IPropertySet>>;
    fn GetPublicFolderAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFolder>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppExtension {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtension";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionVtbl {
        unsafe extern "system" fn Id<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Description<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Package<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppInfo<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetExtensionPropertiesAsync<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionPropertiesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPublicFolderAsync<Impl: IAppExtensionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublicFolderAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtension, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            AppInfo: AppInfo::<Impl, IMPL_OFFSET>,
            GetExtensionPropertiesAsync: GetExtensionPropertiesAsync::<Impl, IMPL_OFFSET>,
            GetPublicFolderAsync: GetPublicFolderAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtension2Impl: Sized {
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExtension2 {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtension2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExtension2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtension2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtension2Vtbl {
        unsafe extern "system" fn AppUserModelId<Impl: IAppExtension2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtension2, BASE_OFFSET>(), AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtension2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppExtensionCatalogImpl: Sized {
    fn FindAllAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppExtension>>>;
    fn RequestRemovePackageAsync(&mut self, packagefullname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn PackageInstalled(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageInstalled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdating(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdating(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageUninstalling(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageUninstalling(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PackageStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePackageStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppExtensionCatalog {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionCatalog";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppExtensionCatalogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionCatalogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionCatalogVtbl {
        unsafe extern "system" fn FindAllAsync<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestRemovePackageAsync<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRemovePackageAsync(&*(&packagefullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageInstalled<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageInstalled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageInstalledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageInstalled<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageInstalled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageUpdating<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageUpdating(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageUpdating<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageUpdating(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageUpdated<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageUpdated<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageUninstalling<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageUninstalling(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageUninstallingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageUninstalling<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageUninstalling(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageStatusChanged<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppExtensionCatalog, AppExtensionPackageStatusChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePackageStatusChanged<Impl: IAppExtensionCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePackageStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionCatalog, BASE_OFFSET>(),
            FindAllAsync: FindAllAsync::<Impl, IMPL_OFFSET>,
            RequestRemovePackageAsync: RequestRemovePackageAsync::<Impl, IMPL_OFFSET>,
            PackageInstalled: PackageInstalled::<Impl, IMPL_OFFSET>,
            RemovePackageInstalled: RemovePackageInstalled::<Impl, IMPL_OFFSET>,
            PackageUpdating: PackageUpdating::<Impl, IMPL_OFFSET>,
            RemovePackageUpdating: RemovePackageUpdating::<Impl, IMPL_OFFSET>,
            PackageUpdated: PackageUpdated::<Impl, IMPL_OFFSET>,
            RemovePackageUpdated: RemovePackageUpdated::<Impl, IMPL_OFFSET>,
            PackageUninstalling: PackageUninstalling::<Impl, IMPL_OFFSET>,
            RemovePackageUninstalling: RemovePackageUninstalling::<Impl, IMPL_OFFSET>,
            PackageStatusChanged: PackageStatusChanged::<Impl, IMPL_OFFSET>,
            RemovePackageStatusChanged: RemovePackageStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionCatalog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionCatalogStaticsImpl: Sized {
    fn Open(&mut self, appextensionname: &::windows::core::HSTRING) -> ::windows::core::Result<AppExtensionCatalog>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExtensionCatalogStatics {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionCatalogStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExtensionCatalogStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionCatalogStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionCatalogStaticsVtbl {
        unsafe extern "system" fn Open<Impl: IAppExtensionCatalogStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appextensionname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&appextensionname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionCatalogStatics, BASE_OFFSET>(), Open: Open::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionCatalogStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppExtensionPackageInstalledEventArgsImpl: Sized {
    fn AppExtensionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
    fn Extensions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppExtensionPackageInstalledEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionPackageInstalledEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppExtensionPackageInstalledEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionPackageInstalledEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionPackageInstalledEventArgsVtbl {
        unsafe extern "system" fn AppExtensionName<Impl: IAppExtensionPackageInstalledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IAppExtensionPackageInstalledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Extensions<Impl: IAppExtensionPackageInstalledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionPackageInstalledEventArgs, BASE_OFFSET>(),
            AppExtensionName: AppExtensionName::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Extensions: Extensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionPackageInstalledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageStatusChangedEventArgsImpl: Sized {
    fn AppExtensionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExtensionPackageStatusChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionPackageStatusChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExtensionPackageStatusChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionPackageStatusChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionPackageStatusChangedEventArgsVtbl {
        unsafe extern "system" fn AppExtensionName<Impl: IAppExtensionPackageStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IAppExtensionPackageStatusChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionPackageStatusChangedEventArgs, BASE_OFFSET>(),
            AppExtensionName: AppExtensionName::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionPackageStatusChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageUninstallingEventArgsImpl: Sized {
    fn AppExtensionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExtensionPackageUninstallingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUninstallingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExtensionPackageUninstallingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionPackageUninstallingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionPackageUninstallingEventArgsVtbl {
        unsafe extern "system" fn AppExtensionName<Impl: IAppExtensionPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IAppExtensionPackageUninstallingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionPackageUninstallingEventArgs, BASE_OFFSET>(),
            AppExtensionName: AppExtensionName::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionPackageUninstallingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppExtensionPackageUpdatedEventArgsImpl: Sized {
    fn AppExtensionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
    fn Extensions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppExtension>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppExtensionPackageUpdatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppExtensionPackageUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionPackageUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionPackageUpdatedEventArgsVtbl {
        unsafe extern "system" fn AppExtensionName<Impl: IAppExtensionPackageUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IAppExtensionPackageUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Extensions<Impl: IAppExtensionPackageUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionPackageUpdatedEventArgs, BASE_OFFSET>(),
            AppExtensionName: AppExtensionName::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
            Extensions: Extensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionPackageUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppExtensionPackageUpdatingEventArgsImpl: Sized {
    fn AppExtensionName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Package(&mut self) -> ::windows::core::Result<super::Package>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppExtensionPackageUpdatingEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppExtensions.IAppExtensionPackageUpdatingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppExtensionPackageUpdatingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppExtensionPackageUpdatingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppExtensionPackageUpdatingEventArgsVtbl {
        unsafe extern "system" fn AppExtensionName<Impl: IAppExtensionPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppExtensionName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Package<Impl: IAppExtensionPackageUpdatingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppExtensionPackageUpdatingEventArgs, BASE_OFFSET>(),
            AppExtensionName: AppExtensionName::<Impl, IMPL_OFFSET>,
            Package: Package::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppExtensionPackageUpdatingEventArgs as ::windows::core::Interface>::IID
    }
}
